use crate::ParsedValue;
use std::fmt;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct DepthlessRockSample {
    // TODO: Implement known attributes (attachment 3)
    pub attribute: ParsedValue<String>,
    pub value: ParsedValue<String>,
}

impl fmt::Display for DepthlessRockSample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref attribute) = self.attribute.format_display() {
            writeln!(f, "Attribute: {}", attribute)?;
        }

        if let Some(ref value) = self.value.format_display() {
            writeln!(f, "Value: {}", value)?;
        }

        Ok(())
    }
}
