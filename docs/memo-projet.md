# Mémo du projet Lusk PDF

Mémoire du projet, pour reprendre le travail dans une nouvelle conversation avec Claude : il suffit
de lui demander de lire ce fichier (`C:\LuskPdf\docs\memo-projet.md`). La méthode commune est le
document « Adapter une application Craft pour Lusk — cahier des charges » dans les artefacts
Claude de Philippe. État au 17 novembre 2026.

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

- 16 novembre 2026, **écran d'accueil** : fond fourni par l'association
  (`assets/splash/accueil-lusk-pdf.jpg`, réduit à 1440 × 810), logo, barre de chargement et
  mentions, comme Lusk Texte et Lusk Tab (`crates/ui-egui/src/splash.rs`, textes modifiables en
  haut du fichier). Il s'ouvre dans sa propre petite fenêtre sans bordure avant la fenêtre de
  travail (`apps/pdfcraft/src/splash_window.rs`), qui garde ainsi sa taille et sa place
  enregistrées. Pas d'écran d'accueil sur macOS, avec `--control` ni avec `LUSK_NO_SPLASH`.
  `LUSK_SPLASH_SHOT=fichier.png cargo test -p pdfcraft-ui-egui --lib splash` en fait une capture.

- 17 novembre 2026, **marque (étape 3)** : « Lusk PDF » dans les textes de l'interface (titre,
  accueil, Préférences, À propos, messages) ; identifiant `bzh.lusk.pdf` partout (fichiers Linux
  renommés) ; mise à jour vers `declic29/Lusk_Pdf` avec la copie corrigée de craft-update
  (`vendor/craft-update`, comme Lusk Diapo) ; icônes tirées de l'icône de l'association par
  `packaging/icons.py` (remplace `icons.sh`) ; couleur d'accent #A31B1F (clair) et #E0565A
  (sombre) ; marques ArtCraft retirées (`docs/brand/`, À propos, accueil, bouton Discord remplacé
  par « Écrire à l'association Lusk », liens d'aide vers lusk.bzh et GitHub) ; README en
  français ; NOTICE.
- 17 novembre 2026, **français (étape 4)** : catalogues intégrés au dépôt (`i18n/locales/fr/`,
  toujours embarqués, `CRAFT_I18N_DIR` ignoré) ; « Automatique » = français ; seuls l'anglais
  et le français sont proposés ; « PdfCraft » remplacé par « Lusk PDF » dans les traductions ;
  environ 110 textes ajoutés ou traduits (ceux qui manquaient et ceux restés en anglais).

- 17 novembre 2026, **dictionnaires (étape 5)** : le correcteur de Lusk Texte (`crates/proof`,
  paquet `lusk-proof`) et ses dictionnaires, repris à l'octet près (`assets/dictionaries/fr` et
  `br`, `assets/hyphenation`, `assets/spelling`). Dans les commentaires, les réponses et le texte
  ajouté par « Remplir et signer », les mots inconnus sont soulignés en rouge ; clic droit :
  corrections, « Tout ignorer », « Ajouter au dictionnaire » (dictionnaire personnel
  `orthographe.txt`, dans le dossier `Dictionary` des données de l'application). Langue :
  Préférences › Documents et affichage › Orthographe (Français par défaut, Breton, Anglais,
  Désactivé). Le dictionnaire se charge en arrière-plan au démarrage (`src/spelling.rs`).
  Licence MPL des dictionnaires français (2.0) et breton (1.1) : hors de la liste de PdfCraft,
  admise par `cargo xtask assets` pour les seules données de `assets/dictionaries/`
  (`LUSK_DICTIONARY_LICENCES`), mentionnée dans NOTICE. Pas de dictionnaire des synonymes :
  celui de Lusk Texte n'est qu'une petite liste anglaise, sans intérêt ici. Les champs de
  formulaire et l'édition du texte des pages ne sont pas vérifiés (texte du document lui-même).

- 17 novembre 2026, **pages communes (étape 6)** : sur l'Accueil, le renard de l'association
  (`assets/avatar/`, repris de Lusk Texte) à côté de « Bonjour », « Bon après-midi » ou
  « Bonsoir » selon l'heure, puis « Ouvrir » avec trois boutons carrés 96 × 88 : Parcourir…,
  OneDrive, Google Drive (grisés avec un « ? » quand l'application n'est pas installée ; ils
  ouvrent le sélecteur dans le dossier synchronisé, Lusk PDF ne se connecte à aucun compte)
  (`crates/ui-egui/src/home/lusk.rs`). « À propos » : la tête du renard, et un onglet
  « Mises à jour » (version installée, « Rechercher des mises à jour », recherche au démarrage,
  ce qui est envoyé). Pas encore fait : Enregistrer sous avec OneDrive et Google Drive (Lusk PDF
  enregistre par la fenêtre du système, qui a déjà OneDrive dans sa barre latérale sous Windows) ;
  le lien Confidentialité viendra avec l'étape 7.

- 17 novembre 2026, **aide et confidentialité (étape 7)** : Aide › **Aide de Lusk PDF**, une
  fenêtre avec sommaire et sept guides en français (`crates/ui-egui/aide/*.md`, affichés par
  `src/help.rs`) : Bien démarrer, Lire et naviguer, Commenter et annoter, Remplir et signer,
  Organiser/modifier/protéger, Convertir/imprimer/OCR, Confidentialité ; plus un lien vers les
  Raccourcis clavier. Aide › **Confidentialité** ouvre directement la page « Vos données et
  votre vie privée », aussi accessible depuis À propos › Mises à jour et l'Accueil. Le relevé
  réseau qui la fonde est dans `docs/releve-reseau.md` : seule la recherche de mises à jour
  (GitHub) se connecte ; liens ouverts après accord ; formulaires jamais envoyés. Page des
  versions de la version web corrigée vers declic29/Lusk_Pdf. L'aide reste en français même
  quand l'interface est en anglais.

## À savoir

- **Images déposées sur le PC** : en passant par le lien avec l'ordinateur, une image PNG ou JPEG
  reçoit des données de provenance (C2PA) qui changent son empreinte. Après chaque dépôt d'image,
  relire le fichier du PC et mettre son SHA-256 dans `ATTRIBUTION.toml`, puis
  `cargo xtask assets --write`, sinon `cargo xtask assets` échoue en CI.
- **Catalogues de traduction** : chaque modification de `i18n/locales/fr/*.tsv` change leur
  empreinte ; la remettre dans `ATTRIBUTION.toml` puis `cargo xtask assets --write`.
- **Textes encore en anglais** : les messages d'erreur du moteur (affichés tels quels, par
  choix de PdfCraft), l'outil en ligne de commande, et quelques messages construits par
  morceaux. Les noms techniques gardent « PdfCraft » (dossiers de données `PdfCraft`, programme
  `pdfcraft`).
- **Reste de PdfCraft à remplacer plus tard** : l'icône des documents PDF sous Windows
  (`pdfcraft-document.ico`) et le fond du .dmg macOS, encore dessinés d'après le lion de PdfCraft.
- Clippy récent (Rust 1.97) signale une alerte déjà présente dans `crates/xfdf` ; vérifier les
  crates modifiées avec `--no-deps`.

## À faire (dans l'ordre du cahier des charges)

6. (reste) Enregistrer sous avec OneDrive et Google Drive, si l'association le souhaite.
8. Installateurs Windows, macOS, Linux en français.
9. Publication : workflow « Publication », version 1.0.0.
