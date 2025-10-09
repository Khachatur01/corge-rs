use std::process::Command;
use anyhow::Result;

pub trait ExecuteCommand {
    fn execute(&mut self, log: bool) -> Result<String>;
}

impl ExecuteCommand for Command {
    fn execute(&mut self, log: bool) -> Result<String> {
        if log {
            log::trace!("{}", self.cli_str());
        }

        match self.output() {
            Ok(output) => {
                if output.status.success() {
                    let std_out: String = String::from_utf8_lossy(&output.stdout).to_string();

                    log::trace!("{std_out}");

                    Ok(std_out)
                } else {
                    anyhow::bail!(String::from_utf8_lossy(&output.stderr).to_string());
                }
            },
            Err(error) => Err(anyhow::anyhow!(error))
        }
    }
}

pub trait CliStr {
    fn cli_str(&self) -> String;
}

impl CliStr for Command {
    fn cli_str(&self) -> String {
        let mut parts: Vec<&str> = vec![
            self.get_program().to_str().unwrap_or("[Non-UTF8 Program]")
        ];

        let args: Vec<&str> = self.get_args()
            .map(|arg| arg.to_str().unwrap_or("[Non-UTF8 Argument]"))
            .collect();

        parts.extend(args);

        parts.join(" ")
    }
}
