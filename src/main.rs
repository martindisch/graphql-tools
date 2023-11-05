use std::{fmt::Display, str};

use arboard::Clipboard;
use base64::{engine::general_purpose, Engine};
use clap::{Parser, Subcommand, ValueEnum};
use eyre::{eyre, Context, Result};

// Implementation of ID serialization according to
// https://github.com/ChilliCream/graphql-platform/blob/main/src/HotChocolate/Core/src/Types/Types/Relay/IdSerializer.cs

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut clipboard = Clipboard::new()?;
    match cli.command {
        Commands::Encode { name, id, id_type } => {
            let id = general_purpose::STANDARD.encode(format!("{name}\n{id_type}{id}"));
            clipboard.set_text(&id)?;
            println!("{id}",)
        }
        Commands::Decode { id } => {
            let id_bytes = general_purpose::STANDARD
                .decode(id)
                .wrap_err("Input is not valid base64")?;
            let id_string = str::from_utf8(&id_bytes).wrap_err("Input is not valid UTF-8")?;

            match id_string.split_once('\n') {
                Some((name, type_and_id)) if type_and_id.len() >= 2 => {
                    let id_type = &type_and_id[..1];
                    let id = &type_and_id[1..];
                    clipboard.set_text(id)?;
                    println!("{name} {id} ({id_type})")
                }
                _ => Err(eyre!("Input is not a valid GID"))?,
            }
        }
    }

    Ok(())
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode an ID to a base64 GID and copy it to the clipboard.
    Encode {
        /// Name of the type, e.g. Product.
        name: String,
        /// ID you want to encode, e.g. 1234.
        id: u32,
        /// Underlying type of the ID.
        #[arg(short = 't', long = "type", default_value_t = Type::I, value_enum)]
        id_type: Type,
    },
    /// Decode a base64 GID and copy it to the clipboard.
    Decode {
        /// ID you want to decode, e.g. UHJvZHVjdAppMTIzNA==.
        id: String,
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
