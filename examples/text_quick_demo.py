#!/usr/bin/env python3
"""
Quick text rendering demo for imgrs.

This script demonstrates the key text features in a fast, simple way.
"""

import imgrs
import os

def main():
    """Run a quick text demo."""
    print("Running quick text demo...")

    # Ensure output directory exists
    os.makedirs("examples/output", exist_ok=True)

    # Create a simple image
    img = imgrs.new("RGB", (400, 300), (255, 255, 255))

    # Basic text
    img = img.add_text("Hello imgrs!", (20, 20), size=24, color=(0, 0, 0, 255))

    # Styled text with outline
    img = img.add_text_styled(
        "Styled",
        (20, 60),
        size=28,
        color=(255, 255, 255, 255),
        outline=(0, 0, 0, 255, 1.0),
        background=(100, 149, 237, 255)
    )

    # Multi-line text
    img = img.add_text_multiline(
        "Multi-line\ntext demo",
        (20, 120),
        size=16,
        color=(0, 100, 0, 255),
        line_spacing=1.5
    )

    # Text with shadow
    img = img.add_text_styled(
        "Shadow",
        (200, 60),
        size=22,
        color=(255, 0, 0, 255),
        shadow=(2, 2, 128, 128, 128, 200)
    )

    # Show text measurements
    width, height = img.get_text_size("Test", 20)
    bbox = img.get_text_box("Test", 200, 120, 20)
    img = img.add_text(f"Size: {width}x{height}", 200, 150, size=12, color=(0, 0, 150, 255))

    # Save the result
    img.save("examples/output/text_quick_demo.png")
    print("✓ Quick text demo saved as examples/output/text_quick_demo.png")
    print("✓ Demonstrated: basic text, styled text, multi-line text, shadows, and measurements")

if __name__ == "__main__":
    main()