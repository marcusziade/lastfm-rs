// Worker management command implementations

use crate::cli::{
    error::{CliError, Result},
    traits::{Command, CommandArgs, CommandOutput, OutputMetadata},
};
use async_trait::async_trait;

/// Deploy worker command
pub struct DeployCommand;

#[async_trait]
impl Command for DeployCommand {
    async fn execute(&self, _args: &CommandArgs) -> Result<CommandOutput> {
        // Execute wrangler deploy
        let output = std::process::Command::new("wrangler")
            .arg("deploy")
            .output()
            .map_err(|e| CliError::worker(format!("Failed to run wrangler: {e}")))?;

        if !output.status.success() {
            return Err(CliError::worker(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let data = serde_json::json!({
            "status": "deployed",
            "output": String::from_utf8_lossy(&output.stdout)
        });

        Ok(CommandOutput {
            data,
            metadata: OutputMetadata::default(),
        })
    }

    fn name(&self) -> &str {
        "worker.deploy"
    }

    fn description(&self) -> &str {
        "Deploy the worker to Cloudflare"
    }

    fn validate_args(&self, _args: &CommandArgs) -> Result<()> {
        Ok(())
    }
}
