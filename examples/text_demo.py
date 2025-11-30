#!/usr/bin/env python3
"""
Text rendering examples for imgrs TextMixin.

This script demonstrates all the advanced text rendering features available through the TextMixin:
- Basic text rendering with flexible positioning
- Custom font support (TTF, OTF, WOFF, WOFF2)
- Styled text with outlines, shadows, backgrounds, and opacity
- Multi-line text with custom alignment and line spacing
- Centered text rendering
- Text measurement and bounding box calculations
- Convenience methods for common effects (shadow, outline, background)
"""

import imgrs


def demo_basic_text():
    """Demonstrate basic text rendering with different positioning methods."""
    print("Creating basic text demo...")

    # Create a gradient background
    img = imgrs.new("RGB", (600, 400), (240, 248, 255))

    # Add text using tuple position (recommended)
    img = img.add_text("Hello TextMixin!", (20, 20), size=32, color=(0, 0, 0, 255))

    # Add text using separate x,y parameters
    img = img.add_text("Flexible positioning", 20, 70, size=24, color=(255, 0, 0, 255))

    # Add text with different colors and sizes
    img = img.add_text("Large Text", (20, 100), size=48, color=(0, 100, 0, 255))
    img = img.add_text("Small Text", (20, 160), size=16, color=(0, 0, 255, 255))

    # Add text with custom font (if available)
    img = img.add_text(
        "Custom font support", (20, 190), size=20, color=(128, 0, 128, 255)
    )

    img.save("examples/output/text/text_basic_demo.png")
    print("✓ Basic text demo saved as examples/output/text/text_basic_demo.png")


def demo_styled_text():
    """Demonstrate styled text with all formatting options."""
    print("Creating styled text demo...")

    img = imgrs.new("RGB", (800, 600), (250, 250, 250))

    # Text with outline effect
    img = img.add_text_styled(
        "OUTLINED TEXT",
        (50, 50),
        size=36,
        color=(255, 255, 255, 255),
        outline=(0, 0, 0, 255, 2.0),  # Black outline, 2px width
    )

    # Text with drop shadow
    img = img.add_text_styled(
        "SHADOW TEXT",
        (50, 120),
        size=36,
        color=(255, 0, 0, 255),
        shadow=(3, 3, 128, 128, 128, 200),  # Gray shadow, offset by 3px
    )

    # Text with background rectangle
    img = img.add_text_styled(
        "BACKGROUND",
        (50, 190),
        size=32,
        color=(255, 255, 255, 255),
        background=(0, 100, 200, 255),  # Blue background
    )

    # Text with opacity
    img = img.add_text_styled(
        "50% OPACITY",
        (50, 240),
        size=28,
        color=(0, 150, 0, 128),  # Semi-transparent green
        opacity=0.5,
    )

    # Text with all effects combined
    img = img.add_text_styled(
        "FULL STYLE",
        (50, 290),
        size=40,
        color=(255, 215, 0, 255),  # Gold text
        outline=(139, 69, 19, 255, 1.5),  # Brown outline
        shadow=(2, 2, 105, 105, 105, 180),  # Dark gray shadow
        background=(25, 25, 112, 255),  # Midnight blue background
        opacity=0.9,
    )

    # Text with text wrapping (max_width)
    img = img.add_text_styled(
        "This text will wrap when it exceeds the maximum width specified",
        (400, 50),
        size=20,
        color=(0, 0, 0, 255),
        max_width=200,  # Wrap at 200 pixels
    )

    # Text with alignment
    img = img.add_text_styled(
        "CENTER\nALIGNED\nTEXT",
        (500, 150),
        size=24,
        color=(0, 0, 150, 255),
        align="center",
    )

    img.save("examples/output/text/text_styled_demo.png")
    print("✓ Styled text demo saved as examples/output/text/text_styled_demo.png")


def demo_multiline_text():
    """Demonstrate multi-line text rendering with alignment."""
    print("Creating multi-line text demo...")

    img = imgrs.new("RGB", (700, 500), (255, 250, 240))

    # Left-aligned multi-line text (default)
    img = img.add_text_multiline(
        "Left aligned text\nspans multiple lines\nwith default spacing",
        (30, 30),
        size=24,
        color=(0, 0, 0, 255),
    )

    # Center-aligned multi-line text
    img = img.add_text_multiline(
        "Center aligned\nmulti-line text\nlooks great",
        (250, 30),
        size=22,
        color=(0, 100, 0, 255),
        align="center",
    )

    # Right-aligned multi-line text
    img = img.add_text_multiline(
        "Right aligned text\nis useful for\ncertain layouts",
        (470, 30),
        size=20,
        color=(0, 0, 150, 255),
        align="right",
    )

    # Multi-line text with custom line spacing
    img = img.add_text_multiline(
        "Tight spacing\nmakes text\nmore compact",
        (30, 150),
        size=20,
        color=(150, 0, 0, 255),
        line_spacing=1.1,  # Tighter spacing
    )

    # Multi-line text with wide line spacing
    img = img.add_text_multiline(
        "Wide spacing\nmakes text\neasier to read",
        (30, 220),
        size=18,
        color=(0, 100, 100, 255),
        line_spacing=2.0,  # Double spacing
    )

    # Multi-line text with styled background
    img = img.add_text_styled(
        "Multi-line\nwith background\nand outline",
        (400, 150),
        size=22,
        color=(255, 255, 255, 255),
        outline=(0, 0, 0, 255, 1.0),
        background=(100, 149, 237, 255),  # Cornflower blue
    )

    img.save("examples/output/text/text_multiline_demo.png")
    print(
        "✓ Multi-line text demo saved as examples/output/text/text_multiline_demo.png"
    )


def demo_centered_text():
    """Demonstrate centered text rendering."""
    print("Creating centered text demo...")

    img = imgrs.new("RGB", (600, 400), (255, 255, 255))

    # Add some visual guides
    img = img.draw_line(
        0, 200, 600, 200, (200, 200, 200, 255)
    )  # Horizontal center line
    img = img.draw_line(300, 0, 300, 400, (200, 200, 200, 255))  # Vertical center line

    # Basic centered text
    img = img.add_text_centered("CENTERED TEXT", 180, size=32, color=(0, 0, 0, 255))

    # Centered text with styling
    img = img.add_text_centered(
        "STYLED CENTERED",
        220,
        size=28,
        color=(255, 255, 255, 255),
        outline=(0, 0, 0, 255, 1.5),
        background=(70, 130, 180, 255),  # Steel blue
    )

    # Centered text with shadow
    img = img.add_text_centered(
        "SHADOW CENTERED",
        260,
        size=24,
        color=(255, 0, 0, 255),
        shadow=(2, 2, 128, 128, 128, 180),
    )

    # Centered multi-line text
    img = img.add_text_centered(
        "Multi-line\ncentered text\nworks perfectly",
        300,
        size=18,
        color=(0, 100, 0, 255),
    )

    img.save("examples/output/text/text_centered_demo.png")
    print("✓ Centered text demo saved as examples/output/text/text_centered_demo.png")


def demo_convenience_methods():
    """Demonstrate convenience methods for common text effects."""
    print("Creating convenience methods demo...")

    img = imgrs.new("RGB", (800, 500), (245, 245, 245))

    # Text with shadow using convenience method
    img = img.add_text_with_shadow(
        "SHADOW TEXT",
        (50, 50),
        size=32,
        color=(255, 0, 0, 255),
        shadow_color=(0, 0, 0, 180),
        shadow_offset=(3, 3),
    )

    # Text with outline using convenience method
    img = img.add_text_with_outline(
        "OUTLINE TEXT",
        (50, 120),
        size=32,
        color=(255, 255, 255, 255),
        outline_color=(255, 0, 255, 255),
        outline_width=2.0,
    )

    # Text with background using convenience method
    img = img.add_text_with_background(
        "BACKGROUND TEXT",
        (50, 190),
        size=28,
        color=(255, 255, 255, 255),
        background_color=(0, 100, 200, 255),
    )

    # Combine multiple convenience methods
    img = img.add_text_with_shadow(
        "SHADOW + OUTLINE",
        (400, 50),
        size=24,
        color=(255, 255, 0, 255),
        shadow_color=(128, 128, 128, 200),
        shadow_offset=(2, 2),
    )
    img = img.add_text_with_outline(
        "SHADOW + OUTLINE",
        (400, 50),
        size=24,
        color=(255, 255, 0, 255),
        outline_color=(0, 0, 0, 255),
        outline_width=1.0,
    )

    # Show that these are just convenience methods - you can achieve the same with add_text_styled
    img = img.add_text_styled(
        "MANUAL STYLE",
        (400, 120),
        size=24,
        color=(255, 255, 0, 255),
        outline=(0, 0, 0, 255, 1.0),
        shadow=(2, 2, 128, 128, 128, 200),
    )

    img.save("examples/output/text/text_convenience_demo.png")
    print(
        "✓ Convenience methods demo saved as examples/output/text/text_convenience_demo.png"
    )


def demo_text_measurement():
    """Demonstrate text measurement and bounding box functions."""
    print("Creating text measurement demo...")

    img = imgrs.new("RGB", (800, 600), (255, 255, 255))

    test_cases = [
        ("Small", 16),
        ("Medium", 24),
        ("Large", 40),
        ("Hello World!", 32),
        ("Short", 48),
        ("Very long text for measurement", 20),
    ]

    y_offset = 50

    for text, size in test_cases:
        # Get text dimensions
        width, height, ascent, descent = img.get_text_dimensions(text, size)

        # Get bounding box
        bbox = img.get_text_bounding_box(text, 50, y_offset, size)

        # Draw bounding box background
        img = img.draw_rectangle(
            45, y_offset - ascent, width + 10, height + 10, (240, 240, 240, 255)
        )

        # Add the text
        img = img.add_text(text, 50, y_offset, size=size, color=(0, 0, 0, 255))

        # Add measurement info
        info_lines = [
            f"Size: {width}x{height}",
            f"Ascent: {ascent}, Descent: {descent}",
            f"Baseline Y: {bbox['baseline_y']}",
            f"Bottom Y: {bbox['bottom_y']}",
        ]

        for i, line in enumerate(info_lines):
            img = img.add_text(
                line,
                400,
                y_offset + (i * 18) - ascent + 10,
                size=11,
                color=(100, 100, 100, 255),
            )

        y_offset += height + 40

    img.save("examples/output/text/text_measurement_demo.png")
    print(
        "✓ Text measurement demo saved as examples/output/text/text_measurement_demo.png"
    )


def demo_text_composition():
    """Demonstrate combining text with other image operations."""
    print("Creating text composition demo...")

    # Start with a gradient-like background using drawing operations
    img = imgrs.new("RGB", (800, 600), (255, 255, 255))

    # Add some shapes for context
    img = img.draw_rectangle(
        50, 50, 700, 500, (240, 248, 255, 255)
    )  # Light blue background
    img = img.draw_circle(200, 150, 80, (255, 200, 200, 255))  # Light red circle
    img = img.draw_circle(600, 150, 80, (200, 255, 200, 255))  # Light green circle
    img = img.draw_rectangle(
        150, 400, 500, 100, (200, 200, 255, 255)
    )  # Light blue rectangle

    # Add various text elements using TextMixin
    img = img.add_text_styled(
        "TEXT COMPOSITION",
        (250, 80),
        size=36,
        color=(255, 255, 255, 255),
        outline=(0, 0, 0, 255, 2.0),
        shadow=(3, 3, 128, 128, 128, 200),
        background=(70, 130, 180, 255),  # Steel blue
    )

    # Add descriptive text
    img = img.add_text_multiline(
        "This demonstrates\ntext rendering\ncombined with\nother drawing\noperations",
        (100, 200),
        size=18,
        color=(0, 0, 0, 255),
        line_spacing=1.5,
    )

    # Add labels for shapes
    img = img.add_text("Circle", 170, 250, size=16, color=(139, 0, 0, 255))
    img = img.add_text("Rectangle", 300, 520, size=16, color=(0, 0, 139, 255))

    # Add centered text
    img = img.add_text_centered(
        "Centered Title",
        320,
        size=24,
        color=(0, 0, 0, 255),
        background=(255, 255, 200, 255),
    )

    # Add a signature-style text
    img = img.add_text_styled(
        "TextMixin Demo",
        (600, 550),
        size=14,
        color=(128, 128, 128, 255),
        outline=(64, 64, 64, 255, 0.5),
    )

    img.save("examples/output/text/text_composition_demo.png")
    print(
        "✓ Text composition demo saved as examples/output/text/text_composition_demo.png"
    )


def demo_font_formats():
    """Demonstrate custom font loading with different formats."""
    print("Creating font formats demo...")

    img = imgrs.new("RGB", (800, 600), (255, 255, 255))

    # Title
    img = img.add_text_styled(
        "FONT FORMAT SUPPORT",
        (50, 30),
        size=36,
        color=(0, 0, 0, 255),
        outline=(100, 100, 100, 255, 1.0),
    )

    # Information text
    info_text = """imgrs supports multiple font formats:
• TTF (TrueType Font)
• OTF (OpenType Font)
• WOFF (Web Open Font Format)
• WOFF2 (Web Open Font Format 2)

Fonts are automatically detected and converted as needed."""

    img = img.add_text_multiline(
        info_text,
        (50, 100),
        size=18,
        color=(60, 60, 60, 255),
        line_spacing=1.4,
    )

    # Example with default font
    img = img.add_text_styled(
        "Default Embedded Font (DejaVu Sans)",
        (50, 320),
        size=24,
        color=(0, 100, 200, 255),
    )

    # Note about custom fonts
    note_text = """To use custom fonts, pass the font_path parameter:

img.add_text("Hello", (x, y), font_path="path/to/font.ttf")
img.add_text_styled("Styled", (x, y), font_path="font.woff2")

The font format is automatically detected from the file
extension or magic bytes."""

    img = img.add_text_multiline(
        note_text,
        (50, 360),
        size=14,
        color=(80, 80, 80, 255),
        line_spacing=1.5,
    )

    # Try to load the user-provided WOFF2 font first, then system fonts
    custom_fonts_to_try = [
        "examples/SVN-Suargie.woff2",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
        "/System/Library/Fonts/Helvetica.ttc",
        "C:\\Windows\\Fonts\\arial.ttf",
    ]

    custom_font_path = None
    for font_path in custom_fonts_to_try:
        import os

        if os.path.exists(font_path):
            custom_font_path = font_path
            break

    if custom_font_path:
        font_name = os.path.basename(custom_font_path)
        print(f"Loading custom font: {font_name}")

        img = img.add_text(
            f"Custom font loaded: {font_name}",
            (50, 500),
            size=24,
            color=(0, 150, 0, 255),
            font_path=custom_font_path,
        )

        # Add another line to show it off better
        img = img.add_text_styled(
            "Testing WOFF2 Support!",
            (50, 540),
            size=32,
            color=(255, 100, 0, 255),
            font_path=custom_font_path,
            shadow=(2, 2, 0, 0, 0, 100),
        )
    else:
        img = img.add_text(
            "No system fonts found - using default embedded font",
            (50, 540),
            size=14,
            color=(150, 150, 0, 255),
        )

    img.save("examples/output/text/text_font_formats_demo.png")
    print(
        "✓ Font formats demo saved as examples/output/text/text_font_formats_demo.png"
    )


def main():
    """Run all text demos."""
    print("Running imgrs TextMixin demos...")
    print("=" * 50)

    # Ensure output directory exists
    import os

    os.makedirs("examples/output/text", exist_ok=True)

    demo_basic_text()
    demo_styled_text()
    demo_multiline_text()
    demo_centered_text()
    demo_convenience_methods()
    demo_text_measurement()
    demo_text_composition()
    demo_font_formats()

    print("=" * 50)
    print("All TextMixin demos completed!")
    print("Check the examples/output/text/ directory for the generated images.")
    print("\nFeatures demonstrated:")
    print("• Basic text rendering with flexible positioning")
    print("• Custom font support (TTF, OTF, WOFF, WOFF2)")
    print("• Styled text with outlines, shadows, backgrounds, opacity")
    print("• Multi-line text with alignment and custom spacing")
    print("• Centered text rendering")
    print("• Text measurement and bounding box calculations")
    print("• Convenience methods for common effects")
    print("• Integration with other drawing operations")


if __name__ == "__main__":
    main()
