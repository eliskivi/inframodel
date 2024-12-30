use crate::ParsedValue;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Format {
    pub version: ParsedValue<String>,
    pub used_software: ParsedValue<String>,
    pub software_version: ParsedValue<String>,
}
