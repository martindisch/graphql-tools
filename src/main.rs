use arboard::Clipboard;
use clap::{Parser, Subcommand};
use eyre::Result;

mod gid;
use gid::{Gid, Type};

// Implementation of ID serialization according to
// https://github.com/ChilliCream/graphql-platform/blob/main/src/HotChocolate/Core/src/Types/Types/Relay/IdSerializer.cs

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut clipboard = Clipboard::new()?;
    match cli.command {
        Commands::Encode { name, id, id_type } => {
            let gid = Gid::new(name, id, id_type);
            let encoded_gid = gid.to_string();
            clipboard.set_text(&encoded_gid)?;
            println!("{encoded_gid}")
        }
        Commands::Decode { id } => {
            let gid = Gid::try_from(id)?;
            clipboard.set_text(&gid.id)?;
            println!("{gid:#?}");
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
        id: String,
        /// Underlying type of the ID.
        #[arg(short = 't', long = "type")]
        id_type: Option<Type>,
    },
    /// Decode a base64 GID and copy it to the clipboard.
    Decode {
        /// ID you want to decode, e.g. UHJvZHVjdAppMTIzNA==.
        id: String,
    },
}
