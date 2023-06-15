Agenda:
- goal of this project: compile C library, generate Rust wrapper
- [workspace](Cargo.toml) members convention (-sys crate, single target dir)
- how [build scipt](pingpong-sys/build.rs) works, caching build results
- how to generate binding for external libraries:
  - using submodule to compile own/custom version: example [libbpf-sys](https://github.com/libbpf/libbpf-sys/tree/master)
  - using library already installed: use[pkg-config](https://crates.io/crates/pkg-config) to find its location (works on Linux)
- types mapping between C and Rust
  - simple types
  - structs, optional layout tests
  - borowed vs owned strings:
    - [CStr](https://doc.rust-lang.org/std/ffi/struct.CStr.html) vs [CString](https://doc.rust-lang.org/std/ffi/struct.CString.html)
    - [OsStr](https://doc.rust-lang.org/std/ffi/struct.OsStr.html) vs [OsString](https://doc.rust-lang.org/std/ffi/struct.OsString.html)
 