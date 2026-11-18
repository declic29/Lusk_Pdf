#!/usr/bin/env python3
"""Lusk PDF : régénère les icônes de l'application à partir de l'icône de l'association,
assets/app-icon/lusk-pdf-icone.png (le livre et le « P », tirés du logo).

Écrit, dans assets/app-icon/ :
  - hicolor/<taille>/apps/bzh.lusk.pdf.png (16 à 512 px, Linux, et l'icône des fenêtres) ;
  - hicolor/scalable/apps/bzh.lusk.pdf.svg (l'image 512 px dans un SVG) ;
  - pdfcraft-1024.png (1024 px sur la grille d'Apple : l'icône du Dock sur macOS) ;
  - pdfcraft.ico (Windows, 16 à 256 px, intégré à pdfcraft.exe) ;
  - pdfcraft.icns (macOS) ;
  - pdfcraft-document.ico (l'icône des fichiers PDF sous Windows, ressource 2 de pdfcraft.exe),
    rendue depuis pdfcraft-document.svg à chaque taille.

Il faut Python 3, Pillow (pip install pillow) et rsvg-convert (paquet librsvg2-bin), pour
l'icône des documents. Ensuite : mettre à jour les sha256 de
ATTRIBUTION.toml, puis `cargo xtask assets --write`.

    python3 packaging/icons.py
"""

import base64
import io
import pathlib
import subprocess

from PIL import Image

ROOT = pathlib.Path(__file__).resolve().parent.parent
DIR = ROOT / "assets" / "app-icon"
ID = "bzh.lusk.pdf"


def square(src: Image.Image, size: int, body: float = 0.94) -> Image.Image:
    """`src` centred on a transparent `size` square, its longer side `body` of it."""
    canvas = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    side = max(1, round(size * body))
    scale = side / max(src.width, src.height)
    w, h = max(1, round(src.width * scale)), max(1, round(src.height * scale))
    canvas.alpha_composite(src.resize((w, h), Image.Resampling.LANCZOS), ((size - w) // 2, (size - h) // 2))
    return canvas


def main() -> None:
    src = Image.open(DIR / "lusk-pdf-icone.png").convert("RGBA")
    src = src.crop(src.getbbox())
    for s in (16, 24, 32, 48, 64, 128, 256, 512):
        out = DIR / "hicolor" / f"{s}x{s}" / "apps" / f"{ID}.png"
        out.parent.mkdir(parents=True, exist_ok=True)
        # Small sizes fill more of the square, so the book stays readable.
        square(src, s, 1.0 if s <= 32 else 0.94).save(out, optimize=True)
    big = io.BytesIO()
    square(src, 512).save(big, format="PNG", optimize=True)
    svg = DIR / "hicolor" / "scalable" / "apps" / f"{ID}.svg"
    svg.parent.mkdir(parents=True, exist_ok=True)
    data = base64.b64encode(big.getvalue()).decode("ascii")
    svg.write_text(
        '<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" '
        'width="512" height="512" viewBox="0 0 512 512">\n'
        f'  <image width="512" height="512" xlink:href="data:image/png;base64,{data}"/>\n</svg>\n'
    )
    # Apple's grid: an 824 px body centred on a transparent 1024 canvas.
    mac = Image.new("RGBA", (1024, 1024), (0, 0, 0, 0))
    mac.alpha_composite(square(src, 824, 1.0), (100, 100))
    mac.save(DIR / "pdfcraft-1024.png", optimize=True)
    square(src, 256, 1.0).save(DIR / "pdfcraft.ico", sizes=[(s, s) for s in (16, 20, 24, 32, 40, 48, 64, 128, 256)])
    mac.save(DIR / "pdfcraft.icns")
    document_icon()


def document_icon() -> None:
    """pdfcraft-document.ico from pdfcraft-document.svg, each size drawn by rsvg-convert."""
    sizes = (16, 20, 24, 32, 40, 48, 64, 128, 256)
    images = []
    for s in sizes:
        png = subprocess.run(
            ["rsvg-convert", "-w", str(s), "-h", str(s), str(DIR / "pdfcraft-document.svg")],
            check=True,
            capture_output=True,
        ).stdout
        images.append(Image.open(io.BytesIO(png)).convert("RGBA"))
    images[-1].save(DIR / "pdfcraft-document.ico", sizes=[(s, s) for s in sizes], append_images=images[:-1])


if __name__ == "__main__":
    main()
