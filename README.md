# Imgrs 🦀

[![BUILD](https://github.com/grandpaej/imgrs/workflows/BUILD/badge.svg)](https://github.com/grandpaej/imgrs/actions)
[![Python](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A **blazingly fast** image processing library for Python, powered by Rust.

## ✨ Features

- **🔥 65+ Image Filters** - Blur, sharpen, edges, artistic effects, auto-enhancement
- **✍️ Rich Text Rendering** - TTF/OTF fonts, styling, outlines, shadows
- **📏 Text Measurement** - Precise text sizing and layout calculations
- **😊 70+ Emoji Overlays** - Add emojis to images
- **📸 EXIF/Metadata** - Read camera data and GPS information
- **🎨 Drawing Tools** - Shapes, lines, and pixel operations
- **🎭 Effects** - Drop shadows, glow effects
- **⚡ High Performance** - Rust-powered speed

## 🚀 Quick Start

### Installation

```bash
pip install imgrs
```

### Basic Usage

```python
import imgrs

# Open and manipulate
img = imgrs.Image.open("photo.jpg")
img = img.resize((800, 600))
img = img.blur(5.0)
img.save("output.png")

# Filters
img = img.sharpen(1.5)
img = img.auto_enhance()
img = img.sepia()

# Text rendering
img = img.add_text("Hello", (50, 50), size=48, color=(255, 0, 0, 255))

# Text with styling
img = img.add_text_styled(
    "Styled Text",
    (100, 100),
    size=64,
    outline=(0, 0, 0, 255, 3.0),
    shadow=(3, 3, 0, 0, 0, 180)
)

# Text measurement (textbox)
width, height = imgrs.Image.get_text_size("Text", size=48)
box = imgrs.Image.get_text_box("Text", 100, 50, size=64)

# Emoji
img = img.add_emoji("heart", (100, 100), size=64)
```

## 📚 Feature List

### Core Operations
- Image I/O: `open()`, `save()`, `new()`
- Transformations: `resize()`, `crop()`, `rotate()`, `thumbnail()`
- Format conversion: `convert()`, `split()`, `paste()`
- NumPy: `fromarray()`, `to_bytes()`

### Image Filters (65+)

**Basic:** blur, sharpen, edge_detect, emboss, brightness, contrast

**Advanced Blur:** gaussian_blur, motion_blur, bilateral_blur, median_blur, radial_blur, zoom_blur

**Edge Detection:** sobel_edge_detect, prewitt_edge_detect, canny_edge_detect, laplacian_edge_detect

**Auto-Enhancement:** auto_enhance, auto_contrast, auto_brightness, auto_white_balance, smart_enhance, histogram_equalization, normalize, exposure_adjust

**Artistic:** oil_painting, watercolor, pencil_sketch, cartoon, sketch, halftone, vignette, glitch

**CSS-Style:** sepia, grayscale_filter, invert, hue_rotate, saturate

**Morphological:** dilate, erode, opening, closing

**Color Effects:** duotone, color_splash, chromatic_aberration

### Text Rendering

**Rendering Methods:**
- `add_text()` - Basic text
- `add_text_styled()` - Full styling (outline, shadow, background, opacity, alignment)
- `add_text_centered()` - Centered text
- `add_text_multiline()` - Multi-line text

**Measurement Methods (Textbox):**
- `get_text_size()` - Get width and height
- `get_multiline_text_size()` - Multi-line dimensions
- `get_text_box()` - Complete bounding box (x, y, width, height, baseline, ascent, descent)

**Features:** TTF/OTF fonts, RGBA colors, outlines, shadows, backgrounds, alignment, opacity, line spacing, text wrapping

### Emoji Overlays (70+)
- `add_emoji()` - Single emoji
- `add_emojis()` - Multiple emojis
- Types: smile, heart, star, fire, thumbsup, rocket, etc.

### Pixel Operations
- `getpixel()`, `putpixel()` - Direct access
- `histogram()`, `dominant_color()`, `average_color()`
- `replace_color()`, `threshold()`, `posterize()`

### Drawing
- `draw_rectangle()`, `draw_circle()`, `draw_line()`

### Effects
- `drop_shadow()`, `inner_shadow()`, `glow()`

### Metadata
- `get_metadata()`, `has_exif()`, `has_gps()`

## 📖 Documentation

For detailed guides and examples, see **[docs/](docs/)** directory:

- **[docs/guides/](docs/guides/)** - Installation, usage, migration guides
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

## 🤝 Contributing

See [CONTRIBUTING.md](contributing.md) for guidelines.

**Contributors:** [GrandpaEJ](https://github.com/GrandpaEJ), [Bilal Tonga](https://github.com/bgunebakan/puhu)

## 📄 License

IRADL License - see [LICENSE](LICENSE) file.

## 🙏 Acknowledgments

Built with [PyO3](https://pyo3.rs/), [image-rs](https://github.com/image-rs/image), and [ab_glyph](https://github.com/alexheretic/ab-glyph)

---

**Made with 🦀 Rust**
