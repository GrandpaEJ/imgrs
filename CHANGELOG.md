# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-10-10

### Added

#### Core Features
- Eager loading (load images immediately like Pillow)
- NumPy integration with `fromarray()` and `to_bytes()`
- Channel operations: `split()` for RGB/RGBA separation
- Image composition with `paste()`

#### Filters (65+ total)
- **Basic Filters:** blur, sharpen, edge_detect, emboss, brightness, contrast
- **Advanced Blur:** box_blur, bilateral_blur, median_blur, motion_blur, radial_blur, zoom_blur
- **Edge Detection:** prewitt_edge_detect, canny_edge_detect, laplacian_edge_detect, scharr_edge_detect
- **Sharpening:** unsharp_mask, edge_enhance, edge_enhance_more
- **CSS-Style:** sepia, grayscale_filter, invert, hue_rotate, saturate
- **Artistic:** oil_painting, watercolor, pencil_sketch, cartoon, sketch, halftone, vignette, glitch
- **Morphological:** dilate, erode, morphological_gradient
- **Noise:** add_gaussian_noise, add_salt_pepper_noise, denoise
- **Color Effects:** duotone, color_splash, chromatic_aberration

#### Auto-Enhancement (9 features)
- `histogram_equalization()` - Histogram equalization
- `auto_contrast()` - Automatic contrast adjustment
- `auto_brightness()` - Automatic brightness adjustment
- `auto_enhance()` - Complete auto optimization
- `exposure_adjust()` - Exposure adjustment
- `auto_level()` - Automatic level adjustment
- `normalize()` - Normalize to full range
- `smart_enhance()` - Smart enhancement
- `auto_white_balance()` - Automatic white balance

#### Rich Text Rendering
- `add_text()` - Basic text rendering with TTF/OTF fonts
- `add_text_styled()` - Full styling (outline, shadow, background, opacity, alignment)
- `add_text_centered()` - Horizontally centered text
- `add_text_multiline()` - Multi-line text with line spacing
- Embedded DejaVuSans font (no external dependencies)
- Full RGBA color support
- Anti-aliased rendering

#### Text Measurement (Textbox)
- `get_text_size()` - Get text dimensions (width, height)
- `get_multiline_text_size()` - Multi-line dimensions with line count
- `get_text_box()` - Complete bounding box (x, y, width, height, ascent, descent, baseline)
- Perfect for dynamic text positioning and layout

#### Pixel Operations
- `getpixel()`, `putpixel()` - Direct pixel access
- `histogram()` - Color histogram
- `dominant_color()`, `average_color()` - Color analysis
- `replace_color()` - Color replacement with tolerance
- `threshold()`, `posterize()` - Color quantization

#### Drawing Operations
- `draw_rectangle()` - Filled rectangles
- `draw_circle()` - Filled circles
- `draw_line()` - Lines with Bresenham's algorithm
- `draw_text()` - Basic text rendering

#### Effects & Shadows
- `drop_shadow()` - Drop shadow with blur and offset
- `inner_shadow()` - Inner shadow effects
- `glow()` - Glow effects with customizable intensity

#### Metadata & EXIF
- `get_metadata()` - Read EXIF data (camera, GPS, settings)
- `get_metadata_summary()` - Human-readable metadata summary
- `has_exif()` - Check for EXIF presence
- `has_gps()` - Check for GPS data

### Changed
- Refactored Python codebase into 10 focused mixins for maintainability
- Filter mixins split into 11 category-specific modules
- Eager loading instead of lazy loading for Pillow compatibility

### Fixed
- All 59 compiler warnings resolved
- Memory safety improvements
- Pillow-compatible behavior

### Testing
- 150+ comprehensive tests
- 99/99 features tested and working (100%)
- All test images generated and documented

### Documentation
- Complete API documentation
- 82+ example images with usage
- Professional cover photos
- Comprehensive test suite with README

### Known Issues
- Emoji rendering needs visual improvement (pinned)
- Arbitrary angle rotation not yet supported (90°, 180°, 270° only)

## [0.1.0] - Initial Release

- Basic image operations (open, save, resize, crop, rotate)
- Core Rust implementation with PyO3 bindings
- Initial Pillow-compatible API

---

**Note:** This changelog follows [Keep a Changelog](https://keepachangelog.com/) format.

