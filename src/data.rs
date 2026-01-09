use std::{
    num::{ParseFloatError, ParseIntError},
    str::{FromStr, ParseBoolError},
};

use crate::extension::DefaultParse;

#[derive(Default, Debug)]
pub struct Player(pub String);
impl FromStr for Player {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Default, Debug)]
pub struct ScreenName(pub String);
impl FromStr for ScreenName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Default, Debug)]
pub struct Team(pub String);
impl FromStr for Team {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Default, Debug)]
pub struct SlashMe(pub bool);
impl FromStr for SlashMe {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(s) => Ok(Self(s)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Default, Debug)]
pub struct Ping(pub u32);
impl FromStr for Ping {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f32>() {
            Ok(s) => Ok(Self((s * 1000.0).round() as u32)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Default, Debug)]
pub struct Score(pub i64);
impl FromStr for Score {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(s) => Ok(Self(s)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Default, Debug)]
pub struct Time(pub chrono::DateTime<chrono::FixedOffset>);

impl FromStr for Time {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match chrono::DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S %z") {
            Ok(s) => Ok(Time(s)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Default, Debug)]
pub struct Duration(pub std::time::Duration);

impl FromStr for Duration {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(s) => Ok(Duration(std::time::Duration::from_secs_f32(s))),
            Err(e) => Err(e),
        }
    }
}

#[derive(Default, Debug)]
pub struct IpAddr(pub Option<std::net::IpAddr>);

impl FromStr for IpAddr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(s) => Ok(Self(Some(s))),
            Err(_) => Ok(Self(None)),
        }
    }
}

#[derive(Default, Debug)]
pub struct TeamMembers(pub Vec<Player>);

impl TeamMembers {
    pub fn from_slice(list: &[&str]) -> TeamMembers {
        TeamMembers(
            list.iter()
                .map(|x| Player::parse_or_default(x.trim()))
                .collect(),
        )
    }
}
