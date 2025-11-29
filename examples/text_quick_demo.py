#!/usr/bin/env python3
"""
Quick text rendering demo for imgrs TextMixin.

This script demonstrates the key TextMixin features in a fast, simple way.
Showcases the new text functionality with minimal code.
"""

import os

import imgrs


def main():
    """Run a quick text demo."""
    print("Running quick TextMixin demo...")

    # Ensure output directory exists
    os.makedirs("examples/output", exist_ok=True)

    # Create a simple background
    img = imgrs.new("RGB", (500, 400), (250, 250, 250))

    # Basic text with flexible positioning
    img = img.add_text("Hello TextMixin!", (20, 20), size=24, color=(0, 0, 0, 255))

    # Styled text with outline and background
    img = img.add_text(
        "Styled",
        (20, 60),
        size=28,
        color=(255, 255, 255, 255),
    )

    # Multi-line text with center alignment
    img = img.add_text_multiline(
        "Multi-line\ncentered text",
        (250, 120),
        size=16,
        color=(0, 100, 0, 255),
        align="center",
        line_spacing=1.5,
    )

    # Text with shadow using convenience method
    img = img.add_text_with_shadow(
        "Shadow Effect",
        (20, 180),
        size=22,
        color=(255, 0, 0, 255),
        shadow_color=(0, 0, 0, 180),
        shadow_offset=(2, 2),
    )

    # Centered text with styling
    img = img.add_text_centered(
        "AUTO CENTERED",
        240,
        size=20,
        color=(0, 0, 150, 255),
        background=(255, 255, 200, 255),
    )

    # Text with outline using convenience method
    img = img.add_text_with_outline(
        "Outlined",
        (300, 180),
        size=18,
        color=(255, 255, 0, 255),
        outline_color=(255, 0, 255, 255),
        outline_width=1.5,
    )

    # Show text measurements
    test_text = "Measurements"
    width, height, ascent, descent = img.get_text_dimensions(test_text, 16)
    bbox = img.get_text_bounding_box(test_text, 20, 300, 16)

    img = img.add_text(
        f"Text: '{test_text}'", 20, 280, size=12, color=(100, 100, 100, 255)
    )
    img = img.add_text(
        f"Size: {width}x{height}, Ascent: {ascent}",
        20,
        300,
        size=12,
        color=(100, 100, 100, 255),
    )
    img = img.add_text(
        f"Baseline Y: {bbox['baseline_y']}",
        20,
        320,
        size=12,
        color=(100, 100, 100, 255),
    )

    # Save the result
    img.save("examples/output/text_quick_demo.png")
    print("✓ Quick TextMixin demo saved as examples/output/text_quick_demo.png")
    print("✓ Demonstrated: basic text, styled text, multi-line text, centered text,")
    print("  shadows, outlines, backgrounds, measurements, and bounding boxes")


if __name__ == "__main__":
    main()
