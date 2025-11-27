#!/usr/bin/env python3
"""
Text rendering examples for imgrs.

This script demonstrates all the advanced text rendering features:
- Basic text rendering with flexible positioning
- Styled text with outlines, shadows, and backgrounds
- Multi-line text with custom line spacing
- Text measurement and bounding box calculations
"""

import imgrs


def demo_basic_text():
    """Demonstrate basic text rendering with different positioning methods."""
    print("Creating basic text demo...")

    # Create a white background image
    img = imgrs.new("RGB", (600, 400), (255, 255, 255))

    # Add text using tuple position
    img = img.add_text("Hello World!", (20, 20), size=32, color=(0, 0, 0, 255))

    # Add text using separate x,y parameters
    img = img.add_text("Separate positioning", 20, 70, size=24, color=(255, 0, 0, 255))

    # Add text with different colors and sizes
    img = img.add_text("Large Text", (20, 100), size=48, color=(0, 100, 0, 255))
    img = img.add_text("Small Text", (20, 160), size=16, color=(0, 0, 255, 255))

    img.save("examples/output/text_basic_demo.png")
    print("✓ Basic text demo saved as examples/output/text_basic_demo.png")


def demo_styled_text():
    """Demonstrate styled text with outlines, shadows, and backgrounds."""
    print("Creating styled text demo...")

    img = imgrs.new("RGB", (800, 600), (240, 240, 240))

    # Text with outline
    img = img.add_text_styled(
        "OUTLINED TEXT",
        (50, 50),
        size=36,
        color=(255, 255, 255, 255),
        outline=(0, 0, 0, 255, 2.0)  # Black outline, 2px width
    )

    # Text with shadow
    img = img.add_text_styled(
        "SHADOW TEXT",
        (50, 120),
        size=36,
        color=(255, 0, 0, 255),
        shadow=(3, 3, 128, 128, 128, 200)  # Gray shadow, offset by 3px
    )

    # Text with background
    img = img.add_text_styled(
        "BACKGROUND",
        (50, 190),
        size=32,
        color=(255, 255, 255, 255),
        background=(0, 100, 200, 255)  # Blue background
    )

    # Text with all effects combined
    img = img.add_text_styled(
        "FULL STYLE",
        (50, 250),
        size=40,
        color=(255, 215, 0, 255),  # Gold text
        outline=(139, 69, 19, 255, 1.5),  # Brown outline
        shadow=(2, 2, 105, 105, 105, 180),  # Dark gray shadow
        background=(25, 25, 112, 255)  # Midnight blue background
    )

    # Text with colored outline
    img = img.add_text_styled(
        "RAINBOW OUTLINE",
        (50, 320),
        size=28,
        color=(255, 255, 255, 255),
        outline=(255, 0, 255, 255, 3.0)  # Magenta outline
    )

    img.save("examples/output/text_styled_demo.png")
    print("✓ Styled text demo saved as examples/output/text_styled_demo.png")


def demo_multiline_text():
    """Demonstrate multi-line text rendering."""
    print("Creating multi-line text demo...")

    img = imgrs.new("RGB", (700, 500), (255, 250, 240))

    # Multi-line text with default line spacing
    img = img.add_text_multiline(
        "This is multi-line text\nwith default spacing\nand multiple lines",
        (30, 30),
        size=24,
        color=(0, 0, 0, 255)
    )

    # Multi-line text with custom line spacing
    img = img.add_text_multiline(
        "Tight spacing\nbetween lines\nmakes text\nmore compact",
        (30, 150),
        size=20,
        color=(0, 100, 0, 255),
        line_spacing=1.1  # Tighter spacing
    )

    # Multi-line text with wide line spacing
    img = img.add_text_multiline(
        "Wide spacing\nmakes text\neasier to read\nwith more\nvertical space",
        (30, 250),
        size=18,
        color=(0, 0, 150, 255),
        line_spacing=2.0  # Double spacing
    )

    # Multi-line text with styled background
    img = img.add_text_styled(
        "Multi-line\nwith background\nand outline",
        (400, 30),
        size=22,
        color=(255, 255, 255, 255),
        outline=(0, 0, 0, 255, 1.0),
        background=(100, 149, 237, 255)  # Cornflower blue
    )

    img.save("examples/output/text_multiline_demo.png")
    print("✓ Multi-line text demo saved as examples/output/text_multiline_demo.png")


def demo_text_measurement():
    """Demonstrate text measurement and bounding box functions."""
    print("Creating text measurement demo...")

    img = imgrs.new("RGB", (800, 600), (255, 255, 255))

    test_texts = [
        ("Small", 16),
        ("Medium", 24),
        ("Large", 40),
        ("Extra Large", 64),
        ("Hello World!", 32),
        ("Short", 48),
    ]

    y_offset = 50

    for text, size in test_texts:
        # Get text dimensions
        width, height = img.get_text_size(text, size)

        # Get bounding box
        bbox = img.get_text_box(text, 50, y_offset, size)

        # Draw bounding box (light gray background)
        img = img.add_text_styled(
            text,
            (50, y_offset),
            size=size,
            color=(0, 0, 0, 255),
            background=(240, 240, 240, 255)
        )

        # Add measurement info
        info_text = f"Size: {width}x{height}, BBox: {bbox}"
        img = img.add_text(info_text, 50, y_offset + height + 10, size=12, color=(100, 100, 100, 255))

        y_offset += height + 60

    img.save("examples/output/text_measurement_demo.png")
    print("✓ Text measurement demo saved as examples/output/text_measurement_demo.png")


def demo_text_composition():
    """Demonstrate combining text with other image operations."""
    print("Creating text composition demo...")

    # Start with a gradient background
    img = imgrs.new("RGB", (800, 600), (255, 255, 255))

    # Add some shapes for context
    img = img.draw_rectangle(50, 50, 700, 500, (240, 248, 255, 255))  # Light blue background
    img = img.draw_circle(200, 150, 80, (255, 200, 200, 255))  # Light red circle
    img = img.draw_circle(600, 150, 80, (200, 255, 200, 255))  # Light green circle
    img = img.draw_rectangle(150, 400, 500, 100, (200, 200, 255, 255))  # Light blue rectangle

    # Add various text elements
    img = img.add_text_styled(
        "TEXT COMPOSITION",
        (250, 80),
        size=36,
        color=(255, 255, 255, 255),
        outline=(0, 0, 0, 255, 2.0),
        shadow=(3, 3, 128, 128, 128, 200),
        background=(70, 130, 180, 255)  # Steel blue
    )

    # Add descriptive text
    img = img.add_text_multiline(
        "This demonstrates\ntext rendering\ncombined with\nother drawing\noperations",
        (100, 200),
        size=18,
        color=(0, 0, 0, 255),
        line_spacing=1.5
    )

    # Add labels for shapes
    img = img.add_text("Circle", 170, 250, size=16, color=(139, 0, 0, 255))
    img = img.add_text("Rectangle", 300, 520, size=16, color=(0, 0, 139, 255))

    # Add a signature-style text
    img = img.add_text_styled(
        "imgrs v0.3.1",
        (600, 550),
        size=14,
        color=(128, 128, 128, 255),
        outline=(64, 64, 64, 255, 0.5)
    )

    img.save("examples/output/text_composition_demo.png")
    print("✓ Text composition demo saved as examples/output/text_composition_demo.png")


def main():
    """Run all text demos."""
    print("Running imgrs text rendering demos...")
    print("=" * 50)

    # Ensure output directory exists
    import os
    os.makedirs("examples/output", exist_ok=True)

    demo_basic_text()
    demo_styled_text()
    demo_multiline_text()
    demo_text_measurement()
    demo_text_composition()

    print("=" * 50)
    print("All text demos completed!")
    print("Check the examples/output/ directory for the generated images.")


if __name__ == "__main__":
    main()