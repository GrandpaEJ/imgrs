import os
import sys

# Add the python directory to sys.path
sys.path.insert(
    0, os.path.abspath(os.path.join(os.path.dirname(__file__), "../python"))
)

from imgrs import Image  # noqa: E402


def test_rotation():
    # Create a simple image
    img = Image.new("RGB", (200, 100), (255, 0, 0))

    # Draw something on it to see rotation
    img.draw_rectangle(10, 10, 50, 50, (0, 255, 0, 255))

    # Test 1: Rotate 45 degrees, expand=True
    rotated_expanded = img.rotate(45, expand=True)
    print(f"Original size: {img.size}")
    print(f"Rotated (expand=True) size: {rotated_expanded.size}")
    rotated_expanded.save("examples/output/rotated_expanded.png")

    # Test 2: Rotate 45 degrees, expand=False
    rotated_cropped = img.rotate(45, expand=False)
    print(f"Rotated (expand=False) size: {rotated_cropped.size}")
    rotated_cropped.save("examples/output/rotated_cropped.png")

    assert rotated_expanded.size != img.size
    assert rotated_cropped.size == img.size

    print("Image rotation tests passed!")


def test_text_rotation():
    img = Image.new("RGB", (400, 400), (255, 255, 255))

    # Draw unrotated text
    img = img.add_text_styled(
        "Normal Text", (200, 50), size=32.0, color=(0, 0, 0, 255), align="center"
    )

    # Draw rotated text (45 degrees)
    img = img.add_text_styled(
        "Rotated 45",
        (200, 150),
        size=32.0,
        color=(255, 0, 0, 255),
        rotation=45.0,
        align="center",
    )

    # Draw rotated text (90 degrees)
    img = img.add_text_styled(
        "Rotated 90",
        (200, 250),
        size=32.0,
        color=(0, 0, 255, 255),
        rotation=90.0,
        align="center",
    )

    # Draw rotated text (180 degrees)
    img = img.add_text_styled(
        "Rotated 180",
        (200, 350),
        size=32.0,
        color=(0, 128, 0, 255),
        rotation=180.0,
        align="center",
    )

    img.save("examples/output/text_rotation.png")
    print("Text rotation test image saved to examples/output/text_rotation.png")


if __name__ == "__main__":
    if not os.path.exists("examples/output"):
        os.makedirs("examples/output")

    test_rotation()
    test_text_rotation()
