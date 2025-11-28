# Color Operations API

The Color Operations API provides advanced color manipulation, transparency handling, masking, and analysis capabilities for Imgrs images.

## Overview

The color operations are implemented through the `ColorMixin` class and provide:

- **Transparency Operations**: Global alpha control and selective transparency
- **Advanced Masking System**: Gradient, color-based, and luminance masks
- **Color Manipulation**: Extraction, quantization, shifting, and selective desaturation
- **Gradient & Pattern Overlays**: Visual effects and patterns
- **Alpha Channel Operations**: Splitting, merging, and converting alpha channels
- **Color Analysis**: Palette extraction and distribution analysis

## Transparency Operations

### `set_alpha(alpha: float) -> Image`

Set global alpha channel for the entire image.

**Parameters:**
- `alpha` (float): Alpha value (0.0 = transparent, 1.0 = opaque)

**Returns:** New Image instance with modified alpha

**Example:**
```python
from imgrs import Image

img = Image.open("photo.jpg")
semi_transparent = img.set_alpha(0.7)
```

### `get_alpha() -> float`

Get the average alpha channel value.

**Returns:** Average alpha value (0.0-1.0)

**Example:**
```python
alpha = img.get_alpha()
print(f"Average alpha: {alpha:.2f}")
```

### `add_transparency(color: tuple, tolerance: int = 0) -> Image`

Add transparency to specific colors.

**Parameters:**
- `color` (tuple): Target color to make transparent (R, G, B)
- `tolerance` (int): Color matching tolerance (0-255)

**Returns:** New Image with transparency added

**Example:**
```python
# Make white backgrounds transparent
transparent = img.add_transparency((255, 255, 255), tolerance=10)
```

### `remove_transparency(background_color: tuple = None) -> Image`

Remove transparency by compositing on background.

**Parameters:**
- `background_color` (tuple): Background color (R, G, B), defaults to white

**Returns:** New opaque Image

**Example:**
```python
# Remove transparency with white background
opaque = transparent_img.remove_transparency((255, 255, 255))
```

## Advanced Masking System

### `apply_mask(mask: Image, invert: bool = False) -> Image`

Apply a mask to the image using alpha channel.

**Parameters:**
- `mask` (Image): Mask image (grayscale or RGBA)
- `invert` (bool): Invert the mask

**Returns:** New masked Image

**Example:**
```python
mask = img.create_gradient_mask("radial", 0.0, 1.0)
masked = img.apply_mask(mask)
```

### `create_gradient_mask(direction: str, start_opacity: float, end_opacity: float) -> Image`

Create a gradient mask.

**Parameters:**
- `direction` (str): "horizontal", "vertical", "radial", or "diagonal"
- `start_opacity` (float): Starting opacity (0.0-1.0)
- `end_opacity` (float): Ending opacity (0.0-1.0)

**Returns:** New gradient mask Image

**Example:**
```python
# Create vignette effect
vignette = img.create_gradient_mask("radial", 1.0, 0.3)
masked = img.apply_mask(vignette)
```

### `create_color_mask(target_color: tuple, tolerance: int, feather: int) -> Image`

Create a mask based on color similarity.

**Parameters:**
- `target_color` (tuple): Target color to mask (R, G, B)
- `tolerance` (int): Color matching tolerance (0-255)
- `feather` (int): Feather amount for soft edges

**Returns:** New color-based mask Image

**Example:**
```python
# Mask blue sky
sky_mask = img.create_color_mask((100, 150, 255), tolerance=30, feather=5)
sky_only = img.apply_mask(sky_mask)
```

### `create_luminance_mask(invert: bool = False) -> Image`

Create a mask based on image luminance.

**Parameters:**
- `invert` (bool): Invert the luminance mask

**Returns:** New luminance mask Image

**Example:**
```python
# Create high-key mask
bright_mask = img.create_luminance_mask(invert=False)
bright_areas = img.apply_mask(bright_mask)
```

### `combine_masks(masks: list, operation: str) -> Image`

Combine multiple masks using mathematical operations.

**Parameters:**
- `masks` (list): List of mask images
- `operation` (str): "multiply", "add", "subtract", "overlay", "screen"

**Returns:** New combined mask Image

**Example:**
```python
mask1 = img.create_gradient_mask("horizontal", 0.0, 1.0)
mask2 = img.create_luminance_mask()
combined = img.combine_masks([mask1, mask2], "multiply")
```

## Color Manipulation

### `extract_color(target_color: tuple, tolerance: int) -> Image`

Extract pixels matching a target color.

**Parameters:**
- `target_color` (tuple): Target color to extract (R, G, B)
- `tolerance` (int): Color matching tolerance (0-255)

**Returns:** New Image with matching pixels extracted

**Example:**
```python
# Extract red objects
red_objects = img.extract_color((255, 0, 0), tolerance=50)
```

### `color_quantize(levels: int) -> Image`

Quantize colors to reduce palette size.

**Parameters:**
- `levels` (int): Number of color levels per channel

**Returns:** New quantized Image

**Example:**
```python
# Create poster effect
posterized = img.color_quantize(levels=4)
```

### `color_shift(shift_amount: float) -> Image`

Shift all colors by a specified amount.

**Parameters:**
- `shift_amount` (float): Color shift amount (-1.0 to 1.0)

**Returns:** New color-shifted Image

**Example:**
```python
# Warm up colors
warmer = img.color_shift(0.2)
```

### `selective_desaturate(target_color: tuple, tolerance: int, desaturate_factor: float) -> Image`

Selectively desaturate specific colors.

**Parameters:**
- `target_color` (tuple): Target color for desaturation (R, G, B)
- `tolerance` (int): Color matching tolerance (0-255)
- `desaturate_factor` (float): Desaturation factor (0.0 = no change, 1.0 = full grayscale)

**Returns:** New selectively desaturated Image

**Example:**
```python
# Desaturate blue sky
desaturated = img.selective_desaturate((100, 150, 255), tolerance=40, desaturate_factor=0.8)
```

### `color_match(reference_image: Image, strength: float) -> Image`

Match colors to a reference image.

**Parameters:**
- `reference_image` (Image): Reference image for color matching
- `strength` (float): Matching strength (0.0-1.0)

**Returns:** New color-matched Image

**Example:**
```python
reference = Image.open("reference.jpg")
matched = img.color_match(reference, strength=0.7)
```

## Gradient & Pattern Overlays

### `apply_gradient_overlay(color: tuple, direction: str, opacity: float) -> Image`

Apply a gradient color overlay.

**Parameters:**
- `color` (tuple): Gradient color (R, G, B, A)
- `direction` (str): "horizontal", "vertical", "radial"
- `opacity` (float): Overlay opacity (0.0-1.0)

**Returns:** New Image with gradient overlay

**Example:**
```python
# Add blue gradient overlay
with_overlay = img.apply_gradient_overlay((0, 100, 255, 150), "vertical", 0.6)
```

### `create_stripe_pattern(color: tuple, width: int, spacing: int, angle: float) -> Image`

Create a stripe pattern overlay.

**Parameters:**
- `color` (tuple): Stripe color (R, G, B, A)
- `width` (int): Stripe width in pixels
- `spacing` (int): Spacing between stripes
- `angle` (float): Rotation angle in degrees

**Returns:** New Image with stripe pattern

**Example:**
```python
# Create zebra stripes
stripes = img.create_stripe_pattern((0, 0, 0, 180), width=20, spacing=20, angle=45.0)
```

### `create_checker_pattern(color1: tuple, color2: tuple, size: int) -> Image`

Create a checkerboard pattern overlay.

**Parameters:**
- `color1` (tuple): First checker color (R, G, B, A)
- `color2` (tuple): Second checker color (R, G, B, A)
- `size` (int): Checker size in pixels

**Returns:** New Image with checker pattern

**Example:**
```python
# Create checkerboard overlay
checker = img.create_checker_pattern(
    (255, 0, 0, 100), (0, 0, 255, 100), size=32
)
```

## Alpha Channel Operations

### `split_alpha() -> tuple`

Split image into RGB and alpha components.

**Returns:** Tuple of (RGB Image, Alpha Image)

**Example:**
```python
rgb_img, alpha_img = img.split_alpha()
```

### `merge_alpha(alpha_image: Image) -> Image`

Merge alpha channel with image.

**Parameters:**
- `alpha_image` (Image): Alpha channel image (grayscale)

**Returns:** New Image with merged alpha

**Example:**
```python
# Recombine split channels
recombined = rgb_img.merge_alpha(alpha_img)
```

### `alpha_to_color(background_color: tuple) -> Image`

Convert alpha channel to solid color.

**Parameters:**
- `background_color` (tuple): Background color (R, G, B)

**Returns:** New Image with alpha converted to color

**Example:**
```python
# Convert alpha to gray
gray_from_alpha = img.alpha_to_color((128, 128, 128))
```


## Color Analysis

### `get_color_palette(max_colors: int) -> list`

Extract dominant colors from the image.

**Parameters:**
- `max_colors` (int): Maximum number of colors to extract

**Returns:** List of dominant colors with alpha

**Example:**
```python
palette = img.get_color_palette(max_colors=8)
for color in palette:
    print(f"RGB{color}")
```

### `analyze_color_distribution() -> dict`

Analyze color distribution in the image.

**Returns:** Dictionary with color distribution statistics

**Example:**
```python
stats = img.analyze_color_distribution()
print(f"Total pixels: {stats['total_pixels']}")
print(f"Unique colors: {stats['unique_colors']}")
print(f"Dominant color: {stats['dominant_color']}")
```

### `find_color_regions(target_color: tuple, tolerance: int) -> list`

Find regions matching a target color.

**Parameters:**
- `target_color` (tuple): Target color to find (R, G, B)
- `tolerance` (int): Color matching tolerance (0-255)

**Returns:** List of bounding boxes (x, y, width, height) for matching regions

**Example:**
```python
regions = img.find_color_regions((255, 0, 0), tolerance=30)
for region in regions:
    x, y, w, h = region
    print(f"Red region at ({x}, {y}) size {w}x{h}")
```

## Related Enums

### MaskType
```python
from imgrs import MaskType

MaskType.GRADIENT
MaskType.COLOR_BASED
MaskType.LUMINANCE
MaskType.SHAPE
```

### ColorFormat
```python
from imgrs import ColorFormat

ColorFormat.RGB
ColorFormat.RGBA
ColorFormat.HSL
ColorFormat.HSV
```

## Examples

### Creating a Complex Composite
```python
from imgrs import Image

# Load base image
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

### Color-Based Object Extraction
```python
# Extract blue objects from image
blue_mask = img.create_color_mask((0, 0, 255), tolerance=50, feather=3)
blue_objects = img.apply_mask(blue_mask)

# Analyze extracted colors
palette = blue_objects.get_color_palette(max_colors=5)
stats = blue_objects.analyze_color_distribution()