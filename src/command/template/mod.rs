mod list;
mod templates;
mod r#use;

pub use list::List;
pub use r#use::Use;

use saucer::{clap, Parser};
use serde::Serialize;

use crate::options::GithubTemplate;
use crate::utils::client::StudioClientConfig;
use crate::{command::RoverOutput, Result};

#[derive(Debug, Clone, Parser, Serialize)]
pub struct Template {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, Parser, Serialize)]
enum Command {
    /// Use a template to generate code
    Use(Use),

    /// List available templates that can be used
    List(List),
}

impl Template {
    pub(crate) fn run(&self, client_config: StudioClientConfig) -> Result<RoverOutput> {
        match &self.command {
            Command::Use(use_template) => use_template.run(client_config),
            Command::List(list) => list.run(),
        }
    }
}
