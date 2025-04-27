# otdrs

**otdrs** est un utilitaire écrit en Rust pour analyser et générer des fichiers SOR (format Bellcore/Telcordia SR‑4731) issus de tests OTDR (Optical Time‑Domain Reflectometry).  
Il permet de :

- Convertir un fichier `.sor` en JSON ou CBOR
- Modifier son contenu (ex : suppression de blocs propriétaires)
- Réécrire un fichier binaire SOR valide

Rust a été choisi pour sa sûreté mémoire, sa robustesse et la richesse de la bibliothèque `nom` pour le parsing.  
`serde` est utilisé pour la (dé)sérialisation vers JSON/CBOR.

---

## Installation

Prerequis : Rust et Cargo (via https://rustup.rs)

```powershell
# Cloner le dépôt
git clone git@github.com:fabry44/otdr-parser.git
cd otdr-parser

# Compiler tous les binaires en mode release
cargo build --release
```

Vous obtiendrez :
- `target/release/otdrs.exe`  
- `target/release/wotdrs.exe`  

---

## Commandes Principales

### otdrs  
Conversion d’un SOR vers JSON ou CBOR, avec options de sortie :

```powershell
# Affiche l’aide
.\target\release\otdrs.exe --help

# Convertir en JSON sur stdout
.\target\release\otdrs.exe --input chemin/vers/fichier.sor

# Convertir en JSON et enregistrer
.\target\release\otdrs.exe --input chemin/vers/fichier.sor --output sortie.json

# Choisir le format CBOR
.\target\release\otdrs.exe --input chemin/vers/fichier.sor -f cbor --output sortie.cbor

# Appliquer un script de modification avant export
.\target\release\otdrs.exe --input chemin/vers/fichier.sor -m script_modif.sh --output sortie.json
```

### wotdrs  
Reconstruction d’un fichier SOR à partir d’un JSON généré par otdrs, avec suppression des blocs propriétaires :

```powershell
# Affiche l’aide
.\target\release\wotdrs.exe --help

# Générer un .sor épuré
.\target\release\wotdrs.exe --input sortie.json --output data/output.sor
```

---

## Testing

Toutes les suites de tests, benchmarks et documentation incluent également le binaire `wotdrs`, notamment avec l’exemple Viavi SmartOTDR (test_parse_viavi_sor et bench_wotdrs_pipeline).

---

## Versions

* 1.0.0 - refactored to avoid some beginner Rust errors; SORFile now owns its data. Updated dependencies.
* 0.2.0 - added support for parsing Viavi SmartOTDR files
* 0.1.0 - initial release

---

## Fork et remerciements

Ce projet est un **fork** du dépôt original [JamesHarrison/otdrs](https://github.com/JamesHarrison/otdrs) (GPL‑3.0). Merci à James Harrison pour l’outil original et sa licence ouverte. Merci à toutes et tous pour votre soutien et vos contributions.

---

## Licence

Ce projet est distribué sous licence **GPL‑3.0**.  
Vous êtes libre de copier, modifier, redistribuer et même commercialiser votre version, à condition de :

1. Conserver et publier le code source complet sous GPL‑3.0  
2. Conserver les mentions de copyright et la notice de licence  
3. Fournir le code sous licence GPL‑3.0 à vos utilisateurs finaux  

Voir le fichier `LICENSE` pour le texte intégral de la licence.

## Écriture de fichiers SOR

`otdrs` propose un support expérimental pour générer des fichiers SOR à partir de structures de données Rust. Le bloc map est entièrement recalculé lors de l’écriture : chaque BlockInfo reçoit un numéro de révision et un en‑tête, tandis que tailles et compteurs sont générés dynamiquement. Cela simplifie la sérialisation, car il est pratiquement impossible de calculer précisément les tailles à l’avance.

Editors sont responsables de la cohérence des données modifiées ; par exemple, si vous changez le nombre de points dans un `DataPointsAtScaleFactor`, vous devez mettre à jour manuellement le champ `n_points`, car `otdrs` ne le fait pas pour vous.

Actuellement, les landmarks et `LinkParameters` ne sont pas réécrits, faute de exemples de données.

Note : le binaire `wotdrs` implémente le pipeline inverse (JSON→SOR), supprime les blocs propriétaires et reconstruit un `.sor` valide depuis un JSON généré par `otdrs`. Merci à toutes et tous pour vos retours et contributions !