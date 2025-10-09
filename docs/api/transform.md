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

### `img.rotate(angle)`

Rotate image by specified angle.

**Parameters:**
- `angle` (float): Rotation angle in degrees
  - Only 90, 180, 270 are supported

**Returns:** `Image`

**Example:**
```python
# Rotate 90° clockwise
rotated = img.rotate(90)

# Rotate 180°
flipped = img.rotate(180)

# Rotate 270° (same as 90° counter-clockwise)
ccw = img.rotate(270)
```

**Note:** For arbitrary angles, use external libraries. Only orthogonal rotations supported.

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
| `rotate()` | O(n×m) | Fast, orthogonal only |
| `transpose()` | O(n×m) | Fast |
| `convert()` | O(n×m) | Per-pixel conversion |
| `split()` | O(n×m) | Creates copies |
| `paste()` | O(n×m) | Alpha blending overhead |

---

**See Also:**
- [Filters API](filters.md) - Color filters
- [Drawing API](drawing.md) - Draw shapes
- [Examples](../examples/transform.md) - Transform examples

