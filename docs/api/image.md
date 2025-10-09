# 📦 Image Class API

Complete reference for the `Image` class - the core of imgrs.

## Class: `Image`

The main image object that represents a raster image.

```python
from imgrs import Image
```

## Constructors

### `Image.open(fp, mode=None, formats=None)`

Open an image file.

**Parameters:**
- `fp` (str | Path | bytes): File path or bytes
- `mode` (str, optional): Mode hint (not yet implemented)
- `formats` (list, optional): Formats to try (not yet implemented)

**Returns:** `Image`

**Example:**
```python
# From file path
img = Image.open("photo.jpg")

# From Path object
from pathlib import Path
img = Image.open(Path("photo.png"))

# From bytes
with open("photo.jpg", "rb") as f:
    img = Image.open(f.read())
```

---

### `Image.new(mode, size, color=0)`

Create a new image.

**Parameters:**
- `mode` (str): Image mode ("RGB", "RGBA", "L", "LA")
- `size` (Tuple[int, int]): Image size (width, height)
- `color` (int | Tuple[int, ...], optional): Fill color

**Returns:** `Image`

**Example:**
```python
# White RGB image
img = Image.new("RGB", (800, 600), color=(255, 255, 255))

# Transparent RGBA image
img = Image.new("RGBA", (400, 300), color=(0, 0, 0, 0))

# Gray image
img = Image.new("L", (640, 480), color=(128, 0, 0, 255))
```

---

### `Image.fromarray(array, mode=None)`

Create an image from a NumPy array.

**Parameters:**
- `array` (numpy.ndarray): NumPy array
  - 2D array (H, W) for grayscale
  - 3D array (H, W, 3) for RGB
  - 3D array (H, W, 4) for RGBA
- `mode` (str, optional): Mode hint

**Returns:** `Image`

**Example:**
```python
import numpy as np

# RGB image from array
array = np.random.randint(0, 255, (480, 640, 3), dtype=np.uint8)
img = Image.fromarray(array)

# Grayscale image
gray = np.zeros((480, 640), dtype=np.uint8)
img = Image.fromarray(gray)
```

---

## Properties

### `img.size`

Get image dimensions.

**Returns:** `Tuple[int, int]` - (width, height)

**Example:**
```python
width, height = img.size
print(f"Image is {width}x{height} pixels")
```

---

### `img.width`

Get image width.

**Returns:** `int`

**Example:**
```python
w = img.width  # e.g., 1920
```

---

### `img.height`

Get image height.

**Returns:** `int`

**Example:**
```python
h = img.height  # e.g., 1080
```

---

### `img.mode`

Get image color mode.

**Returns:** `str` - One of: "L", "LA", "RGB", "RGBA", "I"

**Example:**
```python
mode = img.mode  # "RGB"
if mode == "RGBA":
    print("Image has transparency")
```

---

### `img.format`

Get original file format (if opened from file).

**Returns:** `Optional[str]` - e.g., "JPEG", "PNG", or None

**Example:**
```python
fmt = img.format
if fmt == "JPEG":
    print("Opened from JPEG file")
```

---

## I/O Operations

### `img.save(fp, format=None)`

Save image to file.

**Parameters:**
- `fp` (str | Path): Output file path
- `format` (str, optional): Force format ("JPEG", "PNG", etc.)

**Returns:** `None`

**Example:**
```python
# Save (format auto-detected from extension)
img.save("output.png")

# Force format
img.save("output.jpg", format="JPEG")
```

---

### `img.to_bytes()`

Get raw pixel data as bytes.

**Returns:** `bytes`

**Example:**
```python
data = img.to_bytes()
print(f"Image data: {len(data)} bytes")
```

---

### `img.copy()`

Create a copy of the image.

**Returns:** `Image`

**Example:**
```python
copy = img.copy()
# Modify copy without affecting original
```

---

## See Also

- [Filters API](filters.md) - Image filters
- [Drawing API](drawing.md) - Drawing operations
- [Pixels API](pixels.md) - Pixel manipulation
- [Transform API](transform.md) - Transformations

---

**Next:** Explore specific operations in the API sections above.

