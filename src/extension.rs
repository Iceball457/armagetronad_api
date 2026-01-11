use std::str::FromStr;

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
