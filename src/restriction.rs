use std::str::FromStr;
use std::{fmt, fmt::Display, fmt::Formatter};

#[derive(Debug)]
pub enum Restriction {
    PIDMAX,
    CPU,
    MEMORY
}

impl FromStr for Restriction {
    type Err = ();

    fn from_str(string: &str) -> Result<Namespace, ()> {
        match string {
            "pidmax" => Ok(Restriction::PIDMAX),
            "cpu" => Ok(Restriction::CPU),
            "memory" => Ok(Restriction::MEMORY)
        }
    }
}

impl Display for Restriction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}