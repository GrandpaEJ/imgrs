#!/usr/bin/env python3
"""
Simple test script for WOFF2 font support.
"""

import os

import imgrs


def main():
    print("Running WOFF2 test...")

    # Ensure output directory exists
    os.makedirs("examples/output", exist_ok=True)

    # Font path
    font_path = "examples/SVN-Suargie.woff2"

    if not os.path.exists(font_path):
        print(f"Error: Font file not found at {font_path}")
        return

    # Create a nice background
    img = imgrs.new("RGB", (800, 400), (20, 20, 30))

    # Title
    img = img.add_text_styled(
        "WOFF2 Font Support",
        (50, 50),
        size=48,
        color=(255, 255, 255, 255),
        font_path=font_path,
        shadow=(4, 4, 0, 0, 0, 128),
    )

    # Subtitle
    img = img.add_text(
        "Rendering using UTMAvo.woff2",
        (50, 120),
        size=24,
        color=(200, 200, 200, 255),
        font_path=font_path,
    )

    # Paragraph
    text = """This text is rendered using a WOFF2 web font!
The font is automatically decompressed and loaded by imgrs.
It supports all standard text features like:
• Multi-line text
• Custom colors
• Variable sizes"""

    img = img.add_text_multiline(
        text,
        (50, 180),
        size=20,
        color=(150, 200, 255, 255),
        font_path=font_path,
        line_spacing=1.5,
    )

    # Large transparent text
    img = img.add_text_styled(
        "imgrs",
        (500, 250),
        size=120,
        color=(255, 255, 255, 30),
        font_path=font_path,
        rotation=-15.0,
    )

    output_path = "examples/output/woff2_test.png"
    img.save(output_path)
    print(f"✓ Saved test image to {output_path}")


if __name__ == "__main__":
    main()
