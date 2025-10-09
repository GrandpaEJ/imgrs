# 📷 Basic Examples

Simple examples to get you started.

## Opening Images

### From File Path

```python
from imgrs import Image

# Open JPEG
img = Image.open("photo.jpg")

# Open PNG
img = Image.open("screenshot.png")

# Open from Path object
from pathlib import Path
img = Image.open(Path("images/photo.jpg"))
```

### From Bytes

```python
# Read from file
with open("photo.jpg", "rb") as f:
    data = f.read()
    img = Image.open(data)

# From HTTP response
import requests
response = requests.get("https://example.com/image.jpg")
img = Image.open(response.content)

# From BytesIO
from io import BytesIO
buffer = BytesIO(image_bytes)
img = Image.open(buffer.read())
```

## Creating Images

### Blank Images

```python
# White RGB image
white = Image.new("RGB", (800, 600), color=(255, 255, 255))

# Black image
black = Image.new("RGB", (640, 480), color=(0, 0, 0))

# Colored background
red_bg = Image.new("RGB", (400, 300), color=(255, 0, 0))
```

### Transparent Images

```python
# Fully transparent
transparent = Image.new("RGBA", (400, 300), color=(0, 0, 0, 0))

# Semi-transparent gray
gray_trans = Image.new("RGBA", (500, 400), color=(128, 128, 128, 128))
```

### From NumPy Array

```python
import numpy as np

# Random image
array = np.random.randint(0, 255, (480, 640, 3), dtype=np.uint8)
img = Image.fromarray(array)

# Gradient image
height, width = 400, 600
gradient = np.zeros((height, width, 3), dtype=np.uint8)
for y in range(height):
    gradient[y, :] = [0, int(255 * y / height), 255]
img = Image.fromarray(gradient)
```

## Saving Images

### Basic Save

```python
# Save (format auto-detected)
img.save("output.png")
img.save("output.jpg")
img.save("output.webp")
```

### Specify Format

```python
# Force JPEG even with .png extension
img.save("output.png", format="JPEG")

# Save as WebP
img.save("optimized.webp", format="WEBP")

# Save as PNG
img.save("lossless.png", format="PNG")
```

### Multiple Formats

```python
# Save in multiple formats
for fmt in ["JPEG", "PNG", "WEBP"]:
    img.save(f"output.{fmt.lower()}", format=fmt)
```

## Getting Image Info

### Dimensions

```python
# Get size
width, height = img.size
print(f"Image is {width}x{height}")

# Individual values
w = img.width
h = img.height

# Check aspect ratio
aspect = w / h
if aspect > 1:
    print("Landscape")
elif aspect < 1:
    print("Portrait")
else:
    print("Square")
```

### Mode and Format

```python
# Check color mode
mode = img.mode
print(f"Mode: {mode}")  # RGB, RGBA, L, etc.

# Check format
fmt = img.format
if fmt:
    print(f"Original format: {fmt}")
else:
    print("Created programmatically")

# Check for transparency
if mode in ["RGBA", "LA"]:
    print("Image has alpha channel")
```

## Complete Workflow

```python
from imgrs import Image

# Open image
img = Image.open("input.jpg")
print(f"Loaded: {img.size}, {img.mode}, {img.format}")

# Process
processed = (img
    .resize((1920, 1080), resample="LANCZOS")
    .blur(2.0)
    .brightness(5)
    .contrast(1.1))

# Save result
processed.save("output.png", format="PNG")
print("✅ Done!")
```

---

**Next:** Learn about [Filters](filters.md) or [Transform](transform.md) operations

