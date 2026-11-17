# Icônes de Lusk PDF

<img src="lusk-pdf-icone.png" alt="Icône de Lusk PDF : un livre rouge et la lettre P" width="128">

L'icône de Lusk PDF est tirée du logo de l'association Lusk (`lusk-pdf-logo.png`) : le livre
rouge et le « P » (`lusk-pdf-icone.png`). Ces images appartiennent à l'association
(`LICENSE-lusk.txt`).

| Fichier | Rôle |
|---|---|
| `lusk-pdf-logo.png` | le logo complet (README, À propos, écran d'accueil) |
| `lusk-pdf-icone.png` | l'image source de toutes les icônes ci-dessous |
| `hicolor/<taille>/apps/bzh.lusk.pdf.png` | icônes Linux (16 à 512 px) ; la 256 px est aussi l'icône des fenêtres |
| `hicolor/scalable/apps/bzh.lusk.pdf.svg` | l'icône 512 px dans un SVG |
| `pdfcraft-1024.png` | 1024 px sur la grille d'Apple : l'icône du Dock sur macOS |
| `pdfcraft.icns` | icône macOS |
| `pdfcraft.ico` | icône Windows (16 à 256 px), intégrée à `pdfcraft.exe` par `apps/pdfcraft/build.rs` |
| `pdfcraft-document.svg`, `pdfcraft-document.ico` | icône des fichiers PDF sous Windows, encore celle de PdfCraft |
| `pdfcraft.svg`, `pdfcraft-small.svg` | l'ancienne icône de PdfCraft (le lion), gardée pour le fond du .dmg macOS |

Pour régénérer les icônes après un changement de `lusk-pdf-icone.png` :

```sh
python3 packaging/icons.py        # il faut Pillow (pip install pillow)
```

Puis mettre à jour les `sha256` de `ATTRIBUTION.toml` et lancer `cargo xtask assets --write`.
