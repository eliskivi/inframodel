use crate::ParsedValue;

use chrono::NaiveDate;
use std::fmt;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Program {
    pub name: ParsedValue<String>,
    pub date: ParsedValue<NaiveDate>,
    pub author: ParsedValue<String>,
    pub guide: Vec<ParsedValue<String>>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref name) = self.name.format_display() {
            writeln!(f, "Program name: {}", name)?;
        }

        if let Some(ref date) = self.date.format_display() {
            writeln!(f, "Program date: {}", date)?;
        }

        if let Some(ref author) = self.author.format_display() {
            writeln!(f, "Author: {}", author)?;
        }

        if !self.guide.is_empty() {
            writeln!(f, "Program guide:")?;
            for (i, guide) in self.guide.iter().enumerate() {
                if let Some(ref guide_str) = guide.format_display() {
                    writeln!(f, "  {}. {}", i + 1, guide_str)?;
                }
            }
        }

        Ok(())
    }
}
