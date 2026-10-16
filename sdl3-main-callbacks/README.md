# SDL3 main callbacks

Standalone Rust application extracted from the locally modified
`examples/main-callbacks-api.rs` in [sdl3-rs](https://github.com/vhspace/sdl3-rs).
The source was copied unchanged from the working tree based on commit
`e4d18214`, before updating that checkout to `origin/master`.

The application opens a 640 x 480 window showing the elapsed time and mouse
coordinates. It requests 25 callbacks per second and repaints only when the
mouse position changes inside the focused window. Press Escape or close the
window to exit.

## Build and run

Install Rust, CMake, and a C/C++ compiler. On Windows, use the MSVC Rust
toolchain and Visual Studio Build Tools with Desktop development with C++.

From this directory:

```powershell
cargo build --release --locked
cargo run --release --verbose --locked
```

The `main` and `build-from-source-static` features are enabled in `Cargo.toml`.
This replaces the original example command:

```powershell
cargo run --release --verbose --features "build-from-source-static,main" --example main-callbacks-api
```

The project configures `LIBCLANG_PATH` in `.cargo/config.toml` to use
`D:\bin\_compilers\_latests\clang-msvc\bin` for bindgen. Update that path when
building on another computer, or override it with the environment variable.

SDL3 is built from source and linked statically, so no separate SDL3 DLL is
needed. The first build includes compiling SDL3. Dependencies come from
crates.io; this project does not depend on the neighboring `sdl3-rs` checkout.
Keep `Cargo.lock` in version control to retain the tested dependency versions.

The original MIT license is included in `LICENSE`.
