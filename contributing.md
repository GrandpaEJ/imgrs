# Contributing to Imgrs

Thank you for your interest in contributing to Imgrs! This guide will help you get started with contributing to the project.

## 🚀 Getting Started

### Prerequisites

- **Python 3.8+**
- **Rust 1.70+**
- **Git**
- **Maturin** for building Python extensions

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/grandpaej/imgrs.git
   cd imgrs
   ```

2. **Set up Python Environment**
   ```bash
   python -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   pip install -r requirements.txt
   pip install maturin pytest
   ```

3. **Build Development Version**
   ```bash
   maturin develop --release
   ```

4. **Run Tests**
   ```bash
   pytest python/imgrs/tests/
   # Or run all test scripts
   cd test && python run_all.py
   ```

## 🎯 Areas for Contribution

### 🔥 High Priority

1. **Performance Optimization**
   - Benchmark existing operations
   - Optimize memory usage
   - Implement SIMD optimizations
   - Parallel processing improvements
   - Mobile-specific optimizations

2. **Mobile & Platform Support**
   - Android build optimization
   - iOS compatibility testing
   - ARM architecture optimization
   - WebAssembly support
   - Embedded systems support

### 🚧 Medium Priority

3. **Image Processing Features**
   - More geometric transformations
   - Advanced color manipulation
   - HDR image support
   - RAW image format support
   - Video frame processing

4. **API Enhancements**
   - Better error handling and messages
   - Type hints improvements
   - Async/await support for I/O operations
   - Streaming image processing
   - Plugin system for custom filters

### 📚 Documentation & Testing

5. **Documentation**
   - More real-world examples
   - Performance comparisons
   - Video tutorials
   - API documentation improvements
   - Translation to other languages

6. **Testing**
   - Edge case testing
   - Performance benchmarks
   - Compatibility tests with different Python versions
   - Memory leak testing
   - Cross-platform testing

## 🛠️ Development Workflow

### Code Organization

```
imgrs/
├── src/                     # Rust source code
│   ├── lib.rs              # Main Rust library entry point
│   ├── errors.rs           # Error types
│   ├── operations.rs       # Common operations
│   ├── formats.rs          # Image format handling
│   ├── image/              # Image struct and core
│   │   ├── mod.rs
│   │   ├── core.rs         # PyImage struct
│   │   ├── constructors.rs # Image creation (open, new, frombytes)
│   │   ├── io.rs           # Save/load operations
│   │   ├── transform.rs    # Rotate, resize, crop
│   │   ├── filters.rs      # Image filters
│   │   ├── drawing.rs      # Drawing operations
│   │   ├── effects.rs      # Effects (shadows, glows)
│   │   ├── text_ops.rs     # Text rendering
│   │   └── metadata_ops.rs # EXIF/metadata
│   ├── filters/            # Filter implementations
│   │   ├── blur.rs
│   │   ├── sharpen.rs
│   │   ├── edges.rs
│   │   └── auto_enhance.rs
│   ├── drawing/            # Drawing primitives
│   │   ├── shapes.rs       # Basic shapes (circle, rectangle, line)
│   │   ├── shapes_extended.rs # New shapes (star, polygon, ellipse)
│   │   └── text.rs
│   ├── text/               # Text rendering system
│   ├── metadata/           # EXIF/metadata handling
│   ├── blending/           # Blending modes
│   ├── shadows/            # Shadow effects
│   └── css_filters/        # CSS-style filters
├── python/imgrs/           # Python package
│   ├── __init__.py         # Python API
│   ├── image.py            # Main Image class
│   ├── enums.py            # Enums and constants
│   ├── operations.py       # Helper operations
│   ├── mixins/             # Feature mixins
│   │   ├── core_mixin.py
│   │   ├── transform_mixin.py
│   │   ├── filter_mixin.py
│   │   ├── drawing_mixin.py
│   │   ├── text_mixin.py
│   │   └── ...
│   └── tests/              # Python tests
├── docs/                   # Documentation
├── examples/               # Example scripts
└── test/                   # Test scripts
    ├── scripts/            # Test Python scripts
    └── output/             # Test output images
```

### Making Changes

1. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Your Changes**
   - Follow the coding standards below
   - Add tests for new functionality
   - Update documentation as needed

3. **Test Your Changes**
   ```bash
   # Run Python tests
   pytest python/imgrs/tests/
   
   # Run Rust tests
   cargo test
   
   # Test examples
   python examples/basic_operations.py
   
   # Run comprehensive tests
   cd test && python run_all.py
   ```

4. **Lint and Format**
   ```bash
   # Rust
   cargo fmt
   cargo clippy
   
   # Python
   black python/
   ruff check python/
   ```

5. **Commit and Push**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   git push origin feature/your-feature-name
   ```

6. **Create Pull Request**
   - Describe your changes clearly
   - Include examples if applicable
   - Reference any related issues
   - Add screenshots/benchmarks if relevant

## 📝 Coding Standards

### Rust Code

- Follow standard Rust formatting (`cargo fmt`)
- Use `cargo clippy` for linting
- Add documentation comments for public functions
- Include error handling with proper error types
- Use meaningful variable and function names

```rust
/// Apply Gaussian blur to the image.
/// 
/// # Arguments
/// * `radius` - Blur radius in pixels (must be positive)
/// 
/// # Returns
/// Result containing new blurred image or error
/// 
/// # Example
/// ```rust
/// let blurred = image.blur_impl(2.0)?;
/// ```
/// 
/// # Errors
/// Returns error if radius is negative or image processing fails
pub fn blur_impl(&mut self, radius: f32) -> PyResult<Self> {
    if radius < 0.0 {
        return Err(ImgrsError::InvalidOperation(
            "Blur radius must be non-negative".to_string()
        ).into());
    }
    // Implementation
}
```

### Python Code

- Follow PEP 8 style guide
- Use type hints where possible
- Add docstrings for all public functions
- Include examples in docstrings
- Use descriptive variable names

```python
def blur(self, radius: float) -> 'Image':
    """Apply Gaussian blur to the image.
    
    Args:
        radius: Blur radius in pixels (must be positive)
        
    Returns:
        New blurred Image instance
        
    Raises:
        ValueError: If radius is negative
        
    Example:
        >>> img = imgrs.Image.open("photo.jpg")
        >>> blurred = img.blur(2.0)
        >>> blurred.save("blurred.jpg")
        
    Note:
        Larger radius values produce stronger blur but take longer to process.
    """
    if radius < 0:
        raise ValueError("Blur radius must be non-negative")
    return self._rust_image.blur(radius)
```

### Documentation

- Use clear, concise language
- Include practical examples
- Add performance notes where relevant
- Keep documentation up to date with code changes
- Use code blocks for examples
- Link to related functions/methods

## 🧪 Testing Guidelines

### Writing Tests

1. **Unit Tests** - Test individual functions
2. **Integration Tests** - Test feature combinations
3. **Performance Tests** - Benchmark critical operations
4. **Compatibility Tests** - Ensure Pillow compatibility
5. **Edge Case Tests** - Test boundary conditions

### Test Structure

```python
import pytest
import imgrs
import numpy as np

class TestImageFilters:
    """Test image filter operations."""
    
    def test_blur_basic(self):
        """Test basic blur functionality."""
        img = imgrs.Image.new("RGB", (100, 100), "red")
        blurred = img.blur(2.0)
        
        assert blurred.size == img.size
        assert blurred.mode == img.mode
    
    def test_blur_radius_validation(self):
        """Test blur radius validation."""
        img = imgrs.Image.new("RGB", (100, 100), "red")
        
        with pytest.raises(ValueError):
            img.blur(-1.0)  # Negative radius should fail
    
    @pytest.mark.performance
    def test_blur_performance(self):
        """Test blur performance."""
        img = imgrs.Image.new("RGB", (1000, 1000), "red")
        
        import time
        start = time.time()
        blurred = img.blur(3.0)
        duration = time.time() - start
        
        assert duration < 1.0  # Should complete in under 1 second

class TestNewShapes:
    """Test new v0.2.2 shape drawing features."""
    
    def test_draw_star(self):
        """Test star drawing."""
        img = imgrs.Image.new("RGB", (200, 200), "white")
        result = img.draw_star(100, 100, 50, 25, 5, (255, 0, 0, 255))
        
        assert result.size == img.size
    
    def test_draw_polygon(self):
        """Test polygon drawing."""
        img = imgrs.Image.new("RGB", (200, 200), "white")
        points = [(50, 50), (150, 50), (100, 150)]
        result = img.draw_polygon(points, (0, 255, 0, 255))
        
        assert result.size == img.size

class TestArbitraryRotation:
    """Test v0.2.2 arbitrary angle rotation."""
    
    def test_rotate_45_degrees(self):
        """Test 45-degree rotation."""
        img = imgrs.Image.new("RGB", (100, 100), "red")
        rotated = img.rotate(45.0)
        
        assert rotated is not None
        # Rotated image may have different dimensions
    
    def test_rotate_arbitrary_angle(self):
        """Test arbitrary angle rotation."""
        img = imgrs.Image.new("RGB", (100, 100), "blue")
        
        for angle in [15, 30, 72, 123, 200]:
            rotated = img.rotate(float(angle))
            assert rotated is not None
```

### Running Tests

```bash
# Run all tests
pytest

# Run specific test file
pytest python/imgrs/tests/test_filters.py

# Run with coverage
pytest --cov=imgrs --cov-report=html

# Run performance tests only
pytest -m performance

# Run tests in parallel
pytest -n auto

# Run test suite
cd test && python run_all.py
```

## 🚀 Performance Guidelines

### Benchmarking

Always benchmark performance changes:

```python
import time
import imgrs

def benchmark_operation(operation, iterations=10):
    """Benchmark an image operation."""
    img = imgrs.Image.open("test_image.jpg")
    
    times = []
    for _ in range(iterations):
        start = time.time()
        result = operation(img)
        end = time.time()
        times.append(end - start)
    
    avg_time = sum(times) / len(times)
    min_time = min(times)
    max_time = max(times)
    
    print(f"Average time: {avg_time:.4f}s")
    print(f"Min time: {min_time:.4f}s")
    print(f"Max time: {max_time:.4f}s")
    return avg_time

# Benchmark blur operation
print("Benchmarking blur:")
benchmark_operation(lambda img: img.blur(2.0))

# Benchmark new rotation
print("\nBenchmarking arbitrary rotation:")
benchmark_operation(lambda img: img.rotate(45.0))
```

### Memory Usage

Monitor memory usage for large images:

```python
import psutil
import os

def monitor_memory(func):
    """Monitor memory usage during function execution."""
    process = psutil.Process(os.getpid())
    
    mem_before = process.memory_info().rss / 1024 / 1024  # MB
    result = func()
    mem_after = process.memory_info().rss / 1024 / 1024   # MB
    
    print(f"Memory used: {mem_after - mem_before:.1f} MB")
    print(f"Final memory: {mem_after:.1f} MB")
    return result
```

## 🎨 Adding New Features

### Example: Adding a New Filter

1. **Add Rust Implementation** (e.g., `src/filters/new_filter.rs`)
   ```rust
   use image::DynamicImage;
   use crate::errors::ImgrsError;
   
   pub fn new_filter(image: &DynamicImage, param: f32) -> Result<DynamicImage, ImgrsError> {
       // Implementation
       Ok(image.clone())
   }
   ```

2. **Export in Module** (`src/filters/mod.rs`)
   ```rust
   mod new_filter;
   pub use new_filter::new_filter;
   ```

3. **Add to PyImage** (`src/image/filters.rs`)
   ```rust
   pub fn new_filter_impl(&mut self, param: f32) -> PyResult<Self> {
       let format = self.format;
       let image = self.get_image()?;
       Python::with_gil(|py| {
           py.allow_threads(|| filters::new_filter(image, param))
       }).map(|filtered| PyImage {
           lazy_image: LazyImage::Loaded(filtered),
           format
       }).map_err(|e| e.into())
   }
   ```

4. **Add Python Binding** (`src/image/pymethods.rs`)
   ```rust
   fn new_filter(&mut self, param: f32) -> PyResult<Self> {
       self.new_filter_impl(param)
   }
   ```

5. **Add Python Wrapper** (if needed in mixin)
   ```python
   def new_filter(self, param: float) -> 'Image':
       """Apply new filter.
       
       Args:
           param: Filter parameter
           
       Returns:
           Filtered image
       """
       return self._rust_image.new_filter(param)
   ```

6. **Add Tests**
   ```python
   def test_new_filter():
       img = imgrs.Image.new("RGB", (100, 100), "red")
       result = img.new_filter(2.0)
       assert result.size == img.size
   ```

7. **Update Documentation**
   - Add to relevant docs/*.md file
   - Add example in examples/
   - Update CHANGELOG.md

### Example: Adding a New Shape

See `src/drawing/shapes_extended.rs` for reference on adding new shapes like stars, polygons, and ellipses.

## 📋 Pull Request Guidelines

### Before Submitting

- [ ] Code follows style guidelines (cargo fmt, black)
- [ ] All tests pass locally
- [ ] Documentation is updated
- [ ] Performance impact is considered and benchmarked
- [ ] Breaking changes are documented
- [ ] CHANGELOG.md is updated

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix (non-breaking change fixing an issue)
- [ ] New feature (non-breaking change adding functionality)
- [ ] Performance improvement
- [ ] Documentation update
- [ ] Breaking change (fix or feature causing existing functionality to change)

## New Features (if applicable)
- Feature 1: Description
- Feature 2: Description

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Performance benchmarks run (attach results if relevant)
- [ ] Tested on multiple platforms

## Documentation
- [ ] Code comments added
- [ ] API documentation updated
- [ ] Examples updated
- [ ] CHANGELOG.md updated

## Screenshots/Benchmarks
(If applicable, add screenshots or benchmark results)

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Tests pass locally
- [ ] No breaking changes (or properly documented)
- [ ] Commits are clear and descriptive
```

## 🐛 Bug Reports

### Before Reporting

1. Check existing issues
2. Test with latest version (`pip install --upgrade imgrs`)
3. Create minimal reproduction case
4. Test with different image formats/sizes

### Bug Report Template

```markdown
## Bug Description
Clear description of the bug

## To Reproduce
Steps to reproduce:
1. Load image with `imgrs.Image.open("image.jpg")`
2. Apply operation `img.blur(2.0)`
3. See error

## Expected Behavior
What should happen

## Actual Behavior
What actually happens (include full error traceback)

## Environment
- Imgrs version: 0.2.2
- Python version: 3.11.0
- Operating system: Ubuntu 22.04 / Windows 11 / macOS 14
- Image format/size: JPEG 1920x1080
- NumPy installed: Yes/No

## Code Sample
```python
import imgrs

img = imgrs.Image.open("test.jpg")
img.blur(2.0)  # Error occurs here
```

## Additional Context
Any other relevant information, error messages, or stack traces
```

## 💡 Feature Requests

### Before Requesting

1. Check if feature already exists
2. Search closed issues for previous discussions
3. Consider if it fits Imgrs's scope (Python image processing with Rust performance)
4. Think about API design and Pillow compatibility

### Feature Request Template

```markdown
## Feature Description
Clear description of the proposed feature

## Use Case
Why is this feature needed? What problem does it solve?

## Proposed API
```python
# Example of how the feature would be used
result = img.new_feature(parameters)
```

## Implementation Ideas
Any thoughts on implementation approach

## Alternatives
Other ways to achieve the same goal

## Additional Context
Links to similar features in other libraries, research papers, etc.
```

## 🏆 Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes for significant contributions
- Documentation credits
- Special mention for major features

## 📞 Getting Help

- **GitHub Issues** - For bugs and feature requests
- **GitHub Discussions** - For questions and general discussion
- **Code Review** - All PRs receive thorough review and constructive feedback
- **Documentation** - Check docs/ for detailed guides

## 🔗 Resources

### Learning Resources
- **[Rust Book](https://doc.rust-lang.org/book/)** - Learn Rust programming
- **[PyO3 Guide](https://pyo3.rs/)** - Python-Rust integration
- **[image-rs Documentation](https://docs.rs/image/)** - Core image library we use
- **[imageproc Documentation](https://docs.rs/imageproc/)** - Image processing algorithms
- **[Pillow Documentation](https://pillow.readthedocs.io/)** - API compatibility reference

### Project Resources
- **Repository:** https://github.com/grandpaej/imgrs
- **Documentation:** docs/
- **Examples:** examples/
- **License:** Apache 2.0

## 📝 License

By contributing to Imgrs, you agree that your contributions will be licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

Your contributions must:
- Be your original work or properly attributed
- Not violate any third-party licenses
- Include proper copyright notices

## 🌟 Recent Updates (v0.2.2)

### New Features
- **Arbitrary Angle Rotation** - Rotate images by any angle (not just 90°)
- **Extended Shape Drawing** - Stars, triangles, polygons, ellipses
- **Mobile Support** - `frombytes()` for NumPy-free image creation
- **NumPy Optional** - Base package works without NumPy

### Coming Soon
- WebAssembly support
- More geometric transformations
- HDR image support
- Video frame processing

---

Thank you for contributing to Imgrs! 🎨🚀

Every contribution, no matter how small, helps make Imgrs better for everyone!
