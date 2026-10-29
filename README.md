# 2d-game-experiments

Independent Rust game experiments in a Cargo workspace:

- `raylib-sample` — raylib experiment (Rust 2024).
- `sdl3-main-callbacks` — SDL3 callbacks experiment (Rust 2021).

## Build and run

Install Rust with edition 2024 support, CMake, and a C/C++ compiler. On
Windows, use the MSVC Rust toolchain and Visual Studio Build Tools with
Desktop development with C++.

The root `.cargo/config.toml` sets `LIBCLANG_PATH` for bindgen to
`D:\bin\_compilers\_latests\clang-msvc\bin`. Update it for your computer,
or override it with the `LIBCLANG_PATH` environment variable.

From the repository root, build all experiments:

```powershell
cargo build --workspace --release --locked
```

Build or run a single experiment:

```powershell
cargo build -p raylib-sample --release --locked
cargo run -p raylib-sample --release --locked
cargo run -p sdl3-main-callbacks --release --locked
```

All members share the root `Cargo.lock`, `target/` directory, and release
profile. Each package keeps its own dependencies and Rust edition.
See [the SDL3 README](sdl3-main-callbacks/README.md) for application details.

## Add an experiment

Create a separate package directory and add its path to `workspace.members`
in the root `Cargo.toml`. Rust experiments can use any engine or no engine.
Run `cargo build -p <package-name>` to update the shared lockfile, then commit
the root `Cargo.lock` along with the new package.

Keep experiments that require incompatible toolchains or native dependencies
outside the workspace by listing their directories in `workspace.exclude`.
Projects using other build systems can live alongside the Rust packages and
use their own build commands.
