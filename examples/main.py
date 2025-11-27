from imgrs import Image
import sqlite3
import os
import tempfile
import random
import asyncio
import aiohttp
from fuzzywuzzy import fuzz, process


def parse_color_string(color_str):
    """
    Converts hex "#f8ca3e" to tuple (248, 202, 62, 255)
    """
    if color_str.startswith("#"):
        color_str = color_str[1:]
        return tuple(int(color_str[i : i + 2], 16) for i in (0, 2, 4)) + (255,)
    else:
        # Fallback
        return (255, 255, 255, 255)


# 1. Load the data from DB
conn = sqlite3.connect("animes.db")
cursor = conn.cursor()
cursor.execute("SELECT * FROM animes")
rows = cursor.fetchall()
animes_data = []
for row in rows:
    animes_data.append(
        {
            "_id": row[0],
            "name": row[1],
            "dupe_name": row[2],
            "image_url": row[3],
            "image_original_url": row[4],
            "primary_color": row[5],
            "category": row[6],
            "margin_left": row[7],
            "mask_height": row[8],
        }
    )
conn.close()

# 2. Load static assets (things that don't change per iteration)
layer2_path = "layer-2.png"
layer3_path = "layer-3.png"
slog = "Hello world! Ebtisam here!"

cache_dir = "cache/images"
os.makedirs(cache_dir, exist_ok=True)


def select_anime(id=None, name=None, category=None):
    candidates = animes_data

    if id is not None:
        candidates = [a for a in candidates if a["_id"] == id]

    if category is not None:
        candidates = [
            a for a in candidates if a["category"].lower() == category.lower()
        ]

    if name is not None:
        # First try exact match
        exact_matches = [a for a in candidates if a["name"].lower() == name.lower()]
        if exact_matches:
            candidates = exact_matches
        else:
            # Fuzzy match on name
            names = [a["name"] for a in candidates]
            best_matches = process.extract(name, names, scorer=fuzz.ratio, limit=10)
            # Filter matches with score > 70
            matched_names = [match[0] for match in best_matches if match[1] > 70]
            candidates = [a for a in candidates if a["name"] in matched_names]

    if not candidates:
        return None

    # If multiple, pick random; if one, return it
    return random.choice(candidates) if len(candidates) > 1 else candidates[0]


async def generate_image(anime, custom_name=None, slogan=None):
    if not anime:
        return {"status": "error", "message": "No anime selected."}

    print(f"Processing anime: {anime['name']} (ID: {anime['_id']})...")

    # Check cache
    cache_path = os.path.join(cache_dir, f"{anime['_id']}.png")
    if os.path.exists(cache_path):
        with open(cache_path, "rb") as f:
            image_data = f.read()
    else:
        # Download the image async
        async with aiohttp.ClientSession() as session:
            async with session.get(anime["image_url"]) as response:
                if response.status != 200:
                    return {"status": "error", "message": "Failed to download image."}
                image_data = await response.read()
        # Save to cache
        with open(cache_path, "wb") as f:
            f.write(image_data)

    # Process in thread
    result = await asyncio.to_thread(
        process_image, anime, image_data, custom_name, slogan
    )
    return result


def process_image(anime, image_data, custom_name=None, slogan=None):
    # Save to temp file
    with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as temp_file:
        temp_file.write(image_data)
        img_path = temp_file.name

    try:
        # Convert color
        color_bg = parse_color_string(anime["primary_color"])

        # Open the image
        aboy = Image.open(img_path)

        # Reload layers
        layer2 = Image.open(layer2_path)
        layer3 = Image.open(layer3_path)

        name_text = custom_name or anime["name"]
        slogan_text = slogan or slog

        # --- START IMAGE GENERATION LOGIC ---

        # Create the background layer with the dynamic Color
        img = Image.new(size=(2000, 750), color=color_bg, mode="RGB")

        # Prepare the rotated white background strip
        layer4 = (
            Image.new(size=(1800, 450), color=(255, 255, 255), mode="RGB")
            .rotate(45, expand=True)
            .paste(
                aboy.resize(size=(1500, 1500)).grayscale_filter(1.0).set_alpha(0.15),
                (20, 0),
            )
        )

        # Paste layer 4 onto the colored background
        img = img.paste(layer4, (400, -450))

        # Prepare the transparent overlay
        boy = aboy.resize((1500, 1500)).set_alpha(0.15)

        # Paste static layers
        img = img.paste(layer2)
        img = img.paste(layer3)

        # Paste the large transparent
        img = img.paste(boy, (-300, -450))

        # Create the text sticker
        newimg = (
            Image.new("RGBA", (1500, 500), (0, 0, 0, 0))
            .add_text_styled(
                text=name_text,
                position=(100, 0),
                size=300,
                color=(0, 255, 255, 0),
                font_path="fonts/HighSchoolUsaSerif-6vwM.ttf",
                outline=(0, 0, 0, 255, 3.0),
            )
            .rotate(45, expand=True)
        )

        # Paste the text sticker
        img = img.paste(newimg, (520, -200))

        # Paste the main character image (top layer)
        aboy_top = aboy.resize((1000, 1000))
        img = img.paste(aboy_top, (900, -200))

        # Add standard text
        img = img.add_text(
            text=name_text.upper(),
            position=(400, 380),
            size=67,
            font_path="fonts/GMVDINPro-CondMedium.ttf",
        )
        img = img.add_text(
            text=slogan_text, position=(400, 440), font_path="fonts/arial/ARIAL.TTF"
        )

        # --- END IMAGE GENERATION LOGIC ---

        # Save with a unique filename
        output_filename = f"output_{anime['_id']}.png"
        img.save(output_filename)
        print(f"Saved {output_filename}")
        return {"status": "success", "filename": output_filename}

    finally:
        # Clean up temp file
        os.unlink(img_path)


# For testing, you can call generate_image(1) or something
if __name__ == "__main__":
    # 3. Get User Input
    user_input = input("Enter ID (e.g., 1 or 2): ")

    try:
        target_id = int(user_input)
    except ValueError:
        print("Invalid input. Please enter a number.")
        exit()

    result = generate_image(target_id)
    print(result)
