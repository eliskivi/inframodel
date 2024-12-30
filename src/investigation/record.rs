use crate::{ParseResult, TryParse};

use std::fmt;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Record {
    pub number: ParseResult<i32>,
    pub driller: ParseResult<String>,
    pub inspector: ParseResult<String>,
    pub processor: ParseResult<String>,
    pub digitalized: ParseResult<Digitized>,
    pub condition: ParseResult<String>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Digitized {
    #[default]
    No,
    Yes,
}

impl TryParse for Digitized {
    fn try_parse(input: &str) -> Result<Self, String> {
        if input == "D" {
            Ok(Digitized::Yes)
        } else {
            Ok(Digitized::No)
        }
    }
}

impl fmt::Display for Digitized {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            Digitized::No => "Not digitized",
            Digitized::Yes => "Digitized",
        };
        write!(f, "{}", token_str)
    }
}
