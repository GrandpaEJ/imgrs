"""
Demo of text rendering with emojis - combining rich text features with emoji support
"""

import os
import imgrs

# Create output directory
os.makedirs("output/text_emoji_demo", exist_ok=True)

print("=" * 70)
print("🎨 TEXT WITH EMOJI DEMO - Rich Text + Emojis! 🎉")
print("=" * 70)
print()

# =============================================================================
# 1. BASIC TEXT WITH EMOJIS
# =============================================================================
print("1. Basic Text with Emojis")
img1 = imgrs.Image.new("RGBA", (600, 200), (255, 255, 255, 255))

# Simple text with emojis
texts = [
    "Hello 😊 World 🌍",
    "I ❤️ Python 🐍",
    "Coffee ☕ & Code 💻",
    "Music 🎵 & Art 🎨",
]

y = 40
for text in texts:
    img1 = img1.add_text(text, (50, y), size=32, color=(0, 0, 0, 255))
    y += 45

img1.save("output/text_emoji_demo/01_basic_emoji_text.png")
print("   ✅ Basic text with inline emojis")

# =============================================================================
# 2. STYLED TEXT WITH EMOJIS
# =============================================================================
print("2. Styled Text with Emojis")
img2 = imgrs.Image.new("RGBA", (800, 400), (30, 30, 50, 255))

# Text with styling and emojis
styled_texts = [
    ("Welcome 🎉", (100, 50), (255, 215, 0, 255)),  # Gold
    ("To the Future 🚀", (100, 120), (100, 200, 255, 255)),  # Blue
    ("Of Imaging 🖼️", (100, 190), (150, 255, 150, 255)),  # Green
    ("With Style ✨", (100, 260), (255, 150, 200, 255)),  # Pink
]

for text, pos, color in styled_texts:
    img2 = img2.add_text_styled(
        text,
        pos,
        size=36,
        color=color,
        outline=(255, 255, 255, 255, 2.0),
        shadow=(2, 2, 0, 0, 0, 150),
    )

img2.save("output/text_emoji_demo/02_styled_emoji_text.png")
print("   ✅ Styled text with emojis and effects")

# =============================================================================
# 3. EMOJI IN DIFFERENT CONTEXTS
# =============================================================================
print("3. Emoji in Different Contexts")
img3 = imgrs.Image.new("RGBA", (800, 600), (255, 255, 255, 255))

# Weather themed
weather_text = "Today's Weather: ☀️ Sunny with ☁️ clouds and 🌧️ rain"
img3 = img3.add_text(weather_text, (50, 50), size=28, color=(0, 100, 200, 255))

# Food themed
food_text = "Menu: 🍕 Pizza, 🍔 Burger, 🍟 Fries, 🥤 Drink"
img3 = img3.add_text(food_text, (50, 120), size=28, color=(150, 50, 50, 255))

# Activity themed
activity_text = "Activities: 🏃 Running, 🚴 Cycling, 🏊 Swimming, 🧘 Yoga"
img3 = img3.add_text(activity_text, (50, 190), size=28, color=(50, 150, 50, 255))

# Tech themed
tech_text = "Tech Stack: 💻 Python, 🐍 Django, ☁️ AWS, 🐳 Docker"
img3 = img3.add_text(tech_text, (50, 260), size=28, color=(100, 100, 100, 255))

# Celebration themed
celebrate_text = "🎊 Happy Birthday! 🎂 Hope your day is filled with 🎈 joy! 🎉"
img3 = img3.add_text(celebrate_text, (50, 330), size=28, color=(255, 100, 150, 255))

# Nature themed
nature_text = "Nature: 🌸 Flowers, 🌳 Trees, 🐦 Birds, 🦋 Butterflies"
img3 = img3.add_text(nature_text, (50, 400), size=28, color=(100, 200, 100, 255))

# Travel themed
travel_text = "Travel: ✈️ Planes, 🚂 Trains, 🚗 Cars, 🛳️ Ships"
img3 = img3.add_text(travel_text, (50, 470), size=28, color=(200, 100, 200, 255))

img3.save("output/text_emoji_demo/03_context_emoji_text.png")
print("   ✅ Text with emojis in different contexts")

# =============================================================================
# 4. MULTILINE TEXT WITH EMOJIS
# =============================================================================
print("4. Multiline Text with Emojis")
img4 = imgrs.Image.new("RGBA", (700, 500), (240, 240, 250, 255))

multiline_poem = """🌟 In a world of code and dreams 🌟
💻 Where pixels dance on screens 💻
🐍 Python slithers through the lines 🐍
🎨 Creating art that truly shines 🎨

🚀 Innovation at every turn 🚀
📚 Knowledge we eagerly learn 📚
❤️ With passion burning bright ❤️
✨ We code from morning till night ✨"""

img4 = img4.add_text_multiline(
    multiline_poem,
    (50, 50),
    size=24,
    color=(50, 50, 100, 255),
    line_spacing=1.4,
    align="center",
)

img4.save("output/text_emoji_demo/04_multiline_emoji_poem.png")
print("   ✅ Multiline poem with emojis")

# =============================================================================
# 5. EMOJI TEXT WITH BACKGROUNDS
# =============================================================================
print("5. Emoji Text with Backgrounds")
img5 = imgrs.Image.new("RGBA", (800, 400), (255, 255, 255, 255))

# Create some background boxes first
backgrounds = [
    ((50, 50, 300, 80), (255, 200, 200, 255)),  # Light red
    ((400, 50, 300, 80), (200, 255, 200, 255)),  # Light green
    ((50, 150, 300, 80), (200, 200, 255, 255)),  # Light blue
    ((400, 150, 300, 80), (255, 255, 200, 255)),  # Light yellow
]

for (x, y, w, h), color in backgrounds:
    img5 = img5.draw_rectangle(x, y, w, h, color)

# Add text with emojis over backgrounds
emoji_texts = [
    ("❤️ Love & Peace 🕊️", (70, 85), (100, 50, 150, 255)),
    ("🎵 Music & Dance 💃", (420, 85), (150, 50, 100, 255)),
    ("🌟 Dreams & Stars ⭐", (70, 185), (50, 100, 150, 255)),
    ("📚 Books & Wisdom 🧠", (420, 185), (100, 150, 50, 255)),
]

for text, (x, y), color in emoji_texts:
    img5 = img5.add_text(text, (x, y), size=24, color=color)

img5.save("output/text_emoji_demo/05_emoji_backgrounds.png")
print("   ✅ Text with emojis over colored backgrounds")

# =============================================================================
# 6. MIXED EMOJI SIZES AND STYLES
# =============================================================================
print("6. Mixed Emoji Sizes and Styles")
img6 = imgrs.Image.new("RGBA", (800, 600), (255, 255, 255, 255))

# Large title with emojis
title = "🎨 Creative 🎭 Expression 🎪"
img6 = img6.add_text_styled(
    title,
    (400, 50),
    size=48,
    color=(100, 50, 150, 255),
    align="center",
    outline=(0, 0, 0, 255, 2.0),
)

# Medium subtitles
subtitles = [
    "🎨 Digital Art & Design",
    "🎭 Theater & Performance",
    "🎪 Entertainment & Fun",
]

y = 150
for subtitle in subtitles:
    img6 = img6.add_text_styled(
        subtitle, (400, y), size=32, color=(50, 100, 200, 255), align="center"
    )
    y += 60

# Small descriptions with different styles
descriptions = [
    ("✏️ Drawing & Sketching", (150, 350), (150, 100, 50, 255)),
    ("🎨 Painting & Colors", (150, 400), (200, 50, 100, 255)),
    ("💻 Digital Creation", (150, 450), (50, 150, 100, 255)),
    ("🎭 Acting & Drama", (450, 350), (150, 50, 150, 255)),
    ("🎪 Circus & Magic", (450, 400), (200, 150, 50, 255)),
    ("🎵 Music & Sound", (450, 450), (50, 100, 200, 255)),
]

for desc, (x, y), color in descriptions:
    img6 = img6.add_text_styled(
        desc,
        (x, y),
        size=20,
        color=color,
        background=(240, 240, 240, 200),
        align="center",
    )

img6.save("output/text_emoji_demo/06_mixed_styles.png")
print("   ✅ Mixed emoji sizes and text styles")

# =============================================================================
# 7. EMOJI TEXT MEASUREMENT
# =============================================================================
print("7. Emoji Text Measurement")
img7 = imgrs.Image.new("RGBA", (800, 400), (255, 255, 255, 255))

# Test text with emojis
test_texts = [
    "Hello 😊",
    "🌍 World 🌍",
    "Python 🐍 & Rust 🦀",
    "🎉 Celebration Time! 🎊",
]

y = 50
for i, text in enumerate(test_texts):
    # Get text dimensions
    width, height = imgrs.Image.get_text_size(text, size=36)

    # Draw a box around the text area
    box_x, box_y = 100, y - 10
    img7 = img7.draw_rectangle(box_x, box_y, width, height + 20, (240, 240, 240, 255))
    # Draw border by drawing a slightly smaller rectangle on top
    img7 = img7.draw_rectangle(
        box_x + 1, box_y + 1, width - 2, height + 18, (255, 255, 255, 255)
    )

    # Add the text
    img7 = img7.add_text(text, (box_x, y + height - 8), size=36, color=(0, 0, 0, 255))

    # Add measurement info
    info = f"Size: {width}x{height}px"
    img7 = img7.add_text(
        info, (box_x + width + 20, y + 20), size=16, color=(100, 100, 100, 255)
    )

    y += 80

img7.save("output/text_emoji_demo/07_measurement_demo.png")
print("   ✅ Text measurement with emojis")

print()
print("=" * 70)
print("✨ TEXT WITH EMOJI DEMO COMPLETE!")
print("=" * 70)
print()
print("Features Demonstrated:")
print("  ✅ Basic text with inline emojis")
print("  ✅ Styled text with emoji effects (outline, shadow)")
print("  ✅ Emojis in different contexts (weather, food, tech, etc.)")
print("  ✅ Multiline text with emoji poetry")
print("  ✅ Text with emojis over backgrounds")
print("  ✅ Mixed emoji sizes and text styles")
print("  ✅ Text measurement including emojis")
print()
print("📁 Output: output/text_emoji_demo/ (7 files)")
print("=" * 70)
