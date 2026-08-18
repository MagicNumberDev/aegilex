use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing manifest dir"));
    let root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .expect("example component must live below workspace root");
    let source = root.join("rust/crates/aegilex-runtime/wit");
    let destination = manifest_dir.join("wit");

    fs::create_dir_all(&destination).expect("failed to create WIT output directory");
    for entry in fs::read_dir(&destination).expect("failed to read WIT output directory") {
        let entry = entry.expect("failed to read WIT output directory entry");
        let path = entry.path();
        if path.extension().is_some_and(|extension| extension == "wit")
            && !source.join(entry.file_name()).exists()
        {
            fs::remove_file(path).expect("failed to remove stale canonical Aegilex WIT");
        }
    }
    for entry in fs::read_dir(&source).expect("failed to read canonical Aegilex WIT directory") {
        let entry = entry.expect("failed to read WIT directory entry");
        let path = entry.path();
        if path.extension().is_some_and(|extension| extension == "wit") {
            let file_name = entry.file_name();
            fs::copy(&path, destination.join(&file_name))
                .expect("failed to sync canonical Aegilex WIT");
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}
