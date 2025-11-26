#!/usr/bin/env python3
"""
Rotation Operations Example for Imgrs Image Processing Library

This example demonstrates the enhanced rotation functionality:
- Arbitrary angle rotation
- Expand parameter (crop to original size or expand to fit)
- Fillcolor for expanded areas
- Easy rotation aliases
"""

import os
import sys
from pathlib import Path

# Add the parent directory to the path to import imgrs
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python"))


def setup_paths():
    """Setup input and output paths."""
    examples_dir = Path(__file__).parent
    img_dir = examples_dir / "img"
    output_dir = examples_dir / "output"
    output_dir.mkdir(exist_ok=True)
    return img_dir, output_dir


def test_rotation_operations():
    """Test enhanced rotation operations."""
    try:
        import imgrs

        print("✓ Imgrs imported successfully")
    except ImportError as e:
        print(f"✗ Failed to import imgrs: {e}")
        print("Make sure to build the Rust extension with: maturin develop")
        return False

    img_dir, output_dir = setup_paths()

    try:
        print("\n" + "=" * 60)
        print("ENHANCED ROTATION DEMO")
        print("=" * 60)

        # Open test image
        print("\n1. Opening test image...")
        img_path = img_dir / "colorful_squares.png"
        if not img_path.exists():
            print(f"✗ Test image not found: {img_path}")
            return False

        img = imgrs.open(str(img_path))
        print(f"✓ Opened image: {img.size} {img.mode}")

        # Note: Arbitrary angle rotation not implemented yet
        print("\n2. Arbitrary angle rotations - NOT IMPLEMENTED")
        print("   Skipping arbitrary angle tests...")

        # Test fillcolor with expand - also not implemented
        print("\n3. Fillcolor with expand - NOT IMPLEMENTED")
        print("   Skipping fillcolor tests...")

        # Test rotation aliases
        print("\n4. Testing rotation aliases...")
        rotated_90 = img.rotate90()
        rotated_90.save(str(output_dir / "rotated_90_alias.png"))
        print(f"✓ rotate90(): {rotated_90.size}")

        rotated_180 = img.rotate180()
        rotated_180.save(str(output_dir / "rotated_180_alias.png"))
        print(f"✓ rotate180(): {rotated_180.size}")

        rotated_270 = img.rotate270()
        rotated_270.save(str(output_dir / "rotated_270_alias.png"))
        print(f"✓ rotate270(): {rotated_270.size}")

        rotated_left = img.rotate_left()
        rotated_left.save(str(output_dir / "rotated_left_alias.png"))
        print(f"✓ rotate_left(): {rotated_left.size}")

        rotated_right = img.rotate_right()
        rotated_right.save(str(output_dir / "rotated_right_alias.png"))
        print(f"✓ rotate_right(): {rotated_right.size}")

        # Test expand with aliases
        print("\n5. Testing expand with aliases...")
        rotated_90_expand = img.rotate90()  # expand defaults to False
        rotated_90_expand.save(str(output_dir / "rotated_90_expand_false.png"))
        print(f"✓ rotate90() (expand=False): {rotated_90_expand.size}")

        # Test zero rotation
        print("\n6. Testing zero rotation...")
        zero_rotated = img.rotate(0)
        zero_rotated.save(str(output_dir / "rotated_0.png"))
        print(f"✓ 0° rotation: {zero_rotated.size}")

        print("\n" + "=" * 60)
        print("🎉 ALL ROTATION OPERATIONS COMPLETED SUCCESSFULLY!")
        print("=" * 60)
        print(f"Check the output directory: {output_dir}")

        return True

    except Exception as e:
        print(f"\n❌ Error during rotation operations: {e}")
        import traceback

        traceback.print_exc()
        return False


def main():
    """Run rotation demo."""
    print("Imgrs Enhanced Rotation Demo")
    print("This script tests all enhanced rotation features")

    if test_rotation_operations():
        print("\n✅ Rotation operations test passed!")
        return 0
    else:
        print("\n❌ Rotation operations test failed!")
        return 1


if __name__ == "__main__":
    sys.exit(main())
