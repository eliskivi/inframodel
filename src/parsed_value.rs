use crate::infra_file::{CoordinateSystem, ElevationSystem};
use crate::investigation::{ClassificationName, Digitized, InitialBoreToken, MethodToken, Sampler, TerminationToken};
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

impl TryParse for CoordinateSystem {
    fn try_parse(input: &str) -> Result<Self, String> {
        match input.trim().to_uppercase().as_str() {
            "WGS84" | "WGS" => Ok(CoordinateSystem::WGS84),
            "HKI" => Ok(CoordinateSystem::HKI),
            "VANTAA" => Ok(CoordinateSystem::VANTAA),
            "ESPOO" => Ok(CoordinateSystem::ESPOO),
            "KKJ0" => Ok(CoordinateSystem::KKJ0),
            "KKJ1" => Ok(CoordinateSystem::KKJ1),
            "KKJ2" => Ok(CoordinateSystem::KKJ2),
            "KKJ3" => Ok(CoordinateSystem::KKJ3),
            "KKJ4" => Ok(CoordinateSystem::KKJ4),
            "KKJ5" => Ok(CoordinateSystem::KKJ5),
            "YKJ" => Ok(CoordinateSystem::YKJ),
            "GK19" | "ETRSGK19" | "ETRS-GK19" => Ok(CoordinateSystem::GK19),
            "GK20" | "ETRSGK20" | "ETRS-GK20" => Ok(CoordinateSystem::GK20),
            "GK21" | "ETRSGK21" | "ETRS-GK21" => Ok(CoordinateSystem::GK21),
            "GK22" | "ETRSGK22" | "ETRS-GK22" => Ok(CoordinateSystem::GK22),
            "GK23" | "ETRSGK23" | "ETRS-GK23" => Ok(CoordinateSystem::GK23),
            "GK24" | "ETRSGK24" | "ETRS-GK24" => Ok(CoordinateSystem::GK24),
            "GK25" | "ETRSGK25" | "ETRS-GK25" => Ok(CoordinateSystem::GK25),
            "GK26" | "ETRSGK26" | "ETRS-GK26" => Ok(CoordinateSystem::GK26),
            "GK27" | "ETRSGK27" | "ETRS-GK27" => Ok(CoordinateSystem::GK27),
            "GK28" | "ETRSGK28" | "ETRS-GK28" => Ok(CoordinateSystem::GK28),
            "GK29" | "ETRSGK29" | "ETRS-GK29" => Ok(CoordinateSystem::GK29),
            "GK30" | "ETRSGK30" | "ETRS-GK30" => Ok(CoordinateSystem::GK30),
            "GK31" | "ETRSGK31" | "ETRS-GK31" => Ok(CoordinateSystem::GK31),
            "TM35" | "ETRSTM35" | "ETRS-TM35" | "TM35FIN" | "ETRSTM35FIN" | "ETRS-TM35FIN" => Ok(CoordinateSystem::TM35),
            _ => Err(input.to_string()),
        }
    }
}

impl TryParse for ElevationSystem {
    fn try_parse(input: &str) -> Result<Self, String> {
        match input.trim().to_uppercase().as_str() {
            "N2000" => Ok(ElevationSystem::N2000),
            "N60" => Ok(ElevationSystem::N60),
            "N43" => Ok(ElevationSystem::N43),
            "NN" => Ok(ElevationSystem::NN),
            "LN" => Ok(ElevationSystem::LN),
            _ => Err(input.to_string()),
        }
    }
}

impl TryParse for ClassificationName {
    fn try_parse(input: &str) -> Result<Self, String> {
        match input.trim().to_uppercase().as_str() {
            // TODO: Thinking if default should just be GEO..
            "GEO" => Ok(ClassificationName::GEO),
            "ISO" => Ok(ClassificationName::ISO),
            _ => Err(input.to_string()),
        }
    }
}

impl TryParse for Sampler {
    fn try_parse(input: &str) -> Result<Self, String> {
        match input.trim().to_uppercase().as_str() {
            "K" => Ok(Sampler::K),
            "L" => Ok(Sampler::L),
            "PMK" => Ok(Sampler::PMK),
            "R" => Ok(Sampler::R),
            "ST50" | "ST-50" => Ok(Sampler::ST50),
            "ST60" | "ST-60" => Ok(Sampler::ST60),
            _ => Err(input.to_string()),
        }
    }
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
        // "00000000" is considered unknown date
        if input == "00000000" {
            return Err(input.to_string());
        }
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
