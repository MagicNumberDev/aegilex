# Aegilex

Aegilex is a WebAssembly plugin loader for [Endstone](https://github.com/EndstoneMC/endstone). It connects Rust plugins compiled to Wasm with Endstone and forwards commands, events, tasks, forms, services, and map-rendering callbacks to them.

The runtime combines Rust, Wasmtime, and a C++/Endstone bridge layer. Its interface contract is defined by the repository's [WIT world](rust/crates/aegilex-runtime/wit/world.wit). The default build targets Endstone API `0.11`.

## Architecture

```text
Endstone server
└── plugins/
    ├── endstone_aegilex.dll          # Native Aegilex Endstone plugin (Windows)
    └── <guest-id>/
        ├── plugin.wasm               # Wasm guest plugin
        └── aegilex.toml              # Optional authorization policy

Endstone events / commands / callbacks
        │
        ▼
C++ Endstone bridge ── cxx ──► Rust runtime (Wasmtime)
                                      │
                                      ▼
                       WIT canonical ABI guest exports
```

- `native/`: Endstone plugin, C++ bridges, and the Wasm plugin loader.
- `rust/crates/aegilex-runtime/`: Wasmtime runtime, policy validation, and host bindings generated from WIT.
- `rust/crates/aegilex-core-bindgen/`: Generates host imports and guest-export callers for the Core Wasm canonical ABI.
- `rust/crates/xtask/`: Coordinates the Rust static-library and native-plugin builds.
- `examples/`: Minimal hello, event-coverage, and full-API smoke-test guest plugins.
- `tests/`: Canonical ABI and API probes.

Aegilex scans only direct child directories of the Endstone `plugins` directory for `plugin.wasm`. Guest plugin metadata names must be unique; duplicate names are rejected.

## Prerequisites

### Windows build environment

- Rust stable and the `wasm32-wasip1` target:

  ```powershell
  rustup target add wasm32-wasip1
  ```

- CMake 3.29 or newer.
- Ninja.
- LLVM `clang-cl`. Native plugin builds on Windows explicitly require it.
- Visual Studio C++ Build Tools. The repository's [`.cargo/config.toml`](.cargo/config.toml) configures MSVC `link.exe` for `x86_64-pc-windows-msvc`.
- A working Endstone server. The first native build fetches Endstone `v0.11` through CMake `FetchContent`, so network access is required.

Verify the tools are available:

```powershell
rustc --version
cargo --version
cmake --version
ninja --version
clang-cl --version
```

## Build the native loader

Run this from the repository root:

```powershell
cargo xtask build
```

This first builds the `aegilex-runtime` Rust static library, then configures and builds the Endstone plugin with CMake and Ninja. On Windows, the deployment script expects the Release artifact at:

```text
target/aegilex/release/endstone_aegilex.dll
```

Optional arguments:

```powershell
# Debug build
cargo xtask build --debug

# Select the Endstone API version; defaults to 0.11
cargo xtask build --endstone-api 0.11
```

## Build guest plugins

Guest plugins are compiled for `wasm32-wasip1` and use `wit-bindgen 0.60` against the Aegilex WIT world. Build the minimal example with:

```powershell
cargo build --release --target wasm32-wasip1 --manifest-path examples/hello-component/Cargo.toml
```

Artifact:

```text
examples/hello-component/target/wasm32-wasip1/release/aegilex_hello_component.wasm
```

Build the other repository examples with:

```powershell
cargo build --release --target wasm32-wasip1 --manifest-path examples/event-test-component/Cargo.toml
cargo build --release --target wasm32-wasip1 --manifest-path examples/full-test-component/Cargo.toml
```

Each example's `build.rs` synchronizes `rust/crates/aegilex-runtime/wit/` into its local `wit/` directory during the build. The runtime directory is therefore the single source of truth for WIT; do not maintain the example copies manually.

### Create a guest plugin

1. Copy `examples/hello-component` to `examples/<your-component>`.
2. Change the package name in `Cargo.toml`, and update the name, version, commands, permissions, and event subscriptions returned by `metadata()` in `src/lib.rs`.
3. Keep `build.rs` and the `wit_bindgen::generate!` configuration so the plugin continues to use the canonical WIT world `plugin`.
4. Build it with the `cargo build --release --target wasm32-wasip1 --manifest-path ...` command shown above.
5. Rename the generated `.wasm` file to `plugin.wasm` and place it, together with the policy file, in the Endstone `plugins/<your-component>/` directory.

The `subscriptions` declared by `metadata()` determine which events are bridged to the guest. Guest plugins can also declare Endstone commands and permissions. See [hello-component](examples/hello-component/src/lib.rs) for an example that registers `/hello` (alias `/hi`), logs during enable, and listens for player-join events.

## Authorization policies

`aegilex.toml` is optional and lives next to a guest plugin. Without it, the policy is empty: no host API capabilities, preopened paths, or network rules are granted, and host calls that require authorization are rejected.

Minimal capability policy:

```toml
capabilities = [
  "logger.get-logger",
  "logger.logger.log",
  "actor.player.get-name",
  "player-join-event.player-join-event.get-player",
]
```

Supported top-level keys:

```toml
# Exact capabilities, interface wildcards such as logger.*, or all capabilities (*)
capabilities = ["logger.*"]

# Existing directories relative to the server root; preopened as /data in the guest
paths = ["data"]

# Literal IP addresses and single ports only
network = ["tcp:127.0.0.1:8080", "udp:127.0.0.1:19132"]
```

Policy validation rules:

- Capabilities must exist in the current WIT interfaces. `*` grants every known capability and is suitable for tests, not untrusted guest plugins.
- `paths` must name existing relative directories inside the server root and cannot contain `.` or `..`.
- `network` accepts only `tcp` or `udp`, literal IP addresses, and a single port.
- Capabilities, paths, and network rules cannot be duplicated.

See the [hello example policy](examples/hello-component/aegilex.toml) and the [full-test example policy](examples/full-test-component/aegilex.toml), which grants all capabilities.

## Deploy to Endstone

The repository provides a Windows PowerShell deployment script. It builds the native plugin and all three example guest plugins, copies the artifacts, and verifies every destination file with SHA-256:

```powershell
.\scripts\deploy.ps1 -ServerRoot 'C:\path\to\bedrock_server'
```

Resulting layout:

```text
<server-root>/
└── plugins/
    ├── endstone_aegilex.dll
    ├── example_hello/
    │   ├── plugin.wasm
    │   └── aegilex.toml
    ├── event_test/
    │   ├── plugin.wasm
    │   └── aegilex.toml
    └── full_test/
        ├── plugin.wasm
        └── aegilex.toml
```

Restart Endstone. Aegilex will register its loader, discover `plugin.wasm` files in the directories above, validate their metadata and policies, and enable the guest plugins. `/aegilex` reports whether the runtime is available.

When deploying your own guest plugin manually, use the same directory convention:

```text
<server-root>/plugins/<your-component>/plugin.wasm
<server-root>/plugins/<your-component>/aegilex.toml
```

## Verification and development checks

The runtime's Core Wasm tests load artifacts from `hello-component` and `full-test-component`, so build both guest plugins first:

```powershell
cargo build --release --target wasm32-wasip1 --manifest-path examples/hello-component/Cargo.toml
cargo build --release --target wasm32-wasip1 --manifest-path examples/full-test-component/Cargo.toml
cargo test -p aegilex-runtime
```

Native code checks:

```powershell
# Check formatting
.\scripts\clang-format.ps1

# Build first, then run clang-analyzer checks
.\scripts\clang-tidy.ps1
```

## License

This project is licensed under the [GNU Lesser General Public License v3.0](LICENSE).
