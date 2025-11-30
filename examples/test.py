import os
import tempfile

from imgrs import Image


# --- COLOR PARSER (USED BY IMGRS BACKGROUND) ---
def parse_color_string(color_str) -> tuple[int, int, int]:
    """
    Converts hex "#f8ca3e" to tuple (248, 202, 62)
    """
    if color_str.startswith("#"):
        color_str = color_str[1:]
        r, g, b = [int(color_str[i : i + 2], 16) for i in (0, 2, 4)]
        return (r, g, b)
    else:
        # Fallback
        return (255, 255, 255)


# --- IMGRS IMAGE PROCESSING TOOL ---
def imgrs_generate_image(
    anime,
    image_data,
    layer2_path="examples/img/layer-2.png",
    layer3_path="examples/img/layer-3.png",
    slog="Hello world! Ebtisam here!",
    custom_name=None,
    slogan=None,
    output_filename=None,
):
    """
    Imgrs-only image generation tool.
    All properties and logic are preserved exactly.
    """

    # Save temp image
    with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as temp_file:
        temp_file.write(image_data)
        img_path = temp_file.name

    try:
        # Convert color
        color_bg = parse_color_string(anime["primary_color"]) + (255,)

        # Open main image
        aboy = Image.open(img_path)

        # Load static layers (create dummy transparent layers if not exist)
        try:
            layer2 = Image.open(layer2_path)
        except Exception:
            layer2 = Image.new("RGBA", (2000, 750), (0, 0, 0, 0))
        try:
            layer3 = Image.open(layer3_path)
        except Exception:
            layer3 = Image.new("RGBA", (2000, 750), (0, 0, 0, 0))

        name_text = custom_name or anime["name"]
        slogan_text = slogan or slog

        # --- START IMAGE GENERATION LOGIC ---

        # Create background
        img = Image.new("RGBA", (2000, 750), color_bg)

        # Rotated white background strip
        layer4 = Image.new("RGB", (1800, 450), (255, 255, 255))
        layer4 = layer4.rotate(45, expand=True)
        layer4.paste(
            aboy.resize(size=(1500, 1500)).grayscale_filter(1.0).set_alpha(0.15),
            (20, 0),
        )

        # Paste strip
        img = img.paste(layer4, (400, -450))

        # Transparent overlay
        boy = aboy.resize((1500, 1500)).set_alpha(0.15)

        # Static layers
        img = img.paste(layer2)
        img = img.paste(layer3)

        # Large transparent image
        img = img.paste(boy, (-300, -450))

        # Text sticker - add directly since paste has issues
        img = img.add_text(
            text=name_text,
            position=(620, -100),
            size=300,
            color=(0, 255, 255, 255),
            font_path=None,
        )

        # Main character image
        aboy_top = aboy.resize((1000, 1000))
        img = img.paste(aboy_top, (900, -200))

        # Standard text layers
        img = img.add_text(
            text=name_text.upper(),
            position=(400, 380),
            size=67,
            font_path="fonts/DejaVuSans.ttf",
        )

        img = img.add_text(
            text=slogan_text, position=(400, 440), font_path="fonts/DejaVuSans.ttf"
        )

        # --- END IMAGE GENERATION LOGIC ---

        # Output filename (unchanged behavior)
        if not output_filename:
            output_filename = f"output_{anime['_id']}.png"

        img.save(output_filename)

        return {"status": "success", "filename": output_filename}

    finally:
        os.unlink(img_path)


if __name__ == "__main__":
    img = Image.open("examples/img/Boy.png")
    with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as temp_file:
        img.save(temp_file.name)
        with open(temp_file.name, "rb") as f:
            image_data = f.read()
        os.unlink(temp_file.name)

    anime = {"primary_color": "#f8ca3e", "name": "Test Anime", "_id": "123"}

    result = imgrs_generate_image(anime, image_data=image_data)
    print(result)
