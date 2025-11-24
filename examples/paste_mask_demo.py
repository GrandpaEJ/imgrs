#!/usr/bin/env python3
"""
Enhanced paste with mask support demonstration

This script demonstrates the new mask support in img.paste() method,
compatible with Pillow's mask functionality.
"""

import sys
from pathlib import Path

# Add the parent directory to sys.path to import imgrs
sys.path.insert(0, str(Path(__file__).parent.parent))

from imgrs import new, paste  # noqa: E402


def create_gradient_mask(width, height, direction="horizontal"):
    """Create a simple gradient mask for testing."""
    # Create simple uniform masks for demo
    if direction == "horizontal":
        # Left side transparent, right side opaque
        mask = new("L", (width, height), 128)  # 50% opacity
    else:
        mask = new("L", (width, height), 200)  # ~80% opacity

    return mask


def create_circular_mask(width, height, center_x, center_y, radius):
    """Create a simple circular mask for testing."""
    # Create a simple solid mask for demo
    mask = new("L", (width, height), 200)  # ~80% opacity
    return mask


def demo_basic_paste():
    """Demonstrate basic paste without mask."""
    print("=== Basic Paste Demo ===")

    # Create base image
    base = new("RGB", (300, 200), (255, 255, 255))  # White

    # Create overlay image
    overlay = new("RGB", (100, 100), (255, 0, 0))  # Red

    # Basic paste
    result = base.paste(overlay, (100, 50))
    result.save("examples/output/basic_paste.png")
    print("✓ Basic paste saved to examples/output/basic_paste.png")


def demo_grayscale_mask():
    """Demonstrate paste with grayscale mask."""
    print("\n=== Grayscale Mask Demo ===")

    base = new("RGB", (300, 200), (255, 255, 255))  # White
    overlay = new("RGB", (100, 100), (0, 0, 255))  # Blue

    # Create simple grayscale mask
    mask = create_gradient_mask(100, 100, "horizontal")

    result = base.paste(overlay, (100, 50), mask)
    result.save("examples/output/grayscale_mask.png")
    print("✓ Grayscale mask paste saved to examples/output/grayscale_mask.png")


def demo_circular_mask():
    """Demonstrate paste with circular mask."""
    print("\n=== Circular Mask Demo ===")

    base = new("RGB", (300, 200), (192, 192, 192))  # Light gray
    overlay = new("RGB", (100, 100), (0, 255, 0))  # Green

    # Create simple mask
    mask = create_circular_mask(100, 100, 50, 50, 40)

    result = base.paste(overlay, (100, 50), mask)
    result.save("examples/output/circular_mask.png")
    print("✓ Circular mask paste saved to examples/output/circular_mask.png")


def demo_rgba_mask():
    """Demonstrate paste with RGBA mask."""
    print("\n=== RGBA Mask Demo ===")

    base = new("RGB", (300, 200), (255, 255, 255))  # White
    overlay = new("RGB", (100, 100), (128, 0, 128))  # Purple

    # Create RGBA mask with varying alpha
    mask = new("RGBA", (100, 100), (255, 255, 255, 128))  # 50% alpha

    result = base.paste(overlay, (100, 50), mask)
    result.save("examples/output/rgba_mask.png")
    print("✓ RGBA mask paste saved to examples/output/rgba_mask.png")


def demo_la_mask():
    """Demonstrate paste with LA (grayscale + alpha) mask."""
    print("\n=== LA Mask Demo ===")

    base = new("RGB", (300, 200), (255, 255, 255))  # White
    overlay = new("RGB", (100, 100), (255, 165, 0))  # Orange

    # Create simple grayscale mask to demonstrate the concept
    mask = new("L", (100, 100), 200)  # 80% opacity grayscale

    result = base.paste(overlay, (100, 50), mask)
    result.save("examples/output/la_mask.png")
    print("✓ LA mask paste saved to examples/output/la_mask.png")


def demo_rgb_mask():
    """Demonstrate paste with RGB mask (uses luminance)."""
    print("\n=== RGB Mask Demo ===")

    base = new("RGB", (300, 200), (255, 255, 255))  # White
    overlay = new("RGB", (100, 100), (0, 255, 255))  # Cyan

    # Create RGB mask - white areas fully visible
    mask = new("RGB", (100, 100), (128, 128, 128))  # Gray = 50% opacity

    result = base.paste(overlay, (100, 50), mask)
    result.save("examples/output/rgb_mask.png")
    print("✓ RGB mask paste saved to examples/output/rgb_mask.png")


def demo_functional_api():
    """Demonstrate paste with mask using functional API."""
    print("\n=== Functional API Demo ===")

    base = new("RGB", (300, 200), (255, 255, 255))  # White
    overlay = new("RGB", (100, 100), (255, 0, 255))  # Magenta

    # Create mask
    mask = create_gradient_mask(100, 100, "vertical")

    # Use functional API
    result = paste(base, overlay, (100, 50), mask)
    result.save("examples/output/functional_api.png")
    print("✓ Functional API paste saved to examples/output/functional_api.png")


def demo_transparency_levels():
    """Demonstrate different transparency levels."""
    print("\n=== Transparency Levels Demo ===")

    base = new("RGB", (400, 300), (255, 255, 255))  # White
    overlay = new("RGB", (80, 80), (255, 0, 0))  # Red

    # Create masks with different transparency levels
    masks = [
        ("fully_opaque", new("L", (80, 80), 255)),
        ("half_opaque", new("L", (80, 80), 128)),
        ("fully_transparent", new("L", (80, 80), 0)),
    ]

    positions = [(50, 50), (150, 50), (250, 50)]

    result = base
    for i, (name, mask) in enumerate(masks):
        result = result.paste(overlay, positions[i], mask)

    result.save("examples/output/transparency_levels.png")
    print("✓ Transparency levels saved to examples/output/transparency_levels.png")


def demo_error_handling():
    """Demonstrate error handling for invalid masks."""
    print("\n=== Error Handling Demo ===")

    base = new("RGB", (200, 200), (255, 255, 255))  # White
    overlay = new("RGB", (50, 50), (255, 0, 0))  # Red
    wrong_size_mask = new("L", (30, 30), 128)  # Wrong size

    try:
        # This should raise ValueError
        base.paste(overlay, (75, 75), wrong_size_mask)
        print("❌ Should have raised ValueError")
    except ValueError as e:
        print(f"✓ Correctly caught error: {e}")

    try:
        # This should raise TypeError
        base.paste(overlay, (75, 75), "not_an_image")
        print("❌ Should have raised TypeError")
    except TypeError as e:
        print(f"✓ Correctly caught error: {e}")


def main():
    """Run all demos."""
    print("Enhanced paste() with mask support - Demo Script")
    print("=" * 50)

    # Create output directory
    output_dir = Path("examples/output")
    output_dir.mkdir(exist_ok=True)

    try:
        demo_basic_paste()
        demo_grayscale_mask()
        demo_circular_mask()
        demo_rgba_mask()
        demo_la_mask()
        demo_rgb_mask()
        demo_functional_api()
        demo_transparency_levels()
        demo_error_handling()

        print("\n" + "=" * 50)
        print("🎉 All demos completed successfully!")
        print("\nGenerated files:")
        for file in output_dir.glob("*.png"):
            print(f"  📄 {file}")

    except Exception as e:
        print(f"❌ Demo failed with error: {e}")
        import traceback

        traceback.print_exc()


if __name__ == "__main__":
    main()
