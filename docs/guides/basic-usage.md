# 📚 Basic Usage Guide

Learn the fundamentals of imgrs with step-by-step examples.

## Table of Contents

1. [Opening and Saving](#opening-and-saving)
2. [Image Properties](#image-properties)
3. [Resizing Images](#resizing-images)
4. [Cropping Images](#cropping-images)
5. [Rotating and Flipping](#rotating-and-flipping)
6. [Applying Filters](#applying-filters)
7. [Working with Colors](#working-with-colors)

## Opening and Saving

### Opening Images

```python
from imgrs import Image

# From file
img = Image.open("photo.jpg")

# From Path
from pathlib import Path
img = Image.open(Path("photos/image.png"))

# From bytes
with open("image.jpg", "rb") as f:
    img = Image.open(f.read())
```

### Saving Images

```python
# Save with auto-detected format
img.save("output.png")

# Save with specific format
img.save("output.jpg", format="JPEG")

# Save multiple formats
for fmt in ["PNG", "JPEG", "WEBP"]:
    img.save(f"output.{fmt.lower()}", format=fmt)
```

## Image Properties

### Dimensions

```python
# Get size tuple
width, height = img.size

# Get individual dimensions
w = img.width
h = img.height

print(f"Image: {w}x{h} pixels")
```

### Mode and Format

```python
# Color mode
print(f"Mode: {img.mode}")  # RGB, RGBA, L, LA

# Original format (if opened from file)
print(f"Format: {img.format}")  # JPEG, PNG, etc.

# Check for alpha channel
has_alpha = img.mode in ["RGBA", "LA"]
```

## Resizing Images

### Basic Resize

```python
# Resize to specific dimensions
resized = img.resize((800, 600))

# Create thumbnail
thumb = img.resize((150, 150))
```

### Quality Options

```python
# Highest quality (best for photos)
hq = img.resize((1920, 1080), resample="LANCZOS")

# Fast resize (good for thumbnails)
fast = img.resize((200, 200), resample="NEAREST")

# Balanced (default)
balanced = img.resize((1280, 720), resample="BILINEAR")
```

### Maintain Aspect Ratio

```python
def resize_with_aspect(img, max_width, max_height):
    """Resize maintaining aspect ratio."""
    width, height = img.size
    aspect = width / height
    
    if width > height:
        new_width = max_width
        new_height = int(max_width / aspect)
    else:
        new_height = max_height
        new_width = int(max_height * aspect)
    
    return img.resize((new_width, new_height), resample="LANCZOS")

# Use it
scaled = resize_with_aspect(img, 1920, 1080)
```

## Cropping Images

### Basic Crop

```python
# Crop to region: x=100, y=100, width=400, height=300
cropped = img.crop((100, 100, 400, 300))
```

### Crop to Center

```python
def crop_center(img, crop_width, crop_height):
    """Crop image to center region."""
    width, height = img.size
    left = (width - crop_width) // 2
    top = (height - crop_height) // 2
    return img.crop((left, top, crop_width, crop_height))

# Crop to center 800x600
centered = crop_center(img, 800, 600)
```

### Extract Regions

```python
# Get quarters
w, h = img.size
top_left = img.crop((0, 0, w//2, h//2))
top_right = img.crop((w//2, 0, w//2, h//2))
bottom_left = img.crop((0, h//2, w//2, h//2))
bottom_right = img.crop((w//2, h//2, w//2, h//2))
```

## Rotating and Flipping

### Rotation

```python
# Rotate 90° clockwise
rotated_90 = img.rotate(90)

# Rotate 180°
rotated_180 = img.rotate(180)

# Rotate 270° (90° counter-clockwise)
rotated_270 = img.rotate(270)
```

### Flipping

```python
# Mirror horizontally
mirrored = img.transpose("FLIP_LEFT_RIGHT")

# Mirror vertically
upside_down = img.transpose("FLIP_TOP_BOTTOM")

# Both (same as 180° rotation)
flipped_both = img.transpose("ROTATE_180")
```

## Applying Filters

### Basic Filters

```python
# Blur
blurred = img.blur(5.0)

# Sharpen
sharp = img.sharpen(2.0)

# Brightness
brighter = img.brightness(20)
darker = img.brightness(-20)

# Contrast
high_contrast = img.contrast(1.5)
low_contrast = img.contrast(0.7)
```

### CSS Filters

```python
# Sepia tone
vintage = img.sepia(0.7)

# Grayscale
gray = img.grayscale_filter(1.0)

# Invert
negative = img.invert(1.0)

# Hue rotation
color_shifted = img.hue_rotate(120)

# Saturation
vibrant = img.saturate(1.5)
muted = img.saturate(0.5)
```

### Filter Chains

```python
# Enhance photo
enhanced = (img
    .brightness(10)
    .contrast(1.2)
    .saturate(1.1)
    .sharpen(1.3))

# Vintage look
vintage = (img
    .sepia(0.4)
    .brightness(-10)
    .contrast(1.1))

# Dramatic black and white
dramatic_bw = (img
    .convert("L")
    .contrast(1.5)
    .brightness(5))
```

## Working with Colors

### Convert Between Modes

```python
# Color to grayscale
gray = img.convert("L")

# Add transparency
rgba = img.convert("RGBA")

# Remove transparency
rgb = img.convert("RGB")
```

### Split and Merge Channels

```python
# Split into channels
r, g, b = img.split()

# Save individual channels
r.save("red_channel.png")
g.save("green_channel.png")
b.save("blue_channel.png")

# Analyze channels
r_avg = r.average_color()
print(f"Average red: {r_avg[0]}")
```

### Color Analysis

```python
# Get average color
avg_r, avg_g, avg_b, avg_a = img.average_color()
print(f"Average: RGB({avg_r}, {avg_g}, {avg_b})")

# Get dominant color
dom_r, dom_g, dom_b, dom_a = img.dominant_color()
print(f"Most common: RGB({dom_r}, {dom_g}, {dom_b})")

# Get histogram
r_hist, g_hist, b_hist, a_hist = img.histogram()
print(f"Red distribution: min={min(r_hist)}, max={max(r_hist)}")
```

## Complete Examples

### Create Thumbnail Gallery

```python
from imgrs import Image
from pathlib import Path

def create_thumbnails(input_dir, output_dir, size=(150, 150)):
    """Create thumbnails for all images in directory."""
    Path(output_dir).mkdir(exist_ok=True)
    
    for img_path in Path(input_dir).glob("*.jpg"):
        img = Image.open(img_path)
        thumb = img.resize(size, resample="LANCZOS")
        
        output_path = Path(output_dir) / f"thumb_{img_path.name}"
        thumb.save(output_path, format="JPEG")
        print(f"✅ Created: {output_path}")

# Use it
create_thumbnails("photos/", "thumbnails/")
```

### Add Watermark

```python
def add_watermark(img, text="© 2025", position="bottom-right"):
    """Add text watermark to image."""
    w, h = img.size
    color = (255, 255, 255, 180)  # Semi-transparent white
    scale = 2
    
    # Calculate position
    if position == "bottom-right":
        x = w - len(text) * 8 * scale - 10
        y = h - 8 * scale - 10
    elif position == "bottom-left":
        x = 10
        y = h - 8 * scale - 10
    elif position == "top-right":
        x = w - len(text) * 8 * scale - 10
        y = 10
    else:  # top-left
        x, y = 10, 10
    
    return img.draw_text(text, x, y, color, scale)

# Use it
watermarked = add_watermark(img, "© MYNAME", "bottom-right")
watermarked.save("watermarked.jpg")
```

### Batch Convert Format

```python
from pathlib import Path

def convert_all_to_webp(input_dir, quality="high"):
    """Convert all JPEGs to WebP."""
    for jpg_path in Path(input_dir).glob("*.jpg"):
        img = Image.open(jpg_path)
        
        # Optimize: resize if too large
        if img.width > 1920:
            img = img.resize((1920, 1080), resample="LANCZOS")
        
        # Save as WebP
        webp_path = jpg_path.with_suffix(".webp")
        img.save(webp_path, format="WEBP")
        print(f"✅ Converted: {webp_path}")

# Use it
convert_all_to_webp("photos/")
```

### Photo Enhancement

```python
def enhance_photo(img):
    """Auto-enhance a photo."""
    # Resize to standard size
    img = img.resize((1920, 1080), resample="LANCZOS")
    
    # Subtle enhancements
    img = (img
        .brightness(5)          # Slight brighten
        .contrast(1.15)         # Increase contrast
        .saturate(1.1)          # Boost colors
        .sharpen(1.2))          # Add sharpness
    
    return img

# Enhance all photos
for path in Path("raw/").glob("*.jpg"):
    img = Image.open(path)
    enhanced = enhance_photo(img)
    enhanced.save(f"enhanced/{path.name}")
```

---

**Next:** Explore [Filters](filters.md) or [Drawing](drawing.md) examples

