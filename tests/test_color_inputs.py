import pytest
from imgrs import Image


def test_color_input_formats():
    # Test tuple input (legacy support)
    img_tuple = Image.new("RGB", (100, 100), (255, 0, 0))
    assert img_tuple.getpixel(0, 0) == (255, 0, 0, 255)

    # Test hex string (6 digits)
    img_hex6 = Image.new("RGB", (100, 100), "#00FF00")
    assert img_hex6.getpixel(0, 0) == (0, 255, 0, 255)

    # Test hex string (8 digits)
    img_hex8 = Image.new("RGBA", (100, 100), "#0000FF80")
    assert img_hex8.getpixel(0, 0) == (0, 0, 255, 128)

    # Test hex string (no hash)
    img_no_hash = Image.new("RGB", (100, 100), "FF00FF")
    assert img_no_hash.getpixel(0, 0) == (255, 0, 255, 255)

    # Test short hex
    img_short = Image.new("RGB", (100, 100), "#F00")
    assert img_short.getpixel(0, 0) == (255, 0, 0, 255)


def test_draw_methods_with_color_input():
    img = Image.new("RGB", (100, 100), (0, 0, 0))

    # Draw rectangle with hex
    img_rect = img.draw_rectangle(10, 10, 20, 20, "#FF0000")
    # Debug print
    print(f"Pixel at 15,15: {img_rect.getpixel(15, 15)}")
    assert img_rect.getpixel(15, 15) == (255, 0, 0, 255)

    # Draw rectangle with tuple (control)
    img_rect_tuple = img.draw_rectangle(40, 40, 20, 20, (0, 0, 255, 255))
    assert img_rect_tuple.getpixel(45, 45) == (0, 0, 255, 255)

    # Draw circle with tuple
    img_circle = img.draw_circle(50, 50, 10, (0, 255, 0, 255))
    assert img_circle.getpixel(50, 50) == (0, 255, 0, 255)

    # Draw circle with hex
    img_circle_hex = img.draw_circle(80, 80, 10, "#FFFF00")
    assert img_circle_hex.getpixel(80, 80) == (255, 255, 0, 255)


def test_invalid_color_input():
    # Integer is valid (grayscale)
    img = Image.new("RGB", (100, 100), 123)
    assert img.getpixel(0, 0) == (123, 123, 123, 255)

    with pytest.raises(ValueError):
        Image.new("RGB", (100, 100), "#ZZZZZZ")
