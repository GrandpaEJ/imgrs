#!/usr/bin/env python3
"""
Advanced Blend Test - Test the new blend method with position and mask
"""

import imgrs


def test_advanced_blend():
    """Test the advanced blend method with position and mask."""
    print("Testing advanced blend method...")

    # Create base image
    base = imgrs.Image.new("RGB", (200, 200), (100, 150, 200))

    # Create overlay image
    overlay = imgrs.Image.new("RGBA", (50, 50), (255, 100, 100, 200))

    # Test basic blend without position/mask
    try:
        base.blend("multiply", overlay)
        print("✓ Basic blend works")
    except Exception as e:
        print(f"✗ Basic blend failed: {e}")
        return False

    # Test blend with position
    try:
        base.blend("screen", overlay, position=(25, 25))
        print("✓ Blend with position works")
    except Exception as e:
        print(f"✗ Blend with position failed: {e}")
        return False

    # Test blend with mask
    try:
        mask = imgrs.Image.new("L", (50, 50), 128)  # 50% opacity mask
        base.blend("overlay", overlay, mask=mask, position=(50, 50))
        print("✓ Blend with mask works")
    except Exception as e:
        print(f"✗ Blend with mask failed: {e}")
        return False

    # Test blend with both mask and position
    try:
        base.blend("difference", overlay, mask=mask, position=(100, 100))
        print("✓ Blend with mask and position works")
    except Exception as e:
        print(f"✗ Blend with mask and position failed: {e}")
        return False

    # Test blend without other image (should return copy)
    try:
        base.blend("multiply")  # No other image
        print("✓ Blend without other image works")
    except Exception as e:
        print(f"✗ Blend without other image failed: {e}")
        return False

    return True


def main():
    """Main test function."""
    print("Advanced Blend Test")
    print("=" * 20)

    if test_advanced_blend():
        print("\n✓ All advanced blend tests passed!")
    else:
        print("\n✗ Some advanced blend tests failed!")


if __name__ == "__main__":
    main()
