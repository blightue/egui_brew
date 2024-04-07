use async_trait::async_trait;

use super::{async_loader::Load, brew_cli::BrewCli, package_model::PackageState};

pub struct PackageAllNameListLoader {
    installable_name_loader: PackageNameListLoader,
    installed_name_loader: PackageNameListLoader,
    outdated_name_loader: PackageNameListLoader,

    pub is_installable_retrieved: bool,
    pub is_installed_retrieved: bool,
    pub is_outdated_retrieved: bool,
}

impl PackageAllNameListLoader {
    pub fn new() -> Self {
        Self {
            installable_name_loader: PackageNameListLoader::new(PackageNameList::new(
                PackageState::Installable,
            )),
            installed_name_loader: PackageNameListLoader::new(PackageNameList::new(
                PackageState::Installed,
            )),
            outdated_name_loader: PackageNameListLoader::new(PackageNameList::new(
                PackageState::Outdated,
            )),
            is_installable_retrieved: false,
            is_installed_retrieved: false,
            is_outdated_retrieved: false,
        }
    }

    pub fn get_by_state(&mut self, state: PackageState) -> Option<PackageNameList> {
        let result = match state {
            PackageState::Installable => self.installable_name_loader.get(),
            PackageState::Installed => self.installed_name_loader.get(),
            PackageState::Outdated => self.outdated_name_loader.get(),
            _ => panic!("Invalid PackageState"),
        };

        if result.is_some() {
            match state {
                PackageState::Installable => self.is_installable_retrieved = true,
                PackageState::Installed => self.is_installed_retrieved = true,
                PackageState::Outdated => self.is_outdated_retrieved = true,
                _ => panic!("Invalid PackageState"),
            }
        }

        result
    }
}

pub type PackageNameListLoader = super::async_loader::AsyncLoader<PackageNameList>;

pub struct PackageNameList {
    pkg_state: PackageState,
    pub casks: Vec<String>,
    pub formulae: Vec<String>,
}

impl PackageNameList {
    pub fn new(pkg_state: PackageState) -> Self {
        Self {
            pkg_state,
            casks: Vec::new(),
            formulae: Vec::new(),
        }
    }

    async fn load_all(&mut self) {
        let (casks, formulae) = match self.pkg_state {
            PackageState::Installable => (
                BrewCli::list_installable_casks().await.unwrap().result,
                BrewCli::list_installable_formulae().await.unwrap().result,
            ),
            PackageState::Installed => (
                BrewCli::list_installed_cask().await.unwrap().result,
                BrewCli::list_installed_formula().await.unwrap().result,
            ),
            PackageState::Outdated => (
                BrewCli::list_outdated_casks().await.unwrap().result,
                BrewCli::list_outdated_formulae().await.unwrap().result,
            ),
            _ => {
                return;
            }
        };
        self.casks = casks;
        self.formulae = formulae;
    }
}

#[async_trait]
impl Load for PackageNameList {
    async fn load(&mut self) {
        self.load_all().await;
    }
}
