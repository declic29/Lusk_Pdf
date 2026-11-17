# Vos données et votre vie privée

Vos documents vous appartiennent. Lusk PDF fonctionne entièrement sur votre ordinateur : pas de compte à créer, pas de publicité, pas de mesure d'audience, et aucun document n'est jamais envoyé.

## Ce qui reste sur votre ordinateur

- Vos documents, et les copies de récupération de votre travail en cours.
- Vos signatures enregistrées, vos tampons, votre dictionnaire personnel et vos préférences.

L'affichage, les annotations, les formulaires, les signatures, la reconnaissance du texte (OCR), le correcteur d'orthographe, la compression et les exports se font sur place, sans connexion à Internet.

## Les seuls moments où Lusk PDF utilise Internet

- **Mises à jour** : Lusk PDF demande à GitHub, où l'association Lusk publie ses versions, le numéro de la dernière version. Il le fait au démarrage si la case est cochée, et quand vous cliquez sur **Rechercher des mises à jour**. Aucun document ni aucune donnée personnelle n'est envoyé ; comme tout site web, GitHub voit l'adresse de votre connexion. Une nouvelle version n'est téléchargée que si vous l'acceptez. Pour ne plus rechercher au démarrage, décochez la case dans **Aide › À propos › Mises à jour** ou dans les Préférences.
- **Liens** : un lien d'un document vers un site web ou une adresse e-mail s'ouvre dans votre navigateur ou votre messagerie, seulement après votre accord. Les liens du menu **Aide** (site de Lusk, page GitHub, écrire à l'association) s'ouvrent de la même façon.

## Ce que Lusk PDF ne fait jamais

- **Envoyer un formulaire** : si un formulaire contient un bouton « Envoyer » vers une adresse Internet, Lusk PDF n'envoie rien et vous le signale.
- **Vérifier une signature en ligne** : la vérification des signatures numériques utilise seulement ce que contient le document et les certificats de votre ordinateur.
- **Les outils d'intelligence artificielle** (résumer, interroger, traduire un document) sont affichés mais désactivés : Lusk PDF n'en contient pas et n'envoie vos documents à aucun service.

## OneDrive et Google Drive

Les boutons OneDrive et Google Drive de l'Accueil ouvrent simplement le dossier que ces applications synchronisent sur votre ordinateur. Lusk PDF ne se connecte à aucun compte : c'est l'application OneDrive ou Google Drive qui envoie ensuite les fichiers dans votre espace en ligne.

## Le pilotage par des scripts ou des assistants

Il est désactivé, sauf si Lusk PDF est lancé avec l'option **--control**. Même alors, il n'écoute que votre propre ordinateur, et seuls les programmes qui connaissent sa clé secrète, renouvelée à chaque lancement, peuvent s'en servir.

## Avant de partager un document

Un PDF peut contenir plus que ce qui est affiché : nom de l'auteur, commentaires, pièces jointes, calques cachés, ancien texte.

1. Ouvrez les **Propriétés du document** (**Ctrl+D**) pour vérifier le titre et l'auteur.
2. Dans **Protéger un PDF**, **Retirer les informations masquées** supprime les métadonnées, pièces jointes, calques cachés et autres données invisibles.
3. Pour effacer définitivement un passage visible, utilisez **Biffer un PDF**.
