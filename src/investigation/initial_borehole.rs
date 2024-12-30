use crate::{ParseResult, TryParse};
use std::fmt;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct InitialBorehole {
    pub depth: ParseResult<f32>,
    pub method: ParseResult<InitialBoreToken>,
    pub soil_type: ParseResult<String>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum InitialBoreToken {
    #[default]
    Unknown,
    SI,
    LK,
    AP,
    LY,
    VA,
    JA,
}

impl TryParse for InitialBoreToken {
    fn try_parse(input: &str) -> Result<Self, String> {
        match input.trim().to_uppercase().as_str() {
            "SI" => Ok(InitialBoreToken::SI),
            "LK" => Ok(InitialBoreToken::LK),
            "AP" => Ok(InitialBoreToken::AP),
            "LY" => Ok(InitialBoreToken::LY),
            "VA" => Ok(InitialBoreToken::VA),
            "JA" => Ok(InitialBoreToken::JA),
            _ => Err(input.to_string()),
        }
    }
}

impl fmt::Display for InitialBoreToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            InitialBoreToken::SI => "Through protective pipe",
            InitialBoreToken::LK => "Shovel pit",
            InitialBoreToken::AP => "Opening with drill",
            InitialBoreToken::LY => "Hammered",
            InitialBoreToken::VA => "Water initiation",
            InitialBoreToken::JA => "Continues previous investigation",
            InitialBoreToken::Unknown => "Unknown method",
        };
        write!(f, "{}", token_str)
    }
}
