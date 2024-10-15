use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(about = "Blackjack game client for NakedMCSE's and VictoriqueMoe's Blackjack servers", long_about = None)]
pub struct BlackJackArgs {
    /// Port of the Blackjack server
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start a new game or resume a currently playing game
    Deal,

    /// Draw another card for the player hand
    Hit {
        /// Token of the game to draw a card for
        #[arg(short, long)]
        token: Option<String>,
    },

    /// Don't draw any more cards and let the dealer draw, eventually deciding a winner
    Stay {
        /// Token of the game to stay for
        #[arg(short, long)]
        token: Option<String>,
    },
    /// Show the number of wins, losses, and draws
    Stats,

    /// See game history
    History {
        /// Date from which to get the history from (yyyy-mm-dd)
        #[arg(short, long)]
        start_date: Option<String>,
    },

    /// Delete Game history for the current device
    Delete {
        /// Token for the game to delete (omit to delete all games)
        #[arg(short, long)]
        token: Option<String>,

        /// Flag to pass to ensure the call is made
        #[arg(short, long)]
        sure: bool,
    },
}
