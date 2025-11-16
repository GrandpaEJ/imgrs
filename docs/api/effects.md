# ✨ Effects API

Complete reference for shadow, glow, and blending effects.

## Shadow Effects

### `img.drop_shadow(offset_x, offset_y, blur_radius, shadow_color)`

Add a drop shadow behind the image.

**Parameters:**
- `offset_x` (int): Horizontal shadow offset (pixels)
  - Positive: shadow to the right
  - Negative: shadow to the left
- `offset_y` (int): Vertical shadow offset (pixels)
  - Positive: shadow downward
  - Negative: shadow upward
- `blur_radius` (float): Shadow blur amount (0.0 = sharp)
- `shadow_color` (Tuple[int, int, int, int]): RGBA shadow color

**Returns:** `Image` (larger than original to accommodate shadow)

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

# Hard shadow
hard = img.drop_shadow(8, 8, 2.0, (0, 0, 0, 200))

# Colored shadow
colored = img.drop_shadow(15, 15, 20.0, (255, 0, 0, 100))  # Red shadow
```

**Canvas expansion:**
- Output image is larger than input
- Padding = `(blur_radius × 2) + abs(offset)`
- Original image positioned centered in new canvas

---

### `img.inner_shadow(offset_x, offset_y, blur_radius, shadow_color)`

Add an inner shadow (shadow inside the image edges).

**Parameters:**
- `offset_x` (int): Horizontal shadow offset
- `offset_y` (int): Vertical shadow offset
- `blur_radius` (float): Shadow blur amount
- `shadow_color` (Tuple[int, int, int, int]): RGBA shadow color

**Returns:** `Image` (same size as original)

**Example:**
```python
# Inner shadow for depth
inset = img.inner_shadow(
    offset_x=5,
    offset_y=5,
    blur_radius=10.0,
    shadow_color=(0, 0, 0, 180)
)

# Top-left inner shadow
top_shadow = img.inner_shadow(-3, -3, 8.0, (0, 0, 0, 150))

# Emboss-like effect
embossed = img.inner_shadow(2, 2, 5.0, (0, 0, 0, 100))
```

**Use cases:**
- Button/UI element depth
- Inset text effects
- 3D embossing
- Frame effects

---

### `img.glow(blur_radius, glow_color, intensity)`

Add a glow effect around the image.

**Parameters:**
- `blur_radius` (float): Glow spread (0.0 = no glow)
- `glow_color` (Tuple[int, int, int, int]): RGBA glow color
- `intensity` (float): Glow intensity (0.0 to 1.0+)
  - 0.0: No glow
  - 1.0: Full glow
  - > 1.0: Enhanced glow

**Returns:** `Image` (larger than original)

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

# Subtle glow
subtle = img.glow(10.0, (255, 255, 0, 255), 0.5)  # Yellow

# Intense glow
intense = img.glow(25.0, (255, 0, 255, 255), 1.5)  # Magenta
```

**Canvas expansion:**
- Output image is larger
- Padding = `blur_radius × 2`

---

## Blending Effects

### `img.blend_with(other, mode, opacity)`

Blend image with another using advanced blend modes.

**Parameters:**
- `other` (Image): Image to blend with
- `mode` (str): Blend mode ("normal", "multiply", "screen", "overlay", "soft_light", "hard_light", "color_dodge", "color_burn", "darken", "lighten", "difference", "exclusion")
- `opacity` (float): Blend opacity (0.0-1.0)

**Returns:** `Image` (blended result)

**Example:**
```python
from imgrs import Image, BlendMode

background = Image.open("background.jpg")
overlay = Image.open("overlay.png")

# Different blend modes
multiply = background.blend_with(overlay, BlendMode.MULTIPLY, 0.8)
screen = background.blend_with(overlay, BlendMode.SCREEN, 0.6)
overlay_blend = background.blend_with(overlay, BlendMode.OVERLAY, 0.7)
difference = background.blend_with(overlay, BlendMode.DIFFERENCE, 1.0)
```

### `img.overlay_with(overlay, mode, opacity, position)`

Overlay an image using advanced blending with positioning.

**Parameters:**
- `overlay` (Image): Image to overlay
- `mode` (str): Blend mode
- `opacity` (float): Overlay opacity
- `position` (tuple): Position (x, y), defaults to center

**Returns:** `Image` (overlayed result)

**Example:**
```python
# Center overlay
centered = background.overlay_with(watermark, "normal", 0.5)

# Positioned overlay
positioned = background.overlay_with(watermark, "multiply", 0.8, position=(50, 100))

# Corner watermark
corner = background.overlay_with(watermark, "screen", 0.3, position=(10, 10))
```

### Blend Mode Reference

| Mode | Description | Use Case |
|------|-------------|----------|
| `normal` | Standard alpha blending | Default compositing |
| `multiply` | Multiply colors (darker) | Shadows, darkening |
| `screen` | Screen colors (lighter) | Highlights, lightening |
| `overlay` | Multiply/screen based on base | Contrast enhancement |
| `soft_light` | Soft dodge/burn | Soft lighting effects |
| `hard_light` | Hard dodge/burn | Dramatic lighting |
| `color_dodge` | Lighten with color | Bright highlights |
| `color_burn` | Darken with color | Deep shadows |
| `darken` | Keep darker pixels | Darkening effects |
| `lighten` | Keep lighter pixels | Lightening effects |
| `difference` | Color difference | Special effects |
| `exclusion` | Similar to difference | High contrast |

---

## Effect Combinations

### Layered Effects

```python
# Drop shadow + glow
enhanced = (img
    .drop_shadow(8, 8, 15.0, (0, 0, 0, 120))
    .glow(10.0, (255, 255, 255, 255), 0.6))
```

### UI Button Effect

```python
def create_button(text_img):
    """Create a button with depth."""
    return (text_img
        .drop_shadow(0, 3, 5.0, (0, 0, 0, 100))  # Bottom shadow
        .inner_shadow(0, -1, 3.0, (255, 255, 255, 50)))  # Top highlight

button = create_button(label_img)
```

### Neon Sign Effect

```python
def neon_sign(img, color=(0, 255, 255, 255)):
    """Create neon glow effect."""
    return (img
        .glow(25.0, color, 1.0)
        .glow(15.0, color, 0.8)
        .glow(8.0, color, 1.2))

neon = neon_sign(text_img)
```

---

## Shadow Parameters Guide

### Offset Guidelines

```python
# Natural shadow (bottom-right)
offset_x=10, offset_y=10

# Top shadow (light from bottom)
offset_x=0, offset_y=-5

# Right shadow
offset_x=8, offset_y=0

# No offset (centered shadow/glow)
offset_x=0, offset_y=0
```

### Blur Radius Guidelines

```python
# Sharp shadow (close to object)
blur_radius=2.0

# Medium blur (natural)
blur_radius=10.0

# Soft shadow (far from object)
blur_radius=25.0

# Very diffuse
blur_radius=50.0
```

### Shadow Color Guidelines

```python
# Natural shadow (black with transparency)
shadow_color=(0, 0, 0, 128)

# Soft shadow (more transparent)
shadow_color=(0, 0, 0, 80)

# Hard shadow (more opaque)
shadow_color=(0, 0, 0, 200)

# Colored shadow (creative)
shadow_color=(100, 50, 150, 120)  # Purple
```

---

## Effect Presets

### Realistic Drop Shadow

```python
def realistic_shadow(img):
    return img.drop_shadow(
        offset_x=12,
        offset_y=12,
        blur_radius=20.0,
        shadow_color=(0, 0, 0, 100)
    )
```

### Floating Effect

```python
def floating_effect(img):
    return img.drop_shadow(
        offset_x=0,
        offset_y=15,
        blur_radius=30.0,
        shadow_color=(0, 0, 0, 80)
    )
```

### Neon Glow

```python
def neon_glow(img, color=(0, 255, 255, 255)):
    return img.glow(
        blur_radius=20.0,
        glow_color=color,
        intensity=1.2
    )
```

### Embossed Look

```python
def emboss_effect(img):
    return img.inner_shadow(
        offset_x=3,
        offset_y=3,
        blur_radius=5.0,
        shadow_color=(0, 0, 0, 150)
    )
```

---

## Performance Notes

| Effect | Complexity | Notes |
|--------|------------|-------|
| `drop_shadow()` | O(n×m×r²) | Blur is expensive |
| `inner_shadow()` | O(n×m×r²) | Similar to drop shadow |
| `glow()` | O(n×m×r²) | Blur is expensive |
| `blend_with()` | O(n×m) | Fast pixel operations |
| `overlay_with()` | O(n×m) | Fast pixel operations |

**Tips:**
- Smaller blur radius = faster
- Large images + large blur = slow
- Consider resizing before applying effects
- Effects are CPU-bound (not GPU)

---

## Common Use Cases

### Photo Enhancement

```python
# Add subtle depth
enhanced = img.drop_shadow(5, 5, 10.0, (0, 0, 0, 60))
```

### Logo/Icon Effects

```python
# Make logo pop
logo = (logo
    .drop_shadow(8, 8, 15.0, (0, 0, 0, 120))
    .glow(10.0, (255, 255, 255, 255), 0.5))
```

### Text Effects

```python
# Glowing text
glowing_text = text_img.glow(15.0, (255, 200, 0, 255), 1.0)

# Embossed text
embossed_text = text_img.inner_shadow(2, 2, 3.0, (0, 0, 0, 180))
```

### UI Elements

```python
# Pressed button (inner shadow)
pressed = button.inner_shadow(0, 2, 4.0, (0, 0, 0, 120))

# Floating card (drop shadow)
card = content.drop_shadow(0, 5, 20.0, (0, 0, 0, 100))
```

### Compositing & Blending

```python
# Photo compositing
foreground = Image.open("subject.png")
background = Image.open("scene.jpg")
composited = background.overlay_with(foreground, "normal", 1.0, position=(100, 50))

# Color grading with blend modes
warm_overlay = Image.new("RGB", (800, 600), (255, 200, 150))
graded = photo.blend_with(warm_overlay, "overlay", 0.3)

# Texture overlay
texture = Image.open("paper_texture.jpg")
textured = image.blend_with(texture, "multiply", 0.4)

# Double exposure effect
layer1 = Image.open("portrait.jpg")
layer2 = Image.open("landscape.jpg")
double_exposure = layer1.blend_with(layer2, "screen", 0.7)
```

---

**See Also:**
- [Filters API](filters.md) - Color filters
- [Drawing API](drawing.md) - Draw shapes
- [Examples](../examples/effects.md) - Effect examples

