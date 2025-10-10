# Imgrs 🦀

[![BUILD](https://github.com/grandpaej/imgrs/workflows/BUILD/badge.svg)](https://github.com/grandpaej/imgrs/actions)
[![Python](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A **blazingly fast** image processing library for Python, powered by Rust.

## ✨ Features

- **🔥 65+ Image Filters** - Blur, sharpen, edges, artistic effects, auto-enhancement
- **✍️ Rich Text Rendering** - TTF/OTF fonts, styling, outlines, shadows, alignment
- **📏 Text Measurement** - Precise text sizing and bounding box calculations
- **😊 70+ Emoji Overlays** - Add emojis to images
- **📸 EXIF/Metadata** - Read camera data, GPS, and image metadata
- **🎨 Drawing Tools** - Shapes, lines, and more
- **🎭 Effects** - Drop shadows, inner shadows, glow effects
- **🔧 Pixel Operations** - Direct pixel access and manipulation
- **⚡ High Performance** - Rust-powered speed
- **📦 Easy to Use** - Simple, intuitive API

## 🚀 Quick Start

### Installation

```bash
pip install imgrs
```

### Basic Usage

```python
import imgrs

# Open and save
img = imgrs.Image.open("photo.jpg")
img.save("output.png")

# Resize, crop, rotate
img = img.resize((800, 600))
img = img.crop((100, 100, 500, 400))
img = img.rotate(90)

# Filters
img = img.blur(5.0)
img = img.sharpen(1.5)
img = img.auto_enhance()

# Text
img = img.add_text("Hello", (50, 50), size=48, color=(255, 0, 0, 255))
img = img.add_text_styled("Styled", (100, 100), size=64,
                          outline=(0, 0, 0, 255, 3.0),
                          shadow=(3, 3, 0, 0, 0, 180))

# Textbox - measure before rendering
width, height = imgrs.Image.get_text_size("Text", size=48)
box = imgrs.Image.get_text_box("Text", 100, 50, size=64)

# Emoji
img = img.add_emoji("heart", (100, 100), size=64)
```

## 📚 Feature Categories

### Core Operations
- Open, save, resize, crop, rotate, thumbnail
- Format conversion, channel splitting
- NumPy integration

### Filters (65+)
- **Basic:** blur, sharpen, edge_detect, emboss
- **Advanced Blur:** gaussian_blur, motion_blur, bilateral_blur, median_blur
- **Edge Detection:** sobel, prewitt, canny, laplacian
- **Auto-Enhancement:** auto_contrast, auto_brightness, auto_white_balance, smart_enhance
- **Artistic:** oil_painting, watercolor, pencil_sketch, cartoon
- **CSS-Style:** sepia, grayscale_filter, invert, hue_rotate, saturate

### Text Rendering
- `add_text()` - Basic text
- `add_text_styled()` - Full styling (outline, shadow, background, opacity, alignment)
- `add_text_centered()` - Centered text
- `add_text_multiline()` - Multi-line support

### Text Measurement (Textbox)
- `get_text_size()` - Get dimensions (width, height)
- `get_multiline_text_size()` - Multi-line dimensions
- `get_text_box()` - Complete bounding box (x, y, width, height, ascent, descent, baseline)

### Emoji Overlays
- `add_emoji()` - Single emoji
- `add_emojis()` - Multiple emojis
- 70+ emoji types available

### Pixel Operations
- getpixel, putpixel, histogram
- Color analysis and replacement

### Drawing
- Rectangles, circles, lines
- Text drawing

### Effects
- Drop shadows, inner shadows, glow

### Metadata
- Read EXIF, camera settings, GPS data

## 📖 Documentation

For detailed documentation, examples, and guides:

- **[docs/guides/](docs/guides/)** - Installation, basic usage, migration
- **[docs/api/](docs/api/)** - Complete API reference
- **[docs/examples/](docs/examples/)** - Real-world examples
- **[examples/](examples/)** - Demo scripts

## 🔧 Development

```bash
git clone https://github.com/grandpaej/imgrs.git
cd imgrs
pip install -r requirements.txt
maturin develop --release
```

## 📄 License

IRADL License - see [LICENSE](LICENSE) file.

## 🙏 Credits

- Built with [PyO3](https://pyo3.rs/) and [image-rs](https://github.com/image-rs/image)
- Contributors: [GrandpaEJ](https://github.com/GrandpaEJ), [Bilal Tonga](https://github.com/bgunebakan/puhu)

---

**Made with 🦀 Rust**
