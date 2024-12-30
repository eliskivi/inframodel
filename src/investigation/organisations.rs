use crate::ParsedValue;

use std::fmt;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Organisations {
    pub owner_name: ParsedValue<String>,
    pub investigator_name: ParsedValue<String>,
}

impl fmt::Display for Organisations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref owner) = self.owner_name.format_display() {
            writeln!(f, "Owner: {}", owner)?;
        }

        if let Some(ref investigator) = self.investigator_name.format_display() {
            writeln!(f, "Investigator: {}", investigator)?;
        }

        Ok(())
    }
}
