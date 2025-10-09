# 💿 Installation Guide

## Quick Install

```bash
pip install imgrs
```

That's it! We provide **pre-built wheels** for all major platforms.

## Supported Platforms

### ✅ Linux (manylinux)
- **x86_64** - Standard 64-bit
- **aarch64** - ARM 64-bit (Raspberry Pi 4, AWS Graviton)
- **armv7** - ARM 32-bit (Raspberry Pi 3)
- **s390x** - IBM mainframe
- **ppc64le** - PowerPC

### ✅ Windows
- **x64** - Standard 64-bit
- **x86** - 32-bit Windows
- **ARM64** - Surface Pro X, ARM laptops

### ✅ macOS
- **x86_64** - Intel Macs
- **arm64** - Apple Silicon (M1/M2/M3)

### ✅ Android
- **aarch64** - Modern phones/tablets
- **armv7** - Older Android devices
- **x86_64** - Android emulators
- **i686** - 32-bit emulators

## Python Version Support

- Python **3.8** ✅
- Python **3.9** ✅
- Python **3.10** ✅
- Python **3.11** ✅
- Python **3.12** ✅

## Installation Methods

### pip (Recommended)

```bash
# Latest stable version
pip install imgrs

# Specific version
pip install imgrs==0.1.0

# Upgrade to latest
pip install --upgrade imgrs
```

### From Source

Only needed if your platform doesn't have pre-built wheels.

**Requirements:**
- Python 3.8+
- Rust 1.70+ (install from https://rustup.rs)
- pip and maturin

**Steps:**
```bash
# Clone repository
git clone https://github.com/GrandpaEJ/imgrs.git
cd imgrs

# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install maturin
pip install maturin

# Build and install
maturin develop --release

# Or build wheel
maturin build --release
pip install target/wheels/imgrs-*.whl
```

### Development Install

For contributing to imgrs:

```bash
# Clone repository
git clone https://github.com/GrandpaEJ/imgrs.git
cd imgrs

# Install in editable mode
pip install -e .

# Install development dependencies
pip install -r requirements.txt
pip install pytest black flake8 isort
```

## Optional Dependencies

### NumPy Support

```bash
pip install imgrs numpy
```

Enables `Image.fromarray()` functionality:

```python
import numpy as np
from imgrs import Image

array = np.random.randint(0, 255, (480, 640, 3), dtype=np.uint8)
img = Image.fromarray(array)
```

### Development Tools

```bash
pip install pytest pytest-cov black flake8 isort mypy
```

## Verification

### Check Installation

```python
import imgrs
print(imgrs.__version__)
# Output: 0.1.0
```

### Run Quick Test

```python
from imgrs import Image

# Create test image
img = Image.new("RGB", (100, 100), color=(255, 0, 0))
print(f"Size: {img.size}")
print(f"Mode: {img.mode}")

# Test operation
resized = img.resize((50, 50))
print(f"Resized: {resized.size}")

print("✅ imgrs is working!")
```

## Troubleshooting

### ImportError: No module named 'imgrs'

```bash
# Make sure pip installed successfully
pip list | grep imgrs

# Reinstall
pip uninstall imgrs
pip install imgrs
```

### ImportError: Rust extension not available

```bash
# Check if wheel was installed
pip show imgrs

# If building from source, ensure Rust is installed
rustc --version

# Rebuild
pip install --force-reinstall imgrs
```

### Platform not supported

If no pre-built wheel for your platform:

```bash
# Install Rust first
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Then install imgrs (will build from source)
pip install imgrs
```

### NumPy errors

```bash
# Install NumPy
pip install numpy

# Or install both together
pip install imgrs numpy
```

## Uninstallation

```bash
pip uninstall imgrs
```

---

## Next Steps

- 🚀 [Quick Start](quickstart.md) - Get started in 5 minutes
- 📖 [Basic Usage](basic-usage.md) - Learn the basics
- 💡 [Examples](../examples/) - See code examples

---

**Having issues?** [Report a bug](https://github.com/GrandpaEJ/imgrs/issues)

