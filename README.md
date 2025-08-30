# SA - Super Accelerated Python Package Manager

SA is a next-generation Python package and environment manager inspired by UV, built from scratch in Rust for maximum performance, safety, and simplicity.

## Features
- 🚀 Lightning fast, written in Rust
- 📦 Smart dependency management (auto-updates requirements.txt)
- 🔄 Global wheel cache with hard-linking
- 🌐 Rich metadata and dependency tree display
- 🏗️ Integrated build system and lock files
- 📤 Direct PyPI publishing with authentication
- 🐍 Automatic virtual environment management (.sa_env)
- ⚡ Efficient HTTP range requests for metadata
- 🎯 Single binary, no Python dependency required

## Architecture and Implementation
- Written in Rust for speed, safety, and efficiency
- Single static binary with no Python dependency
- Unified toolchain: pip, pip-tools, pipx, virtualenv, pyenv functionality

## Dependency Resolution and Package Installation
- Optimized dependency solver for complex trees
- Native `pyproject.toml` parsing in Rust
- HTTP range requests for partial metadata fetching
- Global cache + hard linking for disk efficiency

## Virtual Environment and Python Version Management
- Direct filesystem operations for venv creation
- Python version management without Python dependency
- Transparent ephemeral/persistent environments

## Commands

### 1. Add Packages (`sa add`)
Install packages and automatically update requirements.txt:
```bash
sa add requests
sa add requests numpy pandas
```
- Fetches metadata from PyPI
- Displays dependency tree
- Updates/creates requirements.txt
- Installs in .sa_env virtual environment

### 2. Remove Packages (`sa remove`)
Remove packages from the environment:
```bash
sa remove requests
sa remove --package numpy
```

### 3. Run Scripts with Dependencies (`sa run`)
Execute Python scripts with automatic dependency installation:
```bash
sa run --with requests my_script.py
sa run --with pandas data_analysis.py input.csv --output results.json
sa run --with matplotlib -c "import matplotlib; print('OK')"
```

### 4. List Installed Packages (`sa list`)
Display all installed packages in the environment:
```bash
sa list
```

### 5. Build Project (`sa build`)
Build your Python project for distribution:
```bash
sa build
```
- Compiles Python package (sdist and wheel)
- Generates lock file (sa.lock)
- Stores artifacts in target/dist/

### 6. Publish Project (`sa publish`)
Publish your package to PyPI:
```bash
export PYPI_TOKEN=your_token_here
sa publish
```
- Authenticates with PyPI
- Uploads wheel and source distributions

### 7. Version (`sa version`)
Display SA version information:
```bash
sa version
```

## Project Structure
```
your-project/
├── .sa_env/              # Virtual environment
├── requirements.txt      # Package dependencies
├── sa.lock               # Build lock file
├── target/               # Build artifacts
│   └── dist/
│       ├── *.whl
│       └── *.tar.gz
└── your_code.py
```

## Performance and Optimizations
- 10-100x faster than pip/poetry
- Zero-copy deserialization for metadata
- Optimized large package handling
- Parallel installations

## Comparison with pip/poetry
| Feature                | pip | poetry | SA |
|------------------------|-----|--------|----|
| Package Installation   | ✅  | ✅     | ✅ |
| Dependency Resolution  | Basic| Advanced| Advanced |
| requirements.txt       | Manual| ❌   | Auto |
| Lock Files             | ❌  | ✅     | ✅ |
| Dependency Trees       | ❌  | ✅     | ✅ |
| Rich Metadata          | ❌  | Basic  | ✅ |
| Global Caching         | Basic| ❌    | ✅ |
| Virtual Environments   | Manual| ✅   | Auto |
| Build System           | External| ✅ | ✅ |
| Publishing             | External| ✅ | ✅ |
| Performance            | Slow | Medium | Fast |
| Single Binary          | ❌  | ❌     | ✅ |

## Cross-Platform Support
- Works on Windows, macOS, Linux

## Roadmap
- [x] Implement CLI structure in Rust
- [x] Add dependency resolution engine (PyPI metadata parsing)
- [x] Implement package fetching with HTTP range requests
- [x] Add global cache and hard linking
- [x] Advanced dependency resolution (full graph, version constraints)
- [x] Python version management (download, install, switch)
- [x] Build and publish commands
- [x] Lock file support (with build timestamp)
- [x] Full dependency graph in lock file
- [x] Cross-platform path handling
- [x] Multiple index support
- [x] Parallel installations
- [x] Error handling improvements
- [x] Artifact verification before publishing
- [x] Cross-platform testing
- [x] GitHub Actions CI/CD for multi-platform release
- [x] Auto-incrementing version and tagging in release workflow
- [x] Update README automatically with latest version after release

## Installation
Download the latest release binary from [GitHub Releases](https://github.com/isathish/SAPythonBackageManager/releases).

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

## Support
- Issues: [GitHub Issues](https://github.com/isathish/SAPythonBackageManager/issues)
- Discussions: [GitHub Discussions](https://github.com/isathish/SAPythonBackageManager/discussions)
- Documentation: This README and inline help (`sa --help`)

## License
MIT
