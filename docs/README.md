# 📖 imgrs Documentation

Welcome to **imgrs** - A blazing fast image processing library for Python, powered by Rust! 🚀

## 🎯 What is imgrs?

imgrs provides a **Pillow-compatible API** with **Rust performance**:
- ⚡ **10-100x faster** than pure Python
- 🔒 **Memory safe** with Rust's guarantees
- 🎨 **Easy to use** - familiar Pillow-like interface
- 🌍 **Cross-platform** - works on Linux, Windows, macOS, Android

## 📚 Documentation Structure

### For Beginners
- **[Quick Start](guides/quickstart.md)** - Get started in 5 minutes
- **[Installation](guides/installation.md)** - How to install
- **[Basic Usage](guides/basic-usage.md)** - Common operations
- **[Examples](examples/)** - Ready-to-use code examples

### API Reference
- **[Image Class](api/image.md)** - Core Image class
- **[Constructors](api/constructors.md)** - Creating images
- **[Filters](api/filters.md)** - Image filters
- **[Drawing](api/drawing.md)** - Drawing operations
- **[Pixels](api/pixels.md)** - Pixel manipulation
- **[Effects](api/effects.md)** - Shadow and glow effects

### Type Reference
- **[Types & Enums](reference/types.md)** - All types and enums
- **[Color Types](reference/colors.md)** - Color specifications
- **[Image Modes](reference/modes.md)** - Image mode details

### Advanced Topics
- **[Performance Guide](guides/performance.md)** - Optimization tips
- **[Migration from Pillow](guides/migration.md)** - Switching guide
- **[Contributing](guides/contributing.md)** - How to contribute

## 🚀 Quick Example

```python
from imgrs import Image

# Open and resize an image
img = Image.open("photo.jpg")
resized = img.resize((800, 600))
resized.save("resized.jpg")

# Apply filters
blurred = img.blur(5.0)
sharp = img.sharpen(2.0)

# Draw on image
img.draw_rectangle(10, 10, 100, 50, (255, 0, 0, 255))
img.save("output.png")
```

## 📖 Navigation

Choose your path:

- 🆕 **New to imgrs?** Start with [Quick Start](guides/quickstart.md)
- 🔍 **Looking for something?** Check [API Reference](api/)
- 💡 **Need examples?** Browse [Examples](examples/)
- 🎓 **Coming from Pillow?** See [Migration Guide](guides/migration.md)

## 🆘 Getting Help

- 📝 **Documentation**: You're here!
- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/GrandpaEJ/imgrs/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/GrandpaEJ/imgrs/discussions)
- 📧 **Email**: your-email@example.com

## 📄 License

This project is licensed under the IRADL License.

---

**Happy image processing!** 🎨✨

