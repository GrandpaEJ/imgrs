import imgrs 
from imgrs.image import Image

img = imgrs.new('RGB',(300,200), 'green')
img = img.add_text("Imgrs is cool", (50, 50), size=48, color=(0, 0, 0, 255))

img.save("img.jpg", format="JPEG")