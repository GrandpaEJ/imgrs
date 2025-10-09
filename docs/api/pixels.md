# 🎯 Pixels API

Complete reference for pixel-level image manipulation.

## Pixel Access

### `img.getpixel(x, y)`

Get color of a single pixel.

**Parameters:**
- `x` (int): X coordinate (0-based)
- `y` (int): Y coordinate (0-based)

**Returns:** `Tuple[int, int, int, int]` - (R, G, B, A)

**Example:**
```python
# Get pixel at position (100, 50)
r, g, b, a = img.getpixel(100, 50)
print(f"Pixel color: R={r}, G={g}, B={b}, A={a}")

# Check if pixel is red
if r > 200 and g < 50 and b < 50:
    print("This pixel is red!")
```

**Notes:**
- Returns RGBA even for RGB images (A=255)
- Raises error if coordinates out of bounds

---

### `img.putpixel(x, y, color)`

Set color of a single pixel.

**Parameters:**
- `x` (int): X coordinate
- `y` (int): Y coordinate
- `color` (Tuple[int, int, int, int]): RGBA color

**Returns:** `Image`

**Example:**
```python
# Set pixel to red
img = img.putpixel(100, 50, (255, 0, 0, 255))

# Draw single pixels to create pattern
for i in range(100):
    img = img.putpixel(i, i, (255, 0, 0, 255))  # Diagonal line
```

**Note:** For setting many pixels, consider using drawing functions instead.

---

## Color Analysis

### `img.histogram()`

Get histogram of pixel values.

**Parameters:** None

**Returns:** `Tuple[List[int], List[int], List[int], List[int]]`
- (R_histogram, G_histogram, B_histogram, A_histogram)
- Each histogram has 256 values (one per intensity level)

**Example:**
```python
r_hist, g_hist, b_hist, a_hist = img.histogram()

# Find brightest red value
max_red = r_hist.index(max(r_hist))
print(f"Most common red value: {max_red}")

# Check alpha usage
if sum(a_hist[:-1]) == 0:
    print("Image has no transparency")
```

**Use cases:**
- Brightness analysis
- Color distribution
- Transparency detection
- Image statistics

---

### `img.dominant_color()`

Find the most common color in the image.

**Parameters:** None

**Returns:** `Tuple[int, int, int, int]` - (R, G, B, A)

**Example:**
```python
r, g, b, a = img.dominant_color()
print(f"Dominant color: RGB({r}, {g}, {b})")

# Use for background detection
if r > 240 and g > 240 and b > 240:
    print("Image has white background")
```

---

### `img.average_color()`

Calculate average color across entire image.

**Parameters:** None

**Returns:** `Tuple[int, int, int, int]` - (R, G, B, A)

**Example:**
```python
r, g, b, a = img.average_color()
print(f"Average color: RGB({r}, {g}, {b})")

# Detect overall image tone
brightness = (r + g + b) / 3
if brightness > 180:
    print("Bright image")
elif brightness < 75:
    print("Dark image")
```

---

## Color Manipulation

### `img.replace_color(target_color, replacement_color, tolerance)`

Replace all pixels of one color with another.

**Parameters:**
- `target_color` (Tuple[int, int, int, int]): Color to replace
- `replacement_color` (Tuple[int, int, int, int]): New color
- `tolerance` (int): Color matching tolerance (0-255)
  - 0: Exact match only
  - Higher: More lenient matching

**Returns:** `Image`

**Example:**
```python
# Replace exact white with transparent
img = img.replace_color(
    (255, 255, 255, 255),  # White
    (0, 0, 0, 0),          # Transparent
    tolerance=10
)

# Replace red with blue (with tolerance)
img = img.replace_color(
    (255, 0, 0, 255),      # Red
    (0, 0, 255, 255),      # Blue
    tolerance=30
)

# Remove green screen
img = img.replace_color(
    (0, 255, 0, 255),      # Green
    (0, 0, 0, 0),          # Transparent
    tolerance=50
)
```

**Tolerance explained:**
- Calculates Euclidean distance in RGBA space
- `tolerance=0`: Only replaces exact matches
- `tolerance=50`: Replaces similar colors
- `tolerance=255`: Replaces all colors

---

### `img.threshold(threshold_value)`

Apply threshold to create binary (black/white) image.

**Parameters:**
- `threshold_value` (int): Threshold (0-255)
  - Pixels ≥ threshold become white (255)
  - Pixels < threshold become black (0)

**Returns:** `Image`

**Example:**
```python
# Basic threshold
binary = img.threshold(128)

# Dark threshold (more white)
high_thresh = img.threshold(200)

# Light threshold (more black)
low_thresh = img.threshold(50)
```

**Use cases:**
- Document scanning
- Text extraction
- Shape detection
- Segmentation

---

### `img.posterize(levels)`

Reduce number of color levels (posterization effect).

**Parameters:**
- `levels` (int): Number of color levels per channel (2-256)
  - 2: Pure black/white per channel
  - 8: Visible posterization
  - 256: No change

**Returns:** `Image`

**Example:**
```python
# Strong posterization (pop art effect)
poster = img.posterize(4)

# Mild posterization
mild = img.posterize(16)

# Extreme (2 levels = pure black/white per channel)
extreme = img.posterize(2)
```

---

## Pixel Iteration Patterns

### Analyze all pixels

```python
# Find all red pixels
red_pixels = []
for y in range(img.height):
    for x in range(img.width):
        r, g, b, a = img.getpixel(x, y)
        if r > 200 and g < 50 and b < 50:
            red_pixels.append((x, y))

print(f"Found {len(red_pixels)} red pixels")
```

### Modify pixels conditionally

```python
# Darken bright pixels
result = img
for y in range(img.height):
    for x in range(img.width):
        r, g, b, a = img.getpixel(x, y)
        brightness = (r + g + b) / 3
        
        if brightness > 200:
            # Darken by 50
            new_color = (max(0, r-50), max(0, g-50), max(0, b-50), a)
            result = result.putpixel(x, y, new_color)
```

**⚠️ Warning:** Pixel-by-pixel operations are slow. Use built-in filters when possible!

---

## Performance Comparison

| Operation | Speed | When to Use |
|-----------|-------|-------------|
| `replace_color()` | ⚡⚡⚡ Fast | Color replacement |
| `threshold()` | ⚡⚡⚡ Fast | Binary conversion |
| `posterize()` | ⚡⚡⚡ Fast | Reduce colors |
| `histogram()` | ⚡⚡ Medium | Analysis |
| `dominant_color()` | ⚡⚡ Medium | Find main color |
| `average_color()` | ⚡⚡ Medium | Overall tone |
| `getpixel()` single | ⚡⚡⚡ Fast | Single pixel |
| `getpixel()` loop | ⚠️ Slow | Avoid if possible |
| `putpixel()` single | ⚡⚡⚡ Fast | Single pixel |
| `putpixel()` loop | ⚠️ Slow | Use filters instead |

---

## Best Practices

### ✅ DO

```python
# Use built-in operations
binary = img.threshold(128)
replaced = img.replace_color(old, new, 30)

# Single pixel access is fine
pixel = img.getpixel(100, 100)

# Get statistics
avg = img.average_color()
```

### ❌ DON'T

```python
# Don't loop pixels for effects (use filters instead)
for y in range(img.height):
    for x in range(img.width):
        img = img.putpixel(x, y, some_color)  # SLOW!

# Use this instead:
img = img.replace_color(target, replacement, tolerance)
```

---

**See Also:**
- [Filters API](filters.md) - Better than pixel loops!
- [Drawing API](drawing.md) - Drawing shapes
- [Examples](../examples/pixels.md) - Pixel manipulation examples

