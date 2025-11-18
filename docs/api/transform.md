# 🔄 Transform API

Complete reference for image transformation operations.

## Geometric Transforms

### `img.resize(size, resample=None)`

Resize image to new dimensions.

**Parameters:**
- `size` (Tuple[int, int]): New size (width, height)
- `resample` (str, optional): Resampling filter
  - `"NEAREST"` - Fastest, lowest quality
  - `"BILINEAR"` - Good quality (default)
  - `"BICUBIC"` - High quality
  - `"LANCZOS"` - Highest quality, slower

**Returns:** `Image`

**Example:**
```python
# Resize to specific size
resized = img.resize((800, 600))

# High quality resize
hq = img.resize((1920, 1080), resample="LANCZOS")

# Fast resize for thumbnails
thumb = img.resize((150, 150), resample="NEAREST")
```

**Tips:**
- Use LANCZOS for best quality when downscaling photos
- Use NEAREST for pixel art or when speed matters
- Default BILINEAR is good for most cases

---

### `img.crop(box)`

Crop image to a rectangular region.

**Parameters:**
- `box` (Tuple[int, int, int, int]): Crop box (x, y, width, height)

**Returns:** `Image`

**Example:**
```python
# Crop center 400x300 region starting at (100, 100)
cropped = img.crop((100, 100, 400, 300))

# Crop top-left quarter
w, h = img.size
quarter = img.crop((0, 0, w // 2, h // 2))

# Extract face region
face = img.crop((250, 150, 300, 350))
```

**Validation:**
- x + width must be ≤ image width
- y + height must be ≤ image height
- width and height must be > 0

---

### `img.rotate(angle, expand=False, fillcolor=None, resample=None, center=None, translate=None)`

Rotate image by specified angle with advanced options.

**Parameters:**
- `angle` (float): Rotation angle in degrees (counter-clockwise)
  - Any angle supported (0°, 30°, 45°, 90°, etc.)
- `expand` (bool, optional): Expand canvas to fit rotated image. Default: `False`
  - `False`: Crop to original size, fill empty areas with transparent
  - `True`: Expand canvas, fill empty areas with fillcolor or transparent
- `fillcolor` (tuple, optional): Fill color for empty areas when `expand=True`
  - RGB tuple: `(r, g, b)` for RGB images
  - RGBA tuple: `(r, g, b, a)` for RGBA images
- `resample` (str, optional): Resampling method (placeholder for future)
- `center` (tuple, optional): Rotation center (placeholder for future)
- `translate` (tuple, optional): Post-rotation translation (placeholder for future)

**Returns:** `Image`

**Examples:**
```python
# Basic rotations
rotated_90 = img.rotate(90)      # 90° clockwise
rotated_180 = img.rotate(180)    # 180°
rotated_270 = img.rotate(270)    # 270° clockwise

# Arbitrary angles
rotated_45 = img.rotate(45)      # 45° rotation
rotated_30 = img.rotate(30)      # 30° rotation

# Expand vs crop behavior
cropped = img.rotate(45, expand=False)    # Keep original size
expanded = img.rotate(45, expand=True)    # Expand to fit

# Fill expanded areas
filled = img.rotate(45, expand=True, fillcolor=(255, 0, 0))  # Red background

# Negative angles (clockwise)
clockwise = img.rotate(-90)  # 90° clockwise
```

**Rotation Behavior:**
- **expand=False**: Rotates within original bounds, crops overflow, transparent fill
- **expand=True**: Expands canvas to show full rotated image
- **90°/180°/270°**: Always change dimensions appropriately
- **Arbitrary angles**: Smooth bilinear interpolation

**Convenience Methods:**
```python
# Easy aliases
img.rotate90()    # Same as rotate(90)
img.rotate180()   # Same as rotate(180)
img.rotate270()   # Same as rotate(270)
img.rotate_left() # Same as rotate(90)
img.rotate_right() # Same as rotate(-90)
```

**Performance Notes:**
- Arbitrary angles: O(n×m) with bilinear interpolation
- 90° increments: O(n×m) fast path
- expand=True increases processing time and memory

---

### `img.transpose(method)`

Flip or transpose the image.

**Parameters:**
- `method` (str): Transpose method
  - `"FLIP_LEFT_RIGHT"` - Mirror horizontally
  - `"FLIP_TOP_BOTTOM"` - Mirror vertically
  - `"ROTATE_90"` - Rotate 90° clockwise
  - `"ROTATE_180"` - Rotate 180°
  - `"ROTATE_270"` - Rotate 270° clockwise

**Returns:** `Image`

**Example:**
```python
# Mirror horizontally
mirrored = img.transpose("FLIP_LEFT_RIGHT")

# Mirror vertically
upside_down = img.transpose("FLIP_TOP_BOTTOM")

# Rotate (alternative to rotate())
rotated = img.transpose("ROTATE_90")
```

---

## Color Conversion

### `img.convert(mode)`

Convert image to different color mode.

**Parameters:**
- `mode` (str): Target mode
  - `"L"` - Grayscale
  - `"LA"` - Grayscale + Alpha
  - `"RGB"` - RGB color
  - `"RGBA"` - RGB + Alpha

**Returns:** `Image`

**Example:**
```python
# Convert to grayscale
gray = img.convert("L")

# Add alpha channel
with_alpha = img.convert("RGBA")

# Remove alpha channel
no_alpha = img.convert("RGB")
```

**Conversion table:**
```
RGB  → L     : Grayscale conversion
RGB  → RGBA  : Adds opaque alpha
RGBA → RGB   : Removes alpha
RGBA → L     : Grayscale, ignores alpha
L    → RGB   : Duplicates gray to RGB
```

---

### `img.split()`

Split image into individual channels.

**Parameters:** None

**Returns:** `List[Image]` - List of grayscale images (one per channel)

**Example:**
```python
# Split RGB image
r, g, b = img.split()
r.save("red_channel.png")

# Split RGBA image
r, g, b, a = img.split()
a.save("alpha_channel.png")

# Grayscale image
gray_channels = img.split()
# Returns: [img] (single channel)
```

**Channel order:**
- RGB: [R, G, B]
- RGBA: [R, G, B, A]
- L: [L]
- LA: [L, A]

---

## Compositing

### `img.paste(other, position=None, mask=None)`

Paste another image onto this image.

**Parameters:**
- `other` (Image): Image to paste
- `position` (Tuple[int, int], optional): Position (x, y). Default: (0, 0)
- `mask` (Image, optional): Mask image (grayscale)

**Returns:** `Image`

**Example:**
```python
# Paste at top-left
result = base.paste(overlay)

# Paste at specific position
result = base.paste(overlay, position=(100, 50))

# Paste with mask (alpha blending)
mask = Image.new("L", overlay.size, color=(128, 0, 0, 255))
result = base.paste(overlay, position=(50, 50), mask=mask)
```

**Notes:**
- Pixels outside bounds are clipped
- Alpha blending is automatic for RGBA images
- Mask controls transparency (0=transparent, 255=opaque)

---

## Utility Operations

### `img.copy()`

Create an independent copy.

**Returns:** `Image`

**Example:**
```python
original = Image.open("photo.jpg")
copy = original.copy()

# Modify copy, original unchanged
copy = copy.blur(5.0)
```

---

## Transformation Pipelines

Chain transforms for complex operations:

```python
# Create thumbnail pipeline
def create_thumbnail(img, size=(150, 150)):
    return (img
        .resize(size, resample="LANCZOS")
        .sharpen(1.2)
        .contrast(1.1))

# Use it
thumb = create_thumbnail(img)
```

```python
# Photo enhancement pipeline
def enhance_photo(img):
    return (img
        .resize((1920, 1080), resample="LANCZOS")
        .brightness(5)
        .contrast(1.15)
        .saturate(1.1)
        .sharpen(1.3))

result = enhance_photo(img)
```

---

## Performance Notes

| Operation | Complexity | Notes |
|-----------|------------|-------|
| `resize()` | O(n×m) | LANCZOS slower than NEAREST |
| `crop()` | O(1) | Very fast |
| `rotate()` | O(n×m) | Arbitrary angles with bilinear interpolation |
| `transpose()` | O(n×m) | Fast |
| `convert()` | O(n×m) | Per-pixel conversion |
| `split()` | O(n×m) | Creates copies |
| `paste()` | O(n×m) | Alpha blending overhead |

---

**See Also:**
- [Filters API](filters.md) - Color filters
- [Drawing API](drawing.md) - Draw shapes
- [Examples](../examples/transform.md) - Transform examples


