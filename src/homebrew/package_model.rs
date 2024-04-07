use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct PackageBrief {
    pub name: String,
    pub package_type: PackageType,
    pub package_state: PackageState,
}

impl PackageBrief {
    pub fn new(name: String, package_type: PackageType, package_state: PackageState) -> Self {
        Self {
            name: name,
            package_type: package_type,
            package_state: package_state,
        }
    }
}

impl Display for PackageBrief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.package_state {
            PackageState::Installable => "🌐",
            PackageState::Installed => "💾",
            PackageState::Outdated => "🔄",
            PackageState::Undefined => "❓",
        };
        let tail_fix = match self.package_type {
            PackageType::Formula => "📦",
            PackageType::Cask => "🍺",
        };
        write!(f, "{} {} {}", prefix, self.name, tail_fix)
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PackageType {
    Formula,
    Cask,
}

impl Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            PackageType::Formula => "Formulae📦",
            PackageType::Cask => "Casks🍺",
        };
        write!(f, "{}", desc)
    }
}

impl PackageType {
    pub fn to_command(&self) -> &str {
        match self {
            PackageType::Formula => "--formula",
            PackageType::Cask => "--cask",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PackageState {
    Installable,
    Installed,
    Outdated,
    Undefined,
}
