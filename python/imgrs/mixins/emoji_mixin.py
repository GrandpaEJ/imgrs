"""
Emoji and text operations mixin - add emojis and text to images
"""

from typing import TYPE_CHECKING, List, Tuple

if TYPE_CHECKING:
    from .image import Image


class EmojiMixin:
    """Mixin for emoji and text operations"""

    def add_emoji(
        self, emoji_name: str, x: int, y: int, size: int = 64, opacity: float = 1.0
    ) -> "Image":
        """
        Add an emoji to the image using preset emoji type.

        Args:
            emoji_name: Name of emoji (e.g., 'smile', 'heart', 'fire', 'thumbsup')
            x: X position for emoji
            y: Y position for emoji
            size: Size of emoji in pixels (default: 64)
            opacity: Opacity from 0.0 to 1.0 (default: 1.0)

        Returns:
            New Image instance with emoji added

        Available emoji names:
            Smileys: smile, grin, joy, laughing, hearteyes, cool, wink, thinking
            Hearts: heart, blueheart, greenheart, yellowheart, purpleheart
            Gestures: thumbsup, thumbsdown, ok, victory, fire, clap, wave
            Nature: sun, moon, star, rainbow, flower, sparkles
            Food: pizza, burger, cake, coffee
            Activities: party, gift, trophy, camera
            Symbols: check, cross, warning
        """
        return self.__class__(
            self._rust_image.add_emoji(emoji_name, x, y, size, opacity)
        )

    def add_emoji_text(
        self, emoji: str, x: int, y: int, size: int = 64, opacity: float = 1.0
    ) -> "Image":
        """
        Add an emoji to the image using Unicode emoji text.

        Args:
            emoji: Unicode emoji character (e.g., '😊', '❤️', '🔥')
            x: X position for emoji
            y: Y position for emoji
            size: Size of emoji in pixels (default: 64)
            opacity: Opacity from 0.0 to 1.0 (default: 1.0)

        Returns:
            New Image instance with emoji added

        Example:
            img.add_emoji_text('😊', 100, 100, size=80)
        """
        return self.__class__(
            self._rust_image.add_emoji_text(emoji, x, y, size, opacity)
        )

    def add_emoji_quick(
        self, emoji_name: str, x: int, y: int, size: int = 64
    ) -> "Image":
        """
        Quick emoji add with default opacity.

        Args:
            emoji_name: Name of emoji
            x: X position
            y: Y position
            size: Size in pixels (default: 64)

        Returns:
            New Image instance with emoji added
        """
        return self.__class__(self._rust_image.add_emoji_quick(emoji_name, x, y, size))

    def add_emojis(self, emojis: List[Tuple[str, int, int, int, float]]) -> "Image":
        """
        Add multiple emojis at once.

        Args:
            emojis: List of tuples (emoji_name, x, y, size, opacity)

        Returns:
            New Image instance with all emojis added

        Example:
            img.add_emojis([
                ('smile', 100, 100, 64, 1.0),
                ('heart', 200, 200, 80, 0.8),
                ('fire', 150, 150, 70, 1.0)
            ])
        """
        return self.__class__(self._rust_image.add_emojis(emojis))

    def add_textbox(
        self,
        x: int,
        y: int,
        width: int,
        height: int,
        fill_color: Tuple[int, int, int] = (255, 255, 255),
        border_color: Tuple[int, int, int] = (0, 0, 0),
        border_width: float = 1.0,
    ) -> "Image":
        """
        Add a text box (rectangle) to the image.

        Args:
            x: X position for textbox
            y: Y position for textbox
            width: Width of textbox
            height: Height of textbox
            fill_color: RGB fill color (default: white)
            border_color: RGB border color (default: black)
            border_width: Border width (default: 1.0)

        Returns:
            New Image instance with textbox added

        Example:
            img.add_textbox(50, 50, 200, 100, fill_color=(255, 255, 255), border_color=(0, 0, 0))
        """
        return self.__class__(
            self._rust_image.add_textbox(
                x, y, width, height, fill_color, border_color, border_width
            )
        )
