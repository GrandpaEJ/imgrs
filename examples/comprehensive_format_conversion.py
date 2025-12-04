#!/usr/bin/env python3
"""
Comprehensive Image Format Conversion Example

This example demonstrates all the new image format conversion capabilities
added to imgrs, including advanced format options, batch conversion,
format validation, and optimization features.
"""

import os
from pathlib import Path
import imgrs
from imgrs import Image, ImageFormat

def main():
    print("🎨 imgrs Comprehensive Format Conversion Example")
    print("=" * 60)
    
    # Create a test image with some content
    print("\n1. Creating test image...")
    test_image = create_test_image()
    test_image.save("test_original.png")
    print("✅ Test image created and saved as test_original.png")
    
    # Demonstrate format detection
    print("\n2. Format Detection Examples:")
    test_files = [
        "photo.jpg", "image.png", "logo.gif", 
        "icon.bmp", "picture.webp", "unknown.xyz"
    ]
    
    for filename in test_files:
        detected = imgrs.detect_format(filename)
        status = "✅ Supported" if detected else "❓ Unknown"
        print(f"   {filename:<15} → {detected or 'Unknown':<10} {status}")
    
    # Demonstrate format capabilities
    print("\n3. Format Capabilities Analysis:")
    formats_to_test = ['JPEG', 'PNG', 'WEBP', 'GIF', 'TIFF']
    
    for fmt in formats_to_test:
        capabilities = imgrs.get_format_capabilities(fmt)
        print(f"\n   📁 {fmt} Format:")
        print(f"      • Lossy: {'Yes' if capabilities['lossy'] else 'No'}")
        print(f"      • Lossless: {'Yes' if capabilities['lossless'] else 'No'}")
        print(f"      • Transparency: {'Yes' if capabilities['supports_transparency'] else 'No'}")
        print(f"      • Animation: {'Yes' if capabilities['supports_animation'] else 'No'}")
        print(f"      • Max Size: {capabilities['max_dimensions'][0]:,}×{capabilities['max_dimensions'][1]:,}")
        print(f"      • Extensions: {', '.join(capabilities['extensions'])}")
    
    # Individual format conversions with different options
    print("\n4. Individual Format Conversions:")
    
    # JPEG with different quality settings
    print("   🔄 Converting to JPEG with different quality settings...")
    jpeg_high = test_image.convert_format('JPEG', save_options={'quality': 95, 'optimize': True})
    jpeg_medium = test_image.convert_format('JPEG', save_options={'quality': 75, 'optimize': True})
    jpeg_low = test_image.convert_format('JPEG', save_options={'quality': 50, 'optimize': True})
    
    jpeg_high.save("jpeg_quality_95.jpg")
    jpeg_medium.save("jpeg_quality_75.jpg") 
    jpeg_low.save("jpeg_quality_50.jpg")
    
    print("      ✅ JPEG conversions saved (95%, 75%, 50% quality)")
    
    # PNG with different compression levels
    print("   🔄 Converting to PNG with different compression...")
    png_fast = test_image.convert_format('PNG', save_options={'compress_level': 1})
    png_optimal = test_image.convert_format('PNG', save_options={'compress_level': 6})
    png_max = test_image.convert_format('PNG', save_options={'compress_level': 9})
    
    png_fast.save("png_compress_1.png")
    png_optimal.save("png_compress_6.png")
    png_max.save("png_compress_9.png")
    
    print("      ✅ PNG conversions saved (compression levels 1, 6, 9)")
    
    # WEBP with different modes
    print("   🔄 Converting to WEBP (lossy and lossless)...")
    webp_lossy = test_image.convert_format('WEBP', save_options={'quality': 85, 'lossless': False})
    webp_lossless = test_image.convert_format('WEBP', save_options={'quality': 100, 'lossless': True})
    
    webp_lossy.save("webp_lossy.webp")
    webp_lossless.save("webp_lossless.webp")
    
    print("      ✅ WEBP conversions saved (lossy and lossless)")
    
    # Advanced format conversions
    print("\n5. Advanced Format Conversions:")
    
    # TIFF for archival
    print("   🔄 Converting to TIFF for archival...")
    tiff_archive = test_image.convert_format('TIFF', save_options={'compress_level': 9})
    tiff_archive.save("archive.tif")
    print("      ✅ TIFF archival version saved")
    
    # GIF with color optimization
    print("   🔄 Converting to GIF with color optimization...")
    gif_optimized = test_image.convert_format('GIF', save_options={'colors': 256, 'optimize': True})
    gif_optimized.save("optimized.gif")
    print("      ✅ GIF with 256 colors saved")
    
    # Demonstrate batch conversion
    print("\n6. Batch Conversion Example:")
    target_formats = ['JPEG', 'PNG', 'WEBP', 'TIFF']
    output_dir = "batch_output"
    
    results = test_image.batch_convert(
        target_formats=target_formats,
        output_dir=output_dir,
        base_filename="converted",
        save_options={
            'JPEG': {'quality': 85, 'optimize': True},
            'PNG': {'compress_level': 6},
            'WEBP': {'quality': 80},
            'TIFF': {'compress_level': 6}
        }
    )
    
    print("   📦 Batch conversion results:")
    for fmt, path in results.items():
        file_size = os.path.getsize(path) / 1024  # KB
        print(f"      {fmt:<8} → {path:<25} ({file_size:.1f} KB)")
    
    # Format validation and recommendations
    print("\n7. Format Validation and Recommendations:")
    
    # Test validation for different conversions
    validation_tests = [
        ('PNG', 'JPEG'),
        ('RGBA', 'JPEG'),
        ('JPEG', 'PNG'),
        ('PNG', 'GIF')
    ]
    
    for source_mode, target_format in validation_tests:
        # Create test image with specific mode
        if source_mode == 'RGBA':
            test_rgba = test_image.convert('RGBA')
            validation = test_rgba.validate_conversion(target_format)
        else:
            test_rgb = test_image.convert('RGB')
            validation = test_rgb.validate_conversion(target_format)
        
        status = "✅ Can convert" if validation['can_convert'] else "❌ Cannot convert"
        print(f"   {source_mode} → {target_format}: {status}")
        
        if validation['warnings']:
            for warning in validation['warnings']:
                print(f"      ⚠️  Warning: {warning}")
        
        if validation['recommendations']:
            for rec in validation['recommendations']:
                print(f"      💡 Recommendation: {rec}")
    
    # Format recommendations based on use case
    print("\n8. Format Recommendations by Use Case:")
    
    use_cases = ['web', 'print', 'archive', 'mobile']
    for use_case in use_cases:
        recommendation = imgrs.get_format_recommendation(test_image, use_case)
        print(f"   📋 {use_case.capitalize()} Use Case:")
        print(f"      Recommended: {recommendation['recommended']}")
        print(f"      Alternatives: {', '.join(recommendation['alternatives'])}")
        print(f"      Reason: {recommendation['reason']}")
    
    # Format optimization example
    print("\n9. Format Optimization Examples:")
    
    optimization_levels = ['low', 'medium', 'high', 'maximum']
    for level in optimization_levels:
        optimized = imgrs.optimize_for_format(test_image, 'WEBP', level)
        optimized.save(f"webp_{level}_quality.webp")
        
        file_size = os.path.getsize(f"webp_{level}_quality.webp") / 1024
        print(f"   ⚡ {level.capitalize()} quality WEBP: {file_size:.1f} KB")
    
    # Bytes conversion example
    print("\n10. Converting to Bytes (in-memory):")
    
    jpeg_bytes = test_image.convert_to_bytes('JPEG', save_options={'quality': 85})
    webp_bytes = test_image.convert_to_bytes('WEBP', save_options={'quality': 80})
    png_bytes = test_image.convert_to_bytes('PNG', save_options={'compress_level': 6})
    
    print(f"    📊 JPEG bytes: {len(jpeg_bytes):,} bytes")
    print(f"    📊 WEBP bytes: {len(webp_bytes):,} bytes") 
    print(f"    📊 PNG bytes: {len(png_bytes):,} bytes")
    
    # Format info summary
    print("\n11. Complete Format Information:")
    format_info = test_image.get_format_info()
    
    print(f"    📋 Current Image:")
    print(f"       Format: {format_info['current_format']}")
    print(f"       Mode: {format_info['mode']}")
    print(f"       Size: {format_info['size'][0]}×{format_info['size'][1]}")
    
    print(f"    📋 Supported Formats ({len(format_info['supported_formats'])} total):")
    lossy = format_info['lossy_formats']
    lossless = format_info['lossless_formats']
    print(f"       Lossy: {', '.join(lossy)} ({len(lossy)} formats)")
    print(f"       Lossless: {', '.join(lossless)} ({len(lossless)} formats)")
    
    # File size comparison
    print("\n12. File Size Comparison:")
    print("    📏 Comparing output file sizes:")
    
    conversion_files = [
        ("Original PNG", "test_original.png"),
        ("JPEG High", "jpeg_quality_95.jpg"),
        ("JPEG Medium", "jpeg_quality_75.jpg"),
        ("JPEG Low", "jpeg_quality_50.jpg"),
        ("WEBP Lossy", "webp_lossy.webp"),
        ("WEBP Lossless", "webp_lossless.webp"),
        ("TIFF Archive", "archive.tif"),
        ("GIF Optimized", "optimized.gif"),
    ]
    
    for name, filename in conversion_files:
        if os.path.exists(filename):
            size_kb = os.path.getsize(filename) / 1024
            print(f"       {name:<20}: {size_kb:>8.1f} KB")
    
    print("\n🎉 Comprehensive Format Conversion Example Complete!")
    print(f"📁 All output files saved to current directory")
    print(f"💡 Try the batch_output/ directory for multiple format examples")


def create_test_image():
    """Create a test image with some content for testing conversions."""
    # Create a 400x300 image
    img = Image.new('RGB', (400, 300), (255, 255, 255))  # White background
    
    # Add some colored rectangles
    img = img.draw_rectangle(50, 50, 150, 100, (255, 0, 0, 255))    # Red
    img = img.draw_rectangle(200, 50, 300, 100, (0, 255, 0, 255))   # Green
    img = img.draw_rectangle(50, 150, 150, 200, (0, 0, 255, 255))   # Blue
    
    # Add a circle
    img = img.draw_circle(325, 75, 40, (255, 255, 0, 255))          # Yellow
    
    # Add some text
    img = img.draw_text("Format Test", 10, 250, (0, 0, 0, 255), scale=2)
    
    return img


if __name__ == "__main__":
    main()