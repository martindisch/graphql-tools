use std::fmt::Display;

use base64::{engine::general_purpose, Engine};
use clap::{Parser, Subcommand, ValueEnum};

// Implementation of ID serialization according to
// https://github.com/ChilliCream/graphql-platform/blob/main/src/HotChocolate/Core/src/Types/Types/Relay/IdSerializer.cs

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Encode { name, id, id_type } => {
            println!(
                "{}",
                general_purpose::STANDARD_NO_PAD.encode(format!("{name}\n{id_type}{id}"))
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
        /// The data type.
        #[arg(short = 't', long = "type", default_value_t = Type::I, value_enum)]
        id_type: Type,
    },
}

#[derive(Clone, ValueEnum)]
enum Type {
    /// int
    I,
    /// long
    L,
    /// string
    S,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::I => write!(f, "i"),
            Type::L => write!(f, "l"),
            Type::S => write!(f, "d"),
        }
    }
}
