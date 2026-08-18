use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

struct BuildOptions {
    debug: bool,
    endstone_api: String,
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .and_then(|path| path.parent())
        .expect("xtask must live below rust/crates/ in the workspace root")
        .to_owned()
}

fn run(command: &mut Command) -> Result<(), String> {
    let status = command
        .status()
        .map_err(|error| format!("failed to start {:?}: {error}", command))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("command failed with {status}: {:?}", command))
    }
}

fn target_dir(root: &Path) -> PathBuf {
    match env::var_os("CARGO_TARGET_DIR") {
        Some(path) => {
            let path = PathBuf::from(path);
            if path.is_absolute() {
                path
            } else {
                root.join(path)
            }
        }
        None => root.join("target"),
    }
}

fn parse_build_options(args: impl IntoIterator<Item = OsString>) -> Result<BuildOptions, String> {
    let mut options = BuildOptions {
        debug: false,
        endstone_api: "0.11".to_owned(),
    };
    let mut args = args.into_iter();

    while let Some(argument) = args.next() {
        match argument.to_string_lossy().as_ref() {
            "--debug" => options.debug = true,
            "--endstone-api" => {
                let Some(version) = args.next() else {
                    return Err("--endstone-api requires a version".to_owned());
                };
                options.endstone_api = version
                    .into_string()
                    .map_err(|_| "--endstone-api must be UTF-8".to_owned())?;
            }
            value => return Err(format!("unknown build option: {value}")),
        }
    }

    Ok(options)
}

fn build(options: BuildOptions) -> Result<(), String> {
    let root = workspace_root();
    let profile = if options.debug { "debug" } else { "release" };

    let mut cargo = Command::new("cargo");
    cargo
        .current_dir(&root)
        .args(["build", "--package", "aegilex-runtime"]);
    if !options.debug {
        cargo.arg("--release");
    }
    run(&mut cargo)?;

    let runtime_name = if cfg!(windows) {
        "aegilex_runtime.lib"
    } else {
        "libaegilex_runtime.a"
    };
    let runtime_library = target_dir(&root).join(profile).join(runtime_name);
    if !runtime_library.is_file() {
        return Err(format!(
            "Cargo did not produce {}",
            runtime_library.display()
        ));
    }
    let cxx_include = cargo_build_out(&target_dir(&root), profile)?
        .join("cxxbridge")
        .join("include");

    let native_source = root.join("native");
    let native_build = target_dir(&root).join("aegilex").join(profile);
    let mut configure = Command::new("cmake");
    configure
        .current_dir(&root)
        .args(["-S"])
        .arg(native_source)
        .args(["-B"])
        .arg(&native_build)
        .args(["-G", "Ninja"])
        .arg(format!(
            "-DAEGILEX_RUNTIME_LIBRARY={}",
            runtime_library.display()
        ))
        .arg(format!(
            "-DAEGILEX_RUNTIME_CXX_INCLUDE={}",
            cxx_include.display()
        ))
        .arg(format!("-DENDSTONE_API_VERSION={}", options.endstone_api))
        .arg(format!(
            "-DCMAKE_BUILD_TYPE={}",
            if options.debug { "Debug" } else { "Release" }
        ))
        .arg("-DCMAKE_EXPORT_COMPILE_COMMANDS=ON");

    if cfg!(windows) {
        configure
            .arg("-DCMAKE_C_COMPILER=clang-cl")
            .arg("-DCMAKE_CXX_COMPILER=clang-cl")
            .env("CC", "clang-cl")
            .env("CXX", "clang-cl");
    }
    run(&mut configure)?;

    let mut native = Command::new("cmake");
    native
        .current_dir(&root)
        .args(["--build"])
        .arg(native_build);
    run(&mut native)
}

fn cargo_build_out(target: &Path, profile: &str) -> Result<PathBuf, String> {
    let build_dir = target.join(profile).join("build");
    let mut matches = Vec::new();
    for entry in std::fs::read_dir(&build_dir)
        .map_err(|error| format!("cannot read {}: {error}", build_dir.display()))?
    {
        let entry = entry.map_err(|error| format!("cannot read build entry: {error}"))?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if file_name.starts_with("aegilex-runtime-") {
            let out = entry.path().join("out");
            let include = out.join("cxxbridge").join("include");
            let bridge = include.join("aegilex-runtime/src/cxx_host_inventory.rs.h");
            let admin_bridge = include.join("aegilex-runtime/src/cxx_host_admin.rs.h");
            let has_current_inventory_bridge = std::fs::read_to_string(&bridge)
                .is_ok_and(|source| source.contains("struct NbtNode;"));
            let has_current_admin_bridge = std::fs::read_to_string(&admin_bridge)
                .is_ok_and(|source| !source.contains("bindings/endstone/admin.h"));
            if include.is_dir() && has_current_inventory_bridge && has_current_admin_bridge {
                let modified = std::fs::metadata(&include)
                    .and_then(|metadata| metadata.modified())
                    .map_err(|error| format!("cannot inspect {}: {error}", include.display()))?;
                matches.push((modified, out));
            }
        }
    }
    matches.sort_by_key(|(modified, _)| *modified);
    matches
        .into_iter()
        .max_by_key(|(modified, _)| *modified)
        .map(|(_, out)| out)
        .ok_or_else(|| {
            "Cargo did not produce current cxx bridge output for aegilex-runtime".to_owned()
        })
}

fn check_abi() -> Result<(), String> {
    // The legacy C ABI is fully replaced by the typed cxx bridges; the
    // _Static_assert contract and C example were removed with it.
    Ok(())
}

fn usage() -> &'static str {
    "Usage:\n  cargo xtask build [--debug] [--endstone-api <version>]\n  cargo xtask check-abi"
}

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let result = match args.next().as_deref() {
        Some(command) if command == "build" => parse_build_options(args).and_then(build),
        Some(command) if command == "check-abi" => {
            if args.next().is_some() {
                Err("check-abi accepts no options".to_owned())
            } else {
                check_abi()
            }
        }
        _ => Err(usage().to_owned()),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("xtask: {message}");
            ExitCode::FAILURE
        }
    }
}
