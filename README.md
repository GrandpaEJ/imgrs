# Imgrs 🦀

[![BUILD](https://github.com/grandpaej/imgrs/workflows/BUILD/badge.svg)](https://github.com/grandpaej/imgrs/actions)
[![Python](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A **blazingly fast**, modern image processing library for Python, powered by Rust. Imgrs provides a Pillow-compatible API while delivering 65+ filters, rich text rendering, emoji overlays, and professional-grade image effects.

## ✨ Key Features

- **🔥 High Performance**: Rust-powered for blazing fast image operations
- **🎨 65+ Filters**: Gaussian blur, sharpen, edge detection, artistic effects, and more
- **✍️ Rich Text**: Full-featured text rendering with TTF/OTF fonts, outlines, shadows
- **😊 70+ Emojis**: Add emoji overlays to images
- **📸 EXIF Support**: Read and access image metadata
- **🔄 Pillow Compatible**: Drop-in replacement for most Pillow operations
- **🦀 Rust Powered**: Memory-safe and efficient core
- **📦 Easy to Use**: Simple, intuitive API

## 🚀 Quick Start

### Installation

```bash
pip install imgrs
```

### Basic Usage

```python
import imgrs

# ============================================================================
# BASIC OPERATIONS
# ============================================================================

# Open an image
img = imgrs.Image.open("photo.jpg")

# Resize image
resized = img.resize((800, 600))

# Crop image (left, top, right, bottom)
cropped = img.crop((100, 100, 500, 400))

# Rotate image
rotated = img.rotate(90)

# Save image
img.save("output.png")

# Create new image
new_img = imgrs.Image.new("RGB", (800, 600), (255, 0, 0))  # Red image

# Convert image modes
gray_img = img.convert("L")  # RGB to grayscale
rgba_img = img.convert("RGBA")  # Add alpha channel

# Split into channels
r, g, b = img.split()  # Returns 3 grayscale images

# Paste one image onto another
base = imgrs.Image.new("RGB", (200, 200), (255, 255, 255))
overlay = imgrs.Image.new("RGB", (100, 100), (255, 0, 0))
result = base.paste(overlay, (50, 50))

# Create from NumPy array
import numpy as np
array = np.random.randint(0, 256, (100, 100, 3), dtype=np.uint8)
img_from_array = imgrs.Image.fromarray(array)

# ============================================================================
# IMAGE FILTERS (65+)
# ============================================================================

# Basic filters
blurred = img.blur(5.0)
sharpened = img.sharpen(1.5)
edges = img.edge_detect()
embossed = img.emboss()

# Adjustments
brighter = img.brightness(50)
higher_contrast = img.contrast(1.5)

# CSS-style filters
sepia = img.sepia()
grayscale = img.grayscale_filter(1.0)
inverted = img.invert()
hue_shifted = img.hue_rotate(180)

# Advanced effects
gaussian = img.gaussian_blur(10.0)
motion = img.motion_blur(20, 45)
unsharp = img.unsharp_mask(1.5, 1.0)
sobel = img.sobel_edge_detect()

# Auto-enhancement
enhanced = img.auto_enhance()
balanced = img.auto_white_balance()
optimized = img.auto_contrast()
normalized = img.normalize()
exposure_fixed = img.exposure_adjust(0.5)

# Artistic effects
oil = img.oil_painting(5, 20)
watercolor = img.watercolor()
sketch = img.sketch()
cartoon = img.cartoon()
vintage = img.sepia()

# ============================================================================
# RICH TEXT RENDERING
# ============================================================================

# Basic text
img = img.add_text("Hello World", (50, 50), size=48, color=(255, 0, 0, 255))

# Text with outline and shadow
img = img.add_text_styled(
    "BOLD TEXT",
    (100, 100),
    size=64,
    color=(255, 255, 255, 255),
    outline=(0, 0, 0, 255, 3.0),       # Black outline, 3px wide
    shadow=(3, 3, 0, 0, 0, 180)        # Shadow offset and color
)

# Centered text
img = img.add_text_centered("Centered Title", 50, size=56, color=(0, 0, 128, 255))

# Multi-line text
multiline = "Line 1\nLine 2\nLine 3"
img = img.add_text_multiline(multiline, (50, 50), size=32, line_spacing=1.5)

# Text with background
img = img.add_text_styled(
    "Text with BG",
    (50, 50),
    size=40,
    color=(255, 255, 255, 255),
    background=(0, 0, 0, 180)
)

# ============================================================================
# TEXT MEASUREMENT & LAYOUT (TEXTBOX)
# ============================================================================

# Get text dimensions before rendering
width, height = imgrs.Image.get_text_size("Sample Text", size=48)
print(f"Text will be {width}x{height} pixels")

# Get multi-line text dimensions
multiline = "Line 1\nLine 2\nLine 3"
width, height, line_count = imgrs.Image.get_multiline_text_size(
    multiline, size=32, line_spacing=1.5
)
print(f"{width}x{height} pixels, {line_count} lines")

# Get complete text bounding box with all metrics
box = imgrs.Image.get_text_box("Sample", 100, 50, size=64)
print(f"Position: ({box['x']}, {box['y']})")
print(f"Dimensions: {box['width']}x{box['height']}")
print(f"Baseline Y: {box['baseline_y']}")
print(f"Bottom-right: ({box['right_x']}, {box['bottom_y']})")

# Dynamic text centering using measurements
text = "Center Me"
w, h = imgrs.Image.get_text_size(text, size=48)
x = (img.width - w) // 2
y = (img.height - h) // 2
img = img.add_text(text, (x, y), size=48, color=(0, 0, 0, 255))

# ============================================================================
# EMOJI OVERLAYS (70+)
# ============================================================================

# Add single emoji
img = img.add_emoji("smile", (100, 100), size=64)

# Add multiple emojis
emojis = [
    ("heart", 50, 50, 48),
    ("star", 150, 50, 48),
    ("fire", 250, 50, 48)
]
img = img.add_emojis(emojis)

# Available emojis: smile, heart, star, fire, thumbsup, rocket, sparkles, etc.

# ============================================================================
# PIXEL OPERATIONS
# ============================================================================

# Get/set pixels
pixel = img.getpixel(100, 100)
img = img.putpixel(100, 100, (255, 0, 0, 255))

# Color analysis
histogram = img.histogram()
dominant = img.dominant_color()
average = img.average_color()

# Color manipulation
img = img.replace_color((255, 0, 0), (0, 255, 0), tolerance=30)
img = img.threshold(128)
img = img.posterize(4)

# ============================================================================
# DRAWING OPERATIONS
# ============================================================================

# Draw shapes
img = img.draw_rectangle((50, 50, 200, 150), (255, 0, 0, 255))
img = img.draw_circle((300, 100), 50, (0, 0, 255, 255))
img = img.draw_line((0, 0), (200, 200), (0, 0, 0, 255), width=3)

# Draw text (basic)
img = img.draw_text("Label", 10, 10, (0, 0, 0, 255), 24)

# ============================================================================
# EFFECTS & SHADOWS
# ============================================================================

# Drop shadow
img = img.drop_shadow(5, 5, 10.0, (0, 0, 0, 128))

# Inner shadow
img = img.inner_shadow(3, 3, 5.0, (0, 0, 0, 128))

# Glow effect
img = img.glow(15.0, (255, 255, 0, 200), 1.5)

# ============================================================================
# METADATA & EXIF
# ============================================================================

# Read EXIF data
metadata = img.get_metadata("photo.jpg")
if metadata.get('exif'):
    print(f"Camera: {metadata['exif'].get('make')}")
    print(f"Model: {metadata['exif'].get('model')}")

# Check for EXIF/GPS
has_exif = img.has_exif("photo.jpg")
has_gps = img.has_gps("photo.jpg")

# Get metadata summary
summary = img.get_metadata_summary("photo.jpg")
print(summary)  # "1920x1080 | Canon EOS 5D | ISO 400"
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

## 🎨 Complete Feature List

### Core Operations ✅
- `open()`, `new()`, `save()` - I/O operations
- `resize()`, `crop()`, `rotate()`, `transpose()` - Transformations
- `copy()`, `thumbnail()` - Duplication and scaling
- `convert()`, `paste()`, `split()` - Format and composition
- `fromarray()`, `to_bytes()` - NumPy integration
- Properties: `size`, `width`, `height`, `mode`, `format`

### Image Filters (65+) ✅

**Basic Filters:**
- `blur()`, `sharpen()`, `edge_detect()`, `emboss()`
- `brightness()`, `contrast()`

**Advanced Blur:**
- `gaussian_blur()`, `motion_blur()`, `bilateral_blur()`
- `box_blur()`, `median_blur()`, `radial_blur()`, `zoom_blur()`

**Edge Detection:**
- `sobel_edge_detect()`, `prewitt_edge_detect()`, `canny_edge_detect()`
- `laplacian_edge_detect()`, `roberts_edge_detect()`, `scharr_edge_detect()`

**Sharpening:**
- `unsharp_mask()`, `high_pass_sharpen()`, `edge_enhance()`

**CSS-Style Filters:**
- `sepia()`, `grayscale_filter()`, `invert()`
- `hue_rotate()`, `saturate()`

**Artistic Effects:**
- `oil_painting()`, `watercolor()`, `pencil_sketch()`
- `cartoon()`, `sketch()`, `halftone()`, `vignette()`, `glitch()`

**Morphological:**
- `dilate()`, `erode()`, `opening()`, `closing()`, `morphological_gradient()`

**Noise:**
- `add_gaussian_noise()`, `add_salt_pepper_noise()`, `denoise()`

**Color Effects:**
- `duotone()`, `color_splash()`, `chromatic_aberration()`

**Auto-Enhancement:**
- `histogram_equalization()`, `auto_contrast()`, `auto_brightness()`
- `auto_enhance()`, `exposure_adjust()`, `auto_level()`
- `normalize()`, `smart_enhance()`, `auto_white_balance()`

### Rich Text Rendering ✅

**Text Methods:**
- `add_text()` - Basic text rendering
- `add_text_styled()` - Full styling (outline, shadow, background, opacity)
- `add_text_centered()` - Horizontally centered text
- `add_text_multiline()` - Multi-line text with line spacing

**Text Measurement:**
- `get_text_size()` - Get text dimensions (width, height)
- `get_multiline_text_size()` - Multi-line dimensions with line count
- `get_text_box()` - Complete bounding box (x, y, width, height, ascent, descent, baseline)

**Text Features:**
- TTF/OTF font support
- Text colors with full RGBA
- Text alignment (left, center, right)
- Outlines and shadows
- Background colors
- Text opacity
- Line and letter spacing
- Text wrapping
- Anti-aliased rendering

### Emoji Overlays (70+) ✅
- `add_emoji()` - Add single emoji
- `add_emojis()` - Add multiple emojis
- `add_emoji_text()` - Emoji with text label
- Available emojis: smile, heart, star, fire, thumbsup, rocket, and 65+ more

### Pixel Operations ✅
- `getpixel()`, `putpixel()` - Direct pixel access
- `histogram()` - Color histogram
- `dominant_color()`, `average_color()` - Color analysis
- `replace_color()` - Color replacement with tolerance
- `threshold()`, `posterize()` - Color quantization

### Drawing Operations ✅
- `draw_rectangle()` - Filled rectangles
- `draw_circle()` - Filled circles
- `draw_line()` - Lines with width
- `draw_text()` - Basic text (legacy method)

### Effects & Shadows ✅
- `drop_shadow()` - Drop shadow with blur
- `inner_shadow()` - Inner shadow effects
- `glow()` - Glow effects

### Metadata & EXIF ✅
- `get_metadata()` - Read EXIF data (camera, GPS, settings)
- `get_metadata_summary()` - Human-readable summary
- `has_exif()`, `has_gps()` - Check for metadata presence

## 📖 API Examples

### Advanced Text Styling

```python
import imgrs

img = imgrs.Image.new("RGBA", (800, 400), (30, 30, 50, 255))

# Epic text with all effects
img = img.add_text_styled(
    "EPIC TEXT",
    (200, 100),
    size=80,
    color=(255, 215, 0, 255),          # Gold color
    outline=(255, 140, 0, 255, 4.0),   # Orange outline
    shadow=(5, 5, 0, 0, 0, 200),       # Black shadow
    align="center",
    opacity=1.0
)

img.save("epic_text.png")
```

### Dynamic Text Layout with Textbox

```python
import imgrs

img = imgrs.Image.new("RGBA", (600, 400), (255, 255, 255, 255))

# Measure text first
title = "Dynamic Title"
width, height = imgrs.Image.get_text_size(title, size=64)

# Center it based on measurements
x = (img.width - width) // 2
y = 50

# Get complete bounding box for precise positioning
box = imgrs.Image.get_text_box(title, x, y, size=64)
print(f"Text baseline at y={box['baseline_y']}")

# Render the text
img = img.add_text(title, (x, y), size=64, color=(0, 0, 128, 255))

# Add subtitle below
subtitle = "Perfectly Aligned"
w2, h2 = imgrs.Image.get_text_size(subtitle, size=32)
x2 = (img.width - w2) // 2
y2 = box['bottom_y'] + 20

img = img.add_text(subtitle, (x2, y2), size=32, color=(128, 128, 128, 255))

img.save("dynamic_layout.png")
```

### Filter Chains

```python
# Chain multiple filters
result = (img
    .blur(2.0)
    .sharpen(1.5)
    .brightness(20)
    .contrast(1.2)
    .saturate(1.3))

# Auto-enhancement workflow
enhanced = (img
    .auto_brightness()
    .auto_contrast()
    .auto_white_balance()
    .smart_enhance(0.7))
```

### Emoji Overlays

```python
# Add emojis
img = img.add_emoji("heart", (100, 100), size=64)
img = img.add_emoji("star", (200, 100), size=48)
img = img.add_emoji("fire", (300, 100), size=56)

# Batch add emojis
emojis = [
    ("smile", 50, 50, 48),
    ("thumbsup", 150, 50, 48),
    ("rocket", 250, 50, 48)
]
img = img.add_emojis(emojis)
```

### Real-World Example: Meme Generator

```python
import imgrs

# Load image
img = imgrs.Image.open("photo.jpg")

# Add top text (meme style)
img = img.add_text_styled(
    "TOP TEXT",
    (img.width // 2, 20),
    size=56,
    color=(255, 255, 255, 255),
    outline=(0, 0, 0, 255, 3.0),
    align="center"
)

# Add bottom text
img = img.add_text_styled(
    "BOTTOM TEXT",
    (img.width // 2, img.height - 70),
    size=56,
    color=(255, 255, 255, 255),
    outline=(0, 0, 0, 255, 3.0),
    align="center"
)

img.save("meme.png")
```

## 🔄 Pillow Compatibility

Imgrs maintains API compatibility with Pillow for seamless migration:

```python
# ✅ Works with both Pillow and imgrs
from imgrs import Image  # or: from PIL import Image

img = Image.open("photo.jpg")
img = img.resize((800, 600))
img = img.convert("L")
img.save("output.png")
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

# Run examples
python examples/text_demo.py
python examples/textbox_demo.py
python examples/auto_enhance_demo.py
```

### Requirements

- Python 3.8+
- Rust 1.70+
- Maturin for building

## 📊 Performance

Imgrs delivers excellent performance through Rust optimization:

- **Fast I/O**: Efficient image loading and saving
- **Parallel Processing**: Multi-threaded filter operations
- **Memory Efficient**: Zero-copy operations where possible
- **SIMD Optimizations**: Hardware-accelerated operations

## 📚 Documentation

- **[Quick Start Guide](docs/guides/quickstart.md)** - Get started in minutes
- **[Installation Guide](docs/guides/installation.md)** - Detailed setup instructions
- **[Basic Usage Guide](docs/guides/basic-usage.md)** - Core concepts
- **[Migration Guide](docs/guides/migration.md)** - Moving from Pillow
- **[API Reference](docs/api/)** - Complete API documentation
- **[Examples](docs/examples/)** - Real-world examples

## 🎯 Use Cases

- **Photography**: Enhancement, filters, batch processing
- **Web Development**: Image optimization, thumbnails, watermarking
- **Social Media**: Memes, text overlays, emoji additions
- **E-commerce**: Product images, banners, promotional graphics
- **Data Visualization**: Charts with text labels
- **Creative Projects**: Artistic filters, collages, designs

## 🗂️ Project Structure

```
imgrs/
├── src/                   # Rust source code
│   ├── filters/          # Image filters (65+ effects)
│   ├── text/             # Rich text rendering
│   ├── emoji/            # Emoji overlays
│   ├── metadata/         # EXIF/metadata support
│   ├── image/            # Core image operations
│   └── ...
├── python/imgrs/         # Python wrapper
│   ├── mixins/           # Feature mixins (20 files)
│   └── __init__.py
├── examples/             # Demo scripts
│   ├── text_demo.py      # Text rendering demos
│   ├── textbox_demo.py   # Text measurement demos
│   ├── auto_enhance_demo.py
│   └── output/           # Organized demo outputs
└── docs/                 # Documentation
```

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](contributing.md) for guidelines.

### Contributors

- **[GrandpaEJ](https://github.com/GrandpaEJ)** - Feature requests and guidance
- **[Bilal Tonga](https://github.com/bgunebakan/puhu)** - Initial project implementation

## 📄 License

IRADL License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [PyO3](https://pyo3.rs/) for Python-Rust integration
- Uses [image-rs](https://github.com/image-rs/image) for core image processing
- Text rendering powered by [ab_glyph](https://github.com/alexheretic/ab-glyph)
- Inspired by [Pillow](https://pillow.readthedocs.io/) for API design
- First Skeleton by [Bilal Tonga](https://github.com/bgunebakan/puhu)

---

**Made with ❤️ and 🦀 Rust**
