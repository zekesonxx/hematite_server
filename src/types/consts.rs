//! MC Protocol constants.

use std::io::prelude::*;
use std::io;
use std::str::FromStr;

use packet::Protocol;

use num::FromPrimitive;
use rustc_serialize::json::{Json, ToJson};

macro_rules! enum_protocol_impl {
    ($name:ty, $repr:ty, $dec_repr:ident) => {
        impl Protocol for $name {
            type Clean = $name;

            fn proto_len(value: &$name) -> usize { <$repr as Protocol>::proto_len(&(*value as $repr)) }

            fn proto_encode(value: &$name, mut dst: &mut Write) -> io::Result<()> {
                let repr = *value as $repr;
                try!(<$repr as Protocol>::proto_encode(&repr, dst));
                Ok(())
            }

            fn proto_decode(mut src: &mut Read) -> io::Result<$name> {
                let value = try!(<$repr as Protocol>::proto_decode(src));
                match FromPrimitive::$dec_repr(value) {
                    Some(x) => Ok(x),
                    None => Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid enum"))
                }
            }
        }
    }
}

enum_protocol_impl!(Dimension, i8, from_i8);

#[repr(i8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dimension {
    Nether = -1,
    Overworld = 0,
    End = 1
}

impl FromPrimitive for Dimension {
    fn from_i64(n: i64) -> Option<Dimension> {
        match n {
            -1 => Some(Dimension::Nether),
            0 => Some(Dimension::Overworld),
            1 => Some(Dimension::End),
            _ => None
        }
    }

    fn from_u64(n: u64) -> Option<Dimension> {
        match n {
            0 => Some(Dimension::Overworld),
            1 => Some(Dimension::End),
            _ => None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Black       = 0x0,
    DarkBlue    = 0x1,
    DarkGreen   = 0x2,
    DarkCyan    = 0x3,
    DarkRed     = 0x4,
    Purple      = 0x5,
    Gold        = 0x6,
    Gray        = 0x7,
    DarkGray    = 0x8,
    Blue        = 0x9,
    BrightGreen = 0xa,
    Cyan        = 0xb,
    Red         = 0xc,
    Pink        = 0xd,
    Yellow      = 0xe,
    White       = 0xf
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        match *self {
            Color::Black => "black",
            Color::DarkBlue => "dark_blue",
            Color::DarkGreen => "dark_green",
            Color::DarkCyan => "dark_aqua",
            Color::DarkRed => "dark_red",
            Color::Purple => "dark_purple",
            Color::Gold => "gold",
            Color::Gray => "gray",
            Color::DarkGray => "dark_gray",
            Color::Blue => "blue",
            Color::BrightGreen => "green",
            Color::Cyan => "aqua",
            Color::Red => "red",
            Color::Pink => "light_purple",
            Color::Yellow => "yellow",
            Color::White => "white"
        }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(string: &str) -> Result<Color, ()> {
        match string {
            "black"        => Ok(Color::Black),
            "dark_blue"    => Ok(Color::DarkBlue),
            "dark_green"   => Ok(Color::DarkGreen),
            "dark_aqua"    => Ok(Color::DarkCyan),
            "dark_red"     => Ok(Color::DarkRed),
            "dark_purple"  => Ok(Color::Purple),
            "gold"         => Ok(Color::Gold),
            "gray"         => Ok(Color::Gray),
            "dark_gray"    => Ok(Color::DarkGray),
            "blue"         => Ok(Color::Blue),
            "green"        => Ok(Color::BrightGreen),
            "aqua"         => Ok(Color::Cyan),
            "red"          => Ok(Color::Red),
            "light_purple" => Ok(Color::Pink),
            "yellow"       => Ok(Color::Yellow),
            "white"        => Ok(Color::White),
            _              => Err(())
        }
    }
}

impl ToJson for Color {
    fn to_json(&self) -> Json {
        self.as_ref().to_json()
    }
}


enum_protocol_impl!(Difficulty, u8, from_u8);

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Medium = 2,
    Hard = 3
}

impl FromPrimitive for Difficulty {
    fn from_u64(n: u64) -> Option<Difficulty> {
        match n {
            0 => Some(Difficulty::Peaceful),
            1 => Some(Difficulty::Easy),
            2 => Some(Difficulty::Medium),
            3 => Some(Difficulty::Hard),
            _ => None
        }
    }
    fn from_i64(n: i64) -> Option<Difficulty> {
        match n {
            0 => Some(Difficulty::Peaceful),
            1 => Some(Difficulty::Easy),
            2 => Some(Difficulty::Medium),
            3 => Some(Difficulty::Hard),
            _ => None
        }
    }
}

enum_protocol_impl!(Gamemode, u8, from_u8);

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
//FIXME(zekesonxx): Doesn't support hardcode mode
pub enum Gamemode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3
}

impl Gamemode {
    /// Gets the bit flag of a gamemode
    /// Used for the PlayerAbilities packet
    pub fn abilities(me: Self) -> i8 {
        use self::Gamemode::*;
        match me {
            // creative | can fly | flying | god
            Survival => 0b0000,
            Creative => 0b1101,
            Adventure => 0b0001,
            Spectator => 0b0101
        }
    }
}

impl FromPrimitive for Gamemode {
    fn from_u64(n: u64) -> Option<Gamemode> {
        match n {
            0 => Some(Gamemode::Survival),
            1 => Some(Gamemode::Creative),
            2 => Some(Gamemode::Adventure),
            3 => Some(Gamemode::Spectator),
            _ => None
        }
    }
    fn from_i64(n: i64) -> Option<Gamemode> {
        match n {
            0 => Some(Gamemode::Survival),
            1 => Some(Gamemode::Creative),
            2 => Some(Gamemode::Adventure),
            3 => Some(Gamemode::Spectator),
            _ => None
        }
    }
}

