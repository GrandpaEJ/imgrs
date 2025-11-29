# Changelog

All notable changes to this project will be documented in this file.

## [0.3.5] - 2025-11-29

### Added

#### Text Rendering System Upgrade
- **Core Architecture Fix**: Fixed Image class `__init__` to properly call `CoreMixin.__init__()`, ensuring all mixins have access to `_rust_image` attribute
- **TextMixin Full Integration**: All text rendering methods now properly use the Rust backend:
  - `add_text()` - Basic text rendering with proper Rust integration
  - `add_text_styled()` - Full styling support (shadows, outlines, backgrounds, opacity, alignment, wrapping, rotation)
  - `add_text_multiline()` - Proper multi-line text with alignment and line spacing
  - `add_text_centered()` - Centered text with all styling options
- **Enhanced Text Measurement**: All measurement functions now use accurate Rust calculations:
  - `get_text_dimensions()` - Accurate text size and metrics from Rust
  - `get_multiline_text_dimensions()` - Multi-line measurement with proper line spacing
  - `get_text_bounding_box()` - Comprehensive bounding box information from Rust
- **Advanced Styling Features**: All text styling features are now fully functional:
  - Drop shadows with custom offsets and colors
  - Text outlines with customizable width and color
  - Background rectangles with custom colors
  - Opacity control for text and effects
  - Text alignment (left, center, right)
  - Text wrapping within specified width
  - Text rotation support

### Changed

#### Python-Rust Integration
- **Mixin System**: Fixed inheritance chain to ensure proper initialization of PyImage objects
- **Method Signatures**: Updated all TextMixin methods to properly interface with Rust backend
- **Error Handling**: Improved parameter validation and error reporting

#### Performance Improvements
- **Direct Backend Calls**: Replaced placeholder implementations with direct Rust backend calls
- **Accurate Measurements**: Text measurements now use actual font metrics instead of estimates
- **Optimized Rendering**: Full utilization of Rust text rendering capabilities

### Fixed
- **CoreMixin Integration**: Resolved issue where Image class bypassed CoreMixin initialization
- **TextMixin Functionality**: All text methods now work with actual Rust backend instead of broken placeholders
- **Measurement Accuracy**: Text measurement functions now return accurate values from Rust font metrics
- **Convenience Methods**: All text convenience methods (shadow, outline, background) now properly use Rust implementation

### Testing
- **Comprehensive Verification**: All 7 text demo functions tested and working
- **Build System**: Successful compilation with `uv run maturin build`

## [0.3.1] - 2025-11-26

### Added

#### Advanced Text Rendering System (TextMixin)
- **Core Text Methods:**
  - `add_text()` - Flexible text rendering with position tuple or separate x,y parameters
  - `add_text_styled()` - Styled text with outline, shadow, background, opacity, and alignment support
  - `add_text_multiline()` - Multi-line text rendering with line spacing and alignment
  - `add_text_centered()` - Horizontally centered text rendering

- **Text Measurement:**
  - `get_text_dimensions()` - Text size and metrics calculation
  - `get_multiline_text_dimensions()` - Multi-line text dimensions with line count
  - `get_text_bounding_box()` - Complete text bounding box with ascent/descent/baseline

- **Convenience Methods:**
  - `add_text_with_shadow()` - Easy drop shadow effects
  - `add_text_with_outline()` - Simple outline effects
  - `add_text_with_background()` - Quick background text

- **Rust Implementation:**
  - Complete text rendering module (`src/text/`) with fonts, styles, and renderer
  - Advanced text rendering functions in Python bindings
  - Font management with embedded DejaVuSans fallback

#### Modular Blend Mode Architecture
- Split `src/blending/modes.rs` into 14 individual files
- Each blend mode now has its own dedicated module:
  - `normal.rs`, `multiply.rs`, `screen.rs`, `overlay.rs`
  - `soft_light.rs`, `hard_light.rs`, `darken.rs`, `lighten.rs`
  - `difference.rs`, `exclusion.rs`, and more
- Improved code organization and maintainability

### Changed

#### Text System Reorganization
- Moved text functionality from `feat/` directory to `mixins/text_mixin.py`
- Integrated TextMixin into main Image class inheritance
- Updated text examples to showcase new TextMixin API
- Removed old text mixin files from deprecated `feat/` directory

#### Rust Codebase Updates
- Updated deprecated PyO3 APIs to current versions:
  - `PyDict::new_bound()` → `PyDict::new()`
  - `PyBytes::new_bound()` → `PyBytes::new()`
  - `get_type_bound<>()` → `get_type<>()`
- Refactored blending system for better modularity
- Added comprehensive text rendering module with fonts, styles, and renderer

#### Testing Infrastructure
- Updated pytest configuration for isolated testing with `--import-mode=importlib`
- Excluded test files from package distribution
- Streamlined test suite by removing unit tests directory
- Added pytest conftest.py for test configuration

### Fixed
- Resolved import conflicts in isolated testing environments
- Improved build system configuration
- Enhanced code organization and maintainability

## [0.3.0] - 2025-11-26

### Removed

#### Breaking Changes - Cairo/Pango Dependency Removal
- **Emoji Support Completely Removed**
  - `add_emoji()` - Preset emoji addition
  - `add_emoji_text()` - Unicode emoji rendering
  - `add_emoji_quick()` - Quick emoji addition
  - `add_emojis()` - Batch emoji operations
  - `emoji_demo.py` example removed

- **Text Rendering Completely Removed**
  - `add_text()` - Basic text rendering
  - `add_text_styled()` - Styled text with effects
  - `add_text_multiline()` - Multi-line text
  - `add_text_centered()` - Centered text
  - `add_text_with_fonts()` - Multi-font text
  - `get_text_size()` - Text dimension measurement
  - `get_multiline_text_size()` - Multi-line text dimensions
  - `get_text_box()` - Text bounding box
  - `list_available_fonts()` - Font listing
  - `draw_text()` - Basic text drawing
  - `textbox_demo.py`, `text_demo.py`, `text_emoji_demo.py` examples removed

- **Rust Codebase Changes**
  - Removed `src/text/` directory entirely
  - Removed `src/emoji/` directory entirely
  - Removed `src/image/text_ops.rs`
  - Removed `src/image/emoji.rs`
  - Updated `src/lib.rs` and `src/image/mod.rs`

- **Python Codebase Changes**
  - Removed `text_mixin.py` completely
  - Removed `emoji_mixin.py` completely
  - Updated `image.py` and `mixins/__init__.py`

### Changed
- **Dependency Updates**
  - Removed cairo and pango dependencies from `Cargo.toml`
  - Updated `Cargo.lock` with new dependency tree
  - Reduced binary size and build complexity

- **Example Organization**
  - Organized 122+ example output images into categorized subfolders
  - Updated examples to handle removed functionality gracefully
  - Added organized subfolders: blur/, brightness_contrast/, channels_color/, etc.

### Fixed
- Build system simplified without Cairo/Pango dependencies
- Examples now run without text/emoji functionality
- All remaining features (65+ filters, transforms, etc.) still fully functional

### Migration Guide
- **Text Rendering**: Use external libraries like Pillow for text operations
- **Emoji Support**: Use Unicode text rendering or external emoji libraries
- **Build Requirements**: No longer need Cairo/Pango system dependencies

## [0.2.10] - 2025-11-19

### Added

#### Emoji Operations
- `add_emoji()` - Add preset emojis to images
- `add_emoji_text()` - Add Unicode emojis with positioning and sizing
- `add_emoji_quick()` - Quick emoji addition with default settings
- `add_emojis()` - Batch emoji addition for multiple emojis

#### Advanced Color Operations
- `set_alpha()` / `get_alpha()` - Global alpha channel control
- `add_transparency()` / `remove_transparency()` - Transparency management
- `apply_mask()` - Apply masks for selective transparency
- `create_gradient_mask()` - Generate gradient masks (horizontal, vertical, radial, diagonal)
- `create_color_mask()` - Create masks based on color similarity
- `create_luminance_mask()` - Luminance-based masking
- `combine_masks()` - Mathematical mask combination (multiply, add, subtract, overlay, screen, difference)
- `extract_color()` - Extract pixels matching target colors
- `color_quantize()` - Reduce color palette size
- `color_shift()` - Shift all colors by specified amount
- `selective_desaturate()` - Selectively desaturate specific colors
- `color_match()` - Match colors to reference images
- `apply_gradient_overlay()` - Apply gradient color overlays
- `create_stripe_pattern()` - Create stripe pattern overlays
- `create_checker_pattern()` - Create checkerboard pattern overlays
- `split_alpha()` / `merge_alpha()` - Alpha channel splitting and merging
- `alpha_to_color()` - Convert alpha to solid color
- `blend_with()` - Advanced blending modes (normal, multiply, screen, overlay, soft_light, hard_light, color_dodge, color_burn, darken, lighten, difference, exclusion)
- `overlay_with()` - Overlay images with blending
- `get_color_palette()` - Extract dominant colors
- `analyze_color_distribution()` - Color distribution analysis
- `find_color_regions()` - Find regions matching target colors

#### Shape Generation
- `circle()`, `rectangle()`, `triangle()`, `ellipse()`, `star()`, `square()`, `diamond()`, `hexagon()`, `parallelogram()`, `pentagon()`, `octagon()`, `heart()`, `arrow()`, `cross()`, `quadrilateral()` - Create various geometric shapes as images

#### Enhanced Text Rendering
- `add_text()` - Basic text with font family, weight, style, letter spacing
- `add_text_styled()` - Full styling with outline, shadow, glow, background, alignment
- `add_text_multiline()` - Multi-line text with line spacing and justification
- `add_text_centered()` - Horizontally centered text
- `add_text_with_fonts()` - Multi-font text rendering
- `get_text_size()` - Get text dimensions
- `get_multiline_text_size()` - Multi-line text dimensions
- `get_text_box()` - Complete text bounding box with metrics
- `list_available_fonts()` - List system fonts
- Font manager with fallback support
- CSS color parsing and text transforms (uppercase, lowercase, title)

#### Drawing Operations
- `draw_rectangle()` - Draw filled rectangles
- `draw_circle()` - Draw filled circles
- `draw_line()` - Draw lines with Bresenham's algorithm
- `draw_text()` - Basic text drawing

#### Effects & Shadows
- `drop_shadow()` - Drop shadow with blur and offset
- `inner_shadow()` - Inner shadow effects
- `glow()` - Glow effects with customizable intensity

#### Metadata & EXIF
- `get_metadata()` - Read comprehensive EXIF data (camera, GPS, settings)
- `get_metadata_summary()` - Human-readable metadata summary
- `has_exif()` / `has_gps()` - Check for EXIF/GPS presence

#### Pixel Operations
- `getpixel()` / `putpixel()` - Direct pixel access
- `histogram()` - Color histogram generation
- `dominant_color()` / `average_color()` - Color analysis
- `replace_color()` - Color replacement with tolerance
- `threshold()` - Binary thresholding
- `posterize()` - Color level reduction

#### Transform Operations
- `resize()` - Resize with resampling options
- `crop()` - Rectangular cropping
- `rotate()` - Rotation (90°, 180°, 270°, with expand option)
- `rotate90()`, `rotate180()`, `rotate270()`, `rotate_left()`, `rotate_right()` - Convenience rotation methods
- `transpose()` - Image transposition (flip, rotate)
- `thumbnail()` - In-place thumbnail creation
- `convert()` - Color mode conversion
- `split()` - Split into channel images
- `paste()` - Paste images with masking support

#### Enhanced Enums
- `BlendMode` - Blending mode constants
- `MaskType` - Mask type constants
- `ColorFormat` - Color format constants
- `GradientDirection` - Gradient direction constants
- `MaskOperation` - Mask operation constants
- `ColorSpace` - Color space constants

### Changed
- Refactored codebase into focused mixins for better maintainability
- Enhanced IDE support with comprehensive docstrings
- Improved error messages and type hints

### Fixed
- Memory safety improvements
- Performance optimizations
- Better compatibility with existing code

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

