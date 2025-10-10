# Replace this:
# from PIL import Image

# With this:
from imgrs import Image

# Your existing Pillow code works unchanged!
img = Image.open("photo.jpg")
img = img.resize((400, 300))
img.save("resized.jpg")