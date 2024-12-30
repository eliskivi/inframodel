use crate::ParsedValue;

use std::fmt;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Line {
    pub name: ParsedValue<String>,
    pub stake: ParsedValue<f32>,
    pub distance: ParsedValue<f32>,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref name) = self.name.format_display() {
            writeln!(f, "Line name: {}", name)?;
        }

        if let Some(ref stake) = self.stake.format_display() {
            writeln!(f, "Stake: {}", stake)?;
        }

        if let Some(ref distance) = self.distance.format_display() {
            writeln!(f, "Distance: {}", distance)?;
        }

        Ok(())
    }
}
