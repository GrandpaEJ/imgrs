#!/usr/bin/env python3
"""
Simple Text Anchoring Demo - Basic demonstration of anchor functionality

This example shows how to use text anchors with the add_text method.
"""

import os
import sys

# Add the project root to the path
sys.path.insert(
    0, os.path.join(os.path.dirname(__file__), "..", "python")
)  # noqa: E402

from imgrs import Image  # noqa: E402


def create_simple_anchor_demo():
    """Create a simple demonstration of text anchoring"""

    width, height = 800, 600
    image = Image.new("RGB", (width, height), (250, 250, 250))

    print("Creating simple anchor demonstration...")

    # Define anchor positions to test
    test_positions = [
        (100, 100, "tl", "TopLeft"),
        (400, 100, "tm", "TopCenter"),
        (700, 100, "tr", "TopRight"),
        (100, 300, "ml", "MiddleLeft"),
        (400, 300, "mm", "MiddleCenter"),
        (700, 300, "mr", "MiddleRight"),
        (100, 500, "bl", "BottomLeft"),
        (400, 500, "bm", "BottomCenter"),
        (700, 500, "br", "BottomRight"),
    ]

    # Test each anchor position
    for x, y, anchor_code, anchor_name in test_positions:
        print(f"  Testing {anchor_name} ({anchor_code}) at ({x}, {y})")

        # Draw anchor point indicator
        image = image.draw_circle(x, y, 3, (255, 0, 0, 255))  # Red dot

        # Draw text with this anchor
        image = image.add_text(
            text="TEST",
            position=(x, y),
            size=16,
            color=(0, 0, 0, 255),
            anchor=anchor_code,
        )

        # Add label
        image = image.add_text(
            text=anchor_name,
            position=(x, y + 25),
            size=10,
            color=(100, 100, 100, 255),
            anchor="mm",
        )

    # Add title
    image = image.add_text(
        text="Text Anchor Demo",
        position=(width // 2, 20),
        size=24,
        color=(0, 0, 0, 255),
        anchor="mm",
    )

    return image


def main():
    """Main function to run the simple anchor demo"""
    print("Simple Text Anchor Demo")
    print("=" * 40)

    try:
        # Create the demo
        result_image = create_simple_anchor_demo()

        # Save the result
        output_path = os.path.join(
            os.path.dirname(__file__), "output", "simple_anchors_demo.png"
        )
        result_image.save(output_path)

        print("\n✅ Simple anchor demo completed successfully!")
        print(f"📁 Output saved to: {output_path}")
        print(f"🖼️  Image dimensions: {result_image.width}x{result_image.height}")

        print("\n📋 Features Demonstrated:")
        print("   • Basic text anchoring with 9 positions")
        print("   • Visual indicators showing anchor points")
        print("   • Text positioning relative to anchor")

    except Exception as e:
        print(f"\n❌ Error creating simple anchor demo: {e}")
        import traceback

        traceback.print_exc()
        return False

    return True


if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
