#!/usr/bin/env python3
"""
Simple Comprehensive Demo - Shows both text box and anchoring functionality

This example demonstrates:
1. Text box functionality with wrapping and alignment
2. Text anchoring for precise positioning
3. Basic text styling and effects
"""

import os
import sys

# Add the project root to the path
sys.path.insert(
    0, os.path.join(os.path.dirname(__file__), "..", "python")
)  # noqa: E402

from imgrs import Image  # noqa: E402


def create_comprehensive_demo():
    """Create a comprehensive demo showcasing both features"""

    width, height = 900, 700
    image = Image.new("RGB", (width, height), (245, 245, 245))

    print("Creating comprehensive demo...")

    # Section 1: Text Box Demo
    print("  Adding text box examples...")

    demo_text = """
    This is a text box demonstration showing how text automatically wraps
    within a defined rectangular area. The text box feature allows for
    precise control over text layout and formatting.
    """

    # Text box with left alignment
    image = image.add_text_box(
        text=demo_text.strip(),
        box=(50, 50, 350, 200),
        size=14,
        color=(50, 50, 50, 255),
        background=(220, 230, 240, 255),
        align="left",
        vertical_align="top",
        line_spacing=1.2,
        overflow=False,
    )

    # Text box with center alignment
    image = image.add_text_box(
        text="CENTERED TEXT BOX",
        box=(450, 50, 400, 150),
        size=20,
        color=(255, 255, 255, 255),
        background=(70, 130, 180, 255),
        align="center",
        vertical_align="center",
        line_spacing=1.0,
        overflow=True,
    )

    # Section 2: Anchor Demo
    print("  Adding anchor examples...")

    # Create a grid for anchor demonstration
    anchor_tests = [
        (200, 300, "tl", "TopLeft"),
        (450, 300, "mm", "MiddleCenter"),
        (700, 300, "br", "BottomRight"),
    ]

    for x, y, anchor_code, anchor_name in anchor_tests:
        # Draw anchor indicator
        image = image.draw_circle(x, y, 4, (255, 0, 0, 255))

        # Draw text with anchor
        image = image.add_text(
            text=f"ANCHOR: {anchor_name}",
            position=(x, y),
            size=16,
            color=(0, 0, 0, 255),
            anchor=anchor_code,
        )

    # Section 3: Combined Features Demo
    print("  Adding combined features...")

    # Create a card with text box and anchored elements
    card_x, card_y = 50, 400
    card_width, card_height = 800, 250

    # Card background
    image = image.draw_rectangle(
        card_x, card_y, card_width, card_height, (255, 255, 255, 255)
    )

    # Card border
    image = image.draw_rectangle(
        card_x, card_y, card_width, card_height, (200, 200, 200, 255)
    )

    # Card title (anchored to top-center of card)
    image = image.add_text(
        text="COMPREHENSIVE FEATURES DEMO",
        position=(card_x + card_width // 2, card_y + 30),
        size=24,
        color=(0, 0, 0, 255),
        anchor="mm",
    )

    # Card content (text box)
    content_text = """
    This demo showcases the integration of text box functionality with text anchoring.

    • Text boxes provide automatic text wrapping within defined boundaries
    • Text anchors allow precise positioning relative to specific points
    • Both features work together to create professional layouts

    The calculate_anchor_offset function enables precise text positioning,
    while the add_text_box function handles complex text layout requirements.
    """

    image = image.add_text_box(
        text=content_text.strip(),
        box=(card_x + 20, card_y + 70, card_width - 40, card_height - 90),
        size=14,
        color=(60, 60, 60, 255),
        background=(250, 250, 250, 255),
        align="left",
        vertical_align="top",
        line_spacing=1.3,
        overflow=False,
    )

    # Status indicator (bottom-right of card)
    image = image.add_text(
        text="STATUS: WORKING",
        position=(card_x + card_width - 20, card_y + card_height - 20),
        size=12,
        color=(0, 150, 0, 255),
        anchor="br",
    )

    return image


def main():
    """Main function to run the comprehensive demo"""
    print("Simple Comprehensive Demo - Text Box + Anchoring")
    print("=" * 55)

    try:
        # Create the demo
        result_image = create_comprehensive_demo()

        # Save the result
        output_path = os.path.join(
            os.path.dirname(__file__), "output", "simple_comprehensive_demo.png"
        )
        result_image.save(output_path)

        print("\n✅ Comprehensive demo completed successfully!")
        print(f"📁 Output saved to: {output_path}")
        print(f"🖼️  Image dimensions: {result_image.width}x{result_image.height}")

        print("\n📋 Features Demonstrated:")
        print("   • Text box with automatic wrapping and alignment")
        print("   • Text anchoring with visual indicators")
        print("   • Integration of both features in a card layout")
        print("   • Professional document-style formatting")
        print("   • Status indicators and precise positioning")

        print("\n🎯 Key Improvements in v0.3.6:")
        print("   • Fixed draw_text_box → add_text_box functionality")
        print("   • Implemented calculate_anchor_offset for all anchor positions")
        print("   • Enhanced text rendering with proper error handling")
        print("   • Clean compilation with minimal warnings")

    except Exception as e:
        print(f"\n❌ Error creating comprehensive demo: {e}")
        import traceback

        traceback.print_exc()
        return False

    return True


if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
