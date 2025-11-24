"""CSS Composite/Blending operations mixin"""

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from ..image import Image


class CompositeFiltersMixin:
    """
    Mixin for CSS composite and blending operations.

    Provides 28 different composite modes for advanced image blending:
    - 8 Source/Destination operations
    - 20 Photoshop-style blending modes
    """

    def composite(self, mode: str, opacity: float = 1.0) -> "Image":
        """
        Apply a CSS composite/blending mode to the image.

        Args:
            mode: Composite mode name (see list below)
            opacity: Opacity of the effect (0.0 to 1.0)

        Returns:
            New image with composite mode applied

        ## Source/Destination Group (Alpha Compositing):

        - **source-over**: ডিফল্ট। নতুন ছবি (source) আগের ছবির (destination) উপরে আঁকা হয়।
        - **source-in**: শুধুমাত্র destination এর opaque অংশে source দেখা যায়।
        - **source-out**: শুধুমাত্র destination এর transparent অংশে source দেখা যায়।
        - **source-atop**: source কে destination এর shape অনুযায়ী কাটে।
          শুধুমাত্র destination এর opaque অংশে source দেখা যায়।
        - **destination-over**: নতুন ছবি (source) পেছনে আঁকা হয়।
        - **destination-in**: destination শুধু source এর opaque অংশে দেখা যায়।
        - **destination-out**: destination শুধু source এর transparent অংশে দেখা যায়।
        - **destination-atop**: source কে destination এর shape অনুযায়ী কাটে। (উদাহরণ: avatar mask)

        ## Blending/Photoshop-like Group:

        - **normal**: সাধারণ blending (ডিফল্ট)
        - **multiply**: গুণ করে কালো রঙ বৃদ্ধি করে।
        - **screen**: উজ্জ্বলতা বাড়ায়।
        - **overlay**: multiply + screen এর কম্বিনেশন।
        - **darken**: destination এবং source এর মধ্যে কম উজ্জ্বল রঙ দেখায়।
        - **lighten**: destination এবং source এর মধ্যে বেশি উজ্জ্বল রঙ দেখায়।
        - **color-dodge**: destination উজ্জ্বল করতে source ব্যবহার।
        - **color-burn**: destination গাঢ় করতে source ব্যবহার।
        - **hard-light**: overlay এর মত effect, light/dark mix করে।
        - **soft-light**: soft overlay effect।
        - **difference**: source ও destination এর মধ্যে পার্থক্য দেখায়।
        - **exclusion**: difference এর হালকা সংস্করণ।
        - **lighter**: পিক্সেলগুলোর রঙ যোগ করে উজ্জ্বলতা বাড়ায়।
        - **copy**: source দেখানো হয়, destination লুকানো হয়।
        - **xor**: source এবং destination এর exclusive-or।
        - **hue**: source এর hue destination এ প্রয়োগ করে।
        - **saturation**: source এর saturation destination এ প্রয়োগ করে।
        - **color**: source এর hue+saturation destination এ প্রয়োগ করে।
        - **luminosity**: source এর brightness destination এ প্রয়োগ করে।

        Example:
            ```python
            from imgrs import Image

            # Load image
            img = Image.open("photo.jpg")

            # Apply destination-atop composite mode
            result = img.composite("destination-atop")

            # Apply multiply blend mode with 50% opacity
            result = img.composite("multiply", opacity=0.5)

            # Apply hue blend mode
            result = img.composite("hue")
            ```
        """
        return self.__class__(self._rust_image.composite(mode, opacity))
