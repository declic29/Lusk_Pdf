# Relevé réseau de Lusk PDF

Ce relevé liste chaque endroit où Lusk PDF (version de bureau) peut utiliser le réseau, d'après
le code au 17 novembre 2026. Il sert de base à la page « Vos données et votre vie privée » de
l'aide (`crates/ui-egui/aide/07-confidentialite.md`). À refaire à chaque reprise de code de
PdfCraft : chercher `ureq`, `reqwest`, `TcpStream`, `TcpListener`, `UdpSocket`, `open_url` et
`https://` dans `crates/`, `apps/` et `vendor/`.

## Le seul client HTTP : les mises à jour

- `vendor/craft-update` (copie corrigée de craft-update, `ureq` avec rustls) est la seule
  bibliothèque du programme qui ouvre une connexion.
- Il interroge `https://api.github.com/repos/declic29/Lusk_Pdf/releases?per_page=30` (`UPDATE_APP` dans
  `crates/ui-egui/src/lib.rs`) :
  - au démarrage, si « Rechercher les mises à jour au démarrage » est coché (Préférences, ou
    À propos › Mises à jour) ;
  - quand on clique sur « Rechercher des mises à jour » (Aide, ou À propos › Mises à jour).
- Il n'envoie que la requête et un en-tête User-Agent `Lusk PDF/Update` ; aucun document,
  aucun identifiant, aucune donnée personnelle.
- Le fichier d'installation de la nouvelle version (sur `github.com`, Releases) n'est téléchargé
  que si l'utilisateur accepte la mise à jour.

## Ouvertures du navigateur ou de la messagerie (pas de connexion de Lusk PDF lui-même)

- Liens d'un document (lien, bouton, `app.launchURL` d'un script) : seules les adresses web et
  e-mail sont admises, et seulement après accord de l'utilisateur
  (`request_document_url`, `crates/ui-egui/src/app/links.rs`).
- Menu Aide et carte de l'Accueil : site lusk.bzh, page GitHub, `mailto:contact@lusk.bzh`
  (`crates/engine/src/links.rs`).
- Version web seulement : la page des versions (`RELEASES_PAGE`) au lieu de la recherche de mise
  à jour, et le téléchargement des modèles OCR (la version de bureau les contient).

## Ce qui n'utilise pas le réseau

- Envoi de formulaire (`SubmitForm`) : jamais envoyé ; un message le signale
  (`crates/ui-egui/src/canvas/area.rs`, `workflows/js_ui.rs`).
- Signatures numériques : les adresses OCSP et CRL des certificats sont lues mais jamais
  contactées ; l'horodatage en ligne (`sign.timestamp`) n'est pas réalisé.
- OCR : modèles intégrés aux installateurs, reconnaissance sur l'ordinateur.
- Aucun outil d'IA : Lusk PDF a retiré le groupe « Assistant IA » de PdfCraft (qui n'avait de
  toute façon aucun code de connexion).
- Correcteur d'orthographe, polices, rendu, export, impression : entièrement locaux.
- OneDrive et Google Drive : Lusk PDF ouvre seulement le dossier local synchronisé.

## Écoute locale

- Le canal de pilotage (`--control`) écoute uniquement `127.0.0.1` sur un port choisi par le
  système, avec une clé renouvelée à chaque lancement, et seulement si l'option est donnée
  (`crates/ui-egui/src/control/transport.rs`).
- Le serveur MCP n'existe que dans l'outil en ligne de commande, lancé à la main
  (`pdfcraft-cli mcp`), sans réseau.
