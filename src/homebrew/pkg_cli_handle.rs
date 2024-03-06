use super::cli_handle::CliHandle;

pub enum PkgManageType {
    Install,
    Uninstall,
    Upgrade,
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
