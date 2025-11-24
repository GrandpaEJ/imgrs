#!/usr/bin/env python3
"""
Test script to verify the alpha setting fix
"""

import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "python"))

from imgrs import Image  # noqa: E402


def test_alpha_fix():
    """Test that set_alpha preserves transparency relationships"""

    # Create a simple test image with different alpha levels
    print("Creating test image...")
    img = Image.new("RGBA", (100, 100), (255, 0, 0, 128))  # Semi-transparent red

    # Test 1: Set alpha to 0.5 should give 25% opacity (0.5 * 0.5 = 0.25)
    img_half_alpha = img.set_alpha(0.5)
    half_alpha = img_half_alpha.get_alpha()
    print(f"Original alpha: 0.5, After set_alpha(0.5): {half_alpha:.3f}")
    assert abs(half_alpha - 0.25) < 0.01, f"Expected ~0.25, got {half_alpha}"

    # Test 2: Set alpha to 1.0 should keep original alpha
    img_full_alpha = img.set_alpha(1.0)
    full_alpha = img_full_alpha.get_alpha()
    print(f"Original alpha: 0.5, After set_alpha(1.0): {full_alpha:.3f}")
    assert abs(full_alpha - 0.5) < 0.01, f"Expected ~0.5, got {full_alpha}"

    # Test 3: Set alpha to 0.0 should make fully transparent
    img_no_alpha = img.set_alpha(0.0)
    no_alpha = img_no_alpha.get_alpha()
    print(f"Original alpha: 0.5, After set_alpha(0.0): {no_alpha:.3f}")
    assert abs(no_alpha - 0.0) < 0.01, f"Expected ~0.0, got {no_alpha}"

    # Test 4: Load existing PNG and test alpha
    try:
        # Create a temporary test image
        test_img = Image.new("RGBA", (100, 100), (0, 255, 0, 255))
        test_img.save("test_alpha_img.png")

        png_img = Image.open("test_alpha_img.png")
        original_alpha = png_img.get_alpha()
        print(f"PNG original alpha: {original_alpha:.3f}")

        # Apply different alpha values
        for alpha_val in [0.2, 0.5, 0.8]:
            modified = png_img.set_alpha(alpha_val)
            new_alpha = modified.get_alpha()
            expected = original_alpha * alpha_val
            print(f"set_alpha({alpha_val}): {new_alpha:.3f} (expected ~{expected:.3f})")
            assert abs(new_alpha - expected) < 0.05, f"Alpha mismatch for {alpha_val}"

        print("✓ All alpha tests passed!")
        if os.path.exists("test_alpha_img.png"):
            os.remove("test_alpha_img.png")
        return True

    except Exception as e:
        print(f"PNG test failed: {e}")
        if os.path.exists("test_alpha_img.png"):
            os.remove("test_alpha_img.png")
        return False


if __name__ == "__main__":
    try:
        test_alpha_fix()
        print("✓ Alpha fix verification successful!")
    except Exception as e:
        print(f"✗ Test failed: {e}")
        sys.exit(1)
