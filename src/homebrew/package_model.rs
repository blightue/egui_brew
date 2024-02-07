#[derive(Debug, Clone)]
pub struct PackageBrief {
    pub name: String,
    pub package_type: PackageType,
    pub installed: bool,
    pub outdated: bool,
}

impl PackageBrief {
    pub fn new(name: String, package_type: PackageType, installed: bool, outdated: bool) -> Self {
        Self {
            name: name,
            package_type: package_type,
            installed: installed,
            outdated: outdated,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PackageType {
    Formula,
    Cask,
}
