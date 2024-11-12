# boilerplate-iceb-win

![GitHub Rust](https://github.com/sup2ak/boilerplate-iceb-win/actions/workflows/rust.yml/badge.svg)
![GitHub](https://img.shields.io/github/license/sup2ak/boilerplate-iceb-win)

You can read readme in english too here : 
[![](https://img.shields.io/badge/English-000?style=for-the-badge&logo=github&logoColor=white)](README.md)

Ce projet est un boilerplate pour créer des applications graphiques en Rust en utilisant la bibliothèque Iced. Il est conçu pour vous aider à démarrer rapidement avec une configuration de base.

### Prérequis

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Inno Setup](https://jrsoftware.org/isdl.php) (pour créer un installateur Windows)

### Documentation

- [Iced](https://book.iced.rs/index.html)
- [Iced GitHub](https://github.com/iced-rs/iced)
- [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Installation

1. Clonez le dépôt dans un dossier nommé `yourApp` :
   ```bash
   git clone https://github.com/sup2ak/boilerplate-iceb-win.git yourApp
   cd yourApp
   ```

2. Installez les dépendances :
   ```bash
   cargo install cargo-watch
   ```

### Commandes

- **Build** : Compilez le projet en mode release.
  ```bash
  cargo build --release
  ```

- **Mode Watch** : Recompilez automatiquement le projet lors des modifications.
  ```bash
  cargo watch -x run
  ```

- **Clean** : Supprimez les fichiers de build.
  ```bash
  cargo clean
  ```

- **Créer un installateur** 
- (Nécessite Inno Setup et sera ajouté à l'avenir) :

### Contribuer

- Les pull requests sont les bienvenues. Pour des changements majeurs, veuillez d'abord ouvrir une issue pour discuter de ce que vous aimeriez changer.

### Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.