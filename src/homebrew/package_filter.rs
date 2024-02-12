use super::package_model::{PackageBrief, PackageState, PackageType};

pub struct PackageFilter {
    pub name_filter: String,
    pub type_filter: PackageTypeFilter,
    pub state_filter: PackageStateFilter,
}

impl PackageFilter {
    pub fn new() -> Self {
        Self {
            name_filter: "".to_string(),
            type_filter: PackageTypeFilter::new(),
            state_filter: PackageStateFilter::new(),
        }
    }

    pub fn is_filtered(&self, package: &PackageBrief) -> bool {
        let name_filtered = if self.name_filter.is_empty() {
            true
        } else {
            package.name.contains(&self.name_filter)
        };

        if !name_filtered {
            return false;
        }

        let type_filtered: bool = match package.package_type {
            PackageType::Formula => self.type_filter.is_formula,
            PackageType::Cask => self.type_filter.is_cask,
        };

        if !type_filtered {
            return false;
        }
        let state_filtered: bool = match package.package_state {
            PackageState::Installable => self.state_filter.is_installable,
            PackageState::Installed => self.state_filter.is_installed,
            PackageState::Outdated => self.state_filter.is_outdated,
        };

        if !state_filtered {
            return false;
        }

        true
    }
}

pub struct PackageTypeFilter {
    pub is_formula: bool,
    pub is_cask: bool,
}

impl PackageTypeFilter {
    pub fn new() -> Self {
        Self {
            is_formula: true,
            is_cask: true,
        }
    }
}

pub struct PackageStateFilter {
    pub is_installable: bool,
    pub is_installed: bool,
    pub is_outdated: bool,
}

impl PackageStateFilter {
    pub fn new() -> Self {
        Self {
            is_installable: true,
            is_installed: true,
            is_outdated: true,
        }
    }
}
