# 📝 Text API - Advanced Text Rendering

## Overview

imgrs now provides comprehensive text rendering capabilities with advanced styling options, multi-line support, and precise text measurement. All text functions use bitmap fonts for fast, dependency-free rendering.

## Available Methods

### Basic Text Rendering

#### `add_text(text, position, size=40, color=(0,0,0,255))`

Add text to an image with flexible positioning.

```python
from imgrs import Image

img = Image.new("RGB", (400, 300), (255, 255, 255))

# Using tuple position
img = img.add_text("Hello World", (20, 20), size=32, color=(0, 0, 0, 255))

# Using separate x,y coordinates
img = img.add_text("Hello World", 20, 20, size=32, color=(0, 0, 0, 255))
```

**Parameters:**
- `text` (str): Text to render
- `position` (int, int) or (int): X coordinate or (x, y) tuple
- `size` (int): Font size (default: 40)
- `color` (tuple): RGBA color tuple (default: black)

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
```

**Parameters:**
- `text` (str): Text to render
- `position` (int, int): (x, y) position tuple
- `size` (int): Font size (default: 40)
- `color` (tuple): RGBA color tuple (default: black)
- `outline` (tuple): (r, g, b, a, width) for outline effect
- `shadow` (tuple): (offset_x, offset_y, r, g, b, a) for shadow effect
- `background` (tuple): (r, g, b, a) for background box

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

## Font Information

- **Font Type**: 8x8 bitmap font
- **Supported Characters**: A-Z, 0-9, basic punctuation (! ? . ,)
- **Rendering**: Scalable pixel-perfect rendering
- **Performance**: Fast, dependency-free rendering

## Examples

See the example scripts for comprehensive demonstrations:

- `examples/text_quick_demo.py` - Quick overview of all features
- `examples/text_demo.py` - Detailed examples with multiple demonstrations

## See Also

- [Drawing API](drawing.md) - Basic drawing operations
- [Image API](image.md) - Core image operations