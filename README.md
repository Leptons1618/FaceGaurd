# FaceGaurd

FaceGaurd is a multi-component Rust project for facial recognition and security applications. This repository contains several crates and modules for core logic, UI, and WASM-based web integration.

## Project Structure

- `core/` - Core logic and algorithms
- `ui/` - User interface (desktop, Tauri)
- `dx/` - WASM/web build outputs
- `target/` - Build artifacts

## Setup Instructions

### Prerequisites
- Rust (latest stable) and Cargo
- Node.js & npm (for Tauri UI)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Building the Project

1. **Clone the repository:**
   ```sh
   git clone <repo-url>
   cd FaceGaurd
   ```
2. **Build core crate:**
   ```sh
   cd core
   cargo build --release
   ```
3. **Build UI crate:**
   ```sh
   cd ../ui
   cargo build --release
   ```
4. **Run the desktop app (Tauri):**
   ```sh
   cd src-tauri
   cargo tauri dev
   ```

### WASM/Web Build

To build for web (WASM):
```sh
cd dx/ui/wasm-dev
# Follow project-specific build instructions here
```

## Documentation

See the `docs/` folder for architecture, changelogs, plans, and feature progress.

## Contributing

1. Fork the repo
2. Create a feature branch
3. Submit a PR with a clear description

## License

MIT (see LICENSE file)
