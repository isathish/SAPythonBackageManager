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

## License
MIT
