use std::{
    fmt::{Debug, Display},
    str,
};

use base64::{engine::general_purpose, Engine};
use clap::ValueEnum;
use eyre::{eyre, Context, Report, Result};

pub struct Gid {
    pub name: String,
    pub id: String,
    pub id_type: Type,
}

impl Gid {
    pub fn new(name: String, id: String, id_type: Option<Type>) -> Self {
        let id_type = match id_type {
            Some(id_type) => id_type,
            None => {
                if id.chars().any(|c| !c.is_numeric()) {
                    Type::S
                } else if id.parse::<i128>().unwrap() <= i32::MAX.into() {
                    // This is not great, but it's what we do in the Python implementation
                    Type::I
                } else {
                    Type::L
                }
            }
        };
        Self { name, id, id_type }
    }
}

impl TryFrom<String> for Gid {
    type Error = Report;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        let id_bytes = general_purpose::STANDARD
            .decode(id)
            .wrap_err("Input is not valid base64")?;
        let id_string = str::from_utf8(&id_bytes).wrap_err("Input is not valid UTF-8")?;

        match id_string.split_once('\n') {
            Some((name, type_and_id)) if type_and_id.len() >= 2 => {
                let id_type = Type::try_from(&type_and_id[..1])?;
                let id = &type_and_id[1..];
                Ok(Self {
                    name: name.into(),
                    id: id.into(),
                    id_type,
                })
            }
            _ => Err(eyre!("Input is not a valid GID"))?,
        }
    }
}

impl Display for Gid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            general_purpose::STANDARD.encode(format!("{}\n{}{}", self.name, self.id_type, self.id))
        )
    }
}

impl Debug for Gid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ({})", self.name, self.id, self.id_type)
    }
}

#[derive(Clone, ValueEnum)]
pub enum Type {
    /// int
    I,
    /// long
    L,
    /// string
    S,
}

impl TryFrom<&str> for Type {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "i" => Ok(Self::I),
            "l" => Ok(Self::L),
            "d" => Ok(Self::S),
            _ => Err(eyre!("{value} is not a valid ID type")),
        }
    }
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
