"""
Functional API for image operations
provides Pillow-compatible module-level functions with IDE-friendly suggestions and enhanced error messages
"""

from pathlib import Path
from typing import TYPE_CHECKING, Any, List, Optional, Tuple, Union

from .enums import ImageFormat, Resampling
from .image import Image

if TYPE_CHECKING:
    # Import for IDE auto-completion only
    from .enums import ImageMode


def open(
    fp: Union[str, Path, bytes],
    mode: Optional[str] = None,
    formats: Optional[List[Union[str, ImageFormat]]] = None,
) -> Image:
    """
    Open an image file.

    Args:
        fp: File path, file object, or bytes.
             Supports:
             - str: "/path/to/image.png"
             - Path: Path("/path/to/image.png")
             - bytes: Raw image data bytes

        mode: Optional mode hint (e.g., 'RGB', 'RGBA', 'L').
             Common modes: RGB, RGBA, L, LA, CMYK, YCbCr, HSV

        formats: Optional list of formats to try in order.
                e.g., ['JPEG', 'PNG', 'BMP']

    Returns:
        Image: New Image instance with the loaded image

    Raises:
        FileNotFoundError: If file doesn't exist
        ValueError: If file format is not supported
        OSError: If file cannot be read (corrupted, permission issues)

    Example:
        >>> # Basic usage
        >>> img = imgrs.open("photo.jpg")

        >>> # With specific format hint
        >>> img = imgrs.open("image.dat", formats=['JPEG', 'PNG'])

        >>> # With mode hint
        >>> img = imgrs.open("gray.png", mode='L')

        >>> # From bytes
        >>> with open("photo.jpg", "rb") as f:
        ...     data = f.read()
        >>> img = imgrs.open(data)
    """
    return Image.open(fp, mode, formats)


def new(
    mode: Union[str, "ImageMode"],
    size: Tuple[int, int],
    color: Union[int, Tuple[int, int, int], Tuple[int, int, int, int], str] = 0,
) -> Image:
    """
    Create a new image with the given mode and size.

    Args:
        mode: Image mode (e.g., 'RGB', 'RGBA', 'L', 'LA', 'CMYK', 'YCbCr').
              Common modes:
              - 'RGB': 3 channels, 8-bit per channel (0-255)
              - 'RGBA': RGB + alpha channel, supports transparency
              - 'L': Grayscale, single channel (0-255)
              - 'LA': Grayscale + alpha channel
              - 'CMYK': Print color model (4 channels)
              - 'YCbCr': Luma-chroma color model

        size: Image size as (width, height) in pixels.
              Must be positive integers. Width and height > 0.

        color: Fill color. Supports multiple formats:
               - int: For grayscale modes ('L', 'LA')
                 Example: 0 (black), 255 (white), 128 (mid-gray)
               - tuple[int, int, int]: RGB color (red, green, blue)
                 Example: (255, 0, 0) = red, (255, 255, 255) = white
               - tuple[int, int, int, int]: RGBA color (red, green, blue, alpha)
                 Example: (255, 0, 0, 128) = semi-transparent red
               - str: Named color (case-insensitive)
                 Supported: 'black', 'white', 'red', 'green', 'blue',
                           'yellow', 'cyan', 'magenta', 'transparent'
               - Default: 0 (black/transparent)

    Returns:
        Image: New Image instance filled with specified color

    Raises:
        ValueError: If mode is not supported, size is invalid, or color format is wrong
        TypeError: If mode is not a string or size is not a tuple of ints

    Example:
        >>> # Basic RGB image
        >>> img = imgrs.new('RGB', (800, 600))

        >>> # Red image
        >>> img = imgrs.new('RGB', (400, 300), (255, 0, 0))

        >>> # Transparent image
        >>> img = imgrs.new('RGBA', (200, 200), (255, 0, 0, 128))

        >>> # Grayscale image
        >>> img = imgrs.new('L', (100, 100), 200)

        >>> # Using named color
        >>> img = imgrs.new('RGB', (300, 200), 'blue')
    """
    return Image.new(mode, size, color)


def save(
    image: Image,
    fp: Union[str, Path],
    format: Optional[Union[str, ImageFormat]] = None,
    **options,
) -> None:
    """
    Save an image to a file.

    Args:
        image: Image instance to save. Must be a valid imgrs.Image object.

        fp: File path or file-like object to save to.
            Supported paths:
            - str: "/path/to/image.jpg", "image.png"
            - Path: Path("/path/to/image.jpg")
            - file object: Any object with write() method

        format: Optional image format override.
                Auto-detected from file extension if not specified.
                Supported formats: 'JPEG', 'PNG', 'GIF', 'BMP', 'TIFF',
                                 'WEBP', 'ICO', 'PNM', 'DDS', 'TGA'

        **options: Format-specific save options:
                  - JPEG: quality (1-100, default: 95), optimize (bool)
                  - PNG: compress_level (0-9, default: 6)
                  - TIFF: compression ('lzw', 'zip', etc.)

    Returns:
        None: This function modifies the image and saves it to disk

    Raises:
        ValueError: If format is not supported or parameters are invalid
        OSError: If file cannot be written (permission, disk full, etc.)
        TypeError: If image is not a valid Image instance

    Example:
        >>> img = imgrs.open("photo.jpg")
        >>>
        >>> # Auto-detect format from extension
        >>> img.save("output.png")
        >>>
        >>> # Explicit format
        >>> img.save("output.webp", format='WEBP')
        >>>
        >>> # With quality options
        >>> img.save("output.jpg", format='JPEG', quality=90)
        >>>
        >>> # Path object
        >>> from pathlib import Path
        >>> img.save(Path("output/photo.png"))
    """
    image.save(fp, format, **options)


def resize(
    image: Image,
    size: Tuple[int, int],
    resample: Union[int, str, "Resampling"] = Resampling.BILINEAR,
) -> Image:
    """
    Resize an image to specified dimensions.

    Args:
        image: Image instance to resize. Must be a valid imgrs.Image object.

        size: Target size as (width, height) in pixels.
              Both width and height must be positive integers (> 0).
              Example: (800, 600), (1920, 1080)

        resample: Resampling algorithm for quality control.
                  Options:
                  - str: 'nearest', 'bilinear', 'bicubic', 'lanczos'
                  - int: 0 (NEAREST), 1 (BILINEAR), 2 (BICUBIC), 3 (LANCZOS)
                  - Resampling enum: Resampling.NEAREST, Resampling.BILINEAR, etc.
                  Default: Resampling.BILINEAR (good balance of speed/quality)

    Returns:
        Image: New resized Image instance (original image unchanged)

    Raises:
        ValueError: If size dimensions are invalid (negative, zero) or resample is unsupported
        TypeError: If image is not Image instance or size is not tuple of ints

    Note:
        - Returns a new Image instance (immutable operation)
        - Aspect ratio is not automatically maintained
        - For thumbnails with aspect ratio preservation, use thumbnail() instead

    Example:
        >>> img = imgrs.open("photo.jpg")  # 1000x800
        >>>
        >>> # Basic resize
        >>> resized = imgrs.resize(img, (400, 300))
        >>>
        >>> # High quality resize
        >>> resized = imgrs.resize(img, (800, 600), resample='lanczos')
        >>>
        >>> # Using enums
        >>> from imgrs import Resampling
        >>> resized = imgrs.resize(img, (640, 480), Resampling.BICUBIC)
    """
    return image.resize(size, resample)


def crop(image: Image, box: Tuple[int, int, int, int]) -> Image:
    """
    Crop an image to specified rectangular region.

    Args:
        image: Image instance to crop. Must be a valid imgrs.Image object.

        box: Crop rectangle as (left, top, right, bottom) coordinates.
             Coordinates are in pixels, relative to the top-left corner (0, 0).
             - left: X coordinate of left edge (inclusive)
             - top: Y coordinate of top edge (inclusive)
             - right: X coordinate of right edge (exclusive)
             - bottom: Y coordinate of bottom edge (exclusive)

             Resulting size: (right - left, bottom - top)

    Returns:
        Image: New cropped Image instance (original image unchanged)

    Raises:
        ValueError: If crop box is invalid (negative size, out of bounds)
        TypeError: If image is not Image instance or box is not 4-tuple of ints

    Note:
        - Returns a new Image instance (immutable operation)
        - Coordinates must be within image bounds
        - Right and bottom edges are exclusive (like Python slicing)

    Example:
        >>> img = imgrs.open("photo.jpg")  # 1000x800
        >>>
        >>> # Crop center 400x300 region
        >>> cropped = imgrs.crop(img, (300, 250, 700, 550))
        >>>
        >>> # Crop top-left 200x200 region
        >>> corner = imgrs.crop(img, (0, 0, 200, 200))
        >>>
        >>> # Crop bottom-right region
        >>> bottom_right = imgrs.crop(img, (800, 600, 1000, 800))
    """
    return image.crop(box)


def rotate(image: Image, angle: float, expand: bool = False) -> Image:
    """
    Rotate an image by specified angle.

    Args:
        image: Image instance to rotate. Must be a valid imgrs.Image object.

        angle: Rotation angle in degrees (counter-clockwise).
               Currently supported: 0°, 90°, 180°, 270° (exact values only)
               - 90°: Rotate clockwise 90 degrees
               - 180°: Flip upside down
               - 270°: Rotate counter-clockwise 90 degrees
               - 0°: No rotation (returns copy)

        expand: Whether to expand the image to fit the rotated content.
                False (default): Keep original size, crop rotated content
                True: Expand canvas to fit entire rotated image

    Returns:
        Image: New rotated Image instance (original image unchanged)

    Raises:
        NotImplementedError: If angle is not 0°, 90°, 180°, or 270°
        ValueError: If angle is invalid (non-numeric, infinite, NaN)
        TypeError: If image is not Image instance or angle is not numeric

    Note:
        - Returns a new Image instance (immutable operation)
        - Currently only supports 90-degree increments
        - For arbitrary angles, consider using external tools or PIL/Pillow

    Example:
        >>> img = imgrs.open("photo.jpg")
        >>>
        >>> # Rotate 90 degrees clockwise
        >>> rotated = imgrs.rotate(img, 90)
        >>>
        >>> # Rotate 180 degrees (flip upside down)
        >>> flipped = imgrs.rotate(img, 180)
        >>>
        >>> # Rotate with expand to fit full content
        >>> expanded = imgrs.rotate(img, 90, expand=True)
    """
    return image.rotate(angle, expand)


def convert(image: Image, mode: Union[str, "ImageMode"]) -> Image:
    """
    Convert an image to a different color mode.

    Args:
        image: Image instance to convert. Must be a valid imgrs.Image object.

        mode: Target color mode for conversion.
              Supported modes:
              - 'RGB': 3 channels (Red, Green, Blue), 8-bit per channel
              - 'RGBA': RGB + Alpha channel, supports transparency
              - 'L': Grayscale, single channel (0-255)
              - 'LA': Grayscale + Alpha channel
              - 'CMYK': Print color model (4 channels)
              - 'YCbCr': Digital video color model (3 channels)
              - 'HSV': Hue-Saturation-Value color model (3 channels)

    Returns:
        Image: New converted Image instance (original image unchanged)

    Raises:
        ValueError: If target mode is not supported or conversion fails
        TypeError: If image is not Image instance or mode is not string

    Note:
        - Returns a new Image instance (immutable operation)
        - Color information may be lost in conversions (e.g., RGB → L)
        - Alpha channel handling varies by conversion

    Example:
        >>> img = imgrs.open("photo.jpg")  # Assume JPEG (RGB)
        >>>
        >>> # Convert to grayscale
        >>> gray = imgrs.convert(img, 'L')
        >>>
        >>> # Add transparency
        >>> transparent = imgrs.convert(img, 'RGBA')
        >>>
        >>> # Convert to CMYK for printing
        >>> cmyk = imgrs.convert(img, 'CMYK')
    """
    return image.convert(mode)


def thumbnail(
    image: Image,
    size: Tuple[int, int],
    resample: Union[int, str] = Resampling.BICUBIC,
) -> None:
    """
    Create a thumbnail version of the image in-place.

    Args:
        image: Image instance to thumbnail
        size: Maximum size as (width, height)
        resample: Resampling filter
    """
    image.thumbnail(size, resample)


def fromarray(obj: Any, mode: Optional[str] = None) -> Image:
    """
    Create an image from a numpy array.

    Args:
        obj: Numpy array with shape (H, W) for grayscale or (H, W, C) for RGB/RGBA
        mode: Optional mode hint (not currently used)

    Returns:
        New Image instance

    Raises:
        ImportError: If numpy is not available
        ValueError: If array has unsupported shape or dtype
    """
    return Image.fromarray(obj, mode)


def split(image: Image) -> List[Image]:
    """
    Split an image into individual channel images.

    Args:
        image: Image instance to split

    Returns:
        List of Image instances, one for each channel
        - RGB images return [R, G, B]
        - RGBA images return [R, G, B, A]
        - Grayscale images return [L]
        - LA images return [L, A]
    """
    return image.split()


def paste(
    base_image: Image,
    paste_image: Image,
    position: Optional[Tuple[int, int]] = None,
    mask: Optional[Image] = None,
) -> Image:
    """
    Paste one image onto another with optional masking support.

    Args:
        base_image: Base image to paste onto. Must be a valid imgrs.Image object.
                    This image will remain unchanged; a new image is returned.

        paste_image: Image to paste onto base. Must be same or smaller than base.
                    This image will be composited onto the base image.

        position: Position to paste at as (x, y) coordinates.
                  - x: Horizontal position from left edge (pixels)
                  - y: Vertical position from top edge (pixels)
                  - None or (0, 0): Default position (top-left corner)
                  - Images are clipped if they extend beyond base image bounds

        mask: Optional mask image for controlling paste transparency.
              Must be same size as paste_image.

              Supported mask formats and behavior:
              - 'L' (grayscale): 0=invisible, 255=fully visible, grayscale=partial
              - 'LA' (grayscale + alpha): Uses alpha channel for transparency
              - 'RGB': Uses luminance (0.299*R + 0.587*G + 0.114*B) for opacity
              - 'RGBA': Uses alpha channel for transparency
              - Other formats: Automatically converted to grayscale

    Returns:
        Image: New Image instance with pasted content (original images unchanged)

    Raises:
        ValueError: If mask size doesn't match paste_image size or coordinates are invalid
        TypeError: If any parameter is not the expected type (Image, tuple, etc.)
        OSError: If operation fails due to memory or processing constraints

    Note:
        - Returns a new Image instance (immutable operation)
        - Images extending beyond base bounds are automatically clipped
        - Mask must be exactly the same size as paste_image

    Example:
        >>> base = imgrs.new('RGB', (400, 300), 'white')
        >>> overlay = imgrs.new('RGB', (100, 100), 'red')
        >>>
        >>> # Basic paste at center
        >>> result = imgrs.paste(base, overlay, (150, 100))
        >>>
        >>> # Paste with semi-transparent mask
        >>> mask = imgrs.new('L', (100, 100), 128)  # 50% opacity
        >>> result = imgrs.paste(base, overlay, (150, 100), mask)
        >>>
        >>> # Create circular mask
        >>> mask = imgrs.new('L', (100, 100), 0)
        >>> mask = mask.draw_circle(50, 50, 40, 255)
        >>> result = imgrs.paste(base, overlay, (150, 100), mask)
        >>>
        >>> # Using RGBA overlay (transparent background)
        >>> overlay_rgba = imgrs.new('RGBA', (100, 100), (255, 0, 0, 180))
        >>> result = imgrs.paste(base, overlay_rgba, (150, 100))
    """
    return base_image.paste(paste_image, position, mask)


def blur(image: Image, radius: float) -> Image:
    """
    Apply Gaussian blur to an image to reduce detail and noise.

    Args:
        image: Image instance to blur. Must be a valid imgrs.Image object.
               Works with all color modes (RGB, RGBA, L, LA, etc.).

        radius: Blur radius in pixels. Higher values create more blur.
                - radius ≤ 0: No effect (returns copy)
                - radius ≈ 1-3: Subtle blur, preserves detail
                - radius ≈ 5-10: Moderate blur for noise reduction
                - radius ≥ 15: Strong blur, heavily smoothed
                - Recommended range: 0.5 to 10.0 for most use cases

    Returns:
        Image: New blurred Image instance (original image unchanged)

    Raises:
        ValueError: If radius is negative or infinite
        TypeError: If image is not Image instance or radius is not numeric
        MemoryError: If image is too large to process

    Note:
        - Returns a new Image instance (immutable operation)
        - Uses Gaussian kernel for smooth, natural-looking blur
        - Edge handling: edges are extended to prevent border artifacts
        - Alpha channel (if present) is also blurred consistently

    Example:
        >>> img = imgrs.open("photo.jpg")
        >>>
        >>> # Subtle blur for noise reduction
        >>> blurred = imgrs.blur(img, 1.5)
        >>>
        >>> # Moderate blur for artistic effect
        >>> soft_focus = imgrs.blur(img, 5.0)
        >>>
        >>> # Strong blur for background effects
        >>> background = imgrs.blur(img, 10.0)
    """
    return image.blur(radius)


def sharpen(image: Image, strength: float = 1.0) -> Image:
    """
    Apply sharpening filter to enhance image details and edges.

    Args:
        image: Image instance to sharpen. Must be a valid imgrs.Image object.
               Works with all color modes (RGB, RGBA, L, LA, etc.).

        strength: Sharpening strength multiplier.
                  - strength ≤ 0: No effect (returns copy)
                  - strength = 1.0: Default sharpening
                  - strength 0.1-0.5: Subtle sharpening
                  - strength 1.0-2.0: Moderate to strong sharpening
                  - strength > 2.0: Very strong sharpening (may cause artifacts)
                  - Recommended range: 0.5 to 2.0

    Returns:
        Image: New sharpened Image instance (original image unchanged)

    Raises:
        ValueError: If strength is negative or infinite
        TypeError: If image is not Image instance or strength is not numeric
        MemoryError: If image is too large to process

    Note:
        - Returns a new Image instance (immutable operation)
        - Enhances edges and fine details while preserving overall image quality
        - May amplify noise in low-quality images
        - Works consistently across all color channels

    Example:
        >>> img = imgrs.open("blurry_photo.jpg")
        >>>
        >>> # Subtle sharpening
        >>> enhanced = imgrs.sharpen(img, 0.8)
        >>>
        >>> # Standard sharpening
        >>> sharp = imgrs.sharpen(img, 1.0)
        >>>
        >>> # Strong sharpening for detailed work
        >>> very_sharp = imgrs.sharpen(img, 1.5)
    """
    return image.sharpen(strength)


def edge_detect(image: Image) -> Image:
    """
    Apply edge detection filter to highlight image boundaries and transitions.

    Args:
        image: Image instance to process. Must be a valid imgrs.Image object.
               Automatically converts to grayscale for edge detection.
               Works with all input modes but output is always grayscale.

    Returns:
        Image: New grayscale Image instance with edges highlighted in white.
               Non-edge areas are black, creating a clean line-art effect.

    Raises:
        TypeError: If image is not Image instance
        MemoryError: If image is too large to process

    Note:
        - Returns a new Image instance (immutable operation)
        - Uses Sobel operator for reliable edge detection
        - Automatically handles color conversion for optimal results
        - Good for: object detection, boundary finding, artistic effects

    Example:
        >>> img = imgrs.open("photo.jpg")
        >>>
        >>> # Basic edge detection
        >>> edges = imgrs.edge_detect(img)
        >>>
        >>> # Use edges for compositing
        >>> edges_colored = edges.convert('RGB')
        >>> comic_effect = img.blend_with(edges_colored, 'multiply', 0.8)
    """
    return image.edge_detect()


def emboss(image: Image) -> Image:
    """
    Apply emboss filter to create a raised, 3D relief effect.

    Args:
        image: Image instance to emboss. Must be a valid imgrs.Image object.
               Works with all color modes but output is typically grayscale.
               Best results with images containing clear edges and textures.

    Returns:
        Image: New Image instance with embossed effect.
               Creates a 3D appearance by simulating light and shadow.

    Raises:
        TypeError: If image is not Image instance
        MemoryError: If image is too large to process

    Note:
        - Returns a new Image instance (immutable operation)
        - Creates relief effect by emphasizing edges with lighting simulation
        - Works well for: textured images, photos with clear features
        - May not be effective on very smooth or noisy images

    Example:
        >>> img = imgrs.open("texture.jpg")
        >>>
        >>> # Basic emboss effect
        >>> embossed = imgrs.emboss(img)
        >>>
        >>> # Use for artistic styling
        >>> artistic = imgrs.emboss(img).convert('RGB')
    """
    return image.emboss()


def brightness(image: Image, adjustment: int) -> Image:
    """
    Adjust image brightness by adding/subtracting a constant value.

    Args:
        image: Image instance to adjust. Must be a valid imgrs.Image object.
               Works with all color modes (RGB, RGBA, L, LA, etc.).

        adjustment: Brightness adjustment value.
                    - adjustment = 0: No change (returns copy)
                    - adjustment > 0: Brighten image (add value to all pixels)
                    - adjustment < 0: Darken image (subtract value from all pixels)
                    - Range: Typically -255 to +255 for 8-bit images
                    - Values beyond ±255 are clamped to prevent overflow

    Returns:
        Image: New Image instance with adjusted brightness (original unchanged)

    Raises:
        ValueError: If adjustment is not an integer or is infinite
        TypeError: If image is not Image instance
        MemoryError: If image is too large to process

    Note:
        - Returns a new Image instance (immutable operation)
        - Adds same value to all color channels uniformly
        - Clamps values to valid range (0-255 for 8-bit)
        - Simple linear adjustment (for advanced controls, use exposure_adjust)

    Example:
        >>> img = imgrs.open("dark_photo.jpg")
        >>>
        >>> # Brighten dark image
        >>> bright = imgrs.brightness(img, 50)
        >>>
        >>> # Darken overexposed image
        >>> dark = imgrs.brightness(img, -30)
        >>>
        >>> # Subtle adjustment
        >>> slightly_brighter = imgrs.brightness(img, 15)
    """
    return image.brightness(adjustment)


def contrast(image: Image, factor: float) -> Image:
    """
    Adjust image contrast by multiplying pixel values around midpoint.

    Args:
        image: Image instance to adjust. Must be a valid imgrs.Image object.
                Works with all color modes (RGB, RGBA, L, LA, etc.).

        factor: Contrast adjustment factor.
                - factor = 1.0: No change (returns copy)
                - factor > 1.0: Increase contrast (stretch values away from 128)
                - factor < 1.0: Decrease contrast (compress values toward 128)
                - factor = 0.0: Maximum decrease (entire image becomes gray)
                - Recommended range: 0.1 to 3.0
                - Extreme values (>5.0 or <0.1) may cause artifacts

    Returns:
        Image: New Image instance with adjusted contrast (original unchanged)

    Raises:
        ValueError: If factor is not positive or is infinite
        TypeError: If image is not Image instance or factor is not numeric
        MemoryError: If image is too large to process

    Note:
        - Returns a new Image instance (immutable operation)
        - Multiplies pixel values around the 128 midpoint (for 8-bit)
        - Preserves overall brightness while enhancing/reducing tonal differences
        - Works consistently across all color channels

    Example:
        >>> img = imgrs.open("flat_photo.jpg")
        >>>
        >>> # Increase contrast for more vibrant image
        >>> vibrant = imgrs.contrast(img, 1.5)
        >>>
        >>> # Decrease contrast for softer appearance
        >>> soft = imgrs.contrast(img, 0.7)
        >>>
        >>> # Subtle contrast enhancement
        >>> enhanced = imgrs.contrast(img, 1.2)
    """
    return image.contrast(factor)


def chroma_key(
    image: Image,
    key_color: Tuple[int, int, int],
    tolerance: float = 0.3,
    feather: float = 0.1,
) -> Image:
    """
    Apply chroma key effect (green screen removal) to make specific colors transparent.

    Args:
        image: Image instance to process. Must be a valid imgrs.Image object.
                Works with RGB and RGBA images. RGB images are converted to RGBA.

        key_color: RGB color to make transparent as (red, green, blue).
                   - Each component: 0-255
                   - Common green screen: (0, 255, 0) or (0, 177, 64)
                   - Common blue screen: (0, 0, 255)

        tolerance: Color matching tolerance (0.0-1.0).
                  - 0.0: Exact color match only
                  - 0.3: Moderate tolerance (recommended for green screens)
                  - 1.0: Match all colors (entire image becomes transparent)
                  - Higher values remove more background but may affect foreground

        feather: Soft edge width for smooth transitions (0.0-1.0).
                - 0.0: Hard edges (pixels are either fully opaque or transparent)
                - 0.1: Moderate feathering (recommended)
                - 0.3: Very soft edges
                - Higher values create smoother blends but may soften details

    Returns:
        Image: New RGBA Image instance with chroma key applied (original unchanged)

    Raises:
        TypeError: If image is not Image instance or parameters are wrong type
        MemoryError: If image is too large to process

    Note:
        - Returns a new Image instance (immutable operation)
        - Always outputs RGBA format to support transparency
        - For best results, use well-lit, evenly colored backgrounds
        - Adjust tolerance and feather based on lighting conditions
        - Consider using color correction before chroma keying

    Example:
        >>> img = imgrs.open("person_on_green_screen.jpg")
        >>>
        >>> # Basic green screen removal
        >>> keyed = imgrs.chroma_key(img, (0, 255, 0))
        >>>
        >>> # Fine-tune with custom tolerance and feathering
        >>> keyed = imgrs.chroma_key(img, (0, 177, 64), tolerance=0.25, feather=0.15)
        >>>
        >>> # Blue screen removal
        >>> keyed = imgrs.chroma_key(img, (0, 0, 255), tolerance=0.4, feather=0.2)
        >>>
        >>> # Composite with new background
        >>> background = imgrs.open("beach.jpg")
        >>> final = imgrs.paste(background, keyed, (100, 50))
    """
    return image.chroma_key(key_color, tolerance, feather)
