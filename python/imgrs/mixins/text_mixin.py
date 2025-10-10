"""
Rich text rendering mixin with full styling support
"""


class TextMixin:
    """
    Mixin for advanced text rendering with full styling support.
    
    Supports:
    - TTF/OTF fonts
    - Colors and transparency
    - Text alignment (left, center, right)
    - Background colors
    - Outlines and shadows
    - Multi-line text
    - Text wrapping
    """

    def add_text(
        self,
        text: str,
        position: tuple[int, int],
        size: float = 32.0,
        color: tuple[int, int, int, int] = (0, 0, 0, 255),
        font_path: str | None = None,
    ):
        """
        Draw text on image with basic styling.
        
        Args:
            text: Text content to render
            position: (x, y) coordinates for text placement
            size: Font size in pixels (default: 32.0)
            color: Text color as (R, G, B, A) tuple (default: black)
            font_path: Path to TTF/OTF font file (default: DejaVuSans)
        
        Returns:
            New Image with text rendered
        
        Example:
            img.add_text("Hello World", (50, 50), size=48, color=(255, 0, 0, 255))
        """
        x, y = position
        rust_image = self._rust_image.text(text, x, y, size, color, font_path)
        return self.__class__(rust_image)

    def add_text_styled(
        self,
        text: str,
        position: tuple[int, int],
        size: float = 32.0,
        color: tuple[int, int, int, int] = (0, 0, 0, 255),
        font_path: str | None = None,
        align: str | None = None,
        background: tuple[int, int, int, int] | None = None,
        outline: tuple[int, int, int, int, float] | None = None,
        shadow: tuple[int, int, int, int, int, int] | None = None,
        opacity: float | None = None,
        line_spacing: float | None = None,
        letter_spacing: float | None = None,
        max_width: int | None = None,
        rotation: float | None = None,
    ):
        """
        Draw text with full styling options.
        
        Args:
            text: Text content (supports multiline with \\n)
            position: (x, y) coordinates
            size: Font size in pixels
            color: Text color (R, G, B, A)
            font_path: Path to font file
            align: Text alignment: 'left', 'center', 'right'
            background: Background color (R, G, B, A) or None
            outline: Outline as (R, G, B, A, width) or None
            shadow: Shadow as (offset_x, offset_y, R, G, B, A) or None
            opacity: Text opacity 0.0-1.0
            line_spacing: Line spacing multiplier (default: 1.2)
            letter_spacing: Letter spacing in pixels
            max_width: Maximum width for text wrapping (pixels)
            rotation: Rotation angle in degrees
        
        Returns:
            New Image with styled text
        
        Examples:
            # Text with outline
            img.add_text_styled("BOLD", (100, 100), size=64,
                              color=(255, 255, 255, 255),
                              outline=(0, 0, 0, 255, 2.0))
            
            # Text with shadow
            img.add_text_styled("Shadow", (100, 200),
                              shadow=(2, 2, 0, 0, 0, 128))
            
            # Centered text with background
            img.add_text_styled("Title", (400, 50),
                              align='center',
                              background=(0, 0, 0, 180))
            
            # Text wrapping
            img.add_text_styled("Long text...", (50, 50),
                              max_width=400)
        """
        x, y = position
        rust_image = self._rust_image.text_styled(
            text, x, y, size, color, font_path,
            align, background, outline, shadow,
            opacity, line_spacing, letter_spacing,
            max_width, rotation
        )
        return self.__class__(rust_image)

    def add_text_centered(
        self,
        text: str,
        y: int,
        size: float = 32.0,
        color: tuple[int, int, int, int] = (0, 0, 0, 255),
        font_path: str | None = None,
    ):
        """
        Draw horizontally centered text.
        
        Args:
            text: Text content
            y: Vertical position
            size: Font size
            color: Text color (R, G, B, A)
            font_path: Path to font file
        
        Returns:
            New Image with centered text
        
        Example:
            img.add_text_centered("Centered Title", 50, size=48)
        """
        rust_image = self._rust_image.text_centered(text, y, size, color, font_path)
        return self.__class__(rust_image)

    def add_text_multiline(
        self,
        text: str,
        position: tuple[int, int],
        size: float = 32.0,
        color: tuple[int, int, int, int] = (0, 0, 0, 255),
        font_path: str | None = None,
        line_spacing: float | None = None,
        align: str | None = None,
    ):
        """
        Draw multi-line text with line breaks.
        
        Args:
            text: Text with \\n for line breaks
            position: (x, y) starting coordinates
            size: Font size
            color: Text color (R, G, B, A)
            font_path: Path to font file
            line_spacing: Line spacing multiplier (default: 1.2)
            align: Text alignment: 'left', 'center', 'right'
        
        Returns:
            New Image with multi-line text
        
        Example:
            text = "Line 1\\nLine 2\\nLine 3"
            img.add_text_multiline(text, (50, 50), line_spacing=1.5, align='center')
        """
        x, y = position
        rust_image = self._rust_image.text_multiline(
            text, x, y, size, color, font_path, line_spacing, align
        )
        return self.__class__(rust_image)

    @staticmethod
    def get_text_size(
        text: str,
        size: float = 32.0,
        font_path: str | None = None,
    ) -> tuple[int, int]:
        """
        Get text dimensions without rendering.
        
        Args:
            text: Text to measure
            size: Font size
            font_path: Path to font file
        
        Returns:
            (width, height) tuple in pixels
        
        Example:
            width, height = Image.get_text_size("Hello World", size=48)
            print(f"Text will be {width}x{height} pixels")
        """
        from imgrs._core import Image as CoreImage
        return CoreImage.get_text_size(text, size, font_path)

    @staticmethod
    def get_multiline_text_size(
        text: str,
        size: float = 32.0,
        line_spacing: float = 1.2,
        font_path: str | None = None,
    ) -> tuple[int, int, int]:
        """
        Get multiline text dimensions.
        
        Args:
            text: Multiline text (with \\n)
            size: Font size
            line_spacing: Line spacing multiplier
            font_path: Path to font file
        
        Returns:
            (width, height, line_count) tuple
        
        Example:
            text = "Line 1\\nLine 2\\nLine 3"
            width, height, lines = Image.get_multiline_text_size(text)
            print(f"{width}x{height} pixels, {lines} lines")
        """
        from imgrs._core import Image as CoreImage
        return CoreImage.get_multiline_text_size(text, size, line_spacing, font_path)

    @staticmethod
    def get_text_box(
        text: str,
        x: int,
        y: int,
        size: float = 32.0,
        font_path: str | None = None,
    ) -> dict:
        """
        Get complete text bounding box information.
        
        Args:
            text: Text to measure
            x: X coordinate
            y: Y coordinate
            size: Font size
            font_path: Path to font file
        
        Returns:
            Dictionary with keys:
            - x, y: Top-left corner
            - width, height: Dimensions
            - ascent, descent: Font metrics
            - baseline_y: Y coordinate of baseline
            - bottom_y: Y coordinate of bottom edge
            - right_x: X coordinate of right edge
        
        Example:
            box = Image.get_text_box("Hello", 100, 50, size=48)
            print(f"Text spans from ({box['x']}, {box['y']}) to ({box['right_x']}, {box['bottom_y']})")
            print(f"Baseline at y={box['baseline_y']}")
        """
        from imgrs._core import Image as CoreImage
        return CoreImage.get_text_box(text, x, y, size, font_path)

