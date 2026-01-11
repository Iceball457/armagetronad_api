use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

pub(crate) trait DefaultParse<T: FromStr + Default> {
    fn parse_or_default(raw: &str) -> T;
}

impl<T: FromStr + Default> DefaultParse<T> for T {
    fn parse_or_default(raw: &str) -> T {
        match T::from_str(raw) {
            Ok(value) => value,
            Err(_) => T::default(),
        }
    }
}

pub trait IpAddrExt {
    fn parse_or_default(raw: &str) -> IpAddr;
}

impl IpAddrExt for IpAddr {
    fn parse_or_default(raw: &str) -> IpAddr {
        match IpAddr::from_str(raw) {
            Ok(addr) => addr,
            Err(_) => IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        }
    }
}

pub(crate) trait BoolExt {
    fn byte(&self) -> u8;
}

impl BoolExt for bool {
    fn byte(&self) -> u8 {
        match *self {
            true => 1,
            false => 0,
        }
    }
}
