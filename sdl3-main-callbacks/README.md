# SDL3 main callbacks

Standalone Rust application extracted from the locally modified
`examples/main-callbacks-api.rs` in [sdl3-rs](https://github.com/vhspace/sdl3-rs).
The source was copied unchanged from the working tree based on commit
`e4d18214`, before updating that checkout to `origin/master`.

The application opens a window with an 800 x 600 logical client area showing
the elapsed time, mouse coordinates, display scale, and a centered rectangle.
It requests 25 callbacks per second and normally repaints only
when the mouse position changes inside the focused window. It also repaints
on startup and after display scale, window size, expose, or restore changes.
Press Escape or close the window to exit.

The window requests a native-resolution high-DPI back buffer. Text and drawing
coordinates scale with the current monitor's display scale, including fractional
scales such as 150%. Windows uses the existing PerMonitorV2 application manifest.
Because `sdl3` 0.20 does not expose SDL's display-scale-changed window event, each
callback checks the current scale before deciding whether a repaint is needed.
Mouse coordinates remain in window units, including for the window bounds check.

The window also scales its client area at startup and when the monitor scale
changes: 800 x 600 pixels at 100%, 1200 x 900 at 150%, and 1600 x 1200 at 200%.
The requested window size uses display scale divided by pixel density, so
platforms that already use logical window coordinates are not scaled twice.
Each size is derived from the original logical dimensions to avoid rounding
drift. Resize requests are deferred while minimized, maximized, or fullscreen;
the window manager may constrain the requested size to the available desktop.

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
