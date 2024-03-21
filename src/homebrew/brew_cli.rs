use std::io::BufWriter;
use std::process::Stdio;
use std::result::Result;
use std::sync::mpsc::channel;
use std::thread;
use std::{error::Error, io::BufReader};

use std::process::Command as stdCommand;
use tokio::process::Command;

use super::cli_handle::CliHandle;
use super::package_model::PackageType;
use super::pkg_cli_handle::{PkgCliHandle, PkgManageType};

pub type CliResult<T> = Result<T, Box<dyn Error + Send>>;

pub struct BrewCli {}

pub struct CliOutput<T> {
    pub result: T,
    pub raw_result: String,
}

impl BrewCli {
    pub fn new() -> Self {
        Self {}
    }

    async fn brew_commands(arguments: &Vec<&str>) -> CliResult<String> {
        let mut cmd = Command::new("brew");
        for arg in arguments {
            cmd.arg(arg);
        }
        let output = cmd
            .output()
            .await
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;
        let result = String::from_utf8(output.stdout)
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;
        Ok(result)
    }

    fn brew_commands_with_stdout(arguments: &Vec<&str>) -> CliResult<CliHandle> {
        let (out_tx, out_rx) = channel();
        let (in_tx, in_rx) = channel();
        let (child_tx, child_rx) = channel();
        let args = arguments
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        thread::spawn(move || {
            let mut child = stdCommand::new("brew")
                .args(args)
                .stdout(Stdio::piped())
                .stdin(Stdio::piped())
                .spawn()
                .unwrap();
            out_tx.send(child.stdout.take().unwrap()).unwrap();
            in_tx.send(child.stdin.take().unwrap()).unwrap();
            child_tx.send(child).unwrap();
        });
        Ok(CliHandle::new(
            BufReader::new(out_rx.recv().unwrap()),
            BufWriter::new(in_rx.recv().unwrap()),
            child_rx.recv().unwrap(),
        ))
    }

    async fn get_output_and_splitby_line(
        arguments: &Vec<&str>,
    ) -> CliResult<CliOutput<Vec<String>>> {
        let raw_result = BrewCli::brew_commands(arguments).await?;
        let result: Vec<String> = raw_result
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Ok(CliOutput { result, raw_result })
    }

    pub async fn list_installed_formula() -> CliResult<CliOutput<Vec<String>>> {
        let result = BrewCli::get_output_and_splitby_line(&vec!["list", "--formula"]).await?;
        Ok(result)
    }

    pub async fn list_installed_cask() -> CliResult<CliOutput<Vec<String>>> {
        let result = BrewCli::get_output_and_splitby_line(&vec!["list", "--cask"]).await?;
        Ok(result)
    }

    pub async fn list_installable_formulae() -> CliResult<CliOutput<Vec<String>>> {
        let result = BrewCli::get_output_and_splitby_line(&vec!["formulae"]).await?;
        Ok(result)
    }

    pub async fn list_installable_casks() -> CliResult<CliOutput<Vec<String>>> {
        let result = BrewCli::get_output_and_splitby_line(&vec!["casks"]).await?;
        Ok(result)
    }

    pub async fn list_outdated_formulae() -> CliResult<CliOutput<Vec<String>>> {
        let result = BrewCli::get_output_and_splitby_line(&vec!["outdated", "--formula"]).await?;
        Ok(result)
    }

    pub async fn list_outdated_casks() -> CliResult<CliOutput<Vec<String>>> {
        let result = BrewCli::get_output_and_splitby_line(&vec!["outdated", "--cask"]).await?;
        Ok(result)
    }

    pub async fn show_info(
        package: &str,
        package_type: PackageType,
    ) -> CliResult<CliOutput<String>> {
        let type_desc = match package_type {
            PackageType::Formula => "--formula",
            PackageType::Cask => "--cask",
        };
        let result = BrewCli::brew_commands(&vec!["info", "--json=v2", type_desc, package]).await?;
        Ok(CliOutput {
            result: result.clone(),
            raw_result: result,
        })
    }

    pub fn manage_package(
        package_name: &str,
        package_type: PackageType,
        manage_type: PkgManageType,
    ) -> CliResult<PkgCliHandle> {
        let manage_command = match manage_type {
            PkgManageType::Install => "install",
            PkgManageType::Uninstall => "uninstall",
            PkgManageType::Upgrade => "upgrade",
        };
        let clihandle = BrewCli::brew_commands_with_stdout(&vec![
            manage_command,
            package_type.to_command(),
            package_name,
        ]);
        match clihandle {
            Ok(handle) => Ok(PkgCliHandle::new(
                handle,
                package_name.to_string(),
                manage_type,
            )),
            Err(e) => Err(e),
        }
    }

    pub fn uninstall_package(
        package_name: &str,
        package_type: PackageType,
    ) -> CliResult<CliHandle> {
        BrewCli::brew_commands_with_stdout(&vec![
            "uninstall",
            package_type.to_command(),
            package_name,
        ])
    }

    pub fn upgrade_package(package_name: &str, package_type: PackageType) -> CliResult<CliHandle> {
        BrewCli::brew_commands_with_stdout(&vec![
            "upgrade",
            package_type.to_command(),
            package_name,
        ])
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn log_result() {
        let formula_result = BrewCli::list_installed_formula().await.unwrap();
        println!(
            "installed formula:\n{:?} ...",
            &formula_result.result[0..10]
        );
        let cask_result = BrewCli::list_installed_cask().await.unwrap();
        println!("installed cask:\n{:?} ...", &cask_result.result[0..10]);
        let formulae_result = BrewCli::list_installable_formulae().await.unwrap();
        println!(
            "installable formula:\n{:?} ...",
            &formulae_result.result[0..10]
        );
        let casks_result = BrewCli::list_installable_casks().await.unwrap();
        println!("installable cask:\n{:?} ...", &casks_result.result[0..10]);
    }

    #[tokio::test]
    async fn test_outdated_cli() {
        let result = BrewCli::list_outdated_formulae().await.unwrap();
        println!("outdated formula:\n{:?} ...", &result.result[0..10]);
    }

    #[tokio::test]
    async fn test_show_info() {
        let result = BrewCli::show_info("git", PackageType::Formula)
            .await
            .unwrap();
        println!("{}", result.result);
    }

    #[test]
    fn test_install_package() {
        let result =
            BrewCli::manage_package("qbittorrent", PackageType::Cask, PkgManageType::Install);
        if let Ok(handle) = result {
            loop {
                if let Ok(output) = handle.cli_handle.stdout.try_recv() {
                    println!("{}", output);
                }
            }
        }
    }

    #[test]
    fn test_uninstall_package() {
        let result =
            BrewCli::manage_package("qbittorrent", PackageType::Cask, PkgManageType::Uninstall);
        if let Ok(handle) = result {
            loop {
                if let Ok(output) = handle.cli_handle.stdout.try_recv() {
                    println!("{}", output);
                }
            }
        }
    }
}
