"""
Transform operations mixin - resize, crop, rotate, etc.
"""

from typing import TYPE_CHECKING, Optional, Tuple, Union

if TYPE_CHECKING:
    from .image import Image


class TransformMixin:
    """Mixin for image transformation operations"""

    def resize(self, size: Tuple[int, int], resample: Optional[str] = None) -> "Image":
        """
        Resize the image to the specified size.

        Args:
            size: Target size as (width, height)
            resample: Resampling filter ('nearest', 'bilinear', 'lanczos')

        Returns:
            New resized Image instance
        """
        rust_image = self._rust_image.resize(size, resample)
        return self.__class__(rust_image)

    def crop(self, box: Tuple[int, int, int, int]) -> "Image":
        """
        Crop the image to the specified box.

        Args:
            box: Box coordinates as (left, top, right, bottom)

        Returns:
            New cropped Image instance
        """
        rust_image = self._rust_image.crop(box)
        return self.__class__(rust_image)

    def rotate(
        self,
        angle: float,
        expand: bool = False,
        fillcolor: Optional[Tuple[int, ...]] = None,
    ) -> "Image":
        """
        Rotate the image by the specified angle.

        Args:
            angle: Rotation angle in degrees (counter-clockwise)
            expand: If True, expand output to fit the rotated image
            fillcolor: Optional fill color for empty areas

        Returns:
            New rotated Image instance
        """
        # Only support 90-degree rotations for now
        angle = angle % 360

        if angle in [90, 180, 270]:
            rust_image = self._rust_image.rotate(angle)
            return self.__class__(rust_image)
        elif angle == 0:
            return self.copy()
        else:
            raise NotImplementedError(
                f"Arbitrary angle rotation ({angle}°) not yet implemented. "
                "Only 90°, 180°, and 270° rotations are supported."
            )

    def transpose(self, method: Union[int, str]) -> "Image":
        """
        Transpose the image.

        Args:
            method: Transpose method (FLIP_LEFT_RIGHT, FLIP_TOP_BOTTOM, etc.)

        Returns:
            New transposed Image instance
        """
        from ..enums import Transpose

        if isinstance(method, str):
            method_upper = method.upper()
            method = getattr(Transpose, method_upper, None)
            if method is None:
                raise ValueError(f"Invalid transpose method: {method}")
        elif isinstance(method, int):
            method = str(method)
        else:
            method = str(method)

        rust_image = self._rust_image.transpose(method)
        return self.__class__(rust_image)

    def thumbnail(
        self,
        size: Tuple[int, int],
        resample: Optional[str] = None,
    ) -> None:
        """
        Make this image into a thumbnail (modifies in place).

        Args:
            size: Maximum size as (width, height)
            resample: Resampling filter
        """
        # Calculate thumbnail size maintaining aspect ratio
        current_width, current_height = self.size
        target_width, target_height = size

        ratio = min(target_width / current_width, target_height / current_height)
        new_width = int(current_width * ratio)
        new_height = int(current_height * ratio)

        # Resize in place
        resized = self.resize((new_width, new_height), resample)
        self._rust_image = resized._rust_image

    def convert(self, mode: str) -> "Image":
        """
        Convert the image to a different mode.

        Args:
            mode: Target mode ('RGB', 'RGBA', 'L', etc.)

        Returns:
            New converted Image instance
        """
        rust_image = self._rust_image.convert(mode)
        return self.__class__(rust_image)

    def split(self) -> list:
        """
        Split the image into individual bands.

        Returns:
            List of single-band Image instances
        """
        rust_images = self._rust_image.split()
        return [self.__class__(img) for img in rust_images]

    def paste(
        self,
        im: "Image",
        position: Optional[Tuple[int, int]] = None,
        mask: Optional["Image"] = None,
    ) -> "Image":
        """
        Paste another image onto this image.

        Args:
            im: Image to paste onto this image
            position: Position as (x, y) tuple or None for (0, 0).
                     The top-left corner of im will be placed at this position.
            mask: Optional mask image. Must be the same size as im.
                  Supports:
                  - 'L' (grayscale) masks: White areas are fully visible, black areas are invisible
                  - 'LA' (grayscale with alpha) masks: Uses alpha channel for transparency
                  - 'RGB' masks: Uses luminance (0.299*R + 0.587*G + 0.114*B) for opacity
                  - 'RGBA' masks: Uses alpha channel for transparency
                  - Other formats are automatically converted to grayscale

        Returns:
            New Image instance with pasted content

        Raises:
            ValueError: If mask size doesn't match paste image size
            TypeError: If mask is not an Image instance

        Example:
            >>> base = Image.new('RGB', (200, 200), 'white')
            >>> overlay = Image.new('RGB', (100, 100), 'red')
            >>> # Basic paste at position (50, 50)
            >>> result = base.paste(overlay, (50, 50))
            
            >>> # Paste with grayscale mask
            >>> mask = Image.new('L', (100, 100), 128)  # 50% opacity
            >>> result = base.paste(overlay, (50, 50), mask)
            
            >>> # Create circular mask
            >>> mask = Image.new('L', (100, 100), 0)
            >>> mask = mask.draw_circle(50, 50, 40, 255)
            >>> result = base.paste(overlay, (50, 50), mask)
        """
        if position is None:
            position = (0, 0)

        if mask is not None:
            # Validate mask is an Image instance
            if not hasattr(mask, '_rust_image'):
                raise TypeError("mask must be an Image instance")
            
            # Validate mask size matches paste image size
            if mask.size != im.size:
                raise ValueError(
                    f"mask size {mask.size} does not match paste image size {im.size}"
                )

        rust_mask = mask._rust_image if mask is not None else None
        rust_image = self._rust_image.paste(im._rust_image, position, rust_mask)
        return self.__class__(rust_image)
