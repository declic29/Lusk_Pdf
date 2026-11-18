# craft-update, copie Lusk PDF

Ce dossier est une copie du module de mise à jour `craft-update` de
[storytold/craft-libs](https://github.com/storytold/craft-libs) (dossier `crates/update`, commit
`e2dc681a3`), sous licence MIT ou Apache 2.0 (voir `LICENSE-MIT`
et `LICENSE-APACHE`).

Le `Cargo.toml` racine de Lusk PDF la met à la place de l'original (`[patch]`).

## Ce qui change par rapport à l'original

Deux choses.

1. Dans `src/self_update.rs` (deux lignes marquées « Lusk PDF ») : pour trouver
le fichier à installer dans une Release, le module cherche sous **tous les noms de
l'application** (`App::stems()` : l'identifiant `pdfcraft` et les noms de `former_ids`, dont
`Lusk-Pdf`), et non plus seulement sous l'identifiant technique `pdfcraft`.

Grâce à cela, les Releases ne contiennent que des fichiers `Lusk-Pdf-<version>-…` : plus besoin
des copies `pdfcraft-<version>-…`, qui apparaissaient dans la liste des téléchargements.

2. Dans `src/detect.rs` (`PUBLISHERS`, marqué « Lusk PDF ») : une installation Windows est
reconnue quand son éditeur est « Lusk » (celui de l'installateur de Lusk PDF), et plus seulement
« Learning Machines LLC ». Sans cela, la mise à jour d'une copie installée par le .msi échouerait
faute de trouver son code produit.

Le `Cargo.toml` de la copie remplace les réglages du workspace de craft-libs par leurs valeurs, et
n'a pas les dépendances de test.

## Mettre à jour la copie

Pour passer à une version plus récente de craft-libs : recopier `crates/update/src` de ce commit,
refaire ces deux modifications, et mettre à jour le commit cité ici et dans le
`Cargo.toml` racine.
