#boilerplate-iceb-win

# boilerplate-iceb-win

![GitHub Rust](https://github.com/sup2ak/boilerplate-iceb-win/actions/workflows/rust.yml/badge.svg)
![GitHub](https://img.shields.io/github/license/sup2ak/boilerplate-iceb-win)

You can read readme in french too here : 
[![](https://img.shields.io/badge/Français-000?style=for-the-badge&logo=github&logoColor=white)](README.fr.md)

This project is a boilerplate for creating graphical applications in Rust using the Iced library. It is designed to help you quickly start with a basic setup.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Inno Setup](https://jrsoftware.org/isdl.php) (to create a Windows installer)

### Documentations

- [Iced](https://book.iced.rs/index.html)
- [Iced Github](https://github.com/iced-rs/iced)
- [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)


### Installation

1. Clone the repository into a folder named as `yourApp`:
   ```bash
   git clone https://github.com/sup2ak/boilerplate-iceb-win.git yourApp
   cd yourApp
   ```

2. Install dependencies:
   ```bash
   cargo install cargo-watch
   ```

### Commands

- **Build**: Compile the project in release mode.
  ```bash
  cargo build --release
  ```

- **Watch Mode**: Automatically recompile the project on changes.
  ```bash
  cargo watch -x run
  ```

- **Clean**: Remove build files.
  ```bash
  cargo clean
  ```

- **Create an Installer** 
- (Need Inno Setup & Will be added in the future):

### Contributing

- Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

### License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.