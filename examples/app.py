import imgrs 
from imgrs.image import Image

img = imgrs.new('RGB',(800, 400), 'green')
img = img.add_text_styled(
    "Imgrs is cool",
    (210, 180),
    size=64,
    color=(255, 255, 0, 255),
    outline=(255, 0, 0, 255, 2.0)
)

img.save("img.jpg", format="JPEG")