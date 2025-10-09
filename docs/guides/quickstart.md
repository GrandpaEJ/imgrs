# 🚀 Quick Start Guide

Get started with imgrs in **5 minutes**!

## Installation

```bash
pip install imgrs
```

That's it! No compilation needed - we provide pre-built wheels for all platforms.

## Your First Image

### 1. Open an Image

```python
from imgrs import Image

# Open from file path
img = Image.open("photo.jpg")

# Open from bytes
with open("photo.jpg", "rb") as f:
    img = Image.open(f.read())
```

### 2. Get Image Information

```python
# Get dimensions
width, height = img.size
print(f"Image size: {width}x{height}")

# Get mode
print(f"Mode: {img.mode}")  # RGB, RGBA, L, etc.

# Get format
print(f"Format: {img.format}")  # JPEG, PNG, etc.
```

### 3. Basic Operations

```python
# Resize image
resized = img.resize((800, 600))

# Crop image
cropped = img.crop((100, 100, 500, 500))  # (x, y, width, height)

# Rotate image
rotated = img.rotate(90)

# Flip image
flipped = img.transpose("FLIP_LEFT_RIGHT")
```

### 4. Apply Filters

```python
# Blur
blurred = img.blur(5.0)

# Sharpen
sharp = img.sharpen(2.0)

# Adjust brightness
bright = img.brightness(30)

# Adjust contrast
contrast = img.contrast(1.5)
```

### 5. Save Your Work

```python
# Save to file
img.save("output.png")

# Save with specific format
img.save("output.jpg", format="JPEG")

# Get as bytes
data = img.to_bytes()
```

## Common Workflows

### Thumbnail Generation

```python
from imgrs import Image

img = Image.open("large_photo.jpg")
thumb = img.resize((150, 150), resample="LANCZOS")
thumb.save("thumbnail.jpg", format="JPEG")
```

### Apply Multiple Filters

```python
from imgrs import Image

img = Image.open("photo.jpg")
processed = (img
    .resize((1200, 800))
    .blur(2.0)
    .brightness(10)
    .contrast(1.2)
    .sharpen(1.5))
processed.save("enhanced.jpg")
```

### Create Image from Scratch

```python
from imgrs import Image

# Create blank RGB image
img = Image.new("RGB", (400, 300), color=(255, 255, 255))

# Draw on it
img = img.draw_rectangle(50, 50, 100, 100, (255, 0, 0, 255))
img = img.draw_circle(200, 150, 50, (0, 255, 0, 255))

img.save("shapes.png")
```

### Work with NumPy

```python
from imgrs import Image
import numpy as np

# From NumPy array
array = np.random.randint(0, 255, (480, 640, 3), dtype=np.uint8)
img = Image.fromarray(array)

# To NumPy array (coming soon)
# array = np.array(img)
```

## Next Steps

- 📖 Read the [Basic Usage Guide](basic-usage.md)
- 🎨 Explore [Examples](../examples/)
- 📚 Check [API Reference](../api/)
- 🔥 Learn [Performance Tips](performance.md)

## Key Differences from Pillow

imgrs is **mostly compatible** with Pillow, but with some differences:

✅ **Same API**
- `Image.open()`, `Image.new()`, `Image.fromarray()`
- `resize()`, `crop()`, `rotate()`, `save()`
- Most common operations work identically

⚡ **Much Faster**
- 10-100x speedup on most operations
- Rust-powered performance
- Parallel processing ready

🔄 **Method Chaining**
```python
# imgrs returns new images (immutable)
result = img.resize((800, 600)).blur(5).save("out.jpg")

# Pillow often modifies in-place
```

---

**Ready to dive deeper?** Check out the [API Reference](../api/) or [Examples](../examples/)!

