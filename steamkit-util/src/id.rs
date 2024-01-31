use std::str::FromStr;

use steamkit_lang::enums::{EAccountType, EUniverse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdError {
    #[error("invalid universe: {0}")]
    InvalidUniverse(i32),
    #[error("invalid instance: {0}")]
    InvalidInstance(u32),
    #[error("invalid account type: {0}")]
    InvalidAccountType(i32),
    // #[error("invalid type char")]
    // InvalidTypeChar(char),
    // #[error("type doesn't have char")]
    // MissingTypeChar(Type),
    // #[error("failed to parse integer")]
    // ParseIntError(#[from] std::num::ParseIntError),
    // #[error("unknown id error")]
    // Unknown,
    // #[error("steam2 id can only be individual type")]
    // IncompatibleVersion,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Instance {
    All = 0,
    Desktop = 1,
    Console = 2,
    Web = 3,
}

impl Default for Instance {
    fn default() -> Self {
        Instance::All
    }
}

impl TryFrom<u32> for Instance {
    type Error = IdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instance::All),
            1 => Ok(Instance::Desktop),
            2 => Ok(Instance::Console),
            3 => Ok(Instance::Web),
            value => Err(IdError::InvalidInstance(value)),
        }
    }
}

pub struct Id(u64);

impl Id {
    pub fn instance(&self) -> Instance {
        let value = ((self.0 >> 32) & 0x000FFFFF) as u32;
        Instance::try_from(value).expect("invalid instance")
    }

    pub fn universe(&self) -> EUniverse {
        EUniverse((self.0 >> 56) as i32)
    }

    pub fn account_type(&self) -> EAccountType {
        EAccountType(((self.0 >> 52) & 0xF) as i32)
    }

    pub fn account_id(&self) -> u32 {
        (self.0 & 0xFFFFFFFF) as u32
    }
}

impl TryFrom<u64> for Id {
    type Error = IdError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Instance::try_from(((value >> 32) & 0x000FFFFF) as u32)?;
        EAccountType::try_from(((value >> 52) & 0xF) as i32)
            .map_err(|x| IdError::InvalidAccountType(x))?;
        EUniverse::try_from((value >> 56) as i32)
            .map_err(|x: <EUniverse as TryFrom<i32>>::Error| IdError::InvalidUniverse(x))?;

        Ok(Id(value))
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u64() {
        let id = Id::try_from(76561198006409530u64);
        assert!(id.is_ok(), "failed to parse u64");

        let id = id.unwrap();
        assert_eq!(id.universe(), EUniverse::PUBLIC, "universe does not match");
        assert_eq!(
            id.account_type(),
            EAccountType::INDIVIDUAL,
            "type does not match"
        );
        assert_eq!(id.instance(), Instance::Desktop, "instance does not match");
        assert_eq!(id.account_id(), 46143802, "account id does not match");
    }

    #[test]
    fn parse_steam2_id() {
        let id = "STEAM_0:0:23071901".parse::<Id>();
        assert!(id.is_ok(), "failed to parse steam2 id");

        let id = id.unwrap();

        assert_eq!(id.universe(), EUniverse::PUBLIC, "universe does not match");
        assert_eq!(
            id.account_type(),
            EAccountType::INDIVIDUAL,
            "type does not match"
        );
        assert_eq!(id.instance(), Instance::Desktop, "instance does not match");
        assert_eq!(id.account_id(), 46143802, "account id does not match");

        // assert_eq!(id.to_steam2(false).unwrap(), "STEAM_0:0:23071901");
        // assert_eq!(id.to_steam2(true).unwrap(), "STEAM_1:0:23071901");
        // assert_eq!(id.to_steam3().unwrap(), "[U:1:46143802]");
    }

    #[test]
    fn parse_steam3_id() {
        let id = "[U:1:46143802]".parse::<Id>();
        assert!(id.is_ok(), "failed to parse steam3 id");

        let id = id.unwrap();

        assert_eq!(id.universe(), EUniverse::PUBLIC, "universe does not match");
        assert_eq!(
            id.account_type(),
            EAccountType::INDIVIDUAL,
            "type does not match"
        );
        assert_eq!(id.instance(), Instance::Desktop, "instance does not match");
        assert_eq!(id.account_id(), 46143802, "account id does not match");

        // assert_eq!(id.to_steam2(false).unwrap(), "STEAM_0:0:23071901");
        // assert_eq!(id.to_steam2(true).unwrap(), "STEAM_1:0:23071901");
        // assert_eq!(id.to_steam3().unwrap(), "[U:1:46143802]");
    }
}
