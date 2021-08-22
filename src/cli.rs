/// Trait to implement for commands.
pub trait Command {
    fn run(self) -> Result<(), String>;
}

/// CLI tool for broadcasting and searching messages on the IOTA Tangle.
#[derive(structopt::StructOpt)]
#[structopt(
    author = "Matthew Maxwell <maxwellmattryan@gmail.com>", 
    version = env!("CARGO_PKG_VERSION")
)]
pub enum Cli {
    /// Broadcast a message to the IOTA Tangle.
    Broadcast(crate::commands::BroadcastCommand),

    // Search,
}

impl Command for Cli {
    fn run(self) -> Result<(), String> {
        match self {
            Self::Broadcast(c) => c.run(),
        }
    }
}

#[derive(Debug, structopt::StructOpt)]
pub struct BroadcastArgs {
    /// The message to broadcast on the IOTA Tangle (must be UTF-8).
    #[structopt(short, long)]
    pub message: Option<String>,
}
