# 🎨 Blending API

Complete reference for advanced image compositing and blending operations.

## Overview

Imgrs provides professional-grade image blending capabilities with 29 different blend modes, supporting both basic compositing and advanced features like positioned overlays with masks.

## Basic Compositing

### `img.composite(other, mode="over")`

Composite two images using a blend mode.

**Parameters:**
- `other` (Image): Image to composite on top
- `mode` (str): Blend mode (default: "over")

**Returns:** `Image`

**Example:**
```python
# Basic alpha blending
base = Image.new('RGB', (400, 300), 'blue')
overlay = Image.new('RGBA', (200, 200), (255, 0, 0, 128))
result = base.composite(overlay, mode="over")

# Different blend modes
multiply = base.composite(overlay, mode="multiply")
screen = base.composite(overlay, mode="screen")
overlay_blend = base.composite(overlay, mode="overlay")
```

---

## Convenience Methods

### `img.blend_over(other)`
### `img.blend_multiply(other)`
### `img.blend_screen(other)`
### `img.blend_overlay(other)`
### `img.blend_darken(other)`
### `img.blend_lighten(other)`
### `img.blend_difference(other)`
### `img.blend_exclusion(other)`
### `img.blend_hard_light(other)`
### `img.blend_soft_light(other)`
### `img.blend_color_dodge(other)`
### `img.blend_color_burn(other)`
### `img.blend_add(other)`

Apply specific blend modes with simplified method names.

**Parameters:**
- `other` (Image): Image to blend with

**Returns:** `Image`

**Example:**
```python
# Using convenience methods
multiply = base.blend_multiply(overlay)
difference = base.blend_difference(overlay)
screen = base.blend_screen(overlay)
```

---

## Advanced Blending

### `img.blend(mode, other=None, mask=None, position=None)`

Advanced blending with position and mask support for precise compositing control.

**Parameters:**
- `mode` (str): Blend mode to use
- `other` (Image, optional): Image to blend on top
- `mask` (Image, optional): Mask image for alpha control (grayscale or RGBA)
- `position` (tuple, optional): (x, y) position to place overlay (default: (0, 0))

**Returns:** `Image`

**Example:**
```python
# Positioned overlay
result = base.blend('multiply', overlay, position=(50, 50))

# Masked blending
mask = Image.new('L', (100, 100), 128)  # 50% opacity mask
result = base.blend('overlay', overlay, mask=mask)

# Full control
result = base.blend('screen', overlay, mask=mask, position=(25, 75))
```

---

## Blend Modes

### Porter-Duff Compositing
- `"clear"`: No source, no destination
- `"source"`: Source only
- `"over"`: Source over destination (normal alpha blending)
- `"in"`: Source inside destination
- `"out"`: Source outside destination
- `"atop"`: Source atop destination
- `"dest"`: Destination only
- `"dest_over"`: Destination over source
- `"dest_in"`: Destination inside source
- `"dest_out"`: Destination outside source
- `"dest_atop"`: Destination atop source
- `"xor"`: Source XOR destination

### Arithmetic Modes
- `"add"`: Add source and destination
- `"saturate"`: Saturated add

### Blend Modes
- `"multiply"`: Multiply colors
- `"screen"`: Screen blend
- `"overlay"`: Overlay blend
- `"darken"`: Darker of source or destination
- `"lighten"`: Lighter of source or destination
- `"color_dodge"`: Color dodge
- `"color_burn"`: Color burn
- `"hard_light"`: Hard light
- `"soft_light"`: Soft light
- `"difference"`: Absolute difference
- `"exclusion"`: Exclusion blend

### Special Modes
- `"hsl_hue"`: Hue from source, others from destination
- `"hsl_saturation"`: Saturation from source, others from destination
- `"hsl_color"`: Hue and saturation from source, luminosity from destination
- `"hsl_luminosity"`: Luminosity from source, others from destination

## Masking

Masks control the opacity of the blend operation:

```python
# Grayscale mask (0 = transparent, 255 = opaque)
mask = Image.new('L', (100, 100), 128)  # 50% opacity

# RGBA mask (alpha channel used)
rgba_mask = Image.new('RGBA', (100, 100), (255, 255, 255, 128))

# Apply masked blend
result = base.blend('multiply', overlay, mask=mask)
```

## Positioning

Position overlays precisely on the base image:

```python
# Place overlay at specific coordinates
result = base.blend('screen', overlay, position=(100, 50))

# Center overlay
x = (base.width - overlay.width) // 2
y = (base.height - overlay.height) // 2
result = base.blend('overlay', overlay, position=(x, y))
```

## Performance

- **Basic compositing**: O(width × height) - very fast
- **Positioned blending**: O(width × height) - same performance
- **Masked blending**: O(width × height) - mask lookup adds minimal overhead
- **Memory**: All operations create new images

## Common Patterns

### Layered Composition
```python
# Build up complex compositions
base = Image.open("background.jpg")
layer1 = Image.open("foreground1.png")
layer2 = Image.open("foreground2.png")

# Composite layers with different modes
result = base.blend('normal', layer1, position=(0, 0))
result = result.blend('multiply', layer2, position=(50, 50))
```

### Masked Effects
```python
# Apply effect only to specific areas
effect = base.blur(5.0)
mask = Image.new('L', base.size, 0)  # Start transparent

# Draw mask shape
mask = mask.draw_circle(base.width//2, base.height//2, 100, 255)

# Apply effect through mask
result = base.blend('normal', effect, mask=mask)
```

### Texture Overlay
```python
# Add texture with controlled opacity
texture = Image.open("paper_texture.jpg")
texture = texture.convert('RGBA').set_alpha(0.3)

result = base.blend('overlay', texture)
```

## Error Handling

```python
try:
    result = base.blend('invalid_mode', overlay)
except ValueError as e:
    print(f"Invalid blend mode: {e}")

try:
    result = base.blend('multiply', overlay, position=(-10, -10))
except ValueError as e:
    print(f"Invalid position: {e}")
```

---

**See Also:**
- [Image API](image.md) - Core image operations
- [Transform API](transform.md) - Resize and positioning
- [Examples](../examples/blending_demo.py) - Complete blending examples