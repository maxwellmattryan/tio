use crate::{
    cli::{BroadcastArgs, Command}
};

#[derive(structopt::StructOpt)]
pub struct BroadcastCommand {
    #[structopt(flatten)]
    pub broadcast: BroadcastArgs,
}

impl Command for BroadcastCommand {
    fn run(self) -> Result<(), String> {
        let message = match self.broadcast.message {
            Some(m) => m,
            None => String::from("TIO_MESSAGE"),
        };

        Ok(println!("{}", message))
    }
}
