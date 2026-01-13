use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
    str::{FromStr, ParseBoolError},
};

use crate::extension::DefaultParse;

pub mod command;
pub mod ladderlog;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
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
impl Player {
    pub fn allow_rename(&self, allowed: bool) {
        crate::allow_rename_player(self, allowed)
    }
    pub fn allow_team_change(&self, allowed: bool) {
        crate::allow_team_change_player(self, allowed)
    }
    pub fn kill(&self) {
        crate::kill(self)
    }
    pub fn message(&self, message: &str) {
        crate::player_message(self, message);
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

pub struct Authority(pub String);

impl Display for Authority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub enum AccessLevel {
    Owner = 0,
    Administrator = 1,
    Moderator = 2,
    _3 = 3,
    _4 = 4,
    Armatrator = 5,
    _6 = 6,
    TeamLeader = 7,
    TeamMember = 8,
    _9 = 9,
    _10 = 10,
    _11 = 11,
    LocalUser = 12,
    _13 = 13,
    _14 = 14,
    RemoteUser = 15,
    FallenFromGrace = 16,
    Shunned = 17,
    _18 = 18,
    Authenticated = 19,
    #[default]
    Program = 20,
}

impl FromStr for AccessLevel {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(num) => match num {
                0 => Ok(AccessLevel::Owner),
                1 => Ok(AccessLevel::Administrator),
                2 => Ok(AccessLevel::Moderator),
                3 => Ok(AccessLevel::_3),
                4 => Ok(AccessLevel::_4),
                5 => Ok(AccessLevel::Armatrator),
                6 => Ok(AccessLevel::_6),
                7 => Ok(AccessLevel::TeamLeader),
                8 => Ok(AccessLevel::TeamMember),
                9 => Ok(AccessLevel::_9),
                10 => Ok(AccessLevel::_10),
                11 => Ok(AccessLevel::_11),
                12 => Ok(AccessLevel::LocalUser),
                13 => Ok(AccessLevel::_13),
                14 => Ok(AccessLevel::_14),
                15 => Ok(AccessLevel::RemoteUser),
                16 => Ok(AccessLevel::FallenFromGrace),
                17 => Ok(AccessLevel::Shunned),
                18 => Ok(AccessLevel::_18),
                19 => Ok(AccessLevel::Authenticated),
                20 => Ok(AccessLevel::Program),
                _ => Ok(AccessLevel::Program),
            },
            Err(e) => Err(e),
        }
    }
}

impl Display for AccessLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &raw const self as u8)
    }
}
