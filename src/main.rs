mod cli;
mod command;
mod response;

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::BlackJackArgs::parse();
    let client = command::GameClient::new(cli.port);
    client.play(cli.command).await?;

    Ok(())
}
