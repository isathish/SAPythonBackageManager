# SA - Super Accelerated Python Package Manager

SA is a next-generation Python package and environment manager inspired by UV, built from scratch in Rust for maximum performance, safety, and simplicity.

## 1. Architecture and Implementation
- **Written in Rust** for speed, safety, and efficiency.
- **Single Static Binary** with no Python dependency.
- **Unified Toolchain** combining pip, pip-tools, pipx, virtualenv, and pyenv functionality.

## 2. Dependency Resolution and Package Installation
- **Optimized Dependency Solver** for complex trees.
- **Native `pyproject.toml` Parsing** in Rust.
- **HTTP Range Requests** for partial metadata fetching.
- **Global Cache + Hard Linking** for disk efficiency.

## 3. Virtual Environment and Python Version Management
- **Direct Filesystem Operations** for venv creation.
- **Python Version Management** without Python dependency.
- **Transparent Ephemeral/Persistent Environments**.

## 4. Usage Flow and Commands
- `sa run --with "package" python script.py`
- `sa add <package>`
- Build and publish to PyPI.
- Lock file support for reproducible environments.

## 5. Performance and Optimizations
- **10-100x Faster** than pip/poetry.
- **Zero-Copy Deserialization** for metadata.
- **Optimized Large Package Handling**.

## 6. Cross-Platform Support
- Works on **Windows, macOS, Linux**.

---

## Roadmap
- [x] Implement CLI structure in Rust
- [x] Add dependency resolution engine (basic resolver with PyPI metadata parsing)
- [x] Implement package fetching with HTTP range requests
- [x] Add global cache and hard linking
- [x] Implement advanced dependency resolution with full graph and version constraints (initial version)
- [x] Implement Python version management (download, install, switch)
- [x] Implement build and publish commands
- [x] Add lock file support (basic, with build timestamp)
- [x] Enhance lock file to store full dependency graph with versions and hashes
- [x] Implement cross-platform path handling
- [x] Add multiple index support
- [x] Implement parallel installations
- [x] Improve error handling
- [x] Add artifact verification before publishing
- [x] Cross-platform testing
- [x] Add GitHub Actions CI/CD for multi-platform release
- [x] Implement auto-incrementing version and tagging in release workflow
- [x] Update README automatically with latest version after release

## Installation

You can install **SA** by downloading the latest release binary from the [GitHub Releases](https://github.com/isathish/SAPythonBackageManager/releases) page.

### macOS / Linux
```bash
curl -L https://github.com/isathish/SAPythonBackageManager/releases/latest/download/sa -o sa
chmod +x sa
sudo mv sa /usr/local/bin/
```

### Windows (PowerShell)
```powershell
Invoke-WebRequest -Uri "https://github.com/isathish/SAPythonBackageManager/releases/latest/download/sa.exe" -OutFile "sa.exe"
# Add the directory containing sa.exe to your PATH
```

---

## Usage

### Check Version
```bash
sa version
```

### Add a Package
```bash
sa add requests
```

### Run a Script with Dependencies
```bash
sa run --with "requests" python script.py
```

### Remove a Package
```bash
sa remove requests
```

### List Installed Packages
```bash
sa list
```

### Build and Publish
```bash
sa build
sa publish
```

---

## License
MIT
