use std::{
    fmt::Display,
    net::Ipv4Addr,
    num::{ParseFloatError, ParseIntError},
    str::{FromStr, ParseBoolError},
};

use crate::extension::DefaultParse;

#[derive(Default, Debug, PartialEq)]
pub struct Player(pub String);
impl FromStr for Player {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ScreenName(pub String);
impl FromStr for ScreenName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Display for ScreenName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Team(pub String);
impl FromStr for Team {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Default, Debug, PartialEq)]
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

impl Display for SlashMe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Default, Debug, PartialEq)]
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

impl Display for Ping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Default, Debug, PartialEq)]
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

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Default, Debug, PartialEq)]
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

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Default, Debug, PartialEq)]
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

impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_secs_f32())
    }
}

#[derive(Default, Debug, PartialEq)]
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

impl Display for IpAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                Some(inner) => inner,
                None => std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            }
        )
    }
}

#[derive(Default, Debug, PartialEq)]
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

impl Display for TeamMembers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self
                .0
                .iter()
                .map(|x| x.0.clone())
                .reduce(|mut accum, next| {
                    accum.push_str(&format!(" {}", next));
                    accum
                }) {
                Some(s) => s,
                None => String::from(""),
            }
        )
    }
}

/// Trail & Text colors go from 0-15 and are clamped (actual = min(input, 15)).
/// Bike colors go from 0-15 and are modulated (actual = input % 15).
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }
}
