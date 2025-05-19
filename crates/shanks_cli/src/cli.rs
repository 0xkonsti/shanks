use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "shanks", version, about, author, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    Play(PlayArgs),
    Debug(DebugArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(name = "play", about = "Play a game of checkers")]
pub struct PlayArgs {}

#[derive(Parser, Debug, Clone)]
#[command(name = "debug", about = "Debug a game of checkers")]
pub struct DebugArgs {}
