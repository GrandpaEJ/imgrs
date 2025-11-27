# 🔤 ImageFont API - Font Loading and Management

## Overview

The ImageFont module provides comprehensive font loading and management capabilities for imgrs. It supports multiple font formats and provides a Pillow-compatible API for seamless integration.

## Supported Font Formats

- **TTF (TrueType Font)** - `.ttf` files
- **OTF (OpenType Font)** - `.otf` files
- **WOFF (Web Open Font Format)** - `.woff` files
- **WOFF2** - `.woff2` files

## Font Class

### `Font`

Represents a loaded font with size information.

```python
from imgrs import ImageFont

# Create font from file
font = ImageFont.Font("arial.ttf", size=24)

# Get font properties
path = font.get_font_path()  # "arial.ttf"
size = font.get_size()       # 24
data = font.get_font_data()  # Raw font bytes
```

**Methods:**
- `get_font_path()` - Returns the font file path or None for default font
- `get_size()` - Returns the font size in points
- `get_font_data()` - Returns raw font data bytes

## Font Loading Functions

### `load(font_path, size=12)`

Load a font from file with specified size.

```python
# Load TTF font
font = ImageFont.load("arial.ttf", size=24)

# Load OTF font
font = ImageFont.load("times.otf", size=18)

# Load WOFF font
font = ImageFont.load("font.woff", size=16)
```

**Parameters:**
- `font_path` (str or Path): Path to font file
- `size` (int): Font size in points (default: 12)

**Returns:** Font object

**Raises:**
- `FileNotFoundError`: If font file doesn't exist
- `ValueError`: If font format is unsupported
- `RuntimeError`: If font loading fails

### `truetype(font_path, size=12)`

Load a TrueType font (Pillow-compatible alias for `load()`).

```python
font = ImageFont.truetype("arial.ttf", size=24)
```

### `load_default(size=12)`

Load the built-in default font (DejaVuSans).

```python
font = ImageFont.load_default(size=16)
```

### `get_font(font_path=None, size=12)`

Get a font with automatic fallback to default font.

```python
# Load specific font
font = ImageFont.get_font("arial.ttf", size=24)

# Get default font
font = ImageFont.get_font(size=16)
```

## Font Utilities

### `getsize(text, font)`

Get the size of text when rendered with the given font (approximate).

```python
from imgrs import ImageFont

font = ImageFont.load_default(24)
width, height = ImageFont.getsize("Hello World", font)
print(f"Text size: {width} x {height}")
```

**Parameters:**
- `text` (str): Text to measure
- `font` (Font): Font object

**Returns:** (width, height) tuple

### `getbbox(text, font)`

Get the bounding box of text when rendered with the given font (approximate).

```python
bbox = ImageFont.getbbox("Hello", font)
print(f"Bounding box: {bbox}")  # (left, top, right, bottom)
```

**Parameters:**
- `text` (str): Text to measure
- `font` (Font): Font object

**Returns:** (left, top, right, bottom) tuple

## Font Caching

The ImageFont module automatically caches loaded fonts for performance:

```python
# First load - font is loaded and cached
font1 = ImageFont.load("arial.ttf", size=24)

# Second load with same parameters - returns cached font
font2 = ImageFont.load("arial.ttf", size=24)

# font1 and font2 are the same cached object
assert font1 is font2
```

## Usage Examples

### Basic Font Loading

```python
from imgrs import Image, ImageFont

# Create image
img = Image.new("RGB", (400, 300), (255, 255, 255))

# Load font
font = ImageFont.load("arial.ttf", size=24)

# Use with text() method
img = img.text((10, 10), "Hello World!", font=font, fill=(0, 0, 0, 255))

# Use with add_text methods
img = img.add_text("Custom Font", (10, 50), font=font, color=(0, 0, 150, 255))
```

### Font Size Management

```python
# Different sizes
small_font = ImageFont.load("arial.ttf", size=12)
large_font = ImageFont.load("arial.ttf", size=48)

# Use different sizes
img = img.text((10, 10), "Small", font=small_font)
img = img.text((10, 40), "Large", font=large_font)
```

### Error Handling

```python
try:
    font = ImageFont.load("nonexistent.ttf", size=24)
except FileNotFoundError:
    print("Font file not found")
    font = ImageFont.load_default(size=24)  # Fallback
```

## Font Compatibility

### Pillow Compatibility

The ImageFont module is designed to be compatible with Pillow's ImageFont:

```python
# Pillow style (works in imgrs)
from PIL import ImageFont as PILFont  # Replace with imgrs.ImageFont
font = PILFont.load("arial.ttf", size=24)
```

### Font Format Support

| Format | Extension | Support |
|--------|-----------|---------|
| TrueType | `.ttf` | ✅ Full |
| OpenType | `.otf` | ✅ Full |
| Web Open Font | `.woff` | ✅ Full |
| Web Open Font 2 | `.woff2` | ✅ Full |

## Performance Notes

- Fonts are cached automatically to improve performance
- Font loading is lazy - fonts are only loaded when first used
- Default font is always available (no external dependencies)
- Font rendering is optimized for speed

## See Also

- [Text API](text.md) - Text rendering methods
- [Image API](image.md) - Core image operations
- [Drawing API](drawing.md) - Basic drawing operations