"""
Transform operations mixin - resize, crop, rotate, etc.
"""

from typing import Optional, Tuple, Union


class TransformMixin:
    """Mixin for image transformation operations"""

    def resize(
        self, size: Tuple[int, int], resample: Optional[str] = None
    ) -> "Image":
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
            im: Image to paste
            position: Position as (x, y) or None for (0, 0)
            mask: Optional mask image

        Returns:
            New Image instance with pasted content
        """
        if position is None:
            position = (0, 0)

        rust_mask = mask._rust_image if mask is not None else None
        rust_image = self._rust_image.paste(im._rust_image, position, rust_mask)
        return self.__class__(rust_image)

