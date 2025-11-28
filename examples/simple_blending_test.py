#!/usr/bin/env python3
"""
Simple Blending Test - Quick verification of blending functionality
"""

import imgrs


def test_basic_blending():
    """Test basic blending functionality."""
    print("Testing basic blending...")

    # Create simple images
    bg = imgrs.Image.new("RGB", (100, 100), (255, 0, 0))  # Red background
    fg = imgrs.Image.new("RGBA", (50, 50), (0, 255, 0, 128))  # Green overlay

    # Test composite method
    try:
        bg.composite(fg, mode="over")
        print("✓ composite() method works")
    except Exception as e:
        print(f"✗ composite() failed: {e}")
        return False

    # Test convenience methods
    try:
        bg.blend_multiply(fg)
        print("✓ blend_multiply() convenience method works")
    except Exception as e:
        print(f"✗ blend_multiply() failed: {e}")
        return False

    # Test different modes
    modes = ["over", "multiply", "screen", "overlay", "difference"]
    for mode in modes:
        try:
            bg.composite(fg, mode=mode)
            print(f"✓ Mode '{mode}' works")
        except Exception as e:
            print(f"✗ Mode '{mode}' failed: {e}")
            return False

    return True


def main():
    """Main test function."""
    print("Simple Blending Test")
    print("=" * 20)

    if test_basic_blending():
        print("\n✓ All tests passed!")
    else:
        print("\n✗ Some tests failed!")


if __name__ == "__main__":
    main()
