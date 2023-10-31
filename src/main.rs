use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Encode { id } => println!("Got {id}"),
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode an ID.
    Encode {
        /// The ID.
        id: u32,
    },
}
