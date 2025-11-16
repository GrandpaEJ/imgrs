"""
Enumerations and constants matching Pillow's API
"""


class ImageMode:
    """Image mode constants."""

    # Grayscale modes
    L = "L"  # 8-bit grayscale
    LA = "LA"  # 8-bit grayscale + alpha
    INTEGER = "I"  # 32-bit integer grayscale

    # Color modes
    RGB = "RGB"  # 8-bit RGB
    RGBA = "RGBA"  # 8-bit RGB + alpha
    CMYK = "CMYK"  # 8-bit CMYK
    YCbCr = "YCbCr"  # 8-bit YCbCr
    HSV = "HSV"  # 8-bit HSV

    # Binary mode
    BINARY = "1"  # 1-bit binary


class ImageFormat:
    """Image format constants."""

    JPEG = "JPEG"
    PNG = "PNG"
    GIF = "GIF"
    BMP = "BMP"
    TIFF = "TIFF"
    WEBP = "WEBP"
    ICO = "ICO"
    PNM = "PNM"
    DDS = "DDS"
    TGA = "TGA"
    FARBFELD = "FARBFELD"
    AVIF = "AVIF"


class Resampling:
    """Resampling filter constants."""

    NEAREST = "NEAREST"
    BILINEAR = "BILINEAR"
    BICUBIC = "BICUBIC"
    LANCZOS = "LANCZOS"

    # Pillow compatibility - numeric constants
    NEAREST_INT = 0
    BILINEAR_INT = 1
    BICUBIC_INT = 2
    LANCZOS_INT = 3

    @classmethod
    def from_int(cls, value: int) -> str:
        """Convert integer resampling constant to string."""
        mapping = {
            cls.NEAREST_INT: cls.NEAREST,
            cls.BILINEAR_INT: cls.BILINEAR,
            cls.BICUBIC_INT: cls.BICUBIC,
            cls.LANCZOS_INT: cls.LANCZOS,
        }
        return mapping.get(value, cls.BILINEAR)


class Transpose:
    """Transpose method constants."""

    FLIP_LEFT_RIGHT = "FLIP_LEFT_RIGHT"
    FLIP_TOP_BOTTOM = "FLIP_TOP_BOTTOM"
    ROTATE_90 = "ROTATE_90"
    ROTATE_180 = "ROTATE_180"
    ROTATE_270 = "ROTATE_270"
    TRANSPOSE = "TRANSPOSE"
    TRANSVERSE = "TRANSVERSE"

    # Pillow compatibility - numeric constants
    FLIP_LEFT_RIGHT_INT = 0
    FLIP_TOP_BOTTOM_INT = 1
    ROTATE_90_INT = 2
    ROTATE_180_INT = 3
    ROTATE_270_INT = 4
    TRANSPOSE_INT = 5
    TRANSVERSE_INT = 6

    @classmethod
    def from_int(cls, value: int) -> str:
        """Convert integer transpose constant to string."""
        mapping = {
            cls.FLIP_LEFT_RIGHT_INT: cls.FLIP_LEFT_RIGHT,
            cls.FLIP_TOP_BOTTOM_INT: cls.FLIP_TOP_BOTTOM,
            cls.ROTATE_90_INT: cls.ROTATE_90,
            cls.ROTATE_180_INT: cls.ROTATE_180,
            cls.ROTATE_270_INT: cls.ROTATE_270,
            cls.TRANSPOSE_INT: cls.TRANSPOSE,
            cls.TRANSVERSE_INT: cls.TRANSVERSE,
        }
        return mapping.get(value, cls.FLIP_LEFT_RIGHT)


# Enhanced Color System Enums


class BlendMode:
    """Blend mode constants for compositing operations."""

    NORMAL = "normal"
    MULTIPLY = "multiply"
    SCREEN = "screen"
    OVERLAY = "overlay"
    SOFT_LIGHT = "soft_light"
    HARD_LIGHT = "hard_light"
    COLOR_DODGE = "color_dodge"
    COLOR_BURN = "color_burn"
    DARKEN = "darken"
    LIGHTEN = "lighten"
    DIFFERENCE = "difference"
    EXCLUSION = "exclusion"


class MaskType:
    """Mask type constants for masking operations."""

    GRADIENT = "gradient"
    COLOR_BASED = "color_based"
    LUMINANCE = "luminance"
    SHAPE = "shape"
    TEXTURE = "texture"
    NOISE = "noise"


class ColorFormat:
    """Color format constants for color space operations."""

    RGB = "rgb"
    RGBA = "rgba"
    HSL = "hsl"
    HSV = "hsv"
    LAB = "lab"
    XYZ = "xyz"
    CMYK = "cmyk"
    YCBCR = "ycbcr"
    YUV = "yuv"
    HSL_PRECISE = "hsl_precise"
    HSV_PRECISE = "hsv_precise"


class GradientDirection:
    """Gradient direction constants."""

    HORIZONTAL = "horizontal"
    VERTICAL = "vertical"
    DIAGONAL = "diagonal"
    RADIAL = "radial"
    ANGULAR = "angular"
    CONICAL = "conical"


class MaskOperation:
    """Mask combination operation constants."""

    MULTIPLY = "multiply"
    ADD = "add"
    SUBTRACT = "subtract"
    OVERLAY = "overlay"
    SCREEN = "screen"
    DIFFERENCE = "difference"


class ColorSpace:
    """Color space constants for conversions."""

    SRGB = "srgb"
    ADOBE_RGB = "adobe_rgb"
    PROPHOTO_RGB = "prophoto_rgb"
    DCI_P3 = "dci_p3"
    REC2020 = "rec2020"
    DISPLAY_P3 = "display_p3"
