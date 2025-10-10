"""Noise filter operations"""


class NoiseFiltersMixin:
    """Mixin for noise operations"""

    def add_gaussian_noise(self, mean: float, stddev: float) -> "Image":
        """Add Gaussian noise to the image."""
        return self.__class__(self._rust_image.add_gaussian_noise(mean, stddev))

    def add_salt_pepper_noise(self, amount: float) -> "Image":
        """Add salt & pepper noise to the image."""
        return self.__class__(self._rust_image.add_salt_pepper_noise(amount))

    def denoise(self, radius: int) -> "Image":
        """Apply denoising filter."""
        return self.__class__(self._rust_image.denoise(radius))

