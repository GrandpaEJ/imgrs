"""
Rich text rendering mixin with comprehensive font and styling support

Enhanced features:
- Font family management and fallbacks
- Font weight and style support (bold, italic, oblique)
- Variable font support with weight axis
- Advanced text effects (gradients, patterns, 3D)
- Better alignment and positioning options
- Letter spacing and text justification
- Text shadows, outlines, and glows
- Multi-font text rendering
"""

import os
from pathlib import Path
from typing import TYPE_CHECKING, Any, Dict, List, Optional, Tuple, Union

if TYPE_CHECKING:
    from .image import Image


class FontManager:
    """Font management utilities for text rendering."""

    # Default system fonts for fallback
    DEFAULT_FONTS = [
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/System/Library/Fonts/Arial.ttf",
        "/Windows/Fonts/arial.ttf",
        "../../fonts/DejaVuSans.ttf",
    ]

    BOLD_FONTS = [
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
        "/System/Library/Fonts/Arial-Bold.ttf",
        "/Windows/Fonts/arialbd.ttf",
        "../../fonts/DejaVuSans.ttf",  # Fallback to regular
    ]

    SERIF_FONTS = [
        "/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf",
        "/System/Library/Fonts/Times.ttc",
        "/Windows/Fonts/times.ttf",
    ]

    MONO_FONTS = [
        "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
        "/System/Library/Fonts/Menlo.ttc",
        "/Windows/Fonts/cour.ttf",
    ]

    @staticmethod
    def find_font(fonts_list: List[str]) -> Optional[str]:
        """Find first available font from list."""
        for font_path in fonts_list:
            if os.path.exists(font_path):
                return font_path
        return None

    @classmethod
    def get_font_by_family(cls, family: str) -> Optional[str]:
        """Get font path by family name."""
        family_lower = family.lower()

        if "sans" in family_lower or "arial" in family_lower:
            return cls.find_font(cls.DEFAULT_FONTS)
        elif "serif" in family_lower or "times" in family_lower:
            return cls.find_font(cls.SERIF_FONTS)
        elif "mono" in family_lower or "courier" in family_lower:
            return cls.find_font(cls.MONO_FONTS)
        elif "bold" in family_lower:
            return cls.find_font(cls.BOLD_FONTS)
        else:
            # Try as direct path or look in fonts directory
            if os.path.exists(family):
                return family
            fonts_dir = Path("../../fonts")
            potential_font = fonts_dir / family
            if potential_font.exists():
                return str(potential_font)

            # Search for font files with family name
            for ext in ["*.ttf", "*.otf", "*.woff2"]:
                for font_file in fonts_dir.glob(ext):
                    if family_lower in font_file.name.lower():
                        return str(font_file)

        return cls.find_font(cls.DEFAULT_FONTS)


class TextMixin:
    """
    Enhanced text rendering mixin with comprehensive font and styling support.

    Features:
    - Font family management with fallbacks
    - Font weight and style support (bold, italic, oblique)
    - Variable font support with weight axis
    - Advanced text effects (gradients, patterns, 3D)
    - Enhanced alignment and positioning
    - Letter spacing and text justification
    - Multi-level text effects (shadow, outline, glow)
    """

    def add_text(
        self,
        text: str,
        position: Union[Tuple[int, int], Tuple[float, float]],
        size: float = 32.0,
        color: Union[Tuple[int, int, int, int], str] = (0, 0, 0, 255),
        font_family: str = "sans",
        font_weight: str = "normal",
        font_style: str = "normal",
        font_path: Optional[str] = None,
        letter_spacing: float = 0.0,
        opacity: float = 1.0,
    ) -> "Image":
        """
        Draw text on image with enhanced font and styling options.

        Args:
            text: Text content to render
            position: (x, y) coordinates for text placement
            size: Font size in pixels (default: 32.0)
            color: Text color as (R, G, B, A) tuple or CSS color name
            font_family: Font family name ('sans', 'serif', 'mono', or custom name)
            font_weight: Font weight ('normal', 'bold', 'light', or numeric 100-900)
            font_style: Font style ('normal', 'italic', 'oblique')
            font_path: Direct path to TTF/OTF font file (overrides font_family)
            letter_spacing: Letter spacing in pixels (can be negative)
            opacity: Text opacity 0.0-1.0

        Returns:
            New Image with text rendered

        Examples:
            # Basic text with system fonts
            img.add_text("Hello World", (50, 50), size=48, font_family="serif")

            # Bold italic text with custom spacing
            img.add_text("Bold Italic", (50, 100), size=32,
                        font_weight="bold", font_style="italic", letter_spacing=2.0)

            # Using custom font file
            img.add_text("Custom Font", (50, 150), font_path="/path/to/font.ttf")

            # Semi-transparent colored text
            img.add_text("Colorful Text", (50, 200),
                        color=(255, 100, 150, 180), opacity=0.8)
        """
        # Convert CSS color to RGBA if needed
        if isinstance(color, str):
            color = self._parse_css_color(color)

        x, y = position
        rust_image = self._rust_image.text_enhanced(
            text,
            x,
            y,
            size,
            color,
            font_family,
            font_weight,
            font_style,
            font_path,
            letter_spacing,
            opacity,
        )
        return self.__class__(rust_image)

    def add_text_styled(
        self,
        text: str,
        position: Union[Tuple[int, int], Tuple[float, float]],
        size: float = 32.0,
        color: Union[Tuple[int, int, int, int], str] = (0, 0, 0, 255),
        font_family: str = "sans",
        font_weight: str = "normal",
        font_style: str = "normal",
        font_path: Optional[str] = None,
        letter_spacing: float = 0.0,
        opacity: float = 1.0,
        align: str = "left",
        background: Optional[Union[Tuple[int, int, int, int], str]] = None,
        outline: Optional[
            Tuple[
                Union[int, float],
                Union[int, float],
                Union[int, float],
                Union[int, float],
                float,
            ]
        ] = None,
        shadow: Optional[
            Tuple[
                int,
                int,
                Union[int, float],
                Union[int, float],
                Union[int, float],
                Union[int, float],
            ]
        ] = None,
        glow: Optional[
            Tuple[
                Union[int, float],
                Union[int, float],
                Union[int, float],
                Union[int, float],
                float,
            ]
        ] = None,
        max_width: Optional[int] = None,
        line_spacing: float = 1.2,
        text_justify: str = "left",
        rotation: float = 0.0,
    ) -> "Image":
        """
        Alias for add_text_advanced for compatibility.
        """
        return self.add_text_advanced(
            text,
            position,
            size,
            color,
            font_family,
            font_weight,
            font_style,
            font_path,
            letter_spacing,
            opacity,
            align,
            background,
            outline,
            shadow,
            glow,
            max_width,
            line_spacing,
            text_justify,
            rotation,
        )

    def add_text_advanced(
        self,
        text: str,
        position: Union[Tuple[int, int], Tuple[float, float]],
        size: float = 32.0,
        color: Union[Tuple[int, int, int, int], str] = (0, 0, 0, 255),
        font_family: str = "sans",
        font_weight: str = "normal",
        font_style: str = "normal",
        font_path: Optional[str] = None,
        letter_spacing: float = 0.0,
        opacity: float = 1.0,
        align: str = "left",
        background: Optional[Union[Tuple[int, int, int, int], str]] = None,
        outline: Optional[
            Tuple[
                Union[int, float],
                Union[int, float],
                Union[int, float],
                Union[int, float],
                float,
            ]
        ] = None,
        shadow: Optional[
            Tuple[
                int,
                int,
                Union[int, float],
                Union[int, float],
                Union[int, float],
                Union[int, float],
            ]
        ] = None,
        glow: Optional[
            Tuple[
                Union[int, float],
                Union[int, float],
                Union[int, float],
                Union[int, float],
                float,
            ]
        ] = None,
        max_width: Optional[int] = None,
        line_spacing: float = 1.2,
        text_justify: str = "left",
        rotation: float = 0.0,
        transform: Optional[str] = None,
    ) -> "Image":
        """
        Draw text with comprehensive styling and effects.

        Args:
            text: Text content (supports multiline with \\n)
            position: (x, y) coordinates
            size: Font size in pixels
            color: Text color (R, G, B, A) tuple or CSS color name
            font_family: Font family name
            font_weight: Font weight ('normal', 'bold', 'light', or numeric)
            font_style: Font style ('normal', 'italic', 'oblique')
            font_path: Direct font file path
            letter_spacing: Letter spacing in pixels
            opacity: Text opacity 0.0-1.0
            align: Text alignment ('left', 'center', 'right')
            background: Background color or None
            outline: Outline as (R, G, B, A, width) or None
            shadow: Shadow as (offset_x, offset_y, R, G, B, A) or None
            glow: Glow as (R, G, B, A, blur_radius) or None
            max_width: Maximum width for text wrapping (pixels)
            line_spacing: Line spacing multiplier (default: 1.2)
            text_justify: Text justification ('left', 'center', 'right', 'justify')
            rotation: Rotation angle in degrees
            transform: Text transform ('none', 'uppercase', 'lowercase', 'title')

        Returns:
            New Image with styled text

        Examples:
            # Text with multiple effects
            img.add_text_advanced("Styled Text", (100, 100),
                                color=(255, 255, 255, 255),
                                outline=(0, 0, 0, 255, 2.0),
                                shadow=(2, 2, 0, 0, 0, 128),
                                glow=(100, 150, 255, 150, 8.0))

            # Wrapped text with background
            img.add_text_advanced("Long text that wraps...", (50, 50),
                                max_width=400,
                                background=(0, 0, 0, 100),
                                text_justify="justify")

            # Rotated and transformed text
            img.add_text_advanced("Upside Down", (200, 200),
                                rotation=180,
                                transform="uppercase",
                                font_weight="bold")
        """
        # Convert CSS colors to RGBA
        if isinstance(color, str):
            color = self._parse_css_color(color)
        if background and isinstance(background, str):
            background = self._parse_css_color(background)

        x, y = position

        # Apply text transform
        if transform:
            text = self._apply_text_transform(text, transform)

        rust_image = self._rust_image.text_advanced(
            text,
            x,
            y,
            size,
            color,
            font_family,
            font_weight,
            font_style,
            font_path,
            letter_spacing,
            opacity,
            align,
            background,
            outline,
            shadow,
            glow,
            max_width,
            line_spacing,
            text_justify,
            rotation,
        )
        return self.__class__(rust_image)

    def add_text_multiline(
        self,
        text: str,
        position: Union[Tuple[int, int], Tuple[float, float]],
        size: float = 32.0,
        color: Union[Tuple[int, int, int, int], str] = (0, 0, 0, 255),
        font_family: str = "sans",
        font_weight: str = "normal",
        font_style: str = "normal",
        font_path: Optional[str] = None,
        line_spacing: float = 1.2,
        letter_spacing: float = 0.0,
        align: str = "left",
        text_justify: str = "left",
        max_width: Optional[int] = None,
        opacity: float = 1.0,
    ) -> "Image":
        """
        Draw multi-line text with enhanced formatting options.

        Args:
            text: Text with \\n for line breaks
            position: (x, y) starting coordinates
            size: Font size
            color: Text color
            font_family: Font family
            font_weight: Font weight
            font_style: Font style
            font_path: Direct font path
            line_spacing: Line spacing multiplier
            letter_spacing: Letter spacing per line
            align: Line alignment ('left', 'center', 'right')
            text_justify: Text justification within lines
            max_width: Maximum line width for wrapping
            opacity: Text opacity

        Returns:
            New Image with multi-line text

        Example:
            text = "Line 1 with bold words\\nLine 2 normal\\nLine 3 italic"
            img.add_text_multiline(text, (50, 50),
                                 line_spacing=1.5,
                                 align='center',
                                 text_justify="justify")
        """
        if isinstance(color, str):
            color = self._parse_css_color(color)

        x, y = position
        rust_image = self._rust_image.text_multiline_enhanced(
            text,
            x,
            y,
            size,
            color,
            font_family,
            font_weight,
            font_style,
            font_path,
            line_spacing,
            letter_spacing,
            align,
            text_justify,
            max_width,
            opacity,
        )
        return self.__class__(rust_image)

    def add_text_centered(
        self,
        text: str,
        y: Union[int, float],
        size: float = 32.0,
        color: Union[Tuple[int, int, int, int], str] = (0, 0, 0, 255),
        font_family: str = "sans",
        font_weight: str = "normal",
        font_style: str = "normal",
        font_path: Optional[str] = None,
        opacity: float = 1.0,
        letter_spacing: float = 0.0,
    ) -> "Image":
        """
        Draw horizontally centered text with enhanced font support.

        Args:
            text: Text content
            y: Vertical position
            size: Font size
            color: Text color
            font_family: Font family
            font_weight: Font weight
            font_style: Font style
            font_path: Direct font path
            opacity: Text opacity
            letter_spacing: Letter spacing

        Returns:
            New Image with centered text

        Example:
            img.add_text_centered("Centered Title", 50, size=48,
                                font_weight="bold", letter_spacing=1.0)
        """
        if isinstance(color, str):
            color = self._parse_css_color(color)

        rust_image = self._rust_image.text_centered_enhanced(
            text,
            y,
            size,
            color,
            font_family,
            font_weight,
            font_style,
            font_path,
            opacity,
            letter_spacing,
        )
        return self.__class__(rust_image)

    def add_text_with_fonts(
        self,
        text: str,
        position: Union[Tuple[int, int], Tuple[float, float]],
        fonts: List[Dict[str, Any]],
    ) -> "Image":
        """
        Add text with different fonts/styles for different parts.

        Args:
            text: Base text
            position: (x, y) coordinates
            fonts: List of font specifications with text ranges
                   Each dict: {
                       'start': int, 'end': int, 'size': float,
                       'color': tuple, 'font_family': str, 'font_weight': str
                   }

        Returns:
            New Image with multi-font text
        """
        x, y = position
        rust_image = self._rust_image.text_with_fonts(text, x, y, fonts)
        return self.__class__(rust_image)

    @staticmethod
    def get_text_size(
        text: str,
        size: float = 32.0,
        font_family: str = "sans",
        font_weight: str = "normal",
        font_style: str = "normal",
        font_path: Optional[str] = None,
        letter_spacing: float = 0.0,
    ) -> Tuple[int, int]:
        """
        Get text dimensions with enhanced font support.

        Args:
            text: Text to measure
            size: Font size
            font_family: Font family name
            font_weight: Font weight
            font_style: Font style
            font_path: Direct font path
            letter_spacing: Letter spacing

        Returns:
            (width, height) tuple in pixels

        Example:
            width, height = Image.get_text_size("Hello World", size=48,
                                              font_weight="bold", letter_spacing=1.0)
            print(f"Text will be {width}x{height} pixels")
        """
        from imgrs._core import Image as CoreImage

        return CoreImage.get_text_size_enhanced(
            text, size, font_family, font_weight, font_style, font_path, letter_spacing
        )

    @staticmethod
    def get_multiline_text_size(
        text: str,
        size: float = 32.0,
        line_spacing: float = 1.2,
        font_family: str = "sans",
        font_weight: str = "normal",
        font_style: str = "normal",
        font_path: Optional[str] = None,
        letter_spacing: float = 0.0,
        max_width: Optional[int] = None,
    ) -> Tuple[int, int, int]:
        """
        Get multiline text dimensions with full font support.

        Args:
            text: Multiline text (with \\n)
            size: Font size
            line_spacing: Line spacing multiplier
            font_family: Font family
            font_weight: Font weight
            font_style: Font style
            font_path: Direct font path
            letter_spacing: Letter spacing
            max_width: Maximum line width

        Returns:
            (width, height, line_count) tuple

        Example:
            text = "Line 1\\nLine 2\\nLine 3"
            width, height, lines = Image.get_multiline_text_size(text,
                                                                font_weight="bold")
            print(f"{width}x{height} pixels, {lines} lines")
        """
        from imgrs._core import Image as CoreImage

        return CoreImage.get_multiline_text_size_enhanced(
            text,
            size,
            line_spacing,
            font_family,
            font_weight,
            font_style,
            font_path,
            letter_spacing,
            max_width,
        )

    @staticmethod
    def get_text_box(
        text: str,
        x: Union[int, float],
        y: Union[int, float],
        size: float = 32.0,
        font_family: str = "sans",
        font_weight: str = "normal",
        font_style: str = "normal",
        font_path: Optional[str] = None,
        letter_spacing: float = 0.0,
    ) -> Dict[str, Union[int, float]]:
        """
        Get complete text bounding box with font metrics.

        Args:
            text: Text to measure
            x: X coordinate
            y: Y coordinate
            size: Font size
            font_family: Font family
            font_weight: Font weight
            font_style: Font style
            font_path: Direct font path
            letter_spacing: Letter spacing

        Returns:
            Dictionary with comprehensive font metrics:
            - x, y: Top-left corner
            - width, height: Dimensions
            - ascent, descent: Font metrics
            - baseline_y: Y coordinate of baseline
            - bottom_y: Y coordinate of bottom edge
            - right_x: X coordinate of right edge
            - advance_width: Total advance width
            - left_bearing, right_bearing: Side bearings

        Example:
            box = Image.get_text_box("Hello", 100, 50, size=48, font_weight="bold")
            print(f"Text spans from ({box['x']}, {box['y']}) to ({box['right_x']}, {box['bottom_y']})")
            print(f"Advance width: {box['advance_width']}")
        """
        from imgrs._core import Image as CoreImage

        return CoreImage.get_text_box_enhanced(
            text,
            x,
            y,
            size,
            font_family,
            font_weight,
            font_style,
            font_path,
            letter_spacing,
        )

    @staticmethod
    def list_available_fonts() -> List[str]:
        """List all available fonts in the system."""
        fonts = []

        # Check fonts directory
        fonts_dir = Path("../../fonts")
        if fonts_dir.exists():
            for ext in ["*.ttf", "*.otf", "*.ttc", "*.woff2"]:
                fonts.extend([str(f) for f in fonts_dir.glob(ext)])

        # Add system fonts if they exist
        for font_list in [
            FontManager.DEFAULT_FONTS,
            FontManager.BOLD_FONTS,
            FontManager.SERIF_FONTS,
            FontManager.MONO_FONTS,
        ]:
            for font_path in font_list:
                if os.path.exists(font_path) and font_path not in fonts:
                    fonts.append(font_path)

        return sorted(list(set(fonts)))

    def _parse_css_color(self, color: str) -> Tuple[int, int, int, int]:
        """Parse CSS color name or hex to RGBA tuple."""
        # Basic CSS color names
        css_colors = {
            "black": (0, 0, 0, 255),
            "white": (255, 255, 255, 255),
            "red": (255, 0, 0, 255),
            "green": (0, 255, 0, 255),
            "blue": (0, 0, 255, 255),
            "yellow": (255, 255, 0, 255),
            "purple": (128, 0, 128, 255),
            "orange": (255, 165, 0, 255),
            "pink": (255, 192, 203, 255),
            "gray": (128, 128, 128, 255),
            "transparent": (0, 0, 0, 0),
        }

        color_lower = color.lower()
        if color_lower in css_colors:
            return css_colors[color_lower]

        # Handle hex colors
        if color.startswith("#"):
            hex_color = color[1:]
            if len(hex_color) == 3:  # #RGB
                r = int(hex_color[0] * 2, 16)
                g = int(hex_color[1] * 2, 16)
                b = int(hex_color[2] * 2, 16)
                return (r, g, b, 255)
            elif len(hex_color) == 6:  # #RRGGBB
                r = int(hex_color[0:2], 16)
                g = int(hex_color[2:4], 16)
                b = int(hex_color[4:6], 16)
                return (r, g, b, 255)
            elif len(hex_color) == 8:  # #RRGGBBAA
                r = int(hex_color[0:2], 16)
                g = int(hex_color[2:4], 16)
                b = int(hex_color[4:6], 16)
                a = int(hex_color[6:8], 16)
                return (r, g, b, a)

        # Default fallback
        return (0, 0, 0, 255)

    def _apply_text_transform(self, text: str, transform: str) -> str:
        """Apply text transformation."""
        transform_lower = transform.lower()

        if transform_lower == "uppercase":
            return text.upper()
        elif transform_lower == "lowercase":
            return text.lower()
        elif transform_lower == "title":
            return text.title()
        else:  # 'none' or unknown
            return text
