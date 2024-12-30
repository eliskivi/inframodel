use crate::ParsedValue;

use std::fmt;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Work {
    pub id: ParsedValue<String>,
    pub name: ParsedValue<String>,
}

impl fmt::Display for Work {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref id) = self.id.format_display() {
            writeln!(f, "Work ID: {}", id)?;
        }

        if let Some(ref name) = self.name.format_display() {
            writeln!(f, "Work Name: {}", name)?;
        }

        Ok(())
    }
}
