use super::package_model::{PackageBrief, PackageState, PackageType};

#[derive(Clone)]
pub struct PackageBriefList {
    pub list: Vec<PackageBrief>,
}

impl PackageBriefList {
    pub fn new() -> Self {
        Self { list: vec![] }
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
