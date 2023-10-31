use base64::{engine::general_purpose, Engine};
use clap::{Parser, Subcommand};

// Implementation of ID serialization according to
// https://github.com/ChilliCream/graphql-platform/blob/main/src/HotChocolate/Core/src/Types/Types/Relay/IdSerializer.cs

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Encode { name, id } => {
            println!(
                "{}",
                general_purpose::STANDARD_NO_PAD.encode(format!("{name}\ni{id}"))
            )
        }
    }
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode an ID.
    Encode {
        /// The name of the type.
        name: String,
        /// The ID.
        id: u32,
    },
}
