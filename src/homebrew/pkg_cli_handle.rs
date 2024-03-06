use std::fmt::Display;

use super::cli_handle::CliHandle;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PkgManageType {
    Install,
    Uninstall,
    Upgrade,
}

impl Display for PkgManageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PkgManageType::Install => write!(f, "install"),
            PkgManageType::Uninstall => write!(f, "uninstall"),
            PkgManageType::Upgrade => write!(f, "upgrade"),
        }
    }
}

pub struct PkgCliHandle {
    pub cli_handle: CliHandle,
    pub package: String,
    pub cli_type: PkgManageType,
}

unsafe impl Send for PkgCliHandle {}

impl PkgCliHandle {
    pub fn new(cli_handle: CliHandle, package: String, cli_type: PkgManageType) -> Self {
        Self {
            cli_handle: cli_handle,
            package: package,
            cli_type: cli_type,
        }
    }
}
