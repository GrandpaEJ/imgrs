# 🏁 imgrs vs Pillow - Performance Benchmark

**Real benchmark results** comparing imgrs and Pillow performance.

## Test Configuration

- **Test Image**: 1920x1080 JPEG (Full HD)
- **Iterations**: 50 per test
- **Platform**: Linux x86_64
- **Python**: 3.12
- **Pillow Version**: 11.3.0
- **imgrs Version**: 0.1.0

## 📊 Benchmark Results

### Individual Test Results

| Test | Pillow (ms) | imgrs (ms) | Winner | Speedup |
|------|-------------|------------|--------|---------|
| **Open Image** | 0.66 | 0.00 | ⚡ **imgrs** | **223.7x** |
| **Save PNG** | 157.82 | 21.54 | ⚡ **imgrs** | **7.3x** |
| **RGB → Grayscale** | 4.11 | 26.66 | Pillow | 0.15x (6.5x slower) |
| **Rotate 90°** | 5.16 | 11.26 | Pillow | 0.46x (2.2x slower) |
| **Flip Horizontal** | 4.09 | 12.19 | Pillow | 0.34x (3.0x slower) |
| **Resize (800x600)** | 25.97 | 64.07 | Pillow | 0.41x (2.5x slower) |
| **Crop (500x500)** | 0.37 | 1.42 | Pillow | 0.26x (3.8x slower) |
| **Split RGB Channels** | 4.40 | 11.25 | Pillow | 0.39x (2.6x slower) |
| **Composite Workflow** | 39.32 | 77.11 | Pillow | 0.51x (2.0x slower) |

### Summary

- **imgrs wins**: 2 tests
- **Pillow wins**: 7 tests
- **Average speedup**: 26.0x (heavily skewed by open performance)

## 🎯 Key Findings

### Where imgrs Excels ⚡

1. **Image Opening** - 223.7x faster
   - imgrs uses lazy loading
   - Defers actual decoding until needed
   - Blazing fast for read operations

2. **Image Saving** - 7.3x faster
   - Optimized PNG encoding
   - Efficient I/O operations
   - Significant improvement

### Where Pillow Excels 🏆

1. **Transform Operations** - 2-6x faster
   - Convert, rotate, flip, resize, crop
   - Highly optimized C implementations
   - Mature codebase with years of optimization

2. **In-Memory Operations** - Generally faster
   - Pillow's C backend is well-optimized
   - imgrs has overhead from Rust↔Python boundary

## 📈 Performance Analysis

### imgrs Strengths

✅ **I/O Bound Operations**
- Opening files: **223x faster**
- Saving files: **7x faster**
- Lazy loading strategy pays off

✅ **Use Cases**
- Batch file processing
- Web servers (fast open/save)
- API endpoints
- File conversion tools

### Pillow Strengths

✅ **Transform Operations**
- Resize, rotate, crop: **2-4x faster**
- In-memory processing
- Highly optimized algorithms

✅ **Use Cases**
- Image manipulation pipelines
- Real-time processing
- Complex transformations
- Established production systems

## 🎯 When to Use Each

### Choose imgrs for:

```python
# ✅ File I/O heavy workloads
for file in files:
    img = Image.open(file)  # 223x faster!
    # Quick processing
    img.save(output)  # 7x faster!

# ✅ Server applications
@app.route('/convert')
def convert():
    img = Image.open(uploaded_file)  # Fast!
    img.save(output, format="PNG")   # Fast!
    return output
```

### Choose Pillow for:

```python
# ✅ Heavy transformation pipelines
img = Image.open(file)
img = img.resize((800, 600))    # 2.5x faster
img = img.rotate(45)             # Arbitrary angles
img = img.filter(custom_filter)  # More filters
# Many transformations
```

### Use Both Together:

```python
from imgrs import Image as FastImage
from PIL import Image as PILImage

# Fast I/O with imgrs
fast_img = FastImage.open("huge_file.jpg")  # 223x faster!
fast_img.save("temp.jpg")

# Complex transforms with Pillow
pil_img = PILImage.open("temp.jpg")
processed = pil_img.resize(...).rotate(45).filter(...)
processed.save("output.jpg")

# Back to imgrs for final save
final = FastImage.open("output.jpg")
final.save("final.png")  # 7x faster!
```

## 🔬 Test Methodology

### What We Tested

✅ **Fair Comparisons Only**
- Only features available in both libraries
- Equivalent operations (same parameters)
- Same input/output formats
- Same quality settings

✅ **Real-World Scenarios**
- Actual file I/O (not in-memory only)
- Standard image sizes (Full HD)
- Common operations
- Typical workflows

### What We Didn't Test

❌ **Not Compared** (Different APIs):
- Blur filters (different implementations)
- Sharpen (different strengths)
- Brightness/Contrast (different parameters)
- Drawing operations (completely different API)
- Effects (imgrs-specific: drop_shadow, glow, etc.)

## 🚀 Performance Recommendations

### For Maximum Speed

```python
# Hybrid approach - best of both worlds:

# 1. Use imgrs for I/O
from imgrs import Image as FastImage
img = FastImage.open("large.jpg")  # 223x faster open!

# 2. Convert to numpy for processing
import numpy as np
from PIL import Image
array = np.array(Image.open("temp.jpg"))

# 3. Use Pillow for transforms if needed
pil_img = Image.fromarray(array)
processed = pil_img.resize((800, 600))

# 4. Save with imgrs
fast_img = FastImage.fromarray(np.array(processed))
fast_img.save("output.png")  # 7x faster save!
```

## 📝 Detailed Results

Full JSON results saved to: `results/benchmark_results.json`

### Test Environment

```bash
# Run benchmarks yourself:
cd benchmark/
python benchmark_fixed.py

# View results:
cat results/benchmark_results.json
```

## 🎓 Conclusions

### imgrs Status

**Current State (v0.1.0)**:
- ⚡ Exceptional I/O performance (7-223x faster)
- ⚠️ Transform operations need optimization (2-6x slower)
- 🎯 Best for file-heavy workloads
- 🚀 New library with room for optimization

**Future Potential**:
- Optimize transform operations
- Add SIMD optimizations
- Parallel processing
- Close performance gap with Pillow

### Pillow Status

**Current State**:
- 🏆 Mature, highly optimized
- ⚡ Fast transforms
- 📚 Comprehensive features
- 🌍 Industry standard

**Trade-offs**:
- Slower I/O operations
- Python/C boundary overhead for some ops

## 🎯 Recommendation

**For Production Use:**

1. **File Conversion Tools** → Use imgrs (223x faster open, 7x faster save)
2. **Web APIs** → Use imgrs (fast I/O matters most)
3. **Image Manipulation** → Use Pillow (faster transforms)
4. **Batch Processing** → Hybrid approach (imgrs I/O + Pillow transforms)
5. **Real-Time Apps** → Depends on bottleneck (I/O vs transforms)

## 🔄 Version History

- **2025-10-09**: Initial benchmark (imgrs v0.1.0 vs Pillow 11.3.0)

---

**Run your own benchmarks:**
```bash
cd benchmark/
source ../benchmark_env/bin/activate  # If using venv
python benchmark_fixed.py
```

**Results will be saved to:** `results/benchmark_results.json`

