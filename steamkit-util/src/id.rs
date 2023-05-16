use lazy_static::lazy_static;
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use regex::Regex;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref STEAM2_ID_REGEX: Regex = Regex::new(r"^STEAM_([0-9]):([0-1]):([0-9]+)$").unwrap();
    static ref STEAM3_ID_REGEX: Regex =
        Regex::new(r"^\[([a-zA-Z]):([0-5]):([0-9]+)(:[0-9]+)?]$").unwrap();
}

#[derive(Error, Debug)]
pub enum IdError {
    #[error("invalid universe")]
    InvalidUniverse(#[from] TryFromPrimitiveError<Universe>),
    #[error("invalid type")]
    InvalidType(#[from] TryFromPrimitiveError<Type>),
    #[error("invalid instance")]
    InvalidInstance(#[from] TryFromPrimitiveError<Instance>),
    #[error("invalid type char")]
    InvalidTypeChar(char),
    #[error("type doesn't have char")]
    MissingTypeChar(Type),
    #[error("failed to parse integer")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("unknown id error")]
    Unknown,
    #[error("steam2 id can only be individual type")]
    IncompatibleVersion,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Clone, Copy)]
#[repr(u64)]
pub enum Universe {
    #[num_enum(default)]
    Invalid = 0,
    Public = 1,
    Beta = 2,
    Internal = 3,
    Dev = 4,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Clone, Copy)]
#[repr(u64)]
pub enum Type {
    #[num_enum(default)]
    Invalid = 0,
    Individual = 1,
    Multiseat = 2,
    GameServer = 3,
    AnonGameServer = 4,
    Pending = 5,
    ContentServer = 6,
    Clan = 7,
    Chat = 8,
    P2PSuperSeeder = 9,
    AnonUser = 10,
}

impl TryInto<char> for Type {
    type Error = IdError;

    fn try_into(self) -> Result<char, Self::Error> {
        match &self {
            Type::Invalid => Ok('I'),
            Type::Individual => Ok('U'),
            Type::Multiseat => Ok('M'),
            Type::GameServer => Ok('G'),
            Type::AnonGameServer => Ok('A'),
            Type::Pending => Ok('P'),
            Type::ContentServer => Ok('C'),
            Type::Clan => Ok('g'),
            Type::Chat => Ok('T'),
            Type::AnonUser => Ok('a'),
            _ => Err(IdError::MissingTypeChar(self)),
        }
    }
}

impl TryFrom<char> for Type {
    type Error = IdError;

    fn try_from(chr: char) -> Result<Self, Self::Error> {
        match chr {
            'I' => Ok(Type::Invalid),
            'U' => Ok(Type::Individual),
            'M' => Ok(Type::Multiseat),
            'G' => Ok(Type::GameServer),
            'A' => Ok(Type::AnonGameServer),
            'P' => Ok(Type::Pending),
            'C' => Ok(Type::ContentServer),
            'g' => Ok(Type::Clan),
            'T' | 'L' | 'c' => Ok(Type::Chat),
            'a' => Ok(Type::AnonUser),
            _ => Err(IdError::InvalidTypeChar(chr)),
        }
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Clone, Copy)]
#[repr(u64)]
pub enum Instance {
    #[num_enum(default)]
    All = 0,
    Desktop = 1,
    Console = 2,
    Web = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Id {
    pub universe: Universe,
    pub type_: Type,
    pub instance: Instance,
    pub account: u64,
}

impl Default for Id {
    fn default() -> Self {
        Self {
            universe: Universe::Invalid,
            type_: Type::Invalid,
            instance: Instance::All,
            account: 0,
        }
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = STEAM2_ID_REGEX.captures(s) {
            let mut universe = Universe::try_from(caps[1].parse::<u64>()?)?;
            if universe == Universe::Invalid {
                universe = Universe::Public;
            }

            let type_ = Type::Individual;
            let instance = Instance::Desktop;
            let account = caps[3].parse::<u64>()? * 2 + caps[2].parse::<u64>()?;

            Ok(Self {
                universe,
                type_,
                instance,
                account,
            })
        } else if let Some(caps) = STEAM3_ID_REGEX.captures(s) {
            let chr = caps[1].chars().next().unwrap();
            let universe = Universe::try_from(caps[2].parse::<u64>()?)?;
            let account = caps[3].parse::<u64>()?;

            let mut instance = Instance::All as u64;
            if let Some(instance_id) = caps.get(4) {
                instance = Instance::try_from(instance_id.as_str().parse::<u64>()?)? as u64;
            }

            let type_ = match chr {
                'U' => {
                    if caps.get(4).is_none() {
                        instance = Instance::Desktop as u64;
                    }
                    Type::Individual
                }
                'c' => {
                    instance |= (0x000FFFFF + 1) >> 1;
                    Type::Chat
                }
                'L' => {
                    instance |= (0x000FFFFF + 1) >> 2;
                    Type::Chat
                }
                chr => Type::try_from(chr)?,
            };

            let instance = Instance::try_from_primitive(instance)?;

            Ok(Self {
                universe,
                type_,
                instance,
                account,
            })
        } else if let Ok(value) = s.parse::<u64>() {
            Self::from_u64(value)
        } else {
            Err(IdError::Unknown)
        }
    }
}

impl Id {
    pub fn from_u64(value: u64) -> Result<Self, IdError> {
        let account = value & 0xFFFFFFFF;
        let instance = Instance::try_from((value >> 32) & 0x000FFFFF)?;
        let type_ = Type::try_from((value >> 52) & 0xF)?;
        let universe = Universe::try_from(value >> 56)?;

        Ok(Self {
            universe,
            type_,
            instance,
            account,
        })
    }

    pub fn from_account_id(account: u64) -> Self {
        Self {
            universe: Universe::Public,
            type_: Type::Individual,
            instance: Instance::Desktop,
            account,
        }
    }

    pub fn to_u64(&self) -> u64 {
        let universe = self.universe as u64;
        let type_ = self.type_ as u64;
        let instance = self.instance as u64;
        let account = self.account as u64;

        (universe << 56) | (type_ << 52) | (instance << 32) | account
    }

    pub fn to_steam2(&self, new_format: bool) -> Result<String, IdError> {
        if self.type_ != Type::Individual {
            Err(IdError::IncompatibleVersion)
        } else {
            let mut universe = self.universe as u64;
            if !new_format && universe == 1 {
                universe = 0;
            }

            Ok(format!(
                "STEAM_{}:{}:{}",
                universe,
                self.account & 1,
                self.account / 2
            ))
        }
    }

    pub fn to_steam3(&self) -> Result<String, IdError> {
        let mut chr: char = self.type_.try_into()?;

        if self.instance as u64 & ((0x000FFFFF + 1) >> 1) > 0 {
            chr = 'c';
        }

        if self.instance as u64 & ((0x000FFFFF + 1) >> 2) > 0 {
            chr = 'L';
        }

        let use_instance = self.type_ == Type::AnonGameServer
            || self.type_ == Type::Multiseat
            || (self.type_ == Type::Individual && self.instance != Instance::Desktop);

        Ok(format!(
            "[{}:{}:{}{}]",
            chr,
            self.universe as u64,
            self.account as u64,
            if use_instance {
                format!(":{}", self.instance as u64)
            } else {
                "".into()
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u64() {
        let id = Id::from_u64(76561198006409530);
        assert!(id.is_ok(), "failed to parse u64");

        let id = id.unwrap();
        assert_eq!(id.universe, Universe::Public, "universe does not match");
        assert_eq!(id.type_, Type::Individual, "type does not match");
        assert_eq!(id.instance, Instance::Desktop, "instance does not match");
        assert_eq!(id.account, 46143802, "account id does not match");
    }

    #[test]
    fn parse_steam2_id() {
        let id = "STEAM_0:0:23071901".parse::<Id>();
        assert!(id.is_ok(), "failed to parse steam2 id");

        let id = id.unwrap();

        assert_eq!(id.universe, Universe::Public, "universe does not match");
        assert_eq!(id.type_, Type::Individual, "type does not match");
        assert_eq!(id.instance, Instance::Desktop, "instance does not match");
        assert_eq!(id.account, 46143802, "account id does not match");

        assert_eq!(id.to_steam2(false).unwrap(), "STEAM_0:0:23071901");
        assert_eq!(id.to_steam2(true).unwrap(), "STEAM_1:0:23071901");
        assert_eq!(id.to_steam3().unwrap(), "[U:1:46143802]");
    }

    #[test]
    fn parse_steam3_id() {
        let id = "[U:1:46143802]".parse::<Id>();
        assert!(id.is_ok(), "failed to parse steam3 id");

        let id = id.unwrap();

        assert_eq!(id.universe, Universe::Public, "universe does not match");
        assert_eq!(id.type_, Type::Individual, "type does not match");
        assert_eq!(id.instance, Instance::Desktop, "instance does not match");
        assert_eq!(id.account, 46143802, "account id does not match");

        assert_eq!(id.to_steam2(false).unwrap(), "STEAM_0:0:23071901");
        assert_eq!(id.to_steam2(true).unwrap(), "STEAM_1:0:23071901");
        assert_eq!(id.to_steam3().unwrap(), "[U:1:46143802]");
    }
}
