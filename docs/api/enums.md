# Enumeration Constants

Imgrs provides comprehensive enumeration constants for various image processing operations, maintaining compatibility with Pillow where possible while adding enhanced functionality.

## ImageMode

Image mode constants defining pixel formats and color spaces.

### Grayscale Modes
```python
from imgrs import ImageMode

ImageMode.L = "L"        # 8-bit grayscale
ImageMode.LA = "LA"      # 8-bit grayscale + alpha
ImageMode.INTEGER = "I"  # 32-bit integer grayscale
```

### Color Modes
```python
ImageMode.RGB = "RGB"    # 8-bit RGB
ImageMode.RGBA = "RGBA"  # 8-bit RGB + alpha
ImageMode.CMYK = "CMYK"  # 8-bit CMYK
ImageMode.YCbCr = "YCbCr"  # 8-bit YCbCr
ImageMode.HSV = "HSV"    # 8-bit HSV
```

### Binary Mode
```python
ImageMode.BINARY = "1"   # 1-bit binary
```

## ImageFormat

Supported image file formats for loading and saving.

```python
from imgrs import ImageFormat

ImageFormat.JPEG = "JPEG"
ImageFormat.PNG = "PNG"
ImageFormat.GIF = "GIF"
ImageFormat.BMP = "BMP"
ImageFormat.TIFF = "TIFF"
ImageFormat.WEBP = "WEBP"
ImageFormat.ICO = "ICO"
ImageFormat.PNM = "PNM"
ImageFormat.DDS = "DDS"
ImageFormat.TGA = "TGA"
ImageFormat.FARBFELD = "FARBFELD"
ImageFormat.AVIF = "AVIF"
```

## Resampling

Resampling filter constants for image resizing operations.

```python
from imgrs import Resampling

# Primary filters
Resampling.NEAREST = "NEAREST"
Resampling.BILINEAR = "BILINEAR"
Resampling.BICUBIC = "BICUBIC"
Resampling.LANCZOS = "LANCZOS"

# Pillow compatibility (numeric constants)
Resampling.NEAREST_INT = 0
Resampling.BILINEAR_INT = 1
Resampling.BICUBIC_INT = 2
Resampling.LANCZOS_INT = 3
```

### Converting Between Formats
```python
# Convert integer to string
filter_name = Resampling.from_int(2)  # Returns "BICUBIC"
```

## Transpose

Image transformation operations for flipping and rotating.

```python
from imgrs import Transpose

Transpose.FLIP_LEFT_RIGHT = "FLIP_LEFT_RIGHT"
Transpose.FLIP_TOP_BOTTOM = "FLIP_TOP_BOTTOM"
Transpose.ROTATE_90 = "ROTATE_90"
Transpose.ROTATE_180 = "ROTATE_180"
Transpose.ROTATE_270 = "ROTATE_270"
Transpose.TRANSPOSE = "TRANSPOSE"
Transpose.TRANSVERSE = "TRANSVERSE"

# Pillow compatibility (numeric constants)
Transpose.FLIP_LEFT_RIGHT_INT = 0
Transpose.FLIP_TOP_BOTTOM_INT = 1
Transpose.ROTATE_90_INT = 2
Transpose.ROTATE_180_INT = 3
Transpose.ROTATE_270_INT = 4
Transpose.TRANSPOSE_INT = 5
Transpose.TRANSVERSE_INT = 6
```

### Converting Between Formats
```python
# Convert integer to string
operation = Transpose.from_int(2)  # Returns "ROTATE_90"
```


## MaskType

Types of masks supported by the masking system.

```python
from imgrs import MaskType

MaskType.GRADIENT = "gradient"
MaskType.COLOR_BASED = "color_based"
MaskType.LUMINANCE = "luminance"
MaskType.SHAPE = "shape"
MaskType.TEXTURE = "texture"
MaskType.NOISE = "noise"
```

### Usage Example
```python
from imgrs import Image, MaskType

img = Image.open("photo.jpg")

# Different mask types
gradient_mask = img.create_gradient_mask("radial", 0.0, 1.0)  # MaskType.GRADIENT
color_mask = img.create_color_mask((255, 0, 0), 30, 5)      # MaskType.COLOR_BASED
luminance_mask = img.create_luminance_mask()                   # MaskType.LUMINANCE
```

## ColorFormat

Color space and format constants for color operations.

```python
from imgrs import ColorFormat

ColorFormat.RGB = "rgb"
ColorFormat.RGBA = "rgba"
ColorFormat.HSL = "hsl"
ColorFormat.HSV = "hsv"
ColorFormat.LAB = "lab"
ColorFormat.XYZ = "xyz"
ColorFormat.CMYK = "cmyk"
ColorFormat.YCBCR = "ycbcr"
ColorFormat.YUV = "yuv"
ColorFormat.HSL_PRECISE = "hsl_precise"
ColorFormat.HSV_PRECISE = "hsv_precise"
```

## GradientDirection

Gradient direction constants for gradient operations.

```python
from imgrs import GradientDirection

GradientDirection.HORIZONTAL = "horizontal"
GradientDirection.VERTICAL = "vertical"
GradientDirection.DIAGONAL = "diagonal"
GradientDirection.RADIAL = "radial"
GradientDirection.ANGULAR = "angular"
GradientDirection.CONICAL = "conical"
```

### Usage Example
```python
from imgrs import Image, GradientDirection

img = Image.open("photo.jpg")

# Create different gradient masks
horizontal = img.create_gradient_mask(GradientDirection.HORIZONTAL, 0.0, 1.0)
vertical = img.create_gradient_mask(GradientDirection.VERTICAL, 0.0, 1.0)
radial = img.create_gradient_mask(GradientDirection.RADIAL, 0.0, 1.0)
```

## MaskOperation

Mathematical operations for combining masks.

```python
from imgrs import MaskOperation

MaskOperation.MULTIPLY = "multiply"
MaskOperation.ADD = "add"
MaskOperation.SUBTRACT = "subtract"
MaskOperation.OVERLAY = "overlay"
MaskOperation.SCREEN = "screen"
MaskOperation.DIFFERENCE = "difference"
```

### Usage Example
```python
from imgrs import Image, MaskOperation

img = Image.open("photo.jpg")

# Create multiple masks
mask1 = img.create_gradient_mask("horizontal", 0.0, 1.0)
mask2 = img.create_luminance_mask()

# Combine with different operations
multiply_combined = img.combine_masks([mask1, mask2], MaskOperation.MULTIPLY)
add_combined = img.combine_masks([mask1, mask2], MaskOperation.ADD)
```

## ColorSpace

Color space constants for color management.

```python
from imgrs import ColorSpace

ColorSpace.SRGB = "srgb"
ColorSpace.ADOBE_RGB = "adobe_rgb"
ColorSpace.PROPHOTO_RGB = "prophoto_rgb"
ColorSpace.DCI_P3 = "dci_p3"
ColorSpace.REC2020 = "rec2020"
ColorSpace.DISPLAY_P3 = "display_p3"
```

## Usage Patterns

### Importing Multiple Enums
```python
from imgrs import (
    ImageMode, ImageFormat, Resampling, Transpose,
    MaskType, ColorFormat, GradientDirection
)
```

### Checking Mode Compatibility
```python
def is_grayscale_mode(mode):
    return mode in (ImageMode.L, ImageMode.LA, ImageMode.INTEGER)

def supports_alpha(mode):
    return mode in (ImageMode.LA, ImageMode.RGBA)
```


### Gradient Direction Mapping
```python
def create_gradient_mask_by_name(img, direction_name, start_opacity=0.0, end_opacity=1.0):
    direction_map = {
        "horizontal": GradientDirection.HORIZONTAL,
        "vertical": GradientDirection.VERTICAL,
        "diagonal": GradientDirection.DIAGONAL,
        "radial": GradientDirection.RADIAL,
    }

    direction = direction_map.get(direction_name, GradientDirection.VERTICAL)
    return img.create_gradient_mask(direction, start_opacity, end_opacity)
```

## Compatibility Notes

- **Pillow Compatibility**: Numeric constants for `Resampling` and `Transpose` maintain compatibility with Pillow's API
- **String Constants**: All enums use string constants for consistency and readability
- **Extensibility**: New enum values can be added without breaking existing code
- **Type Safety**: Using enums prevents typos and provides IDE autocompletion