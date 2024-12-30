use crate::{ParsedValue, TryParse};

use std::fmt;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Record {
    pub number: ParsedValue<i32>,
    pub driller: ParsedValue<String>,
    pub inspector: ParsedValue<String>,
    pub processor: ParsedValue<String>,
    pub digitalized: ParsedValue<Digitized>,
    pub condition: ParsedValue<String>,
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

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref number) = self.number.format_display() {
            writeln!(f, "Record number: {}", number)?;
        }

        if let Some(ref driller) = self.driller.format_display() {
            writeln!(f, "Driller: {}", driller)?;
        }

        if let Some(ref inspector) = self.inspector.format_display() {
            writeln!(f, "Inspector: {}", inspector)?;
        }

        if let Some(ref processor) = self.processor.format_display() {
            writeln!(f, "Processor: {}", processor)?;
        }

        if let Some(ref digitalized) = self.digitalized.format_display() {
            writeln!(f, "Digitalized: {}", digitalized)?;
        }

        if let Some(ref condition) = self.condition.format_display() {
            writeln!(f, "Condition: {}", condition)?;
        }

        Ok(())
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
