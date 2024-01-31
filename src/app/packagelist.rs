use crate::homebrew::brew_cli::{BrewCli, CliOutput, CliResult};
use std::error::Error;
use std::fmt::Display;

type CliVecOutput = CliResult<CliOutput<Vec<String>>>;

pub struct PackageList {
    pub installed_formulae: CliVecOutput,
    pub installed_casks: CliVecOutput,
    pub installable_formulae: CliVecOutput,
    pub installable_casks: CliVecOutput,
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
        Self {
            installed_formulae: Err(Box::new(NotExcuteError::new())),
            installed_casks: Err(Box::new(NotExcuteError::new())),
            installable_formulae: Err(Box::new(NotExcuteError::new())),
            installable_casks: Err(Box::new(NotExcuteError::new())),
        }
    }

    async fn load_installed_formulae(&mut self) {
        self.installed_formulae = BrewCli::list_installed_formula().await;
    }

    async fn load_installed_casks(&mut self) {
        self.installed_casks = BrewCli::list_installed_cask().await;
    }

    async fn load_installable_formulae(&mut self) {
        self.installable_formulae = BrewCli::list_installable_formulae().await;
    }

    async fn load_installable_casks(&mut self) {
        self.installable_casks = BrewCli::list_installable_casks().await;
    }

    pub async fn load_all(&mut self) {
        self.load_installed_formulae().await;
        self.load_installed_casks().await;
        self.load_installable_formulae().await;
        self.load_installable_casks().await;
    }
}
