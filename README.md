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

## Fork et Crédit

Ce projet est un **fork** du dépôt original [JamesHarrison/otdrs](https://github.com/JamesHarrison/otdrs) (GPL‑3.0).  
Merci à James Harrison pour l’outil original et sa licence ouverte.

---

## Licence

Ce projet est distribué sous licence **GPL‑3.0**.  
Vous êtes libre de copier, modifier, redistribuer et même commercialiser votre version, à condition de :

1. Conserver et publier le code source complet sous GPL‑3.0  
2. Conserver les mentions de copyright et la notice de licence  
3. Fournir le code sous licence GPL‑3.0 à vos utilisateurs finaux  

Voir le fichier `LICENSE` pour le texte intégral de la licence.