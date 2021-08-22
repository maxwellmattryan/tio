use async_trait::async_trait;

use crate::{
    error::Result,
};

/// Trait to implement for commands
#[async_trait]
pub trait Command {
    async fn run(self) -> Result<()>;
}

/// CLI tool for broadcasting and searching messages on the IOTA Tangle
#[derive(structopt::StructOpt)]
#[structopt(
    author = "Matthew Maxwell <maxwellmattryan@gmail.com>", 
    version = env!("CARGO_PKG_VERSION")
)]
pub enum Cli {
    /// Broadcast a message to the IOTA Tangle
    Broadcast(crate::commands::BroadcastCommand),

    // Search,
}

#[async_trait]
impl Command for Cli {
    async fn run(self) -> Result<()> {
        match self {
            Self::Broadcast(c) => c.run().await,
        }
    }
}
