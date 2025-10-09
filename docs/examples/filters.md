# 🎨 Filter Examples

Practical examples of using filters in imgrs.

## Basic Filters

### Blur Examples

```python
from imgrs import Image

img = Image.open("photo.jpg")

# Light blur - reduce noise
denoised = img.blur(1.5)

# Medium blur - soft focus
soft = img.blur(5.0)

# Heavy blur - background blur
background = img.blur(15.0)

# Extreme blur - abstract
abstract = img.blur(30.0)
```

### Sharpen Examples

```python
# Subtle sharpen - enhance details
enhanced = img.sharpen(0.8)

# Normal sharpen - fix soft photos
sharpened = img.sharpen(1.5)

# Strong sharpen - dramatic
dramatic = img.sharpen(3.0)

# Extreme sharpen - artistic
artistic = img.sharpen(5.0)
```

## Photo Enhancement

### Auto-Enhance

```python
def auto_enhance(img):
    """Automatically enhance a photo."""
    return (img
        .blur(1.0)              # Slight denoise
        .brightness(10)         # Brighten
        .contrast(1.15)         # Add contrast
        .saturate(1.1)          # Boost colors
        .sharpen(1.3))          # Add sharpness

enhanced = auto_enhance(img)
enhanced.save("enhanced.jpg")
```

### Fix Dark Photo

```python
def fix_dark_photo(img):
    """Fix underexposed photo."""
    return (img
        .brightness(40)         # Significant brighten
        .contrast(1.3)          # Increase contrast
        .saturate(1.2))         # Boost saturation

fixed = fix_dark_photo(dark_img)
```

### Fix Overexposed Photo

```python
def fix_bright_photo(img):
    """Fix overexposed photo."""
    return (img
        .brightness(-30)        # Darken
        .contrast(1.2)          # Add contrast
        .saturate(1.1))         # Restore color

fixed = fix_bright_photo(bright_img)
```

## Creative Effects

### Vintage Look

```python
def vintage_effect(img):
    """Create vintage photo effect."""
    return (img
        .sepia(0.6)             # Sepia tone
        .brightness(-15)        # Slightly darker
        .contrast(0.9)          # Reduce contrast
        .blur(0.8))             # Slight softness

vintage = vintage_effect(img)
```

### High Contrast Black & White

```python
def dramatic_bw(img):
    """Dramatic black and white."""
    return (img
        .convert("L")           # Grayscale
        .contrast(1.8)          # High contrast
        .brightness(10))        # Slight brighten

bw = dramatic_bw(img)
```

### Color Pop

```python
def color_pop(img):
    """Make colors pop."""
    return (img
        .saturate(1.4)          # Boost saturation
        .contrast(1.2)          # Increase contrast
        .sharpen(1.5)           # Add sharpness
        .brightness(5))         # Slight brighten

vibrant = color_pop(img)
```

### Dream/Ethereal Look

```python
def dreamy_effect(img):
    """Soft, dreamy effect."""
    return (img
        .blur(3.0)              # Soft focus
        .brightness(20)         # Lighten
        .saturate(0.8)          # Desaturate slightly
        .contrast(0.9))         # Reduce contrast

dreamy = dreamy_effect(img)
```

## Filter Presets

### Instagram-Style Filters

```python
def filter_nashville(img):
    """Nashville-style filter."""
    return (img
        .saturate(1.2)
        .brightness(10)
        .contrast(1.1)
        .sepia(0.2))

def filter_1977(img):
    """1977-style filter."""
    return (img
        .sepia(0.3)
        .saturate(1.3)
        .brightness(5)
        .contrast(1.1))

def filter_hudson(img):
    """Hudson-style filter."""
    return (img
        .brightness(15)
        .saturate(1.2)
        .contrast(0.9))

# Apply filters
nashville = filter_nashville(img)
retro = filter_1977(img)
hudson = filter_hudson(img)
```

### Artistic Filters

```python
def pencil_sketch(img):
    """Pencil sketch effect."""
    edges = img.edge_detect()
    return edges.invert(1.0)

def oil_painting(img):
    """Oil painting effect."""
    return (img
        .blur(2.0)
        .posterize(8)
        .sharpen(0.5))

def watercolor(img):
    """Watercolor effect."""
    return (img
        .blur(4.0)
        .posterize(12)
        .saturate(1.3))

# Apply artistic filters
sketch = pencil_sketch(img)
oil = oil_painting(img)
watercolor = watercolor(img)
```

## Batch Filter Application

### Apply Filter to Multiple Images

```python
from pathlib import Path

def apply_filter_batch(input_dir, output_dir, filter_func):
    """Apply filter to all images in directory."""
    Path(output_dir).mkdir(exist_ok=True)
    
    for img_path in Path(input_dir).glob("*.jpg"):
        img = Image.open(img_path)
        filtered = filter_func(img)
        
        output_path = Path(output_dir) / img_path.name
        filtered.save(output_path)
        print(f"✅ Processed: {img_path.name}")

# Use it
apply_filter_batch("photos/", "vintage/", vintage_effect)
apply_filter_batch("photos/", "bw/", dramatic_bw)
```

### Create Filter Variants

```python
def create_variants(img, base_name):
    """Create multiple filtered versions."""
    variants = {
        "original": img,
        "bw": img.convert("L"),
        "sepia": img.sepia(0.7),
        "vibrant": img.saturate(1.5),
        "vintage": vintage_effect(img),
        "sharp": img.sharpen(2.0),
        "soft": img.blur(3.0),
    }
    
    for name, variant in variants.items():
        variant.save(f"{base_name}_{name}.jpg")

# Create 7 versions
create_variants(img, "photo")
```

## Performance Tips

### Progressive Enhancement

```python
# ❌ Slow - applying many filters
result = img
for _ in range(10):
    result = result.blur(1.0)
    result = result.sharpen(1.0)

# ✅ Fast - chain operations
result = (img
    .blur(10.0)         # One blur instead of 10
    .sharpen(10.0))     # One sharpen instead of 10
```

### Resize Before Filtering

```python
# ❌ Slower - filter large image
large = Image.open("huge_photo.jpg")  # 4000x3000
filtered = large.blur(10.0)  # Slow on large image

# ✅ Faster - resize then filter
large = Image.open("huge_photo.jpg")
small = large.resize((1920, 1080), resample="LANCZOS")
filtered = small.blur(10.0)  # Much faster!
```

## Real-World Examples

### Product Photo Enhancement

```python
def enhance_product_photo(img):
    """Enhance product photos for e-commerce."""
    return (img
        .resize((1200, 1200), resample="LANCZOS")
        .brightness(15)
        .contrast(1.2)
        .saturate(1.15)
        .sharpen(1.8))

product = enhance_product_photo(raw_photo)
product.save("product_enhanced.jpg")
```

### Portrait Enhancement

```python
def enhance_portrait(img):
    """Enhance portrait photos."""
    return (img
        .blur(0.5)              # Slight skin smoothing
        .brightness(8)          # Brighten face
        .contrast(1.1)          # Subtle contrast
        .saturate(1.05))        # Natural color

portrait = enhance_portrait(selfie)
```

### Landscape Enhancement

```python
def enhance_landscape(img):
    """Enhance landscape photos."""
    return (img
        .saturate(1.3)          # Vivid colors
        .contrast(1.2)          # Sky contrast
        .sharpen(1.4)           # Sharp details
        .brightness(5))         # Slight brighten

landscape = enhance_landscape(scenery)
```

---

**See Also:**
- [Filters API Reference](../api/filters.md)
- [Transform Examples](transform.md)
- [Effects Examples](effects.md)

