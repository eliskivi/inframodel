use crate::{ParseResult, TryParse};
use std::fmt;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Classification {
    pub name: ParseResult<ClassificationName>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum ClassificationName {
    #[default]
    GEO,
    ISO,
}

impl TryParse for ClassificationName {
    fn try_parse(input: &str) -> Result<Self, String> {
        match input.trim().to_uppercase().as_str() {
            // TODO: Default should just be GEO.. ?
            "GEO" => Ok(ClassificationName::GEO),
            "ISO" => Ok(ClassificationName::ISO),
            _ => Err(input.to_string()),
        }
    }
}

impl fmt::Display for ClassificationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            ClassificationName::GEO => "GEO",
            ClassificationName::ISO => "SFS-EN ISO 14688-2",
        };
        write!(f, "{}", token_str)
    }
}
