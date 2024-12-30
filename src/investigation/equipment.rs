use crate::ParsedValue;

use std::fmt;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Equipment {
    pub number: ParsedValue<i32>,
    pub description: ParsedValue<String>,
    pub cone_size: ParsedValue<String>,
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref number) = self.number.format_display() {
            writeln!(f, "Equipment number: {}", number)?;
        }

        if let Some(ref description) = self.description.format_display() {
            writeln!(f, "Description: {}", description)?;
        }

        if let Some(ref cone_size) = self.cone_size.format_display() {
            writeln!(f, "Cone size: {}", cone_size)?;
        }

        Ok(())
    }
}
