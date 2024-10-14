# Blackjack Game Client

This is the Blackjack Game Client for [NakedMCSE](https://github.com/nakedmcse) and [VictoriqueMoe](https://github.com/VictoriqueMoe) Blackjack servers.

Download one of their servers and run it and the client should play nice so you don't have to cURL every time.

This port was taken from the [Typescript](https://github.com/nakedmcse/TSBlackjack) version of the server but the responses should map nicely across all servers so should parse correctly here.

The default port in use here is `3000` but this can be set using the `-p` flag for each call.

For more information, see the relevant server's documentation.

## Install

Clone the repo and run `cargo install` or use `cargo run` from inside the repo

## Usage 

```
Blackjack game client for NakedMCSE's and VictoriqueMoe's Blackjack servers

Usage: rustjack [OPTIONS] <COMMAND>

Commands:
  deal     Start a new game or resume a currently playing game
  hit      Draw another card for the player hand
  stay     Don't draw any more cards and let the dealer draw, eventually deciding a winner
  stats    Show the number of wins, losses, and draws
  history  See game history
  delete   Delete Game history for the current device
  help     Print this message or the help of the given subcommand(s)

Options:
  -p, --port <PORT>  Port of the Blackjack server [default: 3000]
  -h, --help         Print help
  -V, --version      Print version
```
