"""
Metadata/EXIF operations mixin
"""


class MetadataMixin:
    """Mixin for EXIF/metadata operations"""

    def get_metadata(self, path: str) -> dict:
        """
        Get EXIF/metadata from image file.
        
        Args:
            path: Path to image file
        
        Returns:
            Dictionary containing metadata including:
            - width, height: Image dimensions
            - exif: EXIF data (make, model, datetime, artist, copyright, etc.)
            - gps: GPS coordinates (latitude, longitude, altitude)
            - camera: Camera settings (iso, exposure_time, f_number, focal_length, etc.)
        
        Example:
            metadata = img.get_metadata('photo.jpg')
            if 'exif' in metadata:
                print(f"Camera: {metadata['exif'].get('make')} {metadata['exif'].get('model')}")
            if 'gps' in metadata:
                print(f"Location: {metadata['gps']['latitude']}, {metadata['gps']['longitude']}")
        """
        return self._rust_image.get_metadata(path)

    def get_metadata_summary(self, path: str) -> str:
        """
        Get a summary string of the metadata.
        
        Args:
            path: Path to image file
        
        Returns:
            Human-readable summary string
        
        Example:
            summary = img.get_metadata_summary('photo.jpg')
            print(summary)  # "1920x1080 | Canon EOS 5D | ISO 400 | 1/125 | f/5.6 | GPS"
        """
        return self._rust_image.get_metadata_summary(path)

    def has_exif(self, path: str) -> bool:
        """
        Check if image file has EXIF data.
        
        Args:
            path: Path to image file
        
        Returns:
            True if EXIF data exists
        """
        return self._rust_image.has_exif(path)

    def has_gps(self, path: str) -> bool:
        """
        Check if image file has GPS data.
        
        Args:
            path: Path to image file
        
        Returns:
            True if GPS data exists
        """
        return self._rust_image.has_gps(path)

