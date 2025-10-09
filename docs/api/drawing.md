# ✏️ Drawing API

Complete reference for drawing operations on images.

## Shape Drawing

### `img.draw_rectangle(x, y, width, height, color)`

Draw a filled rectangle on the image.

**Parameters:**
- `x` (int): X coordinate of top-left corner
- `y` (int): Y coordinate of top-left corner
- `width` (int): Rectangle width in pixels
- `height` (int): Rectangle height in pixels
- `color` (Tuple[int, int, int, int]): RGBA color (0-255 each)

**Returns:** `Image`

**Example:**
```python
# Red rectangle
img = img.draw_rectangle(50, 50, 200, 100, (255, 0, 0, 255))

# Semi-transparent blue rectangle
img = img.draw_rectangle(100, 150, 150, 75, (0, 0, 255, 128))

# White filled rectangle
img = img.draw_rectangle(10, 10, 100, 100, (255, 255, 255, 255))
```

**Notes:**
- Coordinates can be negative (will be clipped)
- Alpha blending is automatic for RGBA images

---

### `img.draw_circle(center_x, center_y, radius, color)`

Draw a filled circle on the image.

**Parameters:**
- `center_x` (int): X coordinate of circle center
- `center_y` (int): Y coordinate of circle center
- `radius` (int): Circle radius in pixels
- `color` (Tuple[int, int, int, int]): RGBA color

**Returns:** `Image`

**Example:**
```python
# Red circle at center
w, h = img.size
img = img.draw_circle(w // 2, h // 2, 50, (255, 0, 0, 255))

# Green dot
img = img.draw_circle(100, 100, 10, (0, 255, 0, 255))

# Large semi-transparent circle
img = img.draw_circle(200, 200, 100, (0, 0, 255, 128))
```

---

### `img.draw_line(x0, y0, x1, y1, color)`

Draw a line using Bresenham's algorithm.

**Parameters:**
- `x0` (int): Starting X coordinate
- `y0` (int): Starting Y coordinate
- `x1` (int): Ending X coordinate
- `y1` (int): Ending Y coordinate
- `color` (Tuple[int, int, int, int]): RGBA color

**Returns:** `Image`

**Example:**
```python
# Diagonal line
img = img.draw_line(0, 0, 400, 300, (255, 0, 0, 255))

# Horizontal line
img = img.draw_line(50, 100, 350, 100, (0, 255, 0, 255))

# Vertical line
img = img.draw_line(200, 50, 200, 250, (0, 0, 255, 255))
```

**Note:** Line is 1 pixel wide. For thicker lines, draw multiple parallel lines.

---

### `img.draw_text(text, x, y, color, scale)`

Draw text using built-in bitmap font.

**Parameters:**
- `text` (str): Text to draw (supports A-Z, 0-9, space)
- `x` (int): X coordinate for text start
- `y` (int): Y coordinate for text start
- `color` (Tuple[int, int, int, int]): RGBA color
- `scale` (int): Font scale (1 = 8x8 pixels per character)

**Returns:** `Image`

**Example:**
```python
# Small text
img = img.draw_text("HELLO", 10, 10, (255, 255, 255, 255), scale=1)

# Large text
img = img.draw_text("TITLE", 50, 50, (255, 0, 0, 255), scale=4)

# Numbers
img = img.draw_text("12345", 100, 200, (0, 255, 0, 255), scale=2)
```

**Limitations:**
- Built-in 8x8 bitmap font only
- Supports: A-Z, 0-9, space
- No lowercase (yet)
- Fixed-width font

**Character size:** Each character is `(8 × scale)` pixels wide and tall.

---

## Drawing Patterns

### Simple Shapes

```python
from imgrs import Image

# Create canvas
canvas = Image.new("RGBA", (400, 300), (255, 255, 255, 255))

# Draw multiple shapes
canvas = (canvas
    .draw_rectangle(50, 50, 100, 100, (255, 0, 0, 255))    # Red square
    .draw_circle(300, 150, 50, (0, 255, 0, 255))            # Green circle
    .draw_line(50, 150, 350, 150, (0, 0, 255, 255))         # Blue line
    .draw_text("SHAPES", 150, 250, (0, 0, 0, 255), 2))      # Black text

canvas.save("shapes.png")
```

### Overlays and Watermarks

```python
# Add watermark
watermark_color = (255, 255, 255, 128)  # Semi-transparent white
img = img.draw_text("© 2025", 10, img.height - 30, watermark_color, 2)
```

### Data Visualization

```python
# Draw simple bar chart
canvas = Image.new("RGB", (500, 300), (255, 255, 255))

values = [50, 120, 80, 150, 100]
colors = [(255, 0, 0, 255), (0, 255, 0, 255), (0, 0, 255, 255),
          (255, 255, 0, 255), (255, 0, 255, 255)]

for i, (val, col) in enumerate(zip(values, colors)):
    x = 50 + i * 80
    canvas = canvas.draw_rectangle(x, 250 - val, 60, val, col)

canvas.save("chart.png")
```

### Annotations

```python
# Add bounding box
def add_bbox(img, x, y, w, h, label=""):
    # Draw semi-transparent box
    img = img.draw_rectangle(x, y, w, h, (255, 0, 0, 128))
    
    # Add label
    if label:
        img = img.draw_text(label, x, y - 20, (255, 0, 0, 255), 2)
    
    return img

# Use it
result = add_bbox(img, 100, 100, 200, 150, "FACE")
```

---

## Color Format

All drawing functions use RGBA color tuples:

```python
# Color format: (Red, Green, Blue, Alpha)
# Each value: 0-255

# Solid colors (alpha = 255)
red = (255, 0, 0, 255)
green = (0, 255, 0, 255)
blue = (0, 0, 255, 255)
white = (255, 255, 255, 255)
black = (0, 0, 0, 255)

# Transparent colors (alpha < 255)
semi_red = (255, 0, 0, 128)      # 50% transparent
very_transparent = (0, 255, 0, 64)  # 25% opaque
fully_transparent = (0, 0, 0, 0)    # Invisible
```

---

## Alpha Blending

When drawing on RGBA images, colors blend automatically:

```python
# Create transparent canvas
canvas = Image.new("RGBA", (400, 300), (255, 255, 255, 255))

# Draw semi-transparent shapes - they blend!
canvas = canvas.draw_circle(200, 150, 100, (255, 0, 0, 128))
canvas = canvas.draw_circle(250, 150, 100, (0, 0, 255, 128))
# The overlap will blend colors

canvas.save("blended.png")
```

**Formula:** `result = (1 - alpha) × base + alpha × overlay`

---

## Performance Tips

1. **Drawing is fast** - O(area) complexity
2. **Text is slower** - Each character renders separately
3. **Chain operations** - Reduces intermediate copies
4. **Pre-calculate positions** - Faster loops

**Example - Efficient batch drawing:**
```python
# ✅ Good - chain operations
img = img.draw_rectangle(...).draw_circle(...).draw_line(...)

# ❌ Slower - multiple assignments
img = img.draw_rectangle(...)
img = img.draw_circle(...)
img = img.draw_line(...)
```

---

## Common Patterns

### Grid Drawing

```python
# Draw 10x10 grid
canvas = Image.new("RGB", (500, 500), (255, 255, 255))
grid_color = (200, 200, 200, 255)

for i in range(0, 500, 50):
    canvas = canvas.draw_line(i, 0, i, 500, grid_color)  # Vertical
    canvas = canvas.draw_line(0, i, 500, i, grid_color)  # Horizontal
```

### Crosshair

```python
def draw_crosshair(img, x, y, size=20, color=(255, 0, 0, 255)):
    img = img.draw_line(x - size, y, x + size, y, color)  # Horizontal
    img = img.draw_line(x, y - size, x, y + size, color)  # Vertical
    return img

marked = draw_crosshair(img, 200, 150)
```

---

**See Also:**
- [Effects API](effects.md) - Shadows and glows
- [Pixels API](pixels.md) - Pixel-level control
- [Examples](../examples/drawing.md) - More drawing examples

