# 🍺egui_brew

A macOS GUI application for package manager [Homebrew](https://brew.sh/) which is based on [egui](https://www.egui.rs/).

![GitHub License](https://img.shields.io/github/license/blightue/brew_egui)
![GitHub Release](https://img.shields.io/github/v/release/blightue/brew_egui)

![demo](/assets/images/Demo_0.1.0.png)

## Features
| Feature        | Impl version | Detail                                                               |
| -------------- | ------------ | -------------------------------------------------------------------- |
| Package Query  | 0.1.0(init)  | Filter by Formula/Cask Installable/Installed/Outdated Search by name |
| Package Manage | 0.1.0(init)  | Install/Uninstall/Upgrade pkg                                        |

## Todolist
- [ ] CI/CD auto distribute
- [ ] manage package behind proxy
- [ ] homebrew cli test and init
- [ ] progressive loading package list
- [ ] multi-pkg manage

## How to use

### Download prebuilt app

1. Download the latest version from [release](https://github.com/blightue/egui_brew/releases)
2. Unzip the file and move the app to the Application folder
3. Open the app and enjoy it

### Build from source
1. Clone the repo
2. Install [cargo-bundle](https://crates.io/crates/cargo-bundle) by `cargo install cargo-bundle`
3. Run `cargo bundle --release` to build the app
