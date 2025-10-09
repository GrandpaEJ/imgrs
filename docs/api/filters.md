# 🎨 Filters API

Complete reference for image filtering operations.

## Basic Filters

### `img.blur(radius)`

Apply Gaussian blur to the image.

**Parameters:**
- `radius` (float): Blur radius (0.0 = no blur, higher = more blur)

**Returns:** `Image`

**Example:**
```python
# Light blur
light = img.blur(2.0)

# Heavy blur
heavy = img.blur(10.0)

# No blur
unchanged = img.blur(0.0)
```

**Performance:** O(n×m×r²) where r is radius

---

### `img.sharpen(strength)`

Apply sharpening filter.

**Parameters:**
- `strength` (float): Sharpening strength (0.0 = no effect, 1.0 = normal, higher = more sharp)

**Returns:** `Image`

**Example:**
```python
# Gentle sharpen
gentle = img.sharpen(0.5)

# Strong sharpen
sharp = img.sharpen(2.0)

# Extreme sharpen
extreme = img.sharpen(5.0)
```

---

### `img.edge_detect()`

Apply edge detection filter (Sobel operator).

**Parameters:** None

**Returns:** `Image` (grayscale)

**Example:**
```python
edges = img.edge_detect()
edges.save("edges.png")
```

**Note:** Output is always grayscale regardless of input mode.

---

### `img.emboss()`

Apply emboss filter for a 3D effect.

**Parameters:** None

**Returns:** `Image`

**Example:**
```python
embossed = img.emboss()
embossed.save("embossed.png")
```

---

## Adjustment Filters

### `img.brightness(adjustment)`

Adjust image brightness.

**Parameters:**
- `adjustment` (int): Brightness adjustment (-255 to +255)
  - Negative: Darken
  - Positive: Brighten
  - 0: No change

**Returns:** `Image`

**Example:**
```python
# Brighten
brighter = img.brightness(30)

# Darken
darker = img.brightness(-30)

# Extreme brightness
very_bright = img.brightness(100)
```

---

### `img.contrast(factor)`

Adjust image contrast.

**Parameters:**
- `factor` (float): Contrast factor
  - < 1.0: Reduce contrast
  - 1.0: No change
  - > 1.0: Increase contrast

**Returns:** `Image`

**Example:**
```python
# Increase contrast
high_contrast = img.contrast(1.5)

# Decrease contrast
low_contrast = img.contrast(0.5)

# Extreme contrast
extreme = img.contrast(3.0)
```

---

## CSS-Like Filters

### `img.sepia(amount)`

Apply sepia tone filter.

**Parameters:**
- `amount` (float): Sepia amount (0.0 to 1.0)
  - 0.0: No effect
  - 1.0: Full sepia

**Returns:** `Image`

**Example:**
```python
# Subtle sepia
vintage = img.sepia(0.5)

# Full sepia
old_photo = img.sepia(1.0)
```

---

### `img.grayscale_filter(amount)`

Apply grayscale filter (not full conversion).

**Parameters:**
- `amount` (float): Grayscale amount (0.0 to 1.0)

**Returns:** `Image`

**Example:**
```python
# 50% grayscale
half_gray = img.grayscale_filter(0.5)

# Full grayscale (better to use convert("L"))
full_gray = img.grayscale_filter(1.0)
```

**Note:** For full grayscale conversion, use `img.convert("L")` instead.

---

### `img.invert(amount)`

Invert image colors.

**Parameters:**
- `amount` (float): Inversion amount (0.0 to 1.0)

**Returns:** `Image`

**Example:**
```python
# Full invert
negative = img.invert(1.0)

# Partial invert
partial = img.invert(0.5)
```

---

### `img.hue_rotate(degrees)`

Rotate hue of the image.

**Parameters:**
- `degrees` (float): Degrees to rotate hue (-360 to +360)

**Returns:** `Image`

**Example:**
```python
# Shift colors
shifted = img.hue_rotate(90)

# Full rotation
rotated = img.hue_rotate(180)

# Negative rotation
reverse = img.hue_rotate(-45)
```

---

### `img.saturate(amount)`

Adjust color saturation.

**Parameters:**
- `amount` (float): Saturation amount
  - 0.0: Grayscale
  - 1.0: Original
  - > 1.0: Oversaturated

**Returns:** `Image`

**Example:**
```python
# Desaturate
muted = img.saturate(0.5)

# Boost saturation
vibrant = img.saturate(1.5)

# Completely desaturate (grayscale)
gray = img.saturate(0.0)
```

---

## Filter Chaining

All filters return new images, so you can chain them:

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

## Performance Tips

1. **Blur**: Larger radius = slower (O(r²))
2. **Sharpen**: Very fast (single convolution)
3. **Edge Detect**: Fast (two convolutions)
4. **CSS Filters**: Fast (per-pixel operations)
5. **Chaining**: Each operation creates a new image

**Optimization:**
```python
# ❌ Slow - intermediate images created
temp1 = img.blur(5.0)
temp2 = temp1.brightness(20)
result = temp2.contrast(1.5)

# ✅ Better - same result, clearer code
result = img.blur(5.0).brightness(20).contrast(1.5)
```

---

**See Also:**
- [Transform API](transform.md) - Resize, crop, rotate
- [Effects API](effects.md) - Shadows and glows
- [Examples](../examples/filters.md) - More filter examples

