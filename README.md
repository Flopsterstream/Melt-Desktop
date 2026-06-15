# Melt Desktop (`melt-desktop`)

A Linux desktop environment written in Rust based on Wayland, using the modular [Smithay](https://github.com/Smithay/smithay) framework. 

This project is structured as a standalone Wayland compositor, initially configured to run nested inside a window (using the `winit` backend) for ease of development and testing.

---

## 📂 Project Structure

The project has the following modular layout, based on Smithay's learning reference `smallvil`:

- **[`src/main.rs`](file:///Users/sosha/Documents/julien/Melt-dektop/src/main.rs)**: The application entry point. Initializes the Calloop event loop, sets up the Wayland display server state, launches a default terminal (`weston-terminal`), and starts the event loop.
- **[`src/state.rs`](file:///Users/sosha/Documents/julien/Melt-dektop/src/state.rs)**: Defines the central `Smallvil` state struct holding details like outputs, seats, compositor/XDG shell states, popup managers, and lists wayland listener sockets.
- **[`src/input.rs`](file:///Users/sosha/Documents/julien/Melt-dektop/src/input.rs)**: Handles low-level keyboard and pointer inputs, window selection, and focus raising.
- **[`src/winit.rs`](file:///Users/sosha/Documents/julien/Melt-dektop/src/winit.rs)**: Implements the nested Winit backend. Sets up the window on the host OS, maps Wayland outputs, and handles the redraw, resize, and close operations.
- **[`src/grabs/`](file:///Users/sosha/Documents/julien/Melt-dektop/src/grabs)**: Contains modular structs for pointer actions:
  - [`move_grab.rs`](file:///Users/sosha/Documents/julien/Melt-dektop/src/grabs/move_grab.rs): Handles dragging and moving windows.
  - [`resize_grab.rs`](file:///Users/sosha/Documents/julien/Melt-dektop/src/grabs/resize_grab.rs): Handles resizing windows dynamically.
- **[`src/handlers/`](file:///Users/sosha/Documents/julien/Melt-dektop/src/handlers)**: Contains trait implementations delegating Wayland protocol requests to Smithay:
  - [`compositor.rs`](file:///Users/sosha/Documents/julien/Melt-dektop/src/handlers/compositor.rs): Manages surfaces and buffer associations.
  - [`xdg_shell.rs`](file:///Users/sosha/Documents/julien/Melt-dektop/src/handlers/xdg_shell.rs): Processes top-level client window states and maps/unmaps windows.
  - [`mod.rs`](file:///Users/sosha/Documents/julien/Melt-dektop/src/handlers/mod.rs): Handles outputs, input seats, and clipboard/drag-and-drop actions.

---

## 💻 macOS Development Setup

Since you are writing a **Linux desktop environment** on macOS, compiling directly for macOS can fail due to the lack of native Wayland/EGL libraries in the Apple graphics ecosystem.

To solve this, we have configured **cross-platform compilation target checks** for Linux.
A local configuration file [`.cargo/config.toml`](file:///Users/sosha/Documents/julien/Melt-dektop/.cargo/config.toml) has been created to default the target platform to Linux:

```toml
[build]
target = "aarch64-unknown-linux-gnu"
```

### Benefits of this setup:
1. **Zero Setup Errors**: `cargo check` compiles and checks the project using metadata for Linux without needing to link against missing macOS system libraries.
2. **IDE Integration**: Your IDE (e.g., VS Code with Rust-Analyzer extension) will automatically pick up this default target, providing you with full autocomplete, syntax diagnostics, and type inference without compiling errors.

---

## 🚀 How to Run and Build

### Checking Code (on macOS)
Run standard Cargo checks to verify code compiles correctly:
```bash
cargo check
```

### Compiling (on macOS)
To compile the binary:
```bash
cargo build
```

### Running the Desktop Environment (on Linux)
Because this is a Wayland compositor, you need a Wayland-capable Linux environment (e.g., a physical Linux system or a Linux VM like OrbStack or UTM with GPU acceleration) to run it.

1. Transfer the workspace to your Linux machine.
2. Compile and run:
   ```bash
   cargo run
   ```
3. Alternatively, spawn it with a specific command:
   ```bash
   cargo run -- -c "your-favorite-client-app"
   ```
