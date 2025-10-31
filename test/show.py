from imgrs import Image

img = Image.new("RGB", (300, 100), color="green")
img = img.add_text_styled(
    "Grandpa EJ",
    (20, 40),
    size=50,
    color=(255, 255, 255, 255),
    outline=(0, 0, 0, 255, 3.0),
)

img.show()
