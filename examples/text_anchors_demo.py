#!/usr/bin/env python3
"""
Text Anchoring Demo - Demonstrates the calculate_anchor_offset functionality

This example shows how to:
1. Use different text anchor positions (TopLeft, TopCenter, BottomRight, etc.)
2. Understand how anchor points affect text positioning
3. Create visual demonstrations of anchor behavior
4. Use anchors for precise text placement
"""

import os
import sys

# Add the project root to the path
sys.path.insert(
    0, os.path.join(os.path.dirname(__file__), "..", "python")
)  # noqa: E402

from imgrs import Image  # noqa: E402


def create_anchor_examples():
    """Create examples demonstrating different anchor positions"""

    # Create a canvas showing all anchor positions
    width, height = 1000, 1200
    image = Image.new("RGB", (width, height), (250, 250, 250))

    # Define anchor positions and their visual representations
    anchor_positions = [
        # Top row
        ("tl", "TopLeft", 100, 100, (255, 255, 255, 255), (220, 20, 60, 255)),
        ("tm", "TopCenter", 500, 100, (255, 255, 255, 255), (30, 144, 255, 255)),
        ("tr", "TopRight", 900, 100, (255, 255, 255, 255), (255, 140, 0, 255)),
        # Middle row
        ("ml", "MiddleLeft", 100, 400, (255, 255, 255, 255), (50, 205, 50, 255)),
        ("mm", "MiddleCenter", 500, 400, (255, 255, 255, 255), (148, 0, 211, 255)),
        ("mr", "MiddleRight", 900, 400, (255, 255, 255, 255), (220, 20, 60, 255)),
        # Bottom row
        ("bl", "BottomLeft", 100, 700, (255, 255, 255, 255), (255, 20, 147, 255)),
        ("bm", "BottomCenter", 500, 700, (255, 255, 255, 255), (0, 191, 255, 255)),
        ("br", "BottomRight", 900, 700, (255, 255, 255, 255), (255, 99, 71, 255)),
        # Baseline row
        ("sl", "BaselineLeft", 100, 1000, (255, 255, 255, 255), (34, 139, 34, 255)),
        ("sm", "BaselineCenter", 500, 1000, (255, 255, 255, 255), (72, 61, 139, 255)),
        ("sr", "BaselineRight", 900, 1000, (255, 255, 255, 255), (255, 69, 0, 255)),
    ]

    print("Creating anchor position examples...")

    # Draw each anchor example
    for anchor_code, anchor_name, x, y, text_color, bg_color in anchor_positions:
        print(f"  Creating {anchor_name} ({anchor_code}) at ({x}, {y})")

        # Create a background rectangle for the anchor point
        rect_size = 60
        image = image.draw_rectangle(
            x - rect_size // 2, y - rect_size // 2, rect_size, rect_size, bg_color
        )

        # Draw the anchor point indicator (crosshair)
        cross_color = (255, 255, 255, 255)
        cross_size = 8

        # Vertical line
        image = image.draw_line(x, y - cross_size, x, y + cross_size, cross_color)

        # Horizontal line
        image = image.draw_line(x - cross_size, y, x + cross_size, y, cross_color)

        # Draw text with the specific anchor
        sample_text = "Anchor"
        image = image.add_text(
            text=sample_text,
            position=(x, y),
            size=24,
            color=(0, 0, 0, 255),  # Black text
            anchor=anchor_code,  # Use the specific anchor code
        )

        # Add label below each anchor
        image = image.add_text(
            text=anchor_name,
            position=(x, y + 80),
            size=14,
            color=(80, 80, 80, 255),
            anchor="mm",  # Center the labels
        )

    # Add title
    image = image.add_text(
        text="Text Anchor Position Examples",
        position=(width // 2, 20),
        size=36,
        color=(0, 0, 0, 255),
        anchor="mm",
    )

    # Add legend explaining anchor codes
    legend_text = """
Anchor Codes:
tl = TopLeft, tm = TopCenter, tr = TopRight
ml = MiddleLeft, mm = MiddleCenter, mr = MiddleRight
bl = BottomLeft, bm = BottomCenter, br = BottomRight
sl = BaselineLeft, sm = BaselineCenter, sr = BaselineRight

The crosshair shows the anchor point position.
Text is positioned relative to this anchor point.
    """

    image = image.add_text_box(
        text=legend_text.strip(),
        box=(50, 1100, 900, 80),
        size=12,
        color=(60, 60, 60, 255),
        background=(240, 240, 240, 255),
        align="left",
        vertical_align="top",
        line_spacing=1.2,
        overflow=False,
    )

    return image


def create_anchor_comparison():
    """Create a comparison showing the difference between anchors"""

    width, height = 800, 600
    image = Image.new("RGB", (width, height), (255, 255, 255))

    # Sample text for comparison
    sample_text = "SAMPLE TEXT"

    # Draw background grid for reference
    grid_color = (200, 200, 200, 255)
    for x in range(0, width, 50):
        image = image.draw_line(x, 0, x, height, grid_color)
    for y in range(0, height, 50):
        image = image.draw_line(0, y, width, y, grid_color)

    center_x, center_y = width // 2, height // 2

    # Example comparing TopLeft vs BottomRight anchors
    print("Creating anchor comparison...")

    # TopLeft anchor (traditional behavior)
    image = image.add_text_styled(
        text=sample_text,
        position=(center_x, center_y),
        size=32,
        color=(255, 255, 255, 255),
        background=(255, 0, 0, 255),  # Red background
        anchor="tl",  # TopLeft anchor
    )

    # BottomRight anchor (opposite corner)
    image = image.add_text_styled(
        text=sample_text,
        position=(center_x, center_y),
        size=32,
        color=(255, 255, 255, 255),
        background=(0, 0, 255, 255),  # Blue background
        anchor="br",  # BottomRight anchor
    )

    # Add center point indicator
    image = image.draw_circle(center_x, center_y, 5, (0, 255, 0, 255))  # Green dot

    # Add explanation
    explanation = """
This demonstrates anchor positioning:

Red text: Uses TopLeft anchor
- Text starts at the center point
- Extends to the right and down

Blue text: Uses BottomRight anchor
- Text ends at the center point
- Extends to the left and up

Green dot: Center point (x=400, y=300)
    """

    image = image.add_text_box(
        text=explanation.strip(),
        box=(50, 450, 700, 120),
        size=14,
        color=(0, 0, 0, 255),
        background=(255, 255, 255, 200),
        align="left",
        vertical_align="top",
        line_spacing=1.3,
        overflow=False,
    )

    return image


def create_practical_anchor_examples():
    """Create practical examples showing when to use different anchors"""

    width, height = 900, 600
    image = Image.new("RGB", (width, height), (245, 245, 245))

    print("Creating practical anchor usage examples...")

    # Example 1: UI Button with centered text
    button_x, button_y = 100, 80
    button_width, button_height = 150, 50

    # Button background
    image = image.draw_rectangle(
        button_x, button_y, button_width, button_height, (70, 130, 180, 255)
    )

    # Button text centered (using mm anchor)
    image = image.add_text(
        text="CLICK ME",
        position=(button_x + button_width // 2, button_y + button_height // 2),
        size=18,
        color=(255, 255, 255, 255),
        anchor="mm",  # MiddleCenter anchor for perfect centering
    )

    # Example 2: Watermark with bottom-right corner
    watermark_text = "CONFIDENTIAL"

    image = image.add_text(
        text=watermark_text,
        position=(width - 20, height - 20),
        size=16,
        color=(128, 128, 128, 128),  # Semi-transparent
        anchor="br",  # BottomRight anchor
    )

    # Example 3: Image caption with top-center
    caption_text = "Beautiful Landscape Photography"

    image = image.add_text(
        text=caption_text,
        position=(width // 2, 20),
        size=20,
        color=(255, 255, 255, 255),
        anchor="tm",  # TopCenter anchor
    )

    # Example 4: Corner label with top-left
    corner_label = "NEW!"

    image = image.add_text(
        text=corner_label,
        position=(20, 20),
        size=16,
        color=(255, 255, 255, 255),
        anchor="tl",  # TopLeft anchor
    )

    # Example 5: Signature with bottom-left
    signature = "- John Doe, Photographer"

    image = image.add_text(
        text=signature,
        position=(20, height - 20),
        size=14,
        color=(80, 80, 80, 255),
        anchor="bl",  # BottomLeft anchor
    )

    # Add title
    image = image.add_text(
        text="Practical Anchor Usage Examples",
        position=(width // 2, 10),
        size=28,
        color=(0, 0, 0, 255),
        anchor="tm",
    )

    # Add descriptions
    descriptions = [
        (
            "Button Text (mm)",
            button_x + button_width // 2,
            button_y + button_height + 20,
        ),
        ("Watermark (br)", width - 20, height - 40),
        ("Caption (tm)", width // 2, 50),
        ("Corner Label (tl)", 20, 50),
        ("Signature (bl)", 20, height - 40),
    ]

    for desc, x, y in descriptions:
        image = image.add_text(
            text=desc,
            position=(x, y),
            size=12,
            color=(100, 100, 100, 255),
            anchor="mm",
        )

    return image


def main():
    """Main function to run the anchoring demo"""
    print("Text Anchoring Demo - Testing the calculate_anchor_offset functionality")
    print("=" * 70)

    try:
        # Create the anchor position examples
        anchor_image = create_anchor_examples()
        output_path1 = os.path.join(
            os.path.dirname(__file__), "output", "text_anchors_positions.png"
        )
        anchor_image.save(output_path1)
        print(f"✅ Anchor positions example saved to: {output_path1}")

        # Create anchor comparison
        comparison_image = create_anchor_comparison()
        output_path2 = os.path.join(
            os.path.dirname(__file__), "output", "text_anchors_comparison.png"
        )
        comparison_image.save(output_path2)
        print(f"✅ Anchor comparison saved to: {output_path2}")

        # Create practical examples
        practical_image = create_practical_anchor_examples()
        output_path3 = os.path.join(
            os.path.dirname(__file__), "output", "text_anchors_practical.png"
        )
        practical_image.save(output_path3)
        print(f"✅ Practical anchor examples saved to: {output_path3}")

        print("\n📋 Anchor Features Demonstrated:")
        print(
            "   • All 12 anchor positions (tl, tm, tr, ml, mm, mr, bl, bm, br, sl, sm, sr)"
        )
        print("   • Visual crosshair indicators showing anchor points")
        print("   • Side-by-side comparison of different anchors")
        print("   • Practical UI examples (buttons, watermarks, captions)")
        print("   • Precise text positioning using anchor codes")
        print("   • Integration with background colors and styling")

        print("\n🎯 Anchor Usage Guidelines:")
        print("   • tl (TopLeft): Use for corner labels, badges")
        print("   • tm (TopCenter): Use for titles, headers, captions")
        print("   • tr (TopRight): Use for corner notifications")
        print("   • mm (MiddleCenter): Use for buttons, centered content")
        print("   • br (BottomRight): Use for watermarks, timestamps")
        print("   • bl (BottomLeft): Use for signatures, credits")

    except Exception as e:
        print(f"\n❌ Error creating anchoring demo: {e}")
        import traceback

        traceback.print_exc()
        return False

    return True


if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
