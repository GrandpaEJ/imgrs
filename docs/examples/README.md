# 💡 Code Examples

Ready-to-use code examples for common tasks.

## Basic Examples

### [Open and Save](basic.md)
- Opening images from files and bytes
- Saving in different formats
- Working with paths

### [Resizing](resize.md)
- Create thumbnails
- Scale images
- Maintain aspect ratio
- Different quality settings

### [Filters](filters.md)
- Apply blur, sharpen
- Adjust brightness, contrast
- CSS-like filters
- Filter combinations

## Intermediate Examples

### [Drawing](drawing.md)
- Draw shapes (rectangles, circles, lines)
- Add text
- Create diagrams
- Annotate images

### [Effects](effects.md)
- Drop shadows
- Glows
- Inner shadows
- UI effects

### [Pixel Manipulation](pixels.md)
- Get/set pixels
- Color analysis
- Histograms
- Replace colors

## Advanced Examples

### [Batch Processing](batch.md)
- Process multiple images
- Create thumbnail gallery
- Watermark batches
- Format conversion

### [Image Composition](composition.md)
- Layer images
- Alpha blending
- Masking
- Complex compositions

### [Performance Optimization](performance.md)
- Efficient pipelines
- Memory management
- Parallel processing tips

## Quick Reference

```python
from imgrs import Image

# Open
img = Image.open("photo.jpg")

# Resize
small = img.resize((800, 600), resample="LANCZOS")

# Filter
enhanced = img.blur(2.0).sharpen(1.5).brightness(10)

# Draw
annotated = img.draw_rectangle(10, 10, 100, 50, (255, 0, 0, 255))

# Effect
shadowed = img.drop_shadow(10, 10, 15.0, (0, 0, 0, 128))

# Save
result.save("output.png")
```

---

**Browse examples by category above or jump to [API Reference](../api/)**

