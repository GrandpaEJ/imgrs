import os

import imgrs


def test_add_text_font():
    print("Testing add_text with font_path...")

    # Create output directory
    os.makedirs("examples/output", exist_ok=True)

    # Create a blank image
    img = imgrs.new("RGB", (400, 200), (255, 255, 255))

    # Find a font to use
    font_path = None
    possible_fonts = ["examples/SVN-Suargie.woff2"]

    for p in possible_fonts:
        if os.path.exists(p):
            font_path = p
            break

    if font_path:
        print(f"Using font: {font_path}")
        # Test add_text with font_path
        img = img.add_text(
            "Hello Custom Font!",
            (20, 80),
            size=32,
            color=(0, 0, 0, 255),
            font_path=font_path,
        )
    else:
        print("No custom font found, testing API stability with None (default font)")
        # Test add_text without font_path (should still work)
        img = img.add_text(
            "Hello Default Font!", (20, 80), size=32, color=(0, 0, 0, 255)
        )

    output_path = "examples/output/text_add_text_font.png"
    img.save(output_path)
    print(f"Saved to {output_path}")


if __name__ == "__main__":
    test_add_text_font()
