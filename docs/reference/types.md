# 📋 Type Reference

Complete reference for all types, enums, and constants in imgrs.

## Core Types

### Color

Color values in imgrs are represented as tuples:

```python
# RGBA tuple (Red, Green, Blue, Alpha)
Color = Tuple[int, int, int, int]

# Examples
red = (255, 0, 0, 255)        # Solid red
semi_transparent = (0, 255, 0, 128)  # 50% transparent green
black = (0, 0, 0, 255)        # Solid black
transparent = (0, 0, 0, 0)    # Fully transparent
```

**Value ranges**: Each component is 0-255

### Size

Image dimensions are represented as tuples:

```python
# Size tuple (Width, Height)
Size = Tuple[int, int]

# Examples
hd = (1920, 1080)     # Full HD
thumbnail = (150, 150) # Square thumbnail
portrait = (1080, 1920) # Vertical
```

### Box Coordinates

Crop regions and bounding boxes:

```python
# Box tuple (X, Y, Width, Height)
Box = Tuple[int, int, int, int]

# Examples
crop_box = (100, 100, 400, 300)  # x=100, y=100, w=400, h=300
```

### Position

Pixel or element positions:

```python
# Position tuple (X, Y)
Position = Tuple[int, int]

# Examples
top_left = (0, 0)
center = (400, 300)
```

## Image Modes

Image color modes define how pixel data is stored:

### Grayscale Modes

| Mode | Description | Channels | Bit Depth |
|------|-------------|----------|-----------|
| `"L"` | Grayscale | 1 | 8-bit |
| `"LA"` | Grayscale + Alpha | 2 | 8-bit |
| `"I"` | Integer grayscale | 1 | 32-bit |

**Example:**
```python
gray_img = Image.new("L", (640, 480), color=(128, 0, 0, 255))
```

### Color Modes

| Mode | Description | Channels | Bit Depth |
|------|-------------|----------|-----------|
| `"RGB"` | True color | 3 | 8-bit per channel |
| `"RGBA"` | True color + Alpha | 4 | 8-bit per channel |

**Example:**
```python
rgb_img = Image.new("RGB", (800, 600), color=(255, 255, 255, 255))
rgba_img = Image.new("RGBA", (800, 600), color=(0, 0, 0, 0))
```

## Image Formats

Supported file formats:

```python
from imgrs.enums import ImageFormat

# Common formats
ImageFormat.JPEG    # JPEG/JPG
ImageFormat.PNG     # PNG (lossless)
ImageFormat.WEBP    # WebP (modern)
ImageFormat.GIF     # GIF (animated)
ImageFormat.BMP     # Bitmap
ImageFormat.TIFF    # TIFF

# Specialty formats
ImageFormat.AVIF    # AVIF (next-gen)
ImageFormat.ICO     # Icons
ImageFormat.TGA     # Targa
ImageFormat.DDS     # DirectDraw Surface
```

**Usage:**
```python
img.save("output.jpg", format="JPEG")
img.save("output.png", format="PNG")
```

## Resampling Filters

Filters used when resizing images:

```python
from imgrs.enums import Resampling

# Quality (slowest to fastest)
Resampling.LANCZOS   # Highest quality, slower
Resampling.BICUBIC   # High quality
Resampling.BILINEAR  # Good quality (default)
Resampling.NEAREST   # Fastest, lowest quality
```

**Usage:**
```python
# High quality resize
img.resize((1920, 1080), resample="LANCZOS")

# Fast resize
img.resize((800, 600), resample="NEAREST")
```

**When to use:**
- **LANCZOS**: Downscaling photos (best quality)
- **BICUBIC**: General purpose resizing
- **BILINEAR**: Fast resizing (default)
- **NEAREST**: Pixel art, very fast

## Transpose Methods

Image flipping and rotation operations:

```python
from imgrs.enums import Transpose

# Flipping
Transpose.FLIP_LEFT_RIGHT   # Mirror horizontally
Transpose.FLIP_TOP_BOTTOM   # Mirror vertically

# Rotation
Transpose.ROTATE_90         # 90° clockwise
Transpose.ROTATE_180        # 180° 
Transpose.ROTATE_270        # 270° clockwise

# Advanced
Transpose.TRANSPOSE         # Transpose (swap x/y)
Transpose.TRANSVERSE        # Transverse
```

**Usage:**
```python
# Mirror image
mirrored = img.transpose("FLIP_LEFT_RIGHT")

# Rotate
rotated = img.transpose("ROTATE_90")
```

## Return Types

### Image Operations

Most operations return a **new Image** instance (immutable):

```python
# Each operation returns new Image
img2 = img.resize((800, 600))     # Returns: Image
img3 = img.blur(5.0)              # Returns: Image
img4 = img.crop((0, 0, 100, 100)) # Returns: Image

# Original unchanged
print(img.size)   # Original size
print(img2.size)  # New size
```

### Property Getters

```python
size: Tuple[int, int] = img.size          # (width, height)
width: int = img.width                     # Width in pixels
height: int = img.height                   # Height in pixels
mode: str = img.mode                       # "RGB", "RGBA", etc.
format: Optional[str] = img.format         # "JPEG", "PNG", etc.
```

### Pixel Operations

```python
# Get single pixel
pixel: Tuple[int, int, int, int] = img.getpixel(100, 100)
# Returns: (R, G, B, A)

# Histogram
hist: Tuple[List[int], List[int], List[int], List[int]] = img.histogram()
# Returns: (R_histogram, G_histogram, B_histogram, A_histogram)

# Dominant color
color: Tuple[int, int, int, int] = img.dominant_color()
# Returns: (R, G, B, A)

# Average color
avg: Tuple[int, int, int, int] = img.average_color()
# Returns: (R, G, B, A)
```

### Channel Operations

```python
# Split into channels
channels: List[Image] = img.split()
# For RGB: returns [R_channel, G_channel, B_channel]
# For RGBA: returns [R, G, B, A]

# Each channel is grayscale ("L" mode)
```

## Type Aliases (Internal)

For developers extending imgrs:

```rust
// Rust type aliases
pub type Color = (u8, u8, u8, u8);
pub type PixelRegion = Vec<Vec<Color>>;
pub type HistogramData = ([u32; 256], [u32; 256], [u32; 256], [u32; 256]);
```

## Constants

### Default Values

```python
# Default resampling
DEFAULT_RESAMPLE = "BILINEAR"

# Default colors
BLACK = (0, 0, 0, 255)
WHITE = (255, 255, 255, 255)
TRANSPARENT = (0, 0, 0, 0)
```

## Optional Dependencies

```python
# NumPy support (optional)
import numpy as np
from imgrs import Image

# Works if numpy is installed
array = np.random.randint(0, 255, (480, 640, 3), dtype=np.uint8)
img = Image.fromarray(array)
```

## Error Types

```python
from imgrs import (
    ImgrsProcessingError,    # General processing errors
    InvalidImageError,       # Invalid image data
    UnsupportedFormatError,  # Unsupported format
    ImgrsIOError            # I/O errors
)

try:
    img = Image.open("missing.jpg")
except ImgrsIOError as e:
    print(f"File not found: {e}")

try:
    img.save("output.xyz", format="INVALID")
except UnsupportedFormatError as e:
    print(f"Format not supported: {e}")
```

## Type Hints

imgrs is fully typed for IDE support:

```python
from imgrs import Image
from typing import Tuple, Optional

def process_image(path: str, size: Tuple[int, int]) -> Image:
    """Process an image with type hints."""
    img: Image = Image.open(path)
    resized: Image = img.resize(size)
    return resized

# Your IDE will provide autocomplete and type checking!
```

---

**Next:** Learn [Basic Usage](../guides/basic-usage.md) or explore [API Reference](../api/)

