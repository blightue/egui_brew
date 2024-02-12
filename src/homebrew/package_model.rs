use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct PackageBrief {
    pub name: String,
    pub package_type: PackageType,
    pub package_state: PackageState,
}

impl PackageBrief {
    pub fn new(name: String, package_type: PackageType, installed: bool, outdated: bool) -> Self {
        Self {
            name: name,
            package_type: package_type,
            package_state: if installed {
                if outdated {
                    PackageState::Outdated
                } else {
                    PackageState::Installed
                }
            } else {
                PackageState::Installable
            },
        }
    }
}

impl Display for PackageBrief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.package_state {
            PackageState::Installable => "🌐",
            PackageState::Installed => "💾",
            PackageState::Outdated => "🔄",
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

#[derive(Debug, Clone)]
pub enum PackageState {
    Installable,
    Installed,
    Outdated,
}
