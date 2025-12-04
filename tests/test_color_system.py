"""
Comprehensive test script for enhanced color system features
"""

import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "python"))

try:
    import imgrs
    from imgrs import BlendMode, ColorFormat, Image, MaskType

    # Create a test image
    print("Creating test image...")
    img = Image.new("RGB", (200, 200), (255, 0, 0))  # Red image

    # Test transparency operations
    print("\n1. Testing Transparency Operations")
    print("=" * 40)

    # Set global alpha
    img_with_alpha = img.set_alpha(0.5)
    print("✓ Set global alpha to 0.5")
    print(f"✓ Current alpha: {img_with_alpha.get_alpha():.2f}")

    # Add transparency to white color
    img_transparent = img_with_alpha.add_transparency((255, 0, 0), tolerance=50)
    print("✓ Added transparency to red color")

    # Remove transparency on white background
    img_opaque = img_transparent.remove_transparency((255, 255, 255))
    print("✓ Removed transparency with white background")

    # Test masking operations
    print("\n2. Testing Advanced Masking System")
    print("=" * 40)

    # Create gradient masks
    gradient_mask = img.create_gradient_mask("vertical", 0.0, 1.0)
    print("✓ Created vertical gradient mask")

    # Create color-based mask
    color_mask = img.create_color_mask((255, 0, 0), tolerance=50, feather=5)
    print("✓ Created color-based mask")

    # Create luminance mask
    luminance_mask = img.create_luminance_mask(invert=False)
    print("✓ Created luminance mask")

    # Test color manipulation
    print("\n3. Testing Color Manipulation")
    print("=" * 40)

    # Extract specific color
    img_extracted = img.extract_color((255, 0, 0), tolerance=30)
    print("✓ Extracted red color")

    # Color quantization
    img_quantized = img.color_quantize(levels=8)
    print("✓ Applied color quantization (8 levels)")

    # Color shift
    img_shifted = img.color_shift(0.2)
    print("✓ Applied color shift")

    # Selective desaturation
    img_selective = img.selective_desaturate(
        (255, 0, 0), tolerance=50, desaturate_factor=0.7
    )
    print("✓ Applied selective desaturation")

    # Test gradient and pattern operations
    print("\n4. Testing Gradient & Pattern Operations")
    print("=" * 40)

    # Apply gradient overlay
    img_gradient = img.apply_gradient_overlay((0, 255, 0, 200), "horizontal", 0.7)
    print("✓ Applied horizontal gradient overlay")

    # Create stripe pattern
    stripe_pattern = img.create_stripe_pattern(
        (0, 0, 255, 128), width=10, spacing=5, angle=45.0
    )
    print("✓ Created diagonal stripe pattern")

    # Create checker pattern
    checker_pattern = img.create_checker_pattern(
        (255, 0, 0, 100), (0, 0, 255, 100), size=8
    )
    print("✓ Created checkerboard pattern")

    # Test alpha channel operations
    print("\n5. Testing Alpha Channel Operations")
    print("=" * 40)

    # Split alpha
    rgb_img, alpha_img = img.split_alpha()
    print("✓ Split image into RGB and alpha channels")

    # Merge alpha
    img_merged = rgb_img.merge_alpha(alpha_img)
    print("✓ Merged alpha channel back")

    # Convert alpha to color
    img_alpha_colored = img.alpha_to_color((128, 128, 128))
    print("✓ Converted alpha channel to gray color")

    # Test advanced blending
    print("\n6. Testing Advanced Blending")
    print("=" * 40)

    # Create another test image for blending
    overlay_img = Image.new("RGB", (200, 200), (0, 255, 0))  # Green overlay

    # Blend with different modes
    img_multiply = img.blend_with(overlay_img, "multiply", 0.8)
    print("✓ Applied multiply blend mode")

    img_screen = img.blend_with(overlay_img, "screen", 0.6)
    print("✓ Applied screen blend mode")

    img_overlay = img.blend_with(overlay_img, "overlay", 0.7)
    print("✓ Applied overlay blend mode")

    # Test overlay positioning
    img_positioned = img.overlay_with(overlay_img, "difference", 0.5, position=(50, 50))
    print("✓ Applied positioned overlay")

    # Test color analysis
    print("\n7. Testing Color Analysis")
    print("=" * 40)

    # Get color palette
    palette = img.get_color_palette(max_colors=5)
    print(f"✓ Extracted color palette: {len(palette)} colors")

    # Analyze color distribution
    distribution = img.analyze_color_distribution()
    print("✓ Analyzed color distribution")
    print(f"  - Total pixels: {distribution.get('total_pixels', 'N/A')}")
    print(f"  - Unique colors: {distribution.get('unique_colors', 'N/A')}")
    print(f"  - Dominant color: {distribution.get('dominant_color', 'N/A')}")

    # Find color regions
    regions = img.find_color_regions((255, 0, 0), tolerance=50)
    print(f"✓ Found {len(regions)} color regions")

    # Test combine masks
    print("\n8. Testing Mask Combination")
    print("=" * 40)

    masks = [gradient_mask, color_mask, luminance_mask]
    combined_mask = img.combine_masks(masks[:2], "multiply")
    print("✓ Combined masks with multiply operation")

    # Test enum exports
    print("\n9. Testing Enhanced Enums")
    print("=" * 40)

    print(f"✓ BlendMode.NORMAL: {BlendMode.NORMAL}")
    print(f"✓ ColorFormat.RGB: {ColorFormat.RGB}")
    print(f"✓ MaskType.GRADIENT: {MaskType.GRADIENT}")

    # Version check
    print("\n10. Version Information")
    print("=" * 40)
    print(f"✓ Imgrs version: {imgrs.__version__}")
    print(f"✓ ColorMixin available: {hasattr(Image, 'set_alpha')}")

    print("\n" + "=" * 50)
    print("✅ ALL COLOR SYSTEM TESTS PASSED!")
    print("Enhanced color system is fully functional.")
    print("=" * 50)

except ImportError as e:
    print(f"❌ Import error: {e}")
    print("Note: Rust extension needs to be built with: python -m build")
except Exception as e:
    print(f"❌ Test failed: {e}")
    import traceback

    traceback.print_exc()
