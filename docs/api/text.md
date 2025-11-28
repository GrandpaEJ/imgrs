# 📝 Text API - Advanced Text Rendering

## Overview

imgrs provides comprehensive text rendering capabilities with advanced styling options, multi-line support, precise text measurement, and full font loading support. Text rendering supports both built-in fonts and external TTF/OTF/WOFF font files for maximum flexibility.

## Available Methods

### Basic Text Rendering

#### `add_text(text, position, size=40, color=(0,0,0,255), font_path=None, font=None)`

Add text to an image with flexible positioning and font support.

```python
from imgrs import Image, ImageFont

img = Image.new("RGB", (400, 300), (255, 255, 255))

# Using tuple position
img = img.add_text("Hello World", (20, 20), size=32, color=(0, 0, 0, 255))

# Using separate x,y coordinates
img = img.add_text("Hello World", 20, 20, size=32, color=(0, 0, 0, 255))

# Using custom font
font = ImageFont.load("arial.ttf", size=24)
img = img.add_text("Custom Font", (20, 60), font=font, color=(0, 0, 150, 255))

# Using font path directly
img = img.add_text("Font Path", (20, 100), size=28, font_path="times.ttf", color=(150, 0, 0, 255))
```

**Parameters:**
- `text` (str): Text to render
- `position` (int, int) or (int): X coordinate or (x, y) tuple
- `size` (int): Font size (default: 40, ignored if font is provided)
- `color` (tuple): RGBA color tuple (default: black)
- `font_path` (str): Path to TTF/OTF/WOFF font file (optional)
- `font` (Font): ImageFont object (alternative to font_path and size)

### Styled Text Rendering

#### `add_text_styled(text, position, size=40, color=(0,0,0,255), outline=None, shadow=None, background=None)`

Add styled text with outline, shadow, and background effects.

```python
# Text with outline
img = img.add_text_styled(
    "OUTLINED",
    (50, 50),
    size=32,
    color=(255, 255, 255, 255),
    outline=(0, 0, 0, 255, 2.0)  # (r, g, b, a, width)
)

# Text with shadow
img = img.add_text_styled(
    "SHADOW",
    (50, 100),
    size=32,
    color=(255, 0, 0, 255),
    shadow=(3, 3, 128, 128, 128, 200)  # (offset_x, offset_y, r, g, b, a)
)

# Text with background
img = img.add_text_styled(
    "BACKGROUND",
    (50, 150),
    size=32,
    color=(255, 255, 255, 255),
    background=(0, 100, 200, 255)  # (r, g, b, a)
)

# Combined effects
img = img.add_text_styled(
    "FULL STYLE",
    (50, 200),
    size=36,
    color=(255, 215, 0, 255),  # Gold
    outline=(139, 69, 19, 255, 1.5),  # Brown outline
    shadow=(2, 2, 105, 105, 105, 180),  # Gray shadow
    background=(25, 25, 112, 255)  # Midnight blue background
)

# Rotated text
img = img.add_text_styled(
    "ROTATED",
    (100, 100),
    size=32,
    color=(0, 0, 0, 255),
    rotation=45.0
)
```

**Parameters:**
- `text` (str): Text to render
- `position` (int, int): (x, y) position tuple
- `size` (int): Font size (default: 40)
- `color` (tuple): RGBA color tuple (default: black)
- `outline` (tuple): (r, g, b, a, width) for outline effect
- `shadow` (tuple): (offset_x, offset_y, r, g, b, a) for shadow effect
- `background` (tuple): (r, g, b, a) for background box
- `rotation` (float): Rotation angle in degrees (optional)

### Multi-line Text Rendering

#### `add_text_multiline(text, position, size=40, color=(0,0,0,255), line_spacing=1.2)`

Add multi-line text with customizable line spacing.

```python
# Basic multi-line text
img = img.add_text_multiline(
    "Line 1\nLine 2\nLine 3",
    (20, 20),
    size=24,
    color=(0, 0, 0, 255)
)

# Multi-line with custom spacing
img = img.add_text_multiline(
    "Tight spacing\nbetween lines",
    (20, 100),
    size=20,
    color=(0, 100, 0, 255),
    line_spacing=1.1  # Tighter than default
)

# Multi-line with wide spacing
img = img.add_text_multiline(
    "Wide spacing\nmakes text\neasier to read",
    (20, 180),
    size=18,
    color=(0, 0, 150, 255),
    line_spacing=2.0  # Double spacing
)
```

**Parameters:**
- `text` (str): Multi-line text (separated by `\n`)
- `position` (int, int): (x, y) position tuple
- `size` (int): Font size (default: 40)
- `color` (tuple): RGBA color tuple (default: black)
- `line_spacing` (float): Line spacing multiplier (default: 1.2)

### Text Measurement

#### `get_text_size(text, size=40)`

Get the dimensions of rendered text.

```python
width, height = img.get_text_size("Hello World", size=32)
print(f"Text dimensions: {width} x {height}")
```

**Parameters:**
- `text` (str): Text to measure
- `size` (int): Font size (default: 40)

**Returns:** (width, height) tuple

#### `get_text_box(text, x, y, size=40)`

Get complete bounding box information for text.

```python
bbox = img.get_text_box("Hello", 50, 50, size=32)
print(bbox)
# Output: {'x': 50, 'y': 50, 'width': 160, 'height': 32, 'baseline_y': 82}
```

**Parameters:**
- `text` (str): Text to measure
- `x` (int): X position
- `y` (int): Y position
- `size` (int): Font size (default: 40)

**Returns:** Dictionary with bounding box information

### Pillow-Compatible Text Drawing

#### `text(position, text, fill=None, font=None, anchor=None, spacing=4, align="left", direction=None, features=None, language=None, stroke_width=0, stroke_fill=None, embedded_color=False)`

Draw text on the image using Pillow-compatible API.

```python
from imgrs import Image, ImageFont

img = Image.new("RGB", (400, 300), (255, 255, 255))

# Load a font
font = ImageFont.load("arial.ttf", size=24)

# Draw text with font
img = img.text((10, 10), "Hello World!", fill=(0, 0, 0, 255), font=font)

# Multi-line text
img = img.text((10, 50), "Line 1\nLine 2\nLine 3", fill=(0, 100, 0, 255), font=font)

# Text with different alignment
img = img.text((200, 50), "Right aligned\nmulti-line text",
               fill=(0, 0, 150, 255), font=font, align="right")
```

**Parameters:**
- `position` (int, int): (x, y) position tuple
- `text` (str): Text to draw (supports multi-line with `\n`)
- `fill` (tuple): RGBA color tuple for text color
- `font` (Font): ImageFont object
- `anchor` (str): Text anchor point (not yet supported)
- `spacing` (int): Line spacing for multi-line text (default: 4)
- `align` (str): Text alignment - "left", "center", "right" (default: "left")
- `direction` (str): Text direction (not yet supported)
- `features` (list): OpenType features (not yet supported)
- `language` (str): Language code (not yet supported)
- `stroke_width` (int): Text outline width (not yet supported)
- `stroke_fill` (tuple): Outline color (not yet supported)
- `embedded_color` (bool): Use embedded color glyphs (not yet supported)

## Font Information

- **Supported Formats**: TTF, OTF, WOFF, WOFF2 font files
- **Fallback Font**: Built-in DejaVuSans font when no font specified
- **Font Caching**: Automatic caching for performance
- **Character Support**: Full Unicode support (depends on font)
- **Rendering**: Anti-aliased, scalable rendering
- **Performance**: Fast, dependency-free rendering

## Examples

See the example scripts for comprehensive demonstrations:

- `examples/text_quick_demo.py` - Quick overview of all features
- `examples/text_demo.py` - Detailed examples with multiple demonstrations

## See Also

- [Drawing API](drawing.md) - Basic drawing operations
- [Image API](image.md) - Core image operations