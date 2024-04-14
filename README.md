# 🍺egui_brew

A macOS GUI application for package manager [Homebrew](https://brew.sh/) which is based on [egui](https://www.egui.rs/).

![GitHub License](https://img.shields.io/github/license/blightue/brew_egui)
![GitHub Release](https://img.shields.io/github/v/release/blightue/brew_egui)

![demo](/assets/images/Demo_0.1.0.png)

## Features
| Feature          | Impl version | Detail                                                               |
| ---------------- | ------------ | -------------------------------------------------------------------- |
| Package Query    | 0.1.0(init)  | Filter by Formula/Cask Installable/Installed/Outdated Search by name |
| Package Manage   | 0.1.0(init)  | Install/Uninstall/Upgrade pkg                                        |
| Progressive Load | 0.2.0        | Progressive loading package listthread                               |

## Todolist
- [ ] CI/CD auto distribute
- [ ] manage package behind proxy
- [ ] homebrew cli test and init
- [x] progressive loading package list
- [ ] multi-pkg manage

## How to use

### Download prebuilt app

1. Download the latest version from [release](https://github.com/blightue/egui_brew/releases)
2. Unzip the file and open EGUI_Brew.app
   > There is no certification for egui_brew at the current time, so it's necessary to bypass it by opening with (command ⌘ + right click -> open)
4. Move the app to the Application folder
5. Open the app and enjoy it

### Build from source
1. Clone the repo
2. Install [cargo-bundle](https://crates.io/crates/cargo-bundle) by `cargo install cargo-bundle`
3. Run `cargo bundle --release` to build the app
