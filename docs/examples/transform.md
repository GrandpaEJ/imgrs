# 🔄 Transform Examples

Examples of image transformation operations including resizing, cropping, and rotation.

## Rotation

### Basic Rotation

Rotate an image by 90-degree increments.

```python
from imgrs import Image

img = Image.open("input.png")

# Rotate 90 degrees clockwise
rotated = img.rotate_right()
rotated.save("rotated_90.png")

# Rotate 180 degrees
upside_down = img.rotate180()
upside_down.save("rotated_180.png")
```

### Arbitrary Rotation

Rotate by any angle.

```python
# Rotate 45 degrees
# Default: expand=False (crops to original size)
rotated = img.rotate(45)
rotated.save("rotated_45_cropped.png")
```

### Rotation with Expansion

Rotate and expand the canvas to fit the entire rotated image.

```python
# Rotate 45 degrees and expand
# expand=True ensures no parts of the image are cut off
rotated_full = img.rotate(45, expand=True)
rotated_full.save("rotated_45_full.png")
```

### Text Rotation

You can also rotate text when adding it to an image.

```python
img.add_text_styled(
    "Rotated Text",
    (100, 100),
    size=32,
    color=(0, 0, 0, 255),
    rotation=45.0  # Rotate text 45 degrees
)
```

## Resizing

```python
# Resize to specific dimensions
small = img.resize((800, 600))

# High quality resize
hq = img.resize((1920, 1080), resample="LANCZOS")
```

## Cropping

```python
# Crop a region (x, y, width, height)
# Note: This API uses (x, y, width, height), unlike Pillow's (left, top, right, bottom)
crop = img.crop((100, 100, 400, 300))
```
