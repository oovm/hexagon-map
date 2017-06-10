use std::{
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};

use super::*;

impl FromStr for Orientation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        Err(lower)
    }
}

impl Debug for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::H(true) => f.write_str("→→"),
            Orientation::P(true) => f.write_str("↑→"),
            Orientation::Q(true) => f.write_str("↑←"),
            Orientation::H(false) => f.write_str("←←"),
            Orientation::P(false) => f.write_str("↓←"),
            Orientation::Q(false) => f.write_str("↓→"),
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::H(true) => f.write_str("Right"),
            Orientation::P(true) => f.write_str("UpRight"),
            Orientation::Q(true) => f.write_str("UpLeft"),
            Orientation::H(false) => f.write_str("Left"),
            Orientation::P(false) => f.write_str("DownLeft"),
            Orientation::Q(false) => f.write_str("DownRight"),
        }
    }
}
