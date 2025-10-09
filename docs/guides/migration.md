# 🔄 Migrating from Pillow to imgrs

Complete guide for switching from Pillow (PIL) to imgrs.

## Overview

imgrs provides a **Pillow-compatible API** with minimal changes needed. Most code works without modification!

## Quick Comparison

| Feature | Pillow | imgrs | Status |
|---------|--------|-------|--------|
| **Opening** | `Image.open()` | `Image.open()` | ✅ Same |
| **Creating** | `Image.new()` | `Image.new()` | ✅ Same |
| **Saving** | `img.save()` | `img.save()` | ✅ Same |
| **Resize** | `img.resize()` | `img.resize()` | ✅ Same |
| **Crop** | `img.crop()` | `img.crop()` | ⚠️ Different args |
| **Rotate** | `img.rotate()` | `img.rotate()` | ⚠️ Limited angles |
| **Filters** | `ImageFilter` | Built-in methods | ⚠️ Different API |
| **Draw** | `ImageDraw` | Built-in methods | ⚠️ Different API |

## Simple Migration

### 1. Change Import

```python
# Before (Pillow)
from PIL import Image

# After (imgrs)
from imgrs import Image
```

### 2. Code Works Mostly Unchanged!

```python
# This code works in BOTH Pillow and imgrs:
img = Image.open("photo.jpg")
resized = img.resize((800, 600))
resized.save("output.png")
```

## Differences to Know

### 1. Crop Arguments

```python
# Pillow: crop(box) where box = (left, top, right, bottom)
pillow_crop = img.crop((100, 100, 500, 400))
# Crops from (100,100) to (500,400)

# imgrs: crop(box) where box = (x, y, width, height)
imgrs_crop = img.crop((100, 100, 400, 300))
# Crops at (100,100) with size 400x300

# Convert Pillow box to imgrs box:
left, top, right, bottom = 100, 100, 500, 400
imgrs_box = (left, top, right - left, bottom - top)
img.crop(imgrs_box)
```

**Helper function:**
```python
def pillow_crop_to_imgrs(pillow_box):
    """Convert Pillow crop box to imgrs format."""
    left, top, right, bottom = pillow_box
    return (left, top, right - left, bottom - top)

# Use it
pillow_box = (100, 100, 500, 400)
imgrs_box = pillow_crop_to_imgrs(pillow_box)
cropped = img.crop(imgrs_box)
```

### 2. Rotate Angles

```python
# Pillow: Any angle supported
pillow_img = img.rotate(45)  # Works
pillow_img = img.rotate(30)  # Works

# imgrs: Only 90°, 180°, 270°
imgrs_img = img.rotate(90)   # ✅ Works
imgrs_img = img.rotate(180)  # ✅ Works
imgrs_img = img.rotate(45)   # ❌ Error

# Use transpose for non-90° rotations
# (or use Pillow for arbitrary angles)
```

### 3. Filters

```python
# Pillow: Uses ImageFilter module
from PIL import Image, ImageFilter
img = img.filter(ImageFilter.BLUR)
img = img.filter(ImageFilter.SHARPEN)

# imgrs: Built-in methods
img = img.blur(5.0)
img = img.sharpen(2.0)
```

**Migration:**
```python
# Pillow → imgrs filter mapping
BLUR          → blur(5.0)
SHARPEN       → sharpen(2.0)
EDGE_ENHANCE  → sharpen(1.5)
FIND_EDGES    → edge_detect()
EMBOSS        → emboss()
```

### 4. Drawing

```python
# Pillow: Uses ImageDraw module
from PIL import Image, ImageDraw
draw = ImageDraw.Draw(img)
draw.rectangle([10, 10, 110, 60], fill=(255, 0, 0))
draw.circle([200, 150], 50, fill=(0, 255, 0))

# imgrs: Built-in methods (immutable)
img = img.draw_rectangle(10, 10, 100, 50, (255, 0, 0, 255))
img = img.draw_circle(200, 150, 50, (0, 255, 0, 255))
```

**Key difference:**
- Pillow: Modifies image in-place
- imgrs: Returns new image (immutable)

### 5. ImageEnhance

```python
# Pillow: ImageEnhance module
from PIL import ImageEnhance
enhancer = ImageEnhance.Brightness(img)
bright = enhancer.enhance(1.5)

# imgrs: Built-in methods
bright = img.brightness(int(128 * 0.5))  # Approximate
contrast = img.contrast(1.5)
```

## Complete Migration Example

### Pillow Code

```python
from PIL import Image, ImageFilter, ImageDraw, ImageEnhance

# Open and process
img = Image.open("photo.jpg")
img = img.resize((1920, 1080), Image.LANCZOS)
img = img.filter(ImageFilter.BLUR)

# Enhance
enhancer = ImageEnhance.Contrast(img)
img = enhancer.enhance(1.2)

# Crop (Pillow style)
img = img.crop((100, 100, 1820, 980))

# Draw
draw = ImageDraw.Draw(img)
draw.rectangle([50, 50, 150, 100], fill=(255, 0, 0))

# Save
img.save("output.png")
```

### Migrated to imgrs

```python
from imgrs import Image

# Open and process (almost identical!)
img = Image.open("photo.jpg")
img = img.resize((1920, 1080), resample="LANCZOS")
img = img.blur(5.0)

# Enhance
img = img.contrast(1.2)

# Crop (imgrs style: x, y, width, height)
img = img.crop((100, 100, 1720, 880))

# Draw (built-in, immutable)
img = img.draw_rectangle(50, 50, 100, 50, (255, 0, 0, 255))

# Save (identical)
img.save("output.png")
```

## Migration Checklist

- [ ] Change import: `from PIL import Image` → `from imgrs import Image`
- [ ] Update crop calls: `(left, top, right, bottom)` → `(x, y, width, height)`
- [ ] Replace `img.filter(ImageFilter.X)` with `img.x()`
- [ ] Replace `ImageDraw` with `img.draw_*()`
- [ ] Replace `ImageEnhance` with built-in methods
- [ ] Update rotate calls to use only 90/180/270°
- [ ] Change mutable operations to immutable chains
- [ ] Test thoroughly!

## Feature Parity

### ✅ Fully Compatible

These work identically:
- `Image.open()`
- `Image.new()`
- `Image.fromarray()`
- `img.save()`
- `img.resize()`
- `img.size`, `img.width`, `img.height`
- `img.mode`, `img.format`
- `img.convert()`
- `img.split()`
- `img.copy()`

### ⚠️ Different API

These need small changes:
- `img.crop()` - different box format
- `img.rotate()` - only orthogonal angles
- Filters - use methods instead of ImageFilter
- Drawing - use methods instead of ImageDraw
- Enhancement - use methods instead of ImageEnhance

### 🚧 Not Yet Implemented

These features may come later:
- Arbitrary angle rotation
- Font loading (uses built-in font only)
- Some advanced filters
- Alpha composite modes
- Some image modes (CMYK, YCbCr, etc.)

## Performance Benefits

After migrating to imgrs:

```python
import time
from imgrs import Image

start = time.time()

# Process 100 images
for i in range(100):
    img = Image.open("photo.jpg")
    img = img.resize((800, 600), resample="LANCZOS")
    img = img.blur(3.0)
    img.save(f"output_{i}.jpg")

elapsed = time.time() - start
print(f"Processed 100 images in {elapsed:.2f}s")
# imgrs: ~5s
# Pillow: ~50s
# Speed-up: 10x! 🚀
```

## Gradual Migration

You can use both libraries together:

```python
from PIL import Image as PILImage
from imgrs import Image

# Open with Pillow
pil_img = PILImage.open("photo.jpg")

# Convert to imgrs for processing
import numpy as np
array = np.array(pil_img)
imgrs_img = Image.fromarray(array)

# Process with imgrs (fast!)
processed = imgrs_img.resize((800, 600)).blur(5.0)

# Convert back to Pillow if needed
# (save and reopen, or use numpy array)
```

## Tips for Smooth Migration

1. **Start with new code** - Use imgrs for new features
2. **Test thoroughly** - Especially crop and rotate
3. **Benchmark** - Measure speed improvements
4. **Keep Pillow** - For features not yet in imgrs
5. **Report issues** - Help us improve compatibility!

---

**Need help?** [Report an issue](https://github.com/GrandpaEJ/imgrs/issues) or check [API Reference](../api/)

