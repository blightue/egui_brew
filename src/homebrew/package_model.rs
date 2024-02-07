use std::fmt::Display;

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

impl Display for PackageBrief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if self.outdated {
            "🔄"
        } else if self.installed {
            "💾"
        } else {
            "🌐"
        };
        let tail_fix = match self.package_type {
            PackageType::Formula => "📦",
            PackageType::Cask => "🍺",
        };
        write!(f, "{} {} {}", prefix, self.name, tail_fix)
    }
}

#[derive(Debug, Clone)]
pub enum PackageType {
    Formula,
    Cask,
}
