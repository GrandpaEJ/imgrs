"""
Color Input Demo - Demonstrates various color input formats in imgrs

This example showcases the new color input flexibility:
- RGB/RGBA tuples (traditional)
- Hex strings with hash (#FF0000)
- Hex strings without hash (FF0000)
- Short hex codes (#F00)
- 8-digit hex with alpha (#FF0000FF)
"""

from imgrs import Image


def main():
    # Create a canvas
    canvas = Image.new("RGB", (800, 600), "#FFFFFF")

    # Title
    canvas = canvas.draw_text(
        "Color Input Formats Demo", 400, 30, color="#000000", scale=48, anchor="center"
    )

    # Row 1: RGB Tuples (Traditional)
    y_pos = 100
    canvas = canvas.draw_text("RGB Tuples:", 50, y_pos, color=(0, 0, 0, 255), scale=24)
    canvas = canvas.draw_rectangle(50, y_pos + 40, 100, 80, (255, 0, 0, 255))
    canvas = canvas.draw_text("(255, 0, 0)", 50, y_pos + 130, color="#333", scale=18)

    canvas = canvas.draw_circle(250, y_pos + 80, 40, (0, 255, 0, 255))
    canvas = canvas.draw_text("(0, 255, 0)", 210, y_pos + 130, color="#333", scale=18)

    canvas = canvas.draw_star(400, y_pos + 80, 40, 20, 5, (0, 0, 255, 255))
    canvas = canvas.draw_text("(0, 0, 255)", 360, y_pos + 130, color="#333", scale=18)

    # Row 2: Hex Strings with Hash
    y_pos = 280
    canvas = canvas.draw_text("Hex with #:", 50, y_pos, color="#000000", scale=24)
    canvas = canvas.draw_rectangle(50, y_pos + 40, 100, 80, "#FF6B00")
    canvas = canvas.draw_text("#FF6B00", 60, y_pos + 130, color="#333", scale=18)

    canvas = canvas.draw_circle(250, y_pos + 80, 40, "#9B59B6")
    canvas = canvas.draw_text("#9B59B6", 210, y_pos + 130, color="#333", scale=18)

    canvas = canvas.draw_ellipse(400, y_pos + 80, 50, 35, "#E74C3C")
    canvas = canvas.draw_text("#E74C3C", 360, y_pos + 130, color="#333", scale=18)

    # Row 3: Short Hex Codes
    y_pos = 460
    canvas = canvas.draw_text("Short Hex:", 50, y_pos, color="#000", scale=24)
    canvas = canvas.draw_rectangle(50, y_pos + 40, 100, 80, "#F0F")
    canvas = canvas.draw_text("#F0F", 80, y_pos + 130, color="#333", scale=18)

    canvas = canvas.draw_circle(250, y_pos + 80, 40, "#0CF")
    canvas = canvas.draw_text("#0CF", 220, y_pos + 130, color="#333", scale=18)

    canvas = canvas.draw_triangle(
        550, y_pos + 40, 500, y_pos + 120, 600, y_pos + 120, "#FA0"
    )
    canvas = canvas.draw_text("#FA0", 540, y_pos + 130, color="#333", scale=18)

    # Hex without hash
    canvas = canvas.draw_rectangle(650, y_pos + 40, 100, 80, "3498DB")
    canvas = canvas.draw_text("3498DB", 660, y_pos + 130, color="#333", scale=18)

    # Save the result
    canvas.save("examples/output/color_input_demo.png")
    print("✅ Color input demo saved to examples/output/color_input_demo.png")

    # Create a second demo showing alpha channel support
    alpha_canvas = Image.new("RGBA", (600, 400), "#FFFFFF")

    alpha_canvas = alpha_canvas.draw_text(
        "Alpha Channel Support", 300, 30, color="#000000", scale=40, anchor="center"
    )

    # Overlapping rectangles with different alpha values
    alpha_canvas = alpha_canvas.draw_rectangle(
        50, 100, 200, 200, "#FF000080"
    )  # 50% red
    alpha_canvas = alpha_canvas.draw_rectangle(
        150, 150, 200, 200, "#00FF0080"
    )  # 50% green
    alpha_canvas = alpha_canvas.draw_rectangle(
        250, 100, 200, 200, "#0000FF80"
    )  # 50% blue

    alpha_canvas = alpha_canvas.draw_text(
        "8-digit hex with alpha: #RRGGBBAA",
        300,
        350,
        color="#333333",
        scale=20,
        anchor="center",
    )

    alpha_canvas.save("examples/output/color_alpha_demo.png")
    print("✅ Alpha channel demo saved to examples/output/color_alpha_demo.png")


if __name__ == "__main__":
    main()
