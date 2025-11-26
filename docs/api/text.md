# 📝 Text API - Removed in v0.3.0

## Text Rendering Removed

**Important:** All rich text rendering functionality has been **completely removed** in imgrs v0.3.0 due to the removal of Cairo/Pango dependencies.

### Removed Features

The following text-related methods are no longer available:

- `add_text()` - Basic text rendering
- `add_text_styled()` - Styled text with effects
- `add_text_centered()` - Centered text
- `add_text_multiline()` - Multi-line text
- `add_text_advanced()` - Advanced text rendering
- `get_text_size()` - Text dimension measurement
- `get_multiline_text_size()` - Multi-line text dimensions
- `get_text_box()` - Text bounding box
- `list_available_fonts()` - Font listing

### Migration Options

For text rendering needs, consider these alternatives:

#### 1. Use External Libraries
```python
# Pillow for text rendering
from PIL import Image, ImageDraw, ImageFont

img = Image.open("photo.jpg")
draw = ImageDraw.Draw(img)
font = ImageFont.truetype("arial.ttf", 32)
draw.text((50, 50), "Hello World", fill=(255, 0, 0), font=font)

# Convert back to imgrs if needed
import numpy as np
array = np.array(img)
imgrs_img = Image.fromarray(array)
```

#### 2. Use Bitmap Text (Still Available)
```python
from imgrs import Image

img = Image.open("photo.jpg")
# Simple bitmap text (A-Z, 0-9 only)
img = img.draw_text("HELLO", 50, 50, (255, 255, 255, 255), scale=2)
```

#### 3. Pre-render Text Images
```python
# Create text as separate images, then composite
text_img = Image.new("RGBA", (200, 50), (0, 0, 0, 0))
text_img = text_img.draw_text("TITLE", 10, 10, (255, 255, 255, 255), 2)

# Composite onto main image
result = img.paste(text_img, position=(50, 50))
```

### Why Was Text Removed?

- **Simplified Dependencies**: No longer requires Cairo/Pango system libraries
- **Reduced Binary Size**: Smaller installation footprint
- **Faster Builds**: No complex font rendering dependencies
- **Cross-platform**: Easier deployment without font system requirements

### Still Available

Basic bitmap text drawing remains available via `draw_text()` in the [Drawing API](drawing.md).

---

## See Also

- [Drawing API](drawing.md) - Basic drawing operations (bitmap text still available)
- [Migration Guide](../guides/migration.md) - Migration information for v0.3.0