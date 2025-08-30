# SA - Super Accelerated Python Package Manager

Welcome to the official documentation for SA, the fastest Python package manager built with Rust.

## Quick Start

### Installation

Choose your platform:

#### Linux
```bash
curl -L -o sa https://github.com/sathishkumarn/SAPythonPackageManager/releases/latest/download/sa-x86_64-unknown-linux-gnu
chmod +x sa
sudo mv sa /usr/local/bin/
```

#### macOS
```bash
# Intel Macs
curl -L -o sa https://github.com/sathishkumarn/SAPythonPackageManager/releases/latest/download/sa-x86_64-apple-darwin

# Apple Silicon (M1/M2)
curl -L -o sa https://github.com/sathishkumarn/SAPythonPackageManager/releases/latest/download/sa-aarch64-apple-darwin

chmod +x sa && sudo mv sa /usr/local/bin/
```

#### Windows
```powershell
# PowerShell
Invoke-WebRequest -Uri "https://github.com/sathishkumarn/SAPythonPackageManager/releases/latest/download/sa-x86_64-pc-windows-msvc.exe" -OutFile "sa.exe"
# Move to a directory in your PATH
```

### Basic Usage

```bash
# Add packages
sa add requests numpy pandas

# Run script with dependencies
sa run --with matplotlib script.py

# Remove packages
sa remove --package old-dependency

# List installed packages
sa list

# Build project
sa build

# Publish to PyPI
sa publish

# Show version
sa version
```

## Features

- ⚡ **Lightning Fast**: Written in Rust for maximum performance
- 🔒 **Isolated Environments**: Automatic virtual environment management
- 🎯 **Smart Resolution**: Advanced dependency resolution
- 📦 **Rich Metadata**: Beautiful package information display
- 🏗️ **Build & Publish**: Integrated building and publishing tools
- 🌐 **Cross Platform**: Native binaries for all major platforms

## Commands Reference

### `sa add <packages>`
Add one or more packages to your project.

### `sa remove --package <package>`
Remove a package from your project.

### `sa run --with <dependency> <script>`
Run a Python script with a specific dependency installed.

### `sa list`
List all installed packages in the current environment.

### `sa build`
Build your Python project (wheel and sdist).

### `sa publish`
Publish your project to PyPI.

### `sa version`
Show the SA version information.

## Contributing

Contributions are welcome! Please see our [GitHub repository](https://github.com/sathishkumarn/SAPythonPackageManager) for more information.

## License

SA is licensed under the MIT License. See the [LICENSE](https://github.com/sathishkumarn/SAPythonPackageManager/blob/main/LICENSE) file for details.
