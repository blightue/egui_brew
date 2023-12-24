use std::error::Error;
use std::result::Result;

use tokio::process::Command;

type CLIResult<T> = Result<T, Box<dyn Error>>;

pub struct BrewCLI {}

pub struct CLIOutput<T> {
    result: T,
    raw_result: String,
}

impl BrewCLI {
    pub fn new() -> Self {
        Self {}
    }

    async fn brew_commands(arguments: &Vec<&str>) -> CLIResult<String> {
        let mut cmd = Command::new("brew");
        for arg in arguments {
            cmd.arg(arg);
        }
        let output = cmd.output().await?;
        let result = String::from_utf8(output.stdout)?;
        Ok(result)
    }

    async fn get_output_and_splitby_line(
        arguments: &Vec<&str>,
    ) -> CLIResult<CLIOutput<Vec<String>>> {
        let raw_result = BrewCLI::brew_commands(arguments).await?;
        let result: Vec<String> = raw_result
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Ok(CLIOutput { result, raw_result })
    }

    pub async fn list_installed_formula() -> CLIResult<CLIOutput<Vec<String>>> {
        let result = BrewCLI::get_output_and_splitby_line(&vec!["list", "--formula"]).await?;
        Ok(result)
    }

    pub async fn list_installed_cask() -> CLIResult<CLIOutput<Vec<String>>> {
        let result = BrewCLI::get_output_and_splitby_line(&vec!["list", "--cask"]).await?;
        Ok(result)
    }

    pub async fn list_installable_formulae() -> CLIResult<CLIOutput<Vec<String>>> {
        let result = BrewCLI::get_output_and_splitby_line(&vec!["formulae"]).await?;
        Ok(result)
    }

    pub async fn list_installable_casks() -> CLIResult<CLIOutput<Vec<String>>> {
        let result = BrewCLI::get_output_and_splitby_line(&vec!["casks"]).await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn log_result() {
        let formula_result = BrewCLI::list_installed_formula().await.unwrap();
        println!(
            "installed formula:\n{:?} ...",
            &formula_result.result[0..10]
        );
        let cask_result = BrewCLI::list_installed_cask().await.unwrap();
        println!("installed cask:\n{:?} ...", &cask_result.result[0..10]);
        let formulae_result = BrewCLI::list_installable_formulae().await.unwrap();
        println!(
            "installable formula:\n{:?} ...",
            &formulae_result.result[0..10]
        );
        let casks_result = BrewCLI::list_installable_casks().await.unwrap();
        println!("installable cask:\n{:?} ...", &casks_result.result[0..10]);
    }
}
