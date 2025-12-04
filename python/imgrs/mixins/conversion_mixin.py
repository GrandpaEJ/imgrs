"""
Image Format Conversion Mixin - Comprehensive format conversion capabilities
"""

import io
import os
import tempfile
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple, Union

from ..enums import ImageFormat


class ConversionMixin:
    """
    Mixin for comprehensive image format conversion capabilities.
    
    Provides:
    - Convert between all supported image formats
    - Format detection and validation
    - Format-specific save options
    - Batch conversion capabilities
    - Advanced format features (optimization, quality control)
    """
    
    def convert_format(
        self, 
        target_format: Union[str, ImageFormat],
        save_options: Optional[Dict[str, Any]] = None,
        optimize: bool = True,
        progressive: bool = False,
    ) -> "Image":
        """
        Convert image to a different format with advanced options.
        
        Args:
            target_format: Target format name (e.g., 'JPEG', 'PNG', 'WEBP')
            save_options: Format-specific options dict
            optimize: Whether to apply format-specific optimizations
            progressive: For JPEG formats, create progressive JPEG
            
        Returns:
            New Image instance in the target format
            
        Raises:
            ValueError: If target format is not supported
            OSError: If conversion fails due to I/O issues
            
        Example:
            >>> img = Image.open("photo.png")
            >>> jpeg_img = img.convert_format('JPEG', quality=90)
            >>> webp_img = img.convert_format('WEBP', lossless=True)
        """
        from .._core import RustImage
        
        # Validate and normalize format
        format_name = ImageFormat.validate_format(str(target_format))
        
        # Default options based on format
        if save_options is None:
            save_options = self._get_default_format_options(format_name)
        
        # Apply optimization settings
        if optimize:
            save_options = self._apply_optimization(format_name, save_options)
        
        # Handle format-specific options
        save_options = self._process_format_options(format_name, save_options)
        
        # Convert using temporary file approach for format compatibility
        # Get proper extension for the format
        ext = self._get_extension(format_name)
        with tempfile.NamedTemporaryFile(suffix=f'.{ext}', delete=True) as tmp:
            tmp_path = tmp.name
        
        try:
            # Save to temporary file in target format
            # Note: save_options are currently not passed to Rust as the Rust save
            # method doesn't support quality/compression options yet
            self._rust_image.save(tmp_path, format_name)
            
            # Load back as new image
            rust_image = RustImage.open(tmp_path)
            return self.__class__(rust_image)
            
        finally:
            # Clean up temporary file
            try:
                os.unlink(tmp_path)
            except OSError:
                pass
    
    def convert_to_bytes(
        self,
        target_format: Union[str, ImageFormat],
        save_options: Optional[Dict[str, Any]] = None,
    ) -> bytes:
        """
        Convert image to target format and return as bytes.
        
        Args:
            target_format: Target format name
            save_options: Format-specific save options
            
        Returns:
            Image data in target format as bytes
            
        Raises:
            ValueError: If target format is not supported
            OSError: If conversion fails
        """
        # Validate and normalize format
        format_name = ImageFormat.validate_format(str(target_format))
        ext = self._get_extension(format_name)
        
        # Create temporary file with proper extension
        with tempfile.NamedTemporaryFile(suffix=f'.{ext}', delete=True) as tmp:
            tmp_path = tmp.name
            
        try:
            # Save to temp file
            self.convert_format(target_format, save_options).save(tmp_path)
            
            # Read bytes
            with open(tmp_path, 'rb') as f:
                return f.read()
                
        finally:
            try:
                os.unlink(tmp_path)
            except OSError:
                pass
    
    def batch_convert(
        self,
        target_formats: List[Union[str, ImageFormat]],
        output_dir: Union[str, Path],
        base_filename: str,
        save_options: Optional[Dict[str, Dict[str, Any]]] = None,
    ) -> Dict[str, str]:
        """
        Convert image to multiple formats and save to directory.
        
        Args:
            target_formats: List of target formats
            output_dir: Output directory path
            base_filename: Base filename (without extension)
            save_options: Dict mapping format -> save_options dict
            
        Returns:
            Dict mapping format -> output file path
            
        Raises:
            ValueError: If any format is not supported
            OSError: If directory operations fail
        """
        output_dir = Path(output_dir)
        output_dir.mkdir(parents=True, exist_ok=True)
        
        if save_options is None:
            save_options = {}
        
        results = {}
        
        for format_name in target_formats:
            # Validate format
            valid_format = ImageFormat.validate_format(str(format_name))
            
            # Get format-specific options
            opts = save_options.get(valid_format, {})
            
            # Create output path
            output_path = output_dir / f"{base_filename}.{self._get_extension(valid_format)}"
            
            # Convert and save
            converted = self.convert_format(valid_format, opts)
            converted.save(str(output_path))
            
            results[valid_format] = str(output_path)
        
        return results
    
    def get_format_info(self) -> Dict[str, Any]:
        """
        Get comprehensive information about image format capabilities.
        
        Returns:
            Dict with format capabilities, options, and metadata
        """
        info = {
            'current_format': self.format,
            'mode': self.mode,
            'size': self.size,
            'supported_formats': ImageFormat.get_supported_formats(),
            'lossy_formats': ImageFormat.get_lossy_formats(),
            'lossless_formats': ImageFormat.get_lossless_formats(),
            'format_features': {
                'JPEG': {
                    'supports_quality': True,
                    'supports_progressive': True,
                    'supports_optimization': True,
                    'lossy': True,
                    'extensions': ['.jpg', '.jpeg'],
                    'options': ['quality', 'progressive', 'optimize', 'subsampling']
                },
                'PNG': {
                    'supports_compression': True,
                    'supports_transparency': True,
                    'lossless': True,
                    'extensions': ['.png'],
                    'options': ['compress_level', 'optimize', 'interlace']
                },
                'WEBP': {
                    'supports_lossless': True,
                    'supports_quality': True,
                    'supports_transparency': True,
                    'lossy': True,
                    'lossless': True,
                    'extensions': ['.webp'],
                    'options': ['quality', 'lossless', 'method', 'alpha_quality']
                },
                'GIF': {
                    'supports_animation': True,
                    'supports_palette': True,
                    'lossless': False,
                    'extensions': ['.gif'],
                    'options': ['optimize', 'colors', 'transparency', 'duration']
                }
            }
        }
        
        return info
    
    def validate_conversion(
        self,
        target_format: Union[str, ImageFormat],
        check_capabilities: bool = True,
    ) -> Dict[str, Any]:
        """
        Validate if image can be converted to target format.
        
        Args:
            target_format: Target format to validate
            check_capabilities: Check format-specific capabilities
            
        Returns:
            Dict with validation results and recommendations
        """
        # Normalize format
        format_name = ImageFormat.validate_format(str(target_format))
        
        validation = {
            'can_convert': True,
            'target_format': format_name,
            'current_format': self.format,
            'current_mode': self.mode,
            'recommendations': [],
            'warnings': [],
            'errors': []
        }
        
        # Check basic compatibility
        if format_name == self.format:
            validation['warnings'].append("Source and target formats are identical")
        
        # Check mode compatibility
        mode_info = self.get_format_capabilities(format_name)
        if not mode_info['supports_mode']:
            validation['errors'].append(
                f"Format {format_name} does not support mode {self.mode}"
            )
            validation['can_convert'] = False
        
        # Add mode-specific recommendations
        if self.mode == 'RGBA' and format_name in ['JPEG', 'JPG']:
            validation['recommendations'].append(
                "Consider converting to RGB first to avoid background color issues"
            )
        
        if self.mode == 'CMYK' and format_name not in ['TIFF', 'PDF']:
            validation['recommendations'].append(
                "CMYK to RGB conversion may affect color appearance"
            )
        
        # Check size limitations
        if max(self.size) > 65535 and format_name in ['JPEG', 'PNG']:
            validation['warnings'].append(
                "Large image size may cause compatibility issues in some browsers"
            )
        
        return validation
    
    def get_format_capabilities(self, format_name: str) -> Dict[str, Any]:
        """
        Get detailed capabilities of a specific format.
        
        Args:
            format_name: Format name to analyze
            
        Returns:
            Dict with format capabilities and limitations
        """
        format_name = ImageFormat.validate_format(format_name)
        
        capabilities = {
            'format': format_name,
            'supports_mode': {
                'RGB': True,
                'RGBA': True,
                'L': True,
                'LA': True,
                'CMYK': True,
                'YCbCr': True,
                'HSV': True
            },
            'lossy': False,
            'lossless': False,
            'supports_transparency': False,
            'supports_animation': False,
            'supports_icc_profile': False,
            'max_dimensions': (65535, 65535),
            'max_file_size': 2**32 - 1,  # 4GB
            'extensions': [],
            'mime_types': []
        }
        
        # Format-specific capabilities
        if format_name in ['JPEG', 'JPG']:
            capabilities.update({
                'lossy': True,
                'supports_quality': True,
                'supports_progressive': True,
                'max_dimensions': (65535, 65535),
                'extensions': ['.jpg', '.jpeg'],
                'mime_types': ['image/jpeg']
            })
            capabilities['supports_mode']['CMYK'] = False
        
        elif format_name == 'PNG':
            capabilities.update({
                'lossless': True,
                'supports_transparency': True,
                'supports_icc_profile': True,
                'max_dimensions': (65535, 65535),
                'extensions': ['.png'],
                'mime_types': ['image/png']
            })
        
        elif format_name == 'GIF':
            capabilities.update({
                'supports_animation': True,
                'supports_transparency': True,
                'max_dimensions': (65535, 65535),
                'extensions': ['.gif'],
                'mime_types': ['image/gif']
            })
            capabilities['supports_mode']['CMYK'] = False
            capabilities['supports_mode']['YCbCr'] = False
            capabilities['supports_mode']['HSV'] = False
        
        elif format_name == 'WEBP':
            capabilities.update({
                'lossy': True,
                'lossless': True,
                'supports_transparency': True,
                'max_dimensions': (16383, 16383),
                'extensions': ['.webp'],
                'mime_types': ['image/webp']
            })
        
        elif format_name == 'TIFF':
            capabilities.update({
                'lossless': True,
                'supports_icc_profile': True,
                'max_dimensions': (2**32 - 1, 2**32 - 1),
                'extensions': ['.tiff', '.tif'],
                'mime_types': ['image/tiff']
            })
        
        return capabilities
    
    def _get_default_format_options(self, format_name: str) -> Dict[str, Any]:
        """Get default save options for a format."""
        options = {}
        
        if format_name in ['JPEG', 'JPG']:
            options = {
                'quality': 95,
                'optimize': True,
                'progressive': True,
            }
        elif format_name == 'PNG':
            options = {
                'compress_level': 6,
                'optimize': True,
            }
        elif format_name == 'WEBP':
            options = {
                'quality': 80,
                'method': 6,
            }
        elif format_name == 'GIF':
            options = {
                'optimize': True,
                'colors': 256,
            }
        
        return options
    
    def _apply_optimization(self, format_name: str, options: Dict[str, Any]) -> Dict[str, Any]:
        """Apply format-specific optimizations."""
        if format_name in ['JPEG', 'JPG']:
            # Apply progressive JPEG for web use
            if 'progressive' not in options and options.get('quality', 95) <= 90:
                options['progressive'] = True
            
            # Optimize for web
            if 'optimize' not in options:
                options['optimize'] = True
        
        elif format_name == 'PNG':
            # Enable optimization
            if 'optimize' not in options:
                options['optimize'] = True
            
            # Set optimal compression level
            if 'compress_level' not in options:
                options['compress_level'] = 6
        
        return options
    
    def _process_format_options(self, format_name: str, options: Dict[str, Any]) -> Dict[str, Any]:
        """Process and validate format-specific options."""
        # Ensure numeric values are properly typed
        if 'quality' in options:
            options['quality'] = max(1, min(100, int(options['quality'])))
        
        if 'compress_level' in options:
            options['compress_level'] = max(0, min(9, int(options['compress_level'])))
        
        if 'colors' in options:
            options['colors'] = max(1, min(256, int(options['colors'])))
        
        return options
    
    def _get_extension(self, format_name: str) -> str:
        """Get file extension for format."""
        mapping = ImageFormat.get_extension_mapping()
        for ext, fmt in mapping.items():
            if fmt == format_name:
                return ext[1:]  # Remove the dot
        return format_name.lower()