# Imgrs 🦀

[![BUILD](https://github.com/grandpaej/imgrs/workflows/BUILD/badge.svg)](https://github.com/grandpaej/imgrs/actions)
[![Python](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A **blazingly fast**, modern image processing library for Python, powered by Rust. Imgrs provides a Pillow-compatible API while delivering significantly better performance for common image operations.

## 📚 Documentation

- **[Quick Start Guide](docs/guides/quickstart.md)** - Get up and running in minutes
- **[Basic Usage](docs/guides/basic-usage.md)** - Core concepts and common patterns
- **[API Reference](docs/api/)** - Complete method documentation
- **[Examples](docs/examples/README.md)** - Real-world usage examples
- **[Migration Guide](docs/guides/migration.md)** - Migrating from Pillow

## ✨ Key Features

- **🔥 High Performance**: Significantly fast for common image operations
- **🔄 Pillow Compatible**: Drop-in replacement for most Pillow operations
- **🦀 Rust Powered**: Memory-safe and efficient core written in Rust
- **📦 Easy to Use**: Simple, intuitive API that feels familiar
- **🎯 Format Support**: PNG, JPEG, BMP, TIFF, GIF, WEBP

## 🚀 Quick Start

### Installation

```bash
pip install imgrs
```

### Basic Usage

```python
import imgrs

# Open an image
img = imgrs.open("photo.jpg")

# Resize image
resized = img.resize((800, 600))

# Crop image
cropped = img.crop((100, 100, 500, 400))

# Rotate image
rotated = img.rotate(90)

# Save image
img.save("output.png")

# Create new image
new_img = imgrs.new("RGB", (800, 600), "red")

# Convert image modes
gray_img = img.convert("L")  # RGB to grayscale
rgba_img = img.convert("RGBA")  # Add alpha channel

# Split image into channels
r, g, b = img.split()  # RGB image -> 3 grayscale images

# Paste one image onto another
base = imgrs.new("RGB", (200, 200), "white")
overlay = imgrs.new("RGB", (100, 100), "red")
result = base.paste(overlay, (50, 50))  # Paste at position (50, 50)

# Create image from NumPy array (requires numpy)
import numpy as np
array = np.random.randint(0, 256, (100, 100, 3), dtype=np.uint8)
img_from_array = imgrs.fromarray(array)

# Rich text rendering with styling
img = img.add_text("Hello World", (50, 50), size=48, color=(255, 0, 0, 255))

# Text with outline and shadow
img = img.add_text_styled(
    "BOLD TEXT", 
    (100, 100), 
    size=64,
    color=(255, 255, 255, 255),
    outline=(0, 0, 0, 255, 3.0),
    shadow=(3, 3, 0, 0, 0, 180)
)

# Centered text
img = img.add_text_centered("Centered Title", 50, size=56, color=(0, 0, 128, 255))

# Multi-line text
multiline = "Line 1\nLine 2\nLine 3"
img = img.add_text_multiline(multiline, (50, 50), size=32, line_spacing=1.5)

# Measure text dimensions before rendering
width, height = imgrs.Image.get_text_size("Sample Text", size=48)
print(f"Text will be {width}x{height} pixels")

# Get complete text bounding box information
box = imgrs.Image.get_text_box("Sample", 100, 50, size=64)
print(f"Text spans from ({box['x']}, {box['y']}) to ({box['right_x']}, {box['bottom_y']})")
print(f"Baseline at y={box['baseline_y']}")
```

### Drop-in Pillow Replacement

```python
# Replace this:
# from PIL import Image

# With this:
from imgrs import Image

# Your existing Pillow code works unchanged!
img = Image.open("photo.jpg")
img = img.resize((400, 300))
img.save("resized.jpg")
```

## 🔄 Pillow Compatibility

### ✅ Fully Compatible Operations

- `open()`, `new()`, `save()`
- `resize()`, `crop()`, `rotate()`, `transpose()`
- `copy()`, `thumbnail()`
- `convert()`, `paste()`, `split()` - **NEW!**
- `fromarray()` - **NEW!** NumPy Integration
- Properties: `size`, `width`, `height`, `mode`, `format`
- All major image formats (PNG, JPEG, BMP, TIFF, GIF, WEBP)

### 🎨 Image Filters - **NEW!**

**Basic Filters:**
- `blur()` - Gaussian blur with adjustable radius
- `sharpen()` - Sharpening filter with adjustable strength
- `edge_detect()` - Edge detection using Sobel operator
- `emboss()` - Emboss effect
- `brightness()` - Brightness adjustment
- `contrast()` - Contrast adjustment

**CSS-like Filters:**
- `sepia()` - Sepia tone effect
- `grayscale_filter()` - Grayscale conversion with amount control
- `invert()` - Color inversion effect
- `hue_rotate()` - Hue rotation in degrees
- `saturate()` - Saturation adjustment

**Filter chaining** - Combine multiple filters for complex effects

### 🎯 Pixel Manipulation - **NEW!**

- `getpixel()`, `putpixel()` - Direct pixel access and modification
- `histogram()` - Color histogram analysis
- `dominant_color()`, `average_color()` - Color analysis
- `replace_color()` - Color replacement with tolerance
- `threshold()` - Binary thresholding
- `posterize()` - Color quantization

### 🎨 Drawing Operations - **NEW!**

- `draw_rectangle()` - Filled rectangles with alpha blending
- `draw_circle()` - Filled circles with alpha blending
- `draw_line()` - Lines using Bresenham's algorithm
- `draw_text()` - Basic text rendering with bitmap fonts

### ✨ Shadow Effects - **NEW!**

- `drop_shadow()` - Drop shadow with blur and offset
- `inner_shadow()` - Inner shadow effects
- `glow()` - Glow effects with customizable intensity

### 🚧 Planned Features

- `frombytes()`, `tobytes()` - _Enhanced I/O_
- Advanced text rendering with font support
- Path operations and vector graphics
- Additional blend modes and compositing operations

## 📖 API Reference

### Core Functions

```python
# Open image from file or bytes
img = imgrs.open("path/to/image.jpg")
img = imgrs.open(image_bytes)

# Create new image
img = imgrs.new(mode, size, color=None)
# Examples:
img = imgrs.new("RGB", (800, 600))  # Black image
img = imgrs.new("RGB", (800, 600), "red")  # Red image
img = imgrs.new("RGB", (800, 600), (255, 0, 0))  # Red image with RGB tuple
```

### Image Operations

```python
# Resize image
resized = img.resize((width, height), resample=imgrs.Resampling.BILINEAR)

# Crop image (left, top, right, bottom)
cropped = img.crop((x1, y1, x2, y2))

# Rotate image (90°, 180°, 270° supported)
rotated = img.rotate(90)

# Transpose/flip image
flipped = img.transpose(imgrs.Transpose.FLIP_LEFT_RIGHT)
flipped = img.transpose(imgrs.Transpose.FLIP_TOP_BOTTOM)

# Copy image
copy = img.copy()

# Create thumbnail (modifies image in-place)
img.thumbnail((200, 200))

# Save image
img.save("output.jpg", format="JPEG")
img.save("output.png")  # Format auto-detected from extension
```

### Properties

```python
# Image dimensions
width = img.width
height = img.height
size = img.size  # (width, height) tuple

# Image mode and format
mode = img.mode  # "RGB", "RGBA", "L", etc.
format = img.format  # "JPEG", "PNG", etc.

# Raw pixel data
bytes_data = img.to_bytes()
```

## 🔧 Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/grandpaej/imgrs.git
cd imgrs

# Install dependencies
pip install -r requirements.txt

# Build Rust extension
maturin develop --release

# Run tests
pytest python/imgrs/tests/

```

### Requirements

- Python 3.8+
- Rust 1.70+
- Maturin for building

## 📖 Learn More

### 🚀 Getting Started
- **[Quick Start Guide](docs/guides/quickstart.md)** - Installation and first steps
- **[Basic Usage](docs/guides/basic-usage.md)** - Essential concepts and patterns

### 📚 Reference & Examples
- **[API Reference](docs/api/)** - Complete method documentation
- **[Examples](docs/examples/)** - Real-world usage examples
- **[Performance Guide](docs/guides/)** - Optimization techniques

### 🎯 Use Cases
- **Photography**: Portrait enhancement, landscape processing, batch operations
- **Web Development**: Image resizing, format optimization, thumbnail generation
- **Creative Projects**: Artistic filters, collages, social media content
- **Data Visualization**: Charts, infographics, dashboard creation
- **E-commerce**: Product showcases, catalog generation, watermarking

## 🤝 Contributing

### Contributors

- **[GrandpaEJ](https://github.com/GrandpaEJ)** - Feature requests and guidance
- **[Bilal Tonga](https://github.com/bgunebakan/puhu)** - Initial implementation of the project


### How to Contribute

Contributions are welcome! Areas where help is needed:

1. **Medium Priority Features**: `getpixel()`, `putpixel()`, `frombytes()`
2. **Performance Optimization**: Further speed improvements and benchmarking
3. **Format Support**: Additional image formats and metadata handling
4. **Advanced Operations**: CSS-like filters, path operations, text rendering
5. **Documentation**: More examples and tutorials
6. **Testing**: Edge cases, compatibility tests, and performance benchmarks

## 📄 License

IRADL License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [PyO3](https://pyo3.rs/) for Python-Rust integration
- Uses [image-rs](https://github.com/image-rs/image) for core image processing
- Inspired by [Pillow](https://pillow.readthedocs.io/) for API design
- First Skaliton by [Bilal Tonga](https://github.com/bgunebakan/puhu)