# 📚 imgrs Complete API Reference

Complete reference for all imgrs image processing APIs with examples and comments.

## Quick Start

```python
# Import the main Image class
from imgrs import Image

# Open an image from file
img = Image.open("input_img.png")

# Create a new image
new_img = Image.new("RGB", (800, 600), color=(255, 255, 255))

# Save the result
new_img.save("output.png")
```

---

## 📦 Image Class API

### Image Constructors

#### `Image.open(fp, mode=None, formats=None)`
Open an image from file, Path object, or bytes.

**Parameters:**
- `fp` (str | Path | bytes): File path or bytes
- `mode` (str, optional): Mode hint
- `formats` (list, optional): Formats to try

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

#### `Image.new(mode, size, color=0)`
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
img = Image.new("L", (640, 480), color=(128))
```

#### `Image.fromarray(array, mode=None)`
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

### Image Properties

```python
# Get image dimensions
width, height = img.size

# Get individual dimensions
w = img.width  # e.g., 1920
h = img.height  # e.g., 1080

# Get color mode
mode = img.mode  # "RGB", "RGBA", "L", "LA", "I"

# Get original format (if opened from file)
fmt = img.format  # "JPEG", "PNG", or None
```

### Basic I/O Operations

#### `img.save(fp, format=None)`
Save image to file.

**Parameters:**
- `fp` (str | Path): Output file path
- `format` (str, optional): Force format ("JPEG", "PNG", etc.)

**Example:**
```python
# Save (format auto-detected from extension)
img.save("output.png")

# Force format
img.save("output.jpg", format="JPEG")
```

#### `img.to_bytes()`
Get raw pixel data as bytes.

**Example:**
```python
# Get image data
data = img.to_bytes()
print(f"Image data: {len(data)} bytes")
```

#### `img.copy()`
Create a copy of the image.

**Example:**
```python
# Create independent copy
copy = img.copy()
# Modify copy without affecting original
```


---

## ✏️ Drawing API

## 📝 Text API

### Shape Drawing

#### `img.draw_rectangle(x, y, width, height, color)`
Draw a filled rectangle.

**Parameters:**
- `x, y` (int): Top-left corner coordinates
- `width, height` (int): Rectangle dimensions
- `color` (Tuple[int, int, int, int]): RGBA color

**Example:**
```python
# Red rectangle
img = img.draw_rectangle(50, 50, 200, 100, (255, 0, 0, 255))

# Semi-transparent blue rectangle
img = img.draw_rectangle(100, 150, 150, 75, (0, 0, 255, 128))
```

#### `img.draw_circle(center_x, center_y, radius, color)`
Draw a filled circle.

**Example:**
```python
# Red circle at center
w, h = img.size
img = img.draw_circle(w // 2, h // 2, 50, (255, 0, 0, 255))

# Green dot
img = img.draw_circle(100, 100, 10, (0, 255, 0, 255))
```

#### `img.draw_line(x0, y0, x1, y1, color)`
Draw a line using Bresenham's algorithm.

**Example:**
```python
# Diagonal line
img = img.draw_line(0, 0, 400, 300, (255, 0, 0, 255))

# Horizontal line
img = img.draw_line(50, 100, 350, 100, (0, 255, 0, 255))
```

#### `img.draw_text(text, x, y, color, scale)`
Draw text using built-in bitmap font.

**Example:**
```python
# Small text
img = img.draw_text("HELLO", 10, 10, (255, 255, 255, 255), scale=1)

# Large text
img = img.draw_text("TITLE", 50, 50, (255, 0, 0, 255), scale=4)
```

### Drawing Examples

```python
# Create canvas and draw shapes
canvas = Image.new("RGBA", (400, 300), (255, 255, 255, 255))

# Draw multiple shapes (chain operations)
canvas = (canvas
    .draw_rectangle(50, 50, 100, 100, (255, 0, 0, 255))    # Red square
    .draw_circle(300, 150, 50, (0, 255, 0, 255))            # Green circle
    .draw_line(50, 150, 350, 150, (0, 0, 255, 255))         # Blue line
    .draw_text("SHAPES", 150, 250, (0, 0, 0, 255), 2))      # Black text

canvas.save("shapes.png")
```

```python
# Add watermark
watermark_color = (255, 255, 255, 128)  # Semi-transparent white
img = img.draw_text("© 2025", 10, img.height - 30, watermark_color, 2)
```

---

## ✨ Effects API

### Shadow Effects

#### `img.drop_shadow(offset_x, offset_y, blur_radius, shadow_color)`
Add a drop shadow behind the image.

**Parameters:**
- `offset_x, offset_y` (int): Shadow offset (pixels)
- `blur_radius` (float): Shadow blur amount
- `shadow_color` (Tuple[int, int, int, int]): RGBA shadow color

**Example:**
```python
# Classic drop shadow
shadowed = img.drop_shadow(
    offset_x=10,
    offset_y=10,
    blur_radius=15.0,
    shadow_color=(0, 0, 0, 128)  # Semi-transparent black
)

# Soft shadow
soft = img.drop_shadow(5, 5, 25.0, (0, 0, 0, 100))

# Colored shadow
colored = img.drop_shadow(15, 15, 20.0, (255, 0, 0, 100))  # Red shadow
```

#### `img.inner_shadow(offset_x, offset_y, blur_radius, shadow_color)`
Add an inner shadow (shadow inside the image edges).

**Example:**
```python
# Inner shadow for depth
inset = img.inner_shadow(
    offset_x=5,
    offset_y=5,
    blur_radius=10.0,
    shadow_color=(0, 0, 0, 180)
)

# Emboss-like effect
embossed = img.inner_shadow(2, 2, 5.0, (0, 0, 0, 100))
```

#### `img.glow(blur_radius, glow_color, intensity)`
Add a glow effect around the image.

**Example:**
```python
# Soft white glow
glowing = img.glow(
    blur_radius=20.0,
    glow_color=(255, 255, 255, 255),
    intensity=0.8
)

# Colored neon glow
neon = img.glow(15.0, (0, 255, 255, 255), 1.0)  # Cyan
```

### Blending Effects

#### `img.blend_with(other, mode, opacity)`
Blend image with another using advanced blend modes.

**Parameters:**
- `other` (Image): Image to blend with
- `mode` (str): Blend mode ("normal", "multiply", "screen", "overlay", "soft_light", "hard_light", "color_dodge", "color_burn", "darken", "lighten", "difference", "exclusion")
- `opacity` (float): Blend opacity (0.0-1.0)

**Example:**
```python
from imgrs import Image, BlendMode

background = Image.open("background.jpg")
overlay = Image.open("overlay.png")

# Different blend modes
multiply = background.blend_with(overlay, BlendMode.MULTIPLY, 0.8)
screen = background.blend_with(overlay, BlendMode.SCREEN, 0.6)
overlay_blend = background.blend_with(overlay, BlendMode.OVERLAY, 0.7)
```

#### `img.overlay_with(overlay, mode, opacity, position)`
Overlay an image using advanced blending with positioning.

**Example:**
```python
# Center overlay
centered = background.overlay_with(watermark, "normal", 0.5)

# Positioned overlay
positioned = background.overlay_with(watermark, "multiply", 0.8, position=(50, 100))
```

### Effect Combinations

```python
# Drop shadow + glow
enhanced = (img
    .drop_shadow(8, 8, 15.0, (0, 0, 0, 120))
    .glow(10.0, (255, 255, 255, 255), 0.6))

# Create button with depth
def create_button(text_img):
    """Create a button with depth."""
    return (text_img
        .drop_shadow(0, 3, 5.0, (0, 0, 0, 100))  # Bottom shadow
        .inner_shadow(0, -1, 3.0, (255, 255, 255, 50)))  # Top highlight

button = create_button(label_img)
```

---

## 🎨 Filters API

### Basic Filters

#### `img.blur(radius)`
Apply Gaussian blur to the image.

**Example:**
```python
# Light blur
light = img.blur(2.0)

# Heavy blur
heavy = img.blur(10.0)
```

#### `img.sharpen(strength)`
Apply sharpening filter.

**Example:**
```python
# Gentle sharpen
gentle = img.sharpen(0.5)

# Strong sharpen
sharp = img.sharpen(2.0)
```

#### `img.edge_detect()`
Apply edge detection filter (Sobel operator).

**Example:**
```python
edges = img.edge_detect()
edges.save("edges.png")
```

#### `img.emboss()`
Apply emboss filter for a 3D effect.

**Example:**
```python
embossed = img.emboss()
embossed.save("embossed.png")
```

### Adjustment Filters

#### `img.brightness(adjustment)`
Adjust image brightness (-255 to +255).

**Example:**
```python
# Brighten
brighter = img.brightness(30)

# Darken
darker = img.brightness(-30)
```

#### `img.contrast(factor)`
Adjust image contrast (factor > 1.0 increases contrast).

**Example:**
```python
# Increase contrast
high_contrast = img.contrast(1.5)

# Decrease contrast
low_contrast = img.contrast(0.5)
```

### CSS-Like Filters

#### `img.sepia(amount)`
Apply sepia tone filter (0.0 to 1.0).

**Example:**
```python
# Subtle sepia
vintage = img.sepia(0.5)

# Full sepia
old_photo = img.sepia(1.0)
```

#### `img.grayscale_filter(amount)`
Apply grayscale filter (not full conversion).

**Example:**
```python
# 50% grayscale
half_gray = img.grayscale_filter(0.5)

# Full grayscale
full_gray = img.grayscale_filter(1.0)
```

#### `img.invert(amount)`
Invert image colors.

**Example:**
```python
# Full invert
negative = img.invert(1.0)

# Partial invert
partial = img.invert(0.5)
```

#### `img.hue_rotate(degrees)`
Rotate hue of the image (-360 to +360 degrees).

**Example:**
```python
# Shift colors
shifted = img.hue_rotate(90)

# Full rotation
rotated = img.hue_rotate(180)
```

#### `img.saturate(amount)`
Adjust color saturation.

**Example:**
```python
# Desaturate
muted = img.saturate(0.5)

# Boost saturation
vibrant = img.saturate(1.5)
```

### Filter Chaining

```python
# Chain multiple filters
result = (img
    .blur(2.0)
    .brightness(10)
    .contrast(1.2)
    .sharpen(1.5)
    .sepia(0.3))

result.save("processed.jpg")
```

---

## 🔄 Transform API

### Geometric Transforms

#### `img.resize(size, resample=None)`
Resize image to new dimensions.

**Parameters:**
- `size` (Tuple[int, int]): New size (width, height)
- `resample` (str, optional): "NEAREST", "BILINEAR", "BICUBIC", "LANCZOS"

**Example:**
```python
# Resize to specific size
resized = img.resize((800, 600))

# High quality resize
hq = img.resize((1920, 1080), resample="LANCZOS")

# Fast resize for thumbnails
thumb = img.resize((150, 150), resample="NEAREST")
```

#### `img.crop(box)`
Crop image to a rectangular region.

**Parameters:**
- `box` (Tuple[int, int, int, int]): Crop box (x, y, width, height)

**Example:**
```python
# Crop center 400x300 region starting at (100, 100)
cropped = img.crop((100, 100, 400, 300))

# Crop top-left quarter
w, h = img.size
quarter = img.crop((0, 0, w // 2, h // 2))
```

#### `img.rotate(angle)`
Rotate image by specified angle (only 90, 180, 270 supported).

**Example:**
```python
# Rotate 90° clockwise
rotated = img.rotate(90)

# Rotate 180°
flipped = img.rotate(180)
```

#### `img.transpose(method)`
Flip or transpose the image.

**Parameters:**
- `method` (str): "FLIP_LEFT_RIGHT", "FLIP_TOP_BOTTOM", "ROTATE_90", "ROTATE_180", "ROTATE_270"

**Example:**
```python
# Mirror horizontally
mirrored = img.transpose("FLIP_LEFT_RIGHT")

# Mirror vertically
upside_down = img.transpose("FLIP_TOP_BOTTOM")
```

### Color Conversion

#### `img.convert(mode)`
Convert image to different color mode.

**Parameters:**
- `mode` (str): "L" (grayscale), "LA" (grayscale + alpha), "RGB", "RGBA"

**Example:**
```python
# Convert to grayscale
gray = img.convert("L")

# Add alpha channel
with_alpha = img.convert("RGBA")

# Remove alpha channel
no_alpha = img.convert("RGB")
```

#### `img.split()`
Split image into individual channels.

**Example:**
```python
# Split RGB image
r, g, b = img.split()
r.save("red_channel.png")

# Split RGBA image
r, g, b, a = img.split()
a.save("alpha_channel.png")
```

### Compositing

#### `img.paste(other, position=None, mask=None)`
Paste another image onto this image.

**Example:**
```python
# Paste at top-left
result = base.paste(overlay)

# Paste at specific position
result = base.paste(overlay, position=(100, 50))

# Paste with mask
mask = Image.new("L", overlay.size, color=128)
result = base.paste(overlay, position=(50, 50), mask=mask)
```

### Transformation Pipelines

```python
# Create thumbnail pipeline
def create_thumbnail(img, size=(150, 150)):
    return (img
        .resize(size, resample="LANCZOS")
        .sharpen(1.2)
        .contrast(1.1))

thumb = create_thumbnail(img)

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

## 🎯 Pixels API

### Pixel Access

#### `img.getpixel(x, y)`
Get color of a single pixel.

**Example:**
```python
# Get pixel at position (100, 50)
r, g, b, a = img.getpixel(100, 50)
print(f"Pixel color: R={r}, G={g}, B={b}, A={a}")
```

#### `img.putpixel(x, y, color)`
Set color of a single pixel.

**Example:**
```python
# Set pixel to red
img = img.putpixel(100, 50, (255, 0, 0, 255))

# Draw single pixels to create pattern
for i in range(100):
    img = img.putpixel(i, i, (255, 0, 0, 255))  # Diagonal line
```

### Color Analysis

#### `img.histogram()`
Get histogram of pixel values.

**Example:**
```python
r_hist, g_hist, b_hist, a_hist = img.histogram()

# Find brightest red value
max_red = r_hist.index(max(r_hist))
print(f"Most common red value: {max_red}")
```

#### `img.dominant_color()`
Find the most common color in the image.

**Example:**
```python
r, g, b, a = img.dominant_color()
print(f"Dominant color: RGB({r}, {g}, {b})")
```

#### `img.average_color()`
Calculate average color across entire image.

**Example:**
```python
r, g, b, a = img.average_color()
print(f"Average color: RGB({r}, {g}, {b})")
```

### Color Manipulation

#### `img.replace_color(target_color, replacement_color, tolerance)`
Replace all pixels of one color with another.

**Example:**
```python
# Replace exact white with transparent
img = img.replace_color(
    (255, 255, 255, 255),  # White
    (0, 0, 0, 0),          # Transparent
    tolerance=10
)

# Remove green screen
img = img.replace_color(
    (0, 255, 0, 255),      # Green
    (0, 0, 0, 0),          # Transparent
    tolerance=50
)
```

#### `img.threshold(threshold_value)`
Apply threshold to create binary (black/white) image.

**Example:**
```python
# Basic threshold
binary = img.threshold(128)

# Dark threshold (more white)
high_thresh = img.threshold(200)
```

#### `img.posterize(levels)`
Reduce number of color levels (posterization effect).

**Example:**
```python
# Strong posterization (pop art effect)
poster = img.posterize(4)

# Mild posterization
mild = img.posterize(16)
```

### Pixel Iteration Patterns

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

---

## 🎨 Color Operations API

### Transparency Operations

#### `img.set_alpha(alpha)`
Set global alpha channel for the entire image.

**Example:**
```python
from imgrs import Image

img = Image.open("photo.jpg")
semi_transparent = img.set_alpha(0.7)
```

#### `img.add_transparency(color, tolerance=0)`
Add transparency to specific colors.

**Example:**
```python
# Make white backgrounds transparent
transparent = img.add_transparency((255, 255, 255), tolerance=10)
```

#### `img.remove_transparency(background_color=None)`
Remove transparency by compositing on background.

**Example:**
```python
# Remove transparency with white background
opaque = transparent_img.remove_transparency((255, 255, 255))
```

### Advanced Masking System

#### `img.apply_mask(mask, invert=False)`
Apply a mask to the image using alpha channel.

**Example:**
```python
mask = img.create_gradient_mask("radial", 0.0, 1.0)
masked = img.apply_mask(mask)
```

#### `img.create_gradient_mask(direction, start_opacity, end_opacity)`
Create a gradient mask.

**Example:**
```python
# Create vignette effect
vignette = img.create_gradient_mask("radial", 1.0, 0.3)
masked = img.apply_mask(vignette)
```

#### `img.create_color_mask(target_color, tolerance, feather)`
Create a mask based on color similarity.

**Example:**
```python
# Mask blue sky
sky_mask = img.create_color_mask((100, 150, 255), tolerance=30, feather=5)
sky_only = img.apply_mask(sky_mask)
```

#### `img.create_luminance_mask(invert=False)`
Create a mask based on image luminance.

**Example:**
```python
# Create high-key mask
bright_mask = img.create_luminance_mask(invert=False)
bright_areas = img.apply_mask(bright_mask)
```

### Color Manipulation

#### `img.extract_color(target_color, tolerance)`
Extract pixels matching a target color.

**Example:**
```python
# Extract red objects
red_objects = img.extract_color((255, 0, 0), tolerance=50)
```

#### `img.color_quantize(levels)`
Quantize colors to reduce palette size.

**Example:**
```python
# Create poster effect
posterized = img.color_quantize(levels=4)
```

#### `img.color_shift(shift_amount)`
Shift all colors by a specified amount.

**Example:**
```python
# Warm up colors
warmer = img.color_shift(0.2)
```

#### `img.selective_desaturate(target_color, tolerance, desaturate_factor)`
Selectively desaturate specific colors.

**Example:**
```python
# Desaturate blue sky
desaturated = img.selective_desaturate((100, 150, 255), tolerance=40, desaturate_factor=0.8)
```

### Gradient & Pattern Overlays

#### `img.apply_gradient_overlay(color, direction, opacity)`
Apply a gradient color overlay.

**Example:**
```python
# Add blue gradient overlay
with_overlay = img.apply_gradient_overlay((0, 100, 255, 150), "vertical", 0.6)
```

#### `img.create_stripe_pattern(color, width, spacing, angle)`
Create a stripe pattern overlay.

**Example:**
```python
# Create zebra stripes
stripes = img.create_stripe_pattern((0, 0, 0, 180), width=20, spacing=20, angle=45.0)
```

#### `img.create_checker_pattern(color1, color2, size)`
Create a checkerboard pattern overlay.

**Example:**
```python
# Create checkerboard overlay
checker = img.create_checker_pattern(
    (255, 0, 0, 100), (0, 0, 255, 100), size=32
)
```

### Alpha Channel Operations

#### `img.split_alpha()`
Split image into RGB and alpha components.

**Example:**
```python
rgb_img, alpha_img = img.split_alpha()
```

#### `img.merge_alpha(alpha_image)`
Merge alpha channel with image.

**Example:**
```python
# Recombine split channels
recombined = rgb_img.merge_alpha(alpha_img)
```

### Color Analysis

#### `img.get_color_palette(max_colors)`
Extract dominant colors from the image.

**Example:**
```python
palette = img.get_color_palette(max_colors=8)
for color in palette:
    print(f"RGB{color}")
```

#### `img.analyze_color_distribution()`
Analyze color distribution in the image.

**Example:**
```python
stats = img.analyze_color_distribution()
print(f"Total pixels: {stats['total_pixels']}")
print(f"Unique colors: {stats['unique_colors']}")
print(f"Dominant color: {stats['dominant_color']}")
```

### Complex Color Operations Example

```python
# Create a complex composite
img = Image.open("landscape.jpg")

# Create vignette mask
vignette = img.create_gradient_mask("radial", 1.0, 0.4)

# Apply selective color adjustment
adjusted = img.selective_desaturate((100, 150, 200), tolerance=40, desaturate_factor=0.6)

# Add gradient overlay
final = adjusted.apply_gradient_overlay((255, 200, 150, 80), "vertical", 0.4)

# Apply vignette
final = final.apply_mask(vignette)

final.save("enhanced_landscape.png")
```

---

## 📋 Enums Reference

### Import All Enums
```python
from imgrs import (
    ImageMode, ImageFormat, Resampling, Transpose,
    MaskType, ColorFormat, GradientDirection
)
```

### ImageMode
```python
ImageMode.L = "L"        # 8-bit grayscale
ImageMode.LA = "LA"      # 8-bit grayscale + alpha
ImageMode.RGB = "RGB"    # 8-bit RGB
ImageMode.RGBA = "RGBA"  # 8-bit RGB + alpha
ImageMode.INTEGER = "I"  # 32-bit integer grayscale
```

### ImageFormat
```python
ImageFormat.JPEG = "JPEG"
ImageFormat.PNG = "PNG"
ImageFormat.GIF = "GIF"
ImageFormat.BMP = "BMP"
ImageFormat.TIFF = "TIFF"
ImageFormat.WEBP = "WEBP"
```

### Resampling
```python
Resampling.NEAREST = "NEAREST"     # Fastest, lowest quality
Resampling.BILINEAR = "BILINEAR"   # Good quality (default)
Resampling.BICUBIC = "BICUBIC"     # High quality
Resampling.LANCZOS = "LANCZOS"     # Highest quality, slower
```


### MaskType
```python
MaskType.GRADIENT = "gradient"
MaskType.COLOR_BASED = "color_based"
MaskType.LUMINANCE = "luminance"
MaskType.SHAPE = "shape"
```

### GradientDirection
```python
GradientDirection.HORIZONTAL = "horizontal"
GradientDirection.VERTICAL = "vertical"
GradientDirection.DIAGONAL = "diagonal"
GradientDirection.RADIAL = "radial"
```

---

## 🎯 Common Patterns & Examples

### Photo Enhancement Pipeline
```python
def enhance_photo(img):
    """Apply common photo enhancements."""
    return (img
        .brightness(10)          # Slight brighten
        .contrast(1.1)           # Increase contrast
        .saturate(1.05)          # Boost colors slightly
        .sharpen(1.2)           # Light sharpen
        .drop_shadow(5, 5, 15.0, (0, 0, 0, 80)))  # Subtle shadow

enhanced = enhance_photo(Image.open("photo.jpg"))
```

### Create Thumbnail with Effects
```python
def create_thumbnail(img, size=(150, 150)):
    """Create an attractive thumbnail."""
    return (img
        .resize(size, resample="LANCZOS")  # High quality resize
        .sharpen(1.5)                     # Sharpen for small size
        .drop_shadow(3, 3, 8.0, (0, 0, 0, 120))  # Add depth
        .border(2, (255, 255, 255, 255)))  # White border

thumb = create_thumbnail(Image.open("image.jpg"))
```

### Watermark Application
```python
def add_watermark(img, watermark_text):
    """Add a semi-transparent watermark."""
    # Create watermark image
    watermark = (Image.new("RGBA", img.size, (0, 0, 0, 0))
        .draw_text(watermark_text, 10, img.height - 30, (255, 255, 255, 128), 2))

    # Overlay watermark
    return img.overlay_with(watermark, "normal", 0.7)

watermarked = add_watermark(Image.open("photo.jpg"), "© 2025")
```

### Color-Based Object Extraction
```python
def extract_color_objects(img, target_color, tolerance=50):
    """Extract objects of a specific color."""
    # Create color mask
    mask = img.create_color_mask(target_color, tolerance, feather=5)
    
    # Apply mask to extract objects
    extracted = img.apply_mask(mask)
    
    return extracted

# Extract blue objects
blue_objects = extract_color_objects(img, (0, 0, 255), tolerance=30)
```

### Create Social Media Post Template
```python
def create_social_post(template_img, content_text):
    """Create a social media post with text overlay."""
    return (template_img
        .apply_gradient_overlay((0, 0, 0, 100), "vertical", 0.3)  # Dark overlay
        .draw_text(content_text, 50, 200, (255, 255, 255, 255), 4)  # White text
        .drop_shadow(2, 2, 5.0, (0, 0, 0, 150)))  # Text shadow

post = create_social_post(Image.open("template.jpg"), "Hello World!")
```

### Batch Processing Pattern
```python
def process_batch(input_dir, output_dir, operations):
    """Process multiple images with the same operations."""
    import os
    
    for filename in os.listdir(input_dir):
        if filename.lower().endswith(('.png', '.jpg', '.jpeg')):
            img = Image.open(os.path.join(input_dir, filename))
            
            # Apply operations
            processed = img
            for op in operations:
                processed = op(processed)
            
            # Save result
            output_path = os.path.join(output_dir, filename)
            processed.save(output_path)

# Example usage
def brighten(x): return x.brightness(20)
def contrast(x): return x.contrast(1.1)
def sharpen(x): return x.sharpen(1.2)

operations = [brighten, contrast, sharpen]
process_batch("input/", "output/", operations)
```

### Performance Optimization
```python
# ✅ GOOD - Chain operations efficiently
result = (img
    .resize((800, 600))
    .brightness(10)
    .contrast(1.1)
    .sharpen(1.2))

# ❌ SLOW - Avoid intermediate variables
temp1 = img.resize((800, 600))
temp2 = temp1.brightness(10)
temp3 = temp2.contrast(1.1)
result = temp3.sharpen(1.2)
```

---

## 📊 Performance Guide

| Operation | Speed | Complexity | Best For |
|-----------|-------|------------|----------|
| `resize()` | ⚡⚡⚡ Fast | O(n×m) | Changing image dimensions |
| `crop()` | ⚡⚡⚡ Fast | O(1) | Extracting regions |
| `brightness()` | ⚡⚡⚡ Fast | O(n×m) | Simple adjustments |
| `contrast()` | ⚡⚡⚡ Fast | O(n×m) | Contrast changes |
| `blur()` | ⚡⚡ Medium | O(n×m×r²) | Softening images |
| `sharpen()` | ⚡⚡⚡ Fast | O(n×m) | Detail enhancement |
| `drop_shadow()` | ⚡⚡ Slow | O(n×m×r²) | Adding depth |
| `pixel loops` | ⚠️ Avoid | O(n×m×iterations) | Use filters instead |

---

**See Also:**
- [Installation Guide](guides/installation.md) - Setup instructions
- [Basic Usage Guide](guides/basic-usage.md) - Getting started
- [Examples Directory](../examples/) - More code examples
- [Migration Guide](guides/migration.md) - Migrating from other libraries