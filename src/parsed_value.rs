use crate::investigation::{Digitized, MethodToken};
use chrono::NaiveDate;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum ParsedValue<T> {
    #[default]
    None,
    Fallback(String),
    Some(T),
}

impl<T> ParsedValue<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn some(value: T) -> Self {
        ParsedValue::Some(value)
    }

    pub fn fallback(message: String) -> Self {
        ParsedValue::Fallback(message)
    }

    pub fn is_none(&self) -> bool {
        matches!(self, ParsedValue::None)
    }

    pub fn is_some(&self) -> bool {
        matches!(self, ParsedValue::Some(_))
    }

    pub fn is_fallback(&self) -> bool {
        matches!(self, ParsedValue::Fallback(_))
    }

    pub fn unwrap_fallback(self) -> Option<String> {
        if let ParsedValue::Fallback(msg) = self {
            Some(msg)
        } else {
            None
        }
    }
}

pub trait TryParse: Sized {
    fn try_parse(input: &str) -> Result<Self, String>;
}

impl TryParse for MethodToken {
    fn try_parse(input: &str) -> Result<Self, String> {
        let method = Self::from_string(input);
        if method == MethodToken::None {
            Err(input.to_string())
        } else {
            Ok(method)
        }
    }
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

impl TryParse for NaiveDate {
    fn try_parse(input: &str) -> Result<Self, String> {
        NaiveDate::parse_from_str(input, "%d%m%Y").map_err(|_| input.to_string())
    }
}

impl TryParse for String {
    fn try_parse(input: &str) -> Result<Self, String> {
        Ok(input.to_string())
    }
}

impl TryParse for i32 {
    fn try_parse(input: &str) -> Result<Self, String> {
        input.parse::<i32>().map_err(|_| input.to_string())
    }
}

impl TryParse for f32 {
    fn try_parse(input: &str) -> Result<Self, String> {
        let normalized = input.replace(',', ".");
        normalized.parse::<f32>().map_err(|_| input.to_string())
    }
}

impl<T: TryParse> ParsedValue<T> {
    pub fn parse(input: &str) -> Self {
        if input == "-" {
            ParsedValue::None
        } else {
            match T::try_parse(input) {
                Ok(value) => ParsedValue::Some(value),
                Err(original) => ParsedValue::Fallback(original),
            }
        }
    }
}

impl<T: Display> Display for ParsedValue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParsedValue::None => write!(f, "None"),
            ParsedValue::Some(value) => write!(f, "{}", value),
            ParsedValue::Fallback(original) => write!(f, "Fallback({})", original),
        }
    }
}
