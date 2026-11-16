# Mémo du projet Lusk PDF

Mémoire du projet, pour reprendre le travail dans une nouvelle conversation avec Claude : il suffit
de lui demander de lire ce fichier (`C:\LuskPdf\docs\memo-projet.md`). La méthode commune est le
document « Adapter une application Craft pour Lusk — cahier des charges » dans les artefacts
Claude de Philippe. État au 16 novembre 2026.

## Le projet en bref

- **Lusk PDF** : lecteur et éditeur de PDF libre et gratuit (Windows, macOS, Linux), en Rust
  (egui). C'est une adaptation française de **PdfCraft** 0.21.1 (ArtCraft Team, licences MIT ou
  Apache 2.0), partie du commit `efd2655` de storytold/pdfcraft.
- Porté par l'**association Lusk** (loi 1901), présidée par Philippe. Contact public :
  contact@lusk.bzh. Site : https://www.lusk.bzh.
- Code : https://github.com/declic29/Lusk_Pdf, branche `main`. Sur le PC : `C:\LuskPdf`.
- Applications sœurs, qui servent de modèles : Lusk Texte (declic29/LuskTX), Lusk Tab
  (declic29/LuskTab), Lusk Diapo (declic29/Lusk-Diapo).
- **Fins de ligne** : `.gitattributes` (`* -text`) garde chaque fichier à l'octet près, sans
  conversion. Les fichiers déposés sur le PC gardent donc les fins de ligne Unix (LF), pas
  Windows (contrairement à Lusk Texte, Tab et Diapo).
- Philippe fait les commits, les push, les `git rm` et les étiquettes ; Claude prépare, vérifie et
  dépose les fichiers dans `C:\LuskPdf`.

## Décisions prises

- Nom affiché : **Lusk PDF**. Identifiant : `bzh.lusk.pdf`. Éditeur : « Lusk ».
- Fichiers publiés : `Lusk-Pdf-<version>-<système>-<architecture>.<ext>` ; noms courts
  `Lusk-Pdf-Windows.msi`, `Lusk-Pdf-Mac.dmg`, `Lusk-Pdf-Linux.AppImage`, `Lusk-Pdf-Linux.deb`.
- Noms techniques inchangés : programme `pdfcraft`, crates `pdfcraft-*`, identifiant de mise à
  jour `pdfcraft`.
- Couleur : rouge **#A31B1F** (au lieu du rouge PdfCraft #B70228).
- Logo : fourni par l'association le 16 novembre 2026 (livre rouge et « PdF » en dégradé),
  `assets/app-icon/lusk-pdf-logo.png`. Icône : le livre et le « P »
  (`assets/app-icon/lusk-pdf-icone.png`), comme les cartes et le « D » de Lusk Diapo. Le logo
  appartient à l'association (`LicenseRef-Lusk-Trademark`, `assets/app-icon/LICENSE-lusk.txt`),
  admis par `cargo xtask assets` pour les seuls fichiers `assets/app-icon/lusk-*`.
- Mise à jour : copie de craft-update corrigée dans `vendor/` (comme Lusk Diapo), pour que les
  Releases ne contiennent que des fichiers `Lusk-Pdf-…`, sans copies `pdfcraft-…`.

## Ce que PdfCraft a de différent des autres applications

- **Pas de ruban** comme Lusk Texte, Tab et Diapo : une barre d'outils, des menus, un panneau
  « Tous les outils » et une page d'accueil (`crates/ui-egui/src/home`). Les pages communes
  (Accueil avec le renard, Ouvrir, Enregistrer sous, Mises à jour, Aide, Confidentialité) seront
  à placer au plus près de leur place dans les autres applications.
- **Traduction déjà prête** : PdfCraft lit les catalogues de storytold/craft-i18n (commit dans
  `craft-i18n.rev`) ; le français y a déjà environ 4 500 lignes pour PdfCraft et 1 300 communes.
  Il faudra les intégrer au dépôt (sans dépendre de craft-i18n à la compilation), mettre le
  français par défaut, puis compléter ce qui manque.
- Bibliothèques craft-libs au commit `e2dc681a3`, plus récent que celui de Lusk Diapo.
- Version web, Flatpak, FreeBSD, worker OCR : hors publication Lusk au départ.

## Fait

- 16 novembre 2026 : dépôt cloné et étudié ; nom, couleur, identifiant décidés ; ce mémo.
- 16 novembre 2026, **ménage** : retirés les workflows de la suite Craft (freebsd, fuzz,
  house-rules, nightly-macos, nix, sonarcloud, windows-arm64), `.coderabbit.yaml`,
  `sonar-project.properties` et `ROADMAP.md` ; Dependabot mensuel, corrections seulement (repris
  de Lusk Diapo) ; ligne Lusk dans LICENSE-MIT. Logo et image source de l'icône ajoutés, avec
  leurs entrées dans `ATTRIBUTION.toml` (ATTRIBUTION.md régénéré par `cargo xtask assets
  --write`). Gardés pour l'instant : `ci.yml`, `packaging-lint.yml`, `release.yml` (remplacé à
  l'étape Publication), `AGENTS.md` (règles techniques et politique des images, sur lesquelles
  s'appuie `cargo xtask assets`), les marques ArtCraft et les contributeurs (retirés à l'étape
  Marque, avec le code qui les affiche).

## À faire (dans l'ordre du cahier des charges)

3. Marque : nom, `bzh.lusk.pdf`, icônes, À propos, couleur #A31B1F, écran d'accueil animé,
   README, NOTICE ; retirer les marques ArtCraft (`docs/brand/`, À propos, menu Aide, accueil),
   les contributeurs et les captures PdfCraft.
4. Français : catalogues intégrés, français par défaut, 0 texte manquant, relecture des écrans.
5. Dictionnaires (si l'application en utilise : annotations, formulaires).
6. Pages communes : Accueil avec le renard, Ouvrir / Enregistrer sous avec OneDrive et Google
   Drive, Mises à jour, À propos.
7. Aide, Confidentialité (relevé réseau).
8. Installateurs Windows, macOS, Linux en français.
9. Publication : workflow « Publication », version 1.0.0.
