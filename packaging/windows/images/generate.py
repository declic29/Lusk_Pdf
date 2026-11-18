#!/usr/bin/env python3
"""Draw the Windows installer's banner from the Lusk PDF logo and the Lusk fox.

    python3 packaging/windows/images/generate.py

Writes banner.bmp (493 x 58 pixels, 370 x 44 dialog units) next to this script: the strip at the
top of the installer's welcome and finish pages (installer-ui.wxs). The fox and the logo sit on
the right, on a soft tint of Lusk PDF's red, with « association Lusk » under the logo's name.

Sources: assets/app-icon/lusk-pdf-logo.png (the logo), assets/avatar/lusk-renard.png (the fox) and
assets/fonts/Inter-SemiBold.ttf. Needs Pillow. The bitmap is committed, so packaging never runs
this script; run it again after changing the logo or the fox.
"""
import os
import sys

from PIL import Image, ImageDraw, ImageFont

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, "..", "..", ".."))
LOGO = os.path.join(ROOT, "assets", "app-icon", "lusk-pdf-logo.png")
FOX = os.path.join(ROOT, "assets", "avatar", "lusk-renard.png")
FONT = os.path.join(ROOT, "assets", "fonts", "Inter-SemiBold.ttf")

BRAND = (0xA3, 0x1B, 0x1F)  # Lusk PDF's red (UPDATE_APP accent in crates/ui-egui/src/lib.rs)
TINT = (0xFB, 0xF0, 0xF0)  # a very pale tint of it, behind the artwork
WHITE = (255, 255, 255)

SCALE = 4  # draw large, then reduce: smooth edges in the 24-bit bitmap


def fit(path, height):
    """The image at `path`, `height` pixels high, its width in proportion."""
    im = Image.open(path).convert("RGBA")
    width = round(im.width * height / im.height)
    return im.resize((width, height), Image.Resampling.LANCZOS)


def banner():
    s = SCALE
    w, h = 493 * s, 58 * s
    im = Image.new("RGBA", (w, h), WHITE + (255,))
    draw = ImageDraw.Draw(im)
    # White behind the page's title on the left, fading into the tint behind the artwork.
    fade_from, fade = 250 * s, 60 * s
    for x in range(fade_from, w):
        t = min((x - fade_from) / fade, 1.0)
        draw.line([(x, 0), (x, h)], fill=tuple(round(a + (b - a) * t) for a, b in zip(WHITE, TINT)))
    draw.rectangle([0, h - 2 * s, w, h], fill=BRAND)
    logo = fit(LOGO, 34 * s)
    lx, ly = w - logo.width - 10 * s, 5 * s
    im.alpha_composite(logo, (lx, ly))
    font = ImageFont.truetype(FONT, 9 * s)
    caption = "association Lusk"
    box = draw.textbbox((0, 0), caption, font=font)
    draw.text((lx + (logo.width - (box[2] - box[0])) // 2, ly + logo.height + 1 * s), caption, font=font, fill=BRAND)
    fox = fit(FOX, 44 * s)
    im.alpha_composite(fox, (lx - fox.width - 8 * s, (h - 2 * s - fox.height) // 2))
    return im.convert("RGB").resize((493, 58), Image.Resampling.LANCZOS)


def main():
    for path in (LOGO, FOX, FONT):
        if not os.path.isfile(path):
            sys.exit(f"error: {path} not found")
    banner().save(os.path.join(HERE, "banner.bmp"))
    print(f"installer banner written to {HERE}")


if __name__ == "__main__":
    main()
