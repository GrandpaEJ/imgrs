#!/usr/bin/env python3
"""
Test script to demonstrate the alpha issue mentioned by user
"""

import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "python"))

from imgrs import Image  # noqa: E402


def test_alpha_issue():
    """Test the specific alpha issue described by user"""

    # Load an existing PNG to test the issue
    try:
        png_img = Image.open("img.png")
        print(f"Loaded PNG: {png_img.width()}x{png_img.height()}")
        original_alpha = png_img.get_alpha()
        print(f"Original alpha: {original_alpha:.3f}")

        # Test the issue: set different alpha values and see the result
        for alpha_val in [0.8, 0.5, 0.2, 0.1]:
            print(f"\n--- Testing set_alpha({alpha_val}) ---")
            modified = png_img.set_alpha(alpha_val)
            new_alpha = modified.get_alpha()
            print(f"Resulting alpha: {new_alpha:.3f}")

            # Save to visualize the issue
            modified.save(f"test_alpha_{alpha_val}.png")
            print(f"Saved as test_alpha_{alpha_val}.png")

    except Exception as e:
        print(f"PNG test failed: {e}")

    # Create a test image with transparency to understand the behavior
    print("\n--- Creating test image ---")
    test_img = Image.new(
        "RGBA", (200, 200), (255, 255, 255, 128)
    )  # 50% transparent white
    test_img = test_img.draw_circle(50, 50, 30, (255, 0, 0, 255))  # Red opaque circle
    test_img = test_img.draw_circle(
        100, 100, 20, (0, 255, 0, 200)
    )  # Green semi-transparent circle
    test_img = test_img.draw_circle(
        150, 50, 25, (0, 0, 255, 50)
    )  # Blue mostly transparent circle
    test_img.save("test_original.png")

    original_alpha = test_img.get_alpha()
    print(f"Test image original alpha: {original_alpha:.3f}")

    # Apply different alpha values
    for alpha_val in [0.8, 0.5, 0.2]:
        modified = test_img.set_alpha(alpha_val)
        new_alpha = modified.get_alpha()
        print(f"set_alpha({alpha_val}) -> resulting alpha: {new_alpha:.3f}")
        modified.save(f"test_modified_{alpha_val}.png")


if __name__ == "__main__":
    test_alpha_issue()
