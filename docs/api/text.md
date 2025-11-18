# 📝 Text API

Complete reference for rich text rendering operations on images.

## Overview

imgrs provides comprehensive text rendering with support for:
- Multiple font formats (TTF, OTF, WOFF2)
- Advanced styling (outlines, shadows, glows)
- Font families and weights
- Multi-line text with alignment
- Background colors and opacity

## Basic Text Methods

### `img.add_text(text, position, size=32, color=(0, 0, 0, 255), font_family="sans", font_weight="normal", font_style="normal", font_path=None, letter_spacing=0.0, opacity=1.0)`

Draw basic text with enhanced font support.

**Parameters:**
- `text` (str): Text content to render
- `position` (Tuple[int, int] | Tuple[float, float]): (x, y) coordinates
- `size` (float): Font size in pixels (default: 32.0)
- `color` (Tuple[int, int, int, int] | str): Text color as RGBA tuple or CSS color name
- `font_family` (str): Font family ("sans", "serif", "mono", or custom name)
- `font_weight` (str): Font weight ("normal", "bold", "light", or numeric 100-900)
- `font_style` (str): Font style ("normal", "italic", "oblique")
- `font_path` (str, optional): Direct path to TTF/OTF/WOFF2 font file
- `letter_spacing` (float): Letter spacing in pixels (can be negative)
- `opacity` (float): Text opacity 0.0-1.0

**Returns:** `Image`

**Examples:**
```python
# Basic text
img = img.add_text("Hello World", (50, 50), size=48)

# Colored text with custom font
img = img.add_text("Red Text", (50, 100), size=36, color=(255, 0, 0, 255))

# Using CSS color names
img = img.add_text("Blue Text", (50, 150), color="blue", size=32)

# Custom font file
img = img.add_text("Custom Font", (50, 200), font_path="/path/to/font.ttf")

# Semi-transparent text
img = img.add_text("Faded", (50, 250), opacity=0.7)
```

---

### `img.add_text_centered(text, y, size=32, color=(0, 0, 0, 255), font_family="sans", font_weight="normal", font_style="normal", font_path=None, opacity=1.0, letter_spacing=0.0)`

Draw horizontally centered text.

**Parameters:**
- `text` (str): Text content
- `y` (int | float): Vertical position
- `size` (float): Font size in pixels
- `color` (Tuple[int, int, int, int] | str): Text color
- `font_family` (str): Font family
- `font_weight` (str): Font weight
- `font_style` (str): Font style
- `font_path` (str, optional): Font file path
- `opacity` (float): Text opacity
- `letter_spacing` (float): Letter spacing

**Returns:** `Image`

**Example:**
```python
# Centered title
img = img.add_text_centered("Page Title", 50, size=48, font_weight="bold")
```

---

## Advanced Text Methods

### `img.add_text_styled(text, position, size=32, color=(0, 0, 0, 255), font_family="sans", font_weight="normal", font_style="normal", font_path=None, letter_spacing=0.0, opacity=1.0, align="left", background=None, outline=None, shadow=None, glow=None, max_width=None, line_spacing=1.2, text_justify="left", rotation=0.0)`

Draw text with comprehensive styling options (alias for `add_text_advanced`).

**Parameters:**
- `text` (str): Text content (supports multiline with \\n)
- `position` (Tuple[int, int] | Tuple[float, float]): (x, y) coordinates
- `size` (float): Font size in pixels
- `color` (Tuple[int, int, int, int] | str): Text color
- `font_family` (str): Font family
- `font_weight` (str): Font weight
- `font_style` (str): Font style
- `font_path` (str, optional): Font file path
- `letter_spacing` (float): Letter spacing in pixels
- `opacity` (float): Text opacity 0.0-1.0
- `align` (str): Text alignment ("left", "center", "right")
- `background` (Tuple[int, int, int, int] | str, optional): Background color
- `outline` (Tuple[int, int, int, int, float], optional): Outline as (R, G, B, A, width)
- `shadow` (Tuple[int, int, int, int, int, int], optional): Shadow as (offset_x, offset_y, R, G, B, A)
- `glow` (Tuple[int, int, int, int, float], optional): Glow as (R, G, B, A, blur_radius)
- `max_width` (int, optional): Maximum width for text wrapping
- `line_spacing` (float): Line spacing multiplier (default: 1.2)
- `text_justify` (str): Text justification ("left", "center", "right", "justify")
- `rotation` (float): Rotation angle in degrees

**Returns:** `Image`

**Examples:**
```python
# Text with outline
img = img.add_text_styled(
    "OUTLINED",
    (100, 50),
    size=80,
    color=(255, 255, 255, 255),
    outline=(0, 0, 0, 255, 3.0)
)

# Text with shadow
img = img.add_text_styled(
    "Shadow Text",
    (100, 100),
    size=48,
    shadow=(3, 3, 100, 100, 100, 180)
)

# Text with glow effect
img = img.add_text_styled(
    "Glowing Text",
    (100, 150),
    size=48,
    color=(255, 255, 0, 255),
    glow=(255, 255, 0, 150, 5.0)
)

# Combined effects
img = img.add_text_styled(
    "EPIC TEXT",
    (200, 200),
    size=80,
    color=(255, 215, 0, 255),
    outline=(255, 140, 0, 255, 4.0),
    shadow=(5, 5, 0, 0, 0, 200),
    glow=(255, 215, 0, 100, 3.0),
    align="center"
)
```

---

### `img.add_text_advanced(text, position, size=32, color=(0, 0, 0, 255), font_family="sans", font_weight="normal", font_style="normal", font_path=None, letter_spacing=0.0, opacity=1.0, align="left", background=None, outline=None, shadow=None, glow=None, max_width=None, line_spacing=1.2, text_justify="left", rotation=0.0)`

Full-featured text rendering with all styling options (same as `add_text_styled`).

---

### `img.add_text_multiline(text, position, size=32, color=(0, 0, 0, 255), font_family="sans", font_weight="normal", font_style="normal", font_path=None, line_spacing=1.2, letter_spacing=0.0, align="left", text_justify="left", max_width=None, opacity=1.0)`

Draw multi-line text with enhanced formatting.

**Parameters:**
- `text` (str): Text with \\n for line breaks
- `position` (Tuple[int, int] | Tuple[float, float]): Starting (x, y) coordinates
- `size` (float): Font size
- `color` (Tuple[int, int, int, int] | str): Text color
- `font_family` (str): Font family
- `font_weight` (str): Font weight
- `font_style` (str): Font style
- `font_path` (str, optional): Font file path
- `line_spacing` (float): Line spacing multiplier
- `letter_spacing` (float): Letter spacing per line
- `align` (str): Line alignment ("left", "center", "right")
- `text_justify` (str): Text justification within lines
- `max_width` (int, optional): Maximum line width for wrapping
- `opacity` (float): Text opacity

**Returns:** `Image`

**Example:**
```python
multiline_text = """Line 1: First Line
Line 2: Second Line
Line 3: Third Line"""

img = img.add_text_multiline(
    multiline_text,
    (50, 50),
    size=32,
    line_spacing=1.5,
    align="center"
)
```

---

## Font Management

### Font Format Support

imgrs supports multiple font formats:
- **TTF** (TrueType Font) - `.ttf`
- **OTF** (OpenType Font) - `.otf`
- **WOFF2** (Web Open Font Format 2.0) - `.woff2`
- **TTC** (TrueType Collection) - `.ttc`

### Font Families

Built-in font families:
- `"sans"` - Sans-serif fonts (DejaVu Sans, Arial)
- `"serif"` - Serif fonts (DejaVu Serif, Times)
- `"mono"` - Monospace fonts (DejaVu Sans Mono, Menlo)

### Font Resolution

Fonts are resolved in this order:
1. Explicit `font_path` if provided
2. Font family matching in fonts directory
3. System font fallbacks
4. Embedded default font (DejaVu Sans)

---

## Text Measurement

### `Image.get_text_size(text, size=32, font_family="sans", font_weight="normal", font_style="normal", font_path=None, letter_spacing=0.0)`

Get text dimensions.

**Parameters:**
- `text` (str): Text to measure
- `size` (float): Font size
- `font_family` (str): Font family
- `font_weight` (str): Font weight
- `font_style` (str): Font style
- `font_path` (str, optional): Font file path
- `letter_spacing` (float): Letter spacing

**Returns:** `Tuple[int, int]` - (width, height)

**Example:**
```python
width, height = Image.get_text_size("Hello World", size=48, font_weight="bold")
print(f"Text will be {width}x{height} pixels")
```

---

### `Image.get_multiline_text_size(text, size=32, line_spacing=1.2, font_family="sans", font_weight="normal", font_style="normal", font_path=None, letter_spacing=0.0, max_width=None)`

Get multiline text dimensions.

**Parameters:**
- `text` (str): Multiline text
- `size` (float): Font size
- `line_spacing` (float): Line spacing multiplier
- `font_family` (str): Font family
- `font_weight` (str): Font weight
- `font_style` (str): Font style
- `font_path` (str, optional): Font file path
- `letter_spacing` (float): Letter spacing
- `max_width` (int, optional): Maximum line width

**Returns:** `Tuple[int, int, int]` - (width, height, line_count)

---

### `Image.get_text_box(text, x, y, size=32, font_family="sans", font_weight="normal", font_style="normal", font_path=None, letter_spacing=0.0)`

Get complete text bounding box.

**Returns:** Dictionary with font metrics:
- `x`, `y`: Top-left corner
- `width`, `height`: Dimensions
- `ascent`, `descent`: Font metrics
- `baseline_y`: Baseline Y coordinate
- `bottom_y`: Bottom edge Y coordinate
- `right_x`: Right edge X coordinate
- `advance_width`: Total advance width

---

## Font Discovery

### `Image.list_available_fonts()`

List all available fonts in the system.

**Returns:** `List[str]` - List of font file paths

**Example:**
```python
fonts = Image.list_available_fonts()
for font in fonts:
    print(font)
```

---

## Color Formats

Text methods support multiple color formats:

```python
# RGBA tuples (0-255)
red = (255, 0, 0, 255)
semi_transparent = (0, 255, 0, 128)

# CSS color names
img = img.add_text("Blue", (50, 50), color="blue")
img = img.add_text("Red", (50, 100), color="red")

# Hex colors (not yet supported)
# img = img.add_text("Green", (50, 150), color="#00FF00")
```

---

## Advanced Examples

### Meme Generator
```python
def create_meme(image_path, top_text, bottom_text):
    img = Image.open(image_path)

    # Top text
    img = img.add_text_styled(
        top_text, (img.width // 2, 20), size=56,
        color=(255, 255, 255, 255),
        outline=(0, 0, 0, 255, 3.0),
        align="center"
    )

    # Bottom text
    img = img.add_text_styled(
        bottom_text, (img.width // 2, img.height - 70), size=56,
        color=(255, 255, 255, 255),
        outline=(0, 0, 0, 255, 3.0),
        align="center"
    )

    return img
```

### Styled Quote
```python
def create_quote(text, author):
    bg = Image.new("RGBA", (600, 400), (40, 44, 52, 255))

    # Main quote
    bg = bg.add_text_multiline(
        text, (300, 150), size=36,
        color=(230, 230, 230, 255),
        line_spacing=1.4, align="center"
    )

    # Author attribution
    bg = bg.add_text_styled(
        f"— {author}", (300, 320), size=24,
        color=(180, 180, 180, 255), align="center"
    )

    return bg
```

### Glowing Title
```python
def create_glowing_title(text, color=(255, 255, 0, 255)):
    canvas = Image.new("RGBA", (800, 200), (0, 0, 0, 255))

    canvas = canvas.add_text_styled(
        text, (400, 100), size=72,
        color=color,
        glow=(color[0], color[1], color[2], 150, 8.0),
        align="center"
    )

    return canvas
```

---

## Performance Notes

- Text rendering is optimized for common use cases
- Font loading is cached for repeated use
- Complex effects (glow, outline, shadow) may impact performance
- Consider pre-calculating text sizes for dynamic layouts

---

## See Also

- [Drawing API](drawing.md) - Basic drawing operations
- [Effects API](effects.md) - Additional visual effects
- [Examples](../examples/text_demo.py) - Complete text rendering examples