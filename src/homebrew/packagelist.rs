use crate::homebrew::brew_cli::BrewCli;
use async_trait::async_trait;
use std::error::Error;
use std::fmt::Display;

use super::{
    async_loader::Load,
    package_model::{PackageBrief, PackageState, PackageType},
};

// type CliVecOutput = CliResult<CliOutput<Vec<String>>>;
pub type PackageListLoader = super::async_loader::AsyncLoader<PackageList>;

#[derive(Clone)]
pub struct PackageList {
    pub list: Vec<PackageBrief>,
}

#[derive(Debug)]
struct NotExcuteError {}

impl Error for NotExcuteError {
    fn description(&self) -> &str {
        "Not Excute Error"
    }
}

impl Display for NotExcuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Not Excute Error")
    }
}

impl NotExcuteError {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn is_loaded(&self) -> bool {
        self.list.len() > 0
    }

    pub async fn load_all(&mut self) {
        let installed_formulae = BrewCli::list_installed_formula().await.unwrap().result;
        let installed_casks = BrewCli::list_installed_cask().await.unwrap().result;
        let installable_formulae = BrewCli::list_installable_formulae().await.unwrap().result;
        let installable_casks = BrewCli::list_installable_casks().await.unwrap().result;
        let outdated_formulae = BrewCli::list_outdated_formulae().await.unwrap().result;
        let outdated_casks = BrewCli::list_outdated_casks().await.unwrap().result;

        self.add_package_list(
            PackageType::Formula,
            &installable_formulae,
            &installed_formulae,
            &outdated_formulae,
        );
        self.add_package_list(
            PackageType::Cask,
            &installable_casks,
            &installed_casks,
            &outdated_casks,
        );
    }

    fn add_package_list(
        &mut self,
        package_type: PackageType,
        all: &Vec<String>,
        installed: &Vec<String>,
        outdated: &Vec<String>,
    ) {
        for package in all {
            let installed = installed.contains(&package);
            let outdated = outdated.contains(&package);
            // let package_state =
            //     PackageBrief::new(package.into(), package_type.clone(), installed, outdated);
            // self.list.push(package_state);
        }
    }

    pub fn from_all(all_formulae_list: Vec<String>, all_casks_list: Vec<String>) -> Self {
        let mut list = vec![];
        for formula in all_formulae_list {
            list.push(PackageBrief::new(
                formula,
                PackageType::Formula,
                PackageState::Undefined,
            ));
        }
        for cask in all_casks_list {
            list.push(PackageBrief::new(
                cask,
                PackageType::Cask,
                PackageState::Undefined,
            ));
        }
        list.sort_by(|a, b| a.name.cmp(&b.name));
        Self { list: list }
    }

    pub fn push_installed(
        &mut self,
        installed_formulae: Vec<String>,
        installed_casks: Vec<String>,
    ) {
        for package in &mut self.list {
            match package.package_type {
                PackageType::Formula => {
                    if installed_formulae.contains(&package.name) {
                        package.package_state = PackageState::Installed;
                    } else {
                        package.package_state = PackageState::Installable;
                    }
                }
                PackageType::Cask => {
                    if installed_casks.contains(&package.name) {
                        package.package_state = PackageState::Installed;
                    } else {
                        package.package_state = PackageState::Installable;
                    }
                }
            }
        }
    }

    pub fn push_outdated(&mut self, outdated_formulae: Vec<String>, outdated_casks: Vec<String>) {
        for package in &mut self.list {
            match package.package_type {
                PackageType::Formula => {
                    if outdated_formulae.contains(&package.name) {
                        package.package_state = PackageState::Outdated;
                    }
                }
                PackageType::Cask => {
                    if outdated_casks.contains(&package.name) {
                        package.package_state = PackageState::Outdated;
                    }
                }
            }
        }
    }
}

#[async_trait]
impl Load for PackageList {
    async fn load(&mut self) {
        self.load_all().await;
    }
}

unsafe impl Send for PackageList {}
