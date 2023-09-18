use tokio::process::Command;


pub struct Commands{

}

impl Commands {
    pub fn new() -> Self {
        Self {

          }
    }

    pub async fn excute(command: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = Command::new("brew");
        cmd.arg(command);
        let output = cmd.output().await.unwrap();
        let result = String::from_utf8(output.stdout).unwrap();
        Ok(result)
    }
}