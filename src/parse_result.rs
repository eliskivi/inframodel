use chrono::NaiveDate;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum ParseResult<T> {
    #[default]
    None,
    Fallback(String),
    Parsed(T),
}

impl<T> ParseResult<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn some(value: T) -> Self {
        ParseResult::Parsed(value)
    }

    pub fn fallback(message: String) -> Self {
        ParseResult::Fallback(message)
    }

    pub fn is_none(&self) -> bool {
        matches!(self, ParseResult::None)
    }

    pub fn is_some(&self) -> bool {
        matches!(self, ParseResult::Parsed(_))
    }

    pub fn is_fallback(&self) -> bool {
        matches!(self, ParseResult::Fallback(_))
    }

    pub fn unwrap_fallback(self) -> Option<String> {
        if let ParseResult::Fallback(msg) = self {
            Some(msg)
        } else {
            None
        }
    }
}

pub trait TryParse: Sized {
    fn try_parse(input: &str) -> Result<Self, String>;
}

impl TryParse for NaiveDate {
    fn try_parse(input: &str) -> Result<Self, String> {
        // "00000000" is considered unknown date
        if input == "00000000" {
            return Err(input.to_string());
        }
        NaiveDate::parse_from_str(input, "%d%m%Y")
            .map_err(|_| input.to_string())
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

impl<T: TryParse> ParseResult<T> {
    pub fn parse(input: &str) -> Self {
        if input == "-" {
            ParseResult::None
        } else {
            match T::try_parse(input) {
                Ok(value) => ParseResult::Parsed(value),
                Err(original) => ParseResult::Fallback(original),
            }
        }
    }
}

impl<T: Display> Display for ParseResult<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseResult::None => write!(f, "None"),
            ParseResult::Parsed(value) => write!(f, "{}", value),
            ParseResult::Fallback(original) => {
                write!(f, "Fallback({})", original)
            },
        }
    }
}

impl<T: Display> ParseResult<T> {
    pub fn format_display(&self) -> Option<String> {
        match self {
            ParseResult::Parsed(ref value) => Some(format!("{}", value)),
            ParseResult::Fallback(ref value) => {
                Some(format!("{} (fallback)", value))
            },
            ParseResult::None => None,
        }
    }
}
