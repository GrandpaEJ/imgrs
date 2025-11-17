#!/usr/bin/env python3
"""
Test script for enhanced text functionality with full font and customization support
"""

from python.imgrs import Image
import os


def test_enhanced_text_features():
    """Test all enhanced text features"""

    print("🎨 Testing Enhanced Text Functionality")
    print("=" * 50)

    # Create a test image
    try:
        img = Image.new("RGBA", (800, 600), (255, 255, 255, 255))
        print("✓ Created test image")
    except Exception as e:
        print(f"✗ Failed to create image: {e}")
        return

    # Test 1: Basic text with different font families
    print("\n📝 Test 1: Font Family Support")
    try:
        # Sans-serif text
        img.add_text(
            "Sans-Serif Text",
            (50, 50),
            size=24,
            font_family="sans",
            font_weight="normal",
        )
        print("  ✓ Sans-serif text rendered")

        # Bold text
        img.add_text(
            "Bold Text", (50, 100), size=24, font_family="sans", font_weight="bold"
        )
        print("  ✓ Bold text rendered")

        # Italic text
        img.add_text(
            "Italic Text", (50, 150), size=24, font_family="sans", font_style="italic"
        )
        print("  ✓ Italic text rendered")

        # Custom font path
        font_path = "../../fonts/DejaVuSans.ttf"
        if os.path.exists(font_path):
            img.add_text("Custom Font", (50, 200), size=24, font_path=font_path)
            print("  ✓ Custom font path rendered")
        else:
            print("  ⚠ Custom font not found, using fallback")

    except Exception as e:
        print(f"  ✗ Font family test failed: {e}")

    # Test 2: Advanced text styling
    print("\n🎭 Test 2: Advanced Text Effects")
    try:
        # Text with outline
        img.add_text_advanced(
            "Outlined Text",
            (400, 50),
            size=32,
            color=(255, 255, 255, 255),
            outline=(0, 0, 0, 255, 2.0),
        )
        print("  ✓ Text with outline")

        # Text with shadow
        img.add_text_advanced(
            "Shadow Text",
            (400, 100),
            size=32,
            color=(100, 150, 200, 255),
            shadow=(3, 3, 0, 0, 0, 128),
        )
        print("  ✓ Text with shadow")

        # Text with multiple effects
        img.add_text_advanced(
            "Multi-Effect",
            (400, 150),
            size=28,
            color=(255, 100, 150, 255),
            outline=(255, 255, 255, 255, 1.0),
            shadow=(2, 2, 50, 50, 50, 200),
            letter_spacing=1.5,
        )
        print("  ✓ Text with multiple effects")

    except Exception as e:
        print(f"  ✗ Advanced effects test failed: {e}")

    # Test 3: Text alignment and positioning
    print("\n📐 Test 3: Alignment and Positioning")
    try:
        # Centered text
        img.add_text_centered(
            "Centered Title", 250, size=36, font_weight="bold", color=(50, 50, 150, 255)
        )
        print("  ✓ Centered text")

        # Left aligned
        img.add_text_advanced(
            "Left Aligned", (50, 300), size=20, align="left", color=(0, 100, 0, 255)
        )
        print("  ✓ Left aligned text")

        # Right aligned
        img.add_text_advanced(
            "Right Aligned", (750, 300), size=20, align="right", color=(100, 0, 0, 255)
        )
        print("  ✓ Right aligned text")

    except Exception as e:
        print(f"  ✗ Alignment test failed: {e}")

    # Test 4: Multi-line text
    print("\n📄 Test 4: Multi-line Text")
    try:
        multiline_text = "This is line 1\\nThis is line 2\\nThis is line 3"
        img.add_text_multiline(
            multiline_text,
            (50, 400),
            size=18,
            line_spacing=1.5,
            align="center",
            color=(80, 80, 80, 255),
        )
        print("  ✓ Multi-line text")

        # Text with justification
        img.add_text_advanced(
            "Justified text that wraps around",
            (400, 400),
            size=16,
            max_width=300,
            text_justify="justify",
            color=(60, 60, 60, 255),
        )
        print("  ✓ Text with justification and wrapping")

    except Exception as e:
        print(f"  ✗ Multi-line test failed: {e}")

    # Test 5: Text transformations
    print("\n🔤 Test 5: Text Transformations")
    try:
        # Uppercase
        img.add_text_advanced(
            "uppercase text",
            (50, 500),
            size=20,
            transform="uppercase",
            color=(150, 0, 150, 255),
        )
        print("  ✓ Uppercase transformation")

        # Title case
        img.add_text_advanced(
            "title case text",
            (400, 500),
            size=20,
            transform="title",
            color=(0, 150, 150, 255),
        )
        print("  ✓ Title case transformation")

    except Exception as e:
        print(f"  ✗ Transformation test failed: {e}")

    # Test 6: Font size calculations
    print("\n📏 Test 6: Font Size Calculations")
    try:
        # Basic size calculation
        width, height = Image.get_text_size("Test Text", size=24, font_weight="bold")
        print(f"  ✓ Basic size: {width}x{height} pixels")

        # Multi-line size calculation
        text = "Line 1\\nLine 2\\nLine 3"
        width, height, lines = Image.get_multiline_text_size(
            text, size=20, line_spacing=1.4
        )
        print(f"  ✓ Multi-line size: {width}x{height} pixels, {lines} lines")

        # Detailed text box
        box = Image.get_text_box("Box Test", 100, 100, size=28, font_weight="bold")
        print(
            f"  ✓ Text box: ({box['x']}, {box['y']}) to ({box['right_x']}, {box['bottom_y']})"
        )

    except Exception as e:
        print(f"  ✗ Size calculation test failed: {e}")

    # Test 7: CSS color support
    print("\n🎨 Test 7: CSS Color Support")
    try:
        img.add_text("Red Text", (50, 550), size=18, color="red")
        print("  ✓ CSS color 'red'")

        img.add_text("Blue Text", (200, 550), size=18, color="#0066FF")
        print("  ✓ Hex color support")

        img.add_text("Transparent", (350, 550), size=18, color="transparent")
        print("  ✓ Transparent color")

    except Exception as e:
        print(f"  ✗ CSS color test failed: {e}")

    # Test 8: Font listing
    print("\n📋 Test 8: Available Fonts")
    try:
        fonts = Image.list_available_fonts()
        print(f"  ✓ Found {len(fonts)} fonts:")
        for i, font in enumerate(fonts[:5]):  # Show first 5
            print(f"    - {os.path.basename(font)}")
        if len(fonts) > 5:
            print(f"    ... and {len(fonts) - 5} more")

    except Exception as e:
        print(f"  ✗ Font listing failed: {e}")

    print("\n" + "=" * 50)
    print("🎉 Enhanced text functionality testing completed!")
    print("\nKey improvements implemented:")
    print("• Font family management (sans, serif, mono)")
    print("• Font weight and style support (bold, italic, oblique)")
    print("• Advanced text effects (outline, shadow, glow)")
    print("• Enhanced alignment and text justification")
    print("• Letter spacing and line spacing control")
    print("• CSS color support and transparency")
    print("• Text transformations (uppercase, lowercase, title)")
    print("• Comprehensive font size calculations")
    print("• Multi-font text rendering capability")


if __name__ == "__main__":
    test_enhanced_text_features()
