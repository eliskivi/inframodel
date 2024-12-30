use crate::{ParseResult, TryParse};

use std::fmt;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Termination {
    pub token: ParseResult<TerminationToken>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum TerminationToken {
    #[default]
    Unknown,
    TM,
    KI,
    KL,
    KA,
    KK,
    MS,
    KN,
    JA,
}

impl TryParse for TerminationToken {
    fn try_parse(input: &str) -> Result<Self, String> {
        match input.trim().to_uppercase().as_str() {
            "TM" => Ok(TerminationToken::TM),
            "KI" => Ok(TerminationToken::KI),
            "KL" => Ok(TerminationToken::KL),
            "KA" => Ok(TerminationToken::KA),
            "KK" => Ok(TerminationToken::KK),
            "MS" => Ok(TerminationToken::MS),
            "KN" => Ok(TerminationToken::KN),
            "JA" => Ok(TerminationToken::JA),
            _ => Err(input.to_string()),
        }
    }
}

impl fmt::Display for Termination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref token) = self.token.format_display() {
            writeln!(f, "Termination reason: {}", token)?;
        }
        Ok(())
    }
}

impl fmt::Display for TerminationToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            TerminationToken::TM => "Dense soil layer",
            TerminationToken::KI => "Estimated rock or boulder",
            TerminationToken::KL => "Rock, boulder or bedrock contact",
            TerminationToken::KA => "Bedrock contact (verified rock)",
            TerminationToken::KK => "Bedrock surface (verified with trial pit)",
            TerminationToken::MS => "Specified depth",
            TerminationToken::KN => "Bridging between stones and boulders",
            TerminationToken::JA => "Continues as another investigation",
            TerminationToken::Unknown => "Unknown reason",
        };
        write!(f, "{}", token_str)
    }
}
