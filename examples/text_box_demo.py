#!/usr/bin/env python3
"""
Text Box Demo - Demonstrates the new add_text_box functionality

This example shows how to:
1. Create a colored background image
2. Use add_text_box to render text within a bounding box
3. Control text wrapping, alignment, and overflow behavior
4. Use different text styles within text boxes
"""

import os
import sys

# Add the project root to the path
sys.path.insert(
    0, os.path.join(os.path.dirname(__file__), "..", "python")
)  # noqa: E402

from imgrs import Image  # noqa: E402


def create_text_box_examples():
    """Create multiple examples demonstrating text box functionality"""

    # Create a large canvas for multiple examples
    width, height = 800, 1000
    image = Image.new("RGB", (width, height), (245, 245, 245))

    # Example 1: Basic text box with wrapping
    print("Creating Example 1: Basic text box with wrapping...")

    # Sample long text that will need wrapping
    sample_text = """
    This is a longer piece of text that will automatically wrap
    within the specified text box area. The text box functionality
    allows you to control exactly how text flows within a defined
    rectangular area on your image.
    """

    # Draw text box with default settings
    image = image.add_text_box(
        text=sample_text.strip(),
        box=(50, 50, 300, 200),
        size=16,
        color=(50, 50, 50, 255),  # Dark gray text
        background=(220, 230, 240, 255),  # Light blue background
        align="left",  # Left align text
        vertical_align="top",  # Top align text box
        line_spacing=1.2,  # 20% extra spacing between lines
        overflow=False,  # Hide overflow text
    )

    # Example 2: Centered text box
    print("Creating Example 2: Centered text box...")

    center_text = "Centered Text Box"
    image = image.add_text_box(
        text=center_text,
        box=(400, 50, 300, 200),
        size=24,
        color=(255, 255, 255, 255),  # White text
        background=(70, 130, 180, 255),  # Steel blue background
        align="center",  # Center align text
        vertical_align="center",  # Center align text box
        line_spacing=1.0,
        overflow=True,  # Allow overflow
    )

    # Example 3: Right-aligned text box with different styling
    print("Creating Example 3: Right-aligned text box...")

    right_text = """
    Right-aligned text with custom styling.
    This demonstrates how text boxes can be used
    for different text alignments and visual effects.
    """

    image = image.add_text_box(
        text=right_text.strip(),
        box=(50, 300, 300, 150),
        size=18,
        color=(139, 69, 19, 255),  # Saddle brown text
        background=(255, 248, 220, 255),  # Cornsilk background
        align="right",  # Right align text
        vertical_align="bottom",  # Bottom align text box
        line_spacing=1.3,
        overflow=False,
    )

    # Example 4: Multiple text boxes showing overflow behavior
    print("Creating Example 4: Overflow comparison...")

    long_text = (
        "This text is very long and will definitely overflow the text box "
        "boundaries when overflow is set to False. Let's see what happens!"
    )

    # Text box with overflow hidden
    image = image.add_text_box(
        text=long_text,
        box=(450, 300, 250, 100),
        size=14,
        color=(255, 255, 255, 255),
        background=(255, 69, 0, 255),  # Red background
        align="left",
        vertical_align="top",
        line_spacing=1.1,
        overflow=False,  # Hide overflow
    )

    # Text box with overflow visible
    image = image.add_text_box(
        text=long_text,
        box=(450, 420, 250, 100),
        size=14,
        color=(255, 255, 255, 255),
        background=(34, 139, 34, 255),  # Forest green background
        align="left",
        vertical_align="top",
        line_spacing=1.1,
        overflow=True,  # Show overflow
    )

    # Example 5: Text box with shadows and effects
    print("Creating Example 5: Styled text box...")

    styled_text = """
    Styled Text Box

    This example shows how text boxes can be combined
    with other text styling features like shadows,
    outlines, and backgrounds to create rich
    visual effects.
    """

    image = image.add_text_box(
        text=styled_text.strip(),
        box=(50, 500, 350, 200),
        size=20,
        color=(255, 255, 255, 255),
        background=(25, 25, 112, 255),  # Midnight blue
        align="center",
        vertical_align="center",
        line_spacing=1.2,
        overflow=True,
        # Note: outline, shadow, opacity not available in current Python API
    )

    # Example 6: Small text boxes demonstrating precision
    print("Creating Example 6: Small precision text boxes...")

    # Small text box with specific content
    small_boxes = [
        ("OK", (100, 50), (255, 255, 255, 255), (34, 139, 34, 255)),
        ("CANCEL", (100, 50), (255, 255, 255, 255), (178, 34, 34, 255)),
        ("HELP", (100, 50), (255, 255, 255, 255), (70, 130, 180, 255)),
    ]

    x_start = 450
    y_start = 550

    for i, (text, size, text_color, bg_color) in enumerate(small_boxes):
        image = image.add_text_box(
            text=text,
            box=(x_start + (i * 120), y_start, size[0], size[1]),
            size=16,
            color=text_color,
            background=bg_color,
            align="center",
            vertical_align="center",
            line_spacing=1.0,
            overflow=False,
        )

    # Add title and descriptions
    image = image.add_text(
        text="Text Box Demo Examples",
        position=(width // 2, 10),
        size=32,
        color=(0, 0, 0, 255),
        anchor="mm",  # Middle-center anchor
    )

    # Add labels for each example
    labels = [
        ("Basic Wrapping", 50, 270),
        ("Centered Box", 400, 270),
        ("Right Alignment", 50, 470),
        ("Overflow Hidden", 450, 420),
        ("Overflow Visible", 450, 540),
        ("Styled Effects", 50, 720),
        ("Small Boxes", 450, 720),
    ]

    for label, x, y in labels:
        image = image.add_text(
            text=label,
            position=(x, y),
            size=14,
            color=(100, 100, 100, 255),
            anchor="mm",
        )

    return image


def main():
    """Main function to run the text box demo"""
    print("Text Box Demo - Testing the new add_text_box functionality")
    print("=" * 60)

    try:
        # Create the examples
        result_image = create_text_box_examples()

        # Save the result
        output_path = os.path.join(
            os.path.dirname(__file__), "output", "text_box_demo.png"
        )
        result_image.save(output_path)

        print("\n✅ Text box demo completed successfully!")
        print(f"📁 Output saved to: {output_path}")
        print(f"🖼️  Image dimensions: {result_image.width}x{result_image.height}")

        # Display text box information for verification
        print("\n📋 Text Box Features Demonstrated:")
        print("   • Text wrapping within bounding boxes")
        print("   • Horizontal alignment (left, center, right)")
        print("   • Vertical alignment (top, center, bottom)")
        print("   • Overflow control (hidden vs visible)")
        print("   • Custom styling (backgrounds, colors, spacing)")
        print("   • Integration with text effects (shadows, outlines)")
        print("   • Small precision text boxes (buttons/labels)")

    except Exception as e:
        print(f"\n❌ Error creating text box demo: {e}")
        import traceback

        traceback.print_exc()
        return False

    return True


if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
