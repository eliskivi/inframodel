use crate::ParseResult;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Format {
    pub version: ParseResult<String>,
    pub used_software: ParseResult<String>,
    pub software_version: ParseResult<String>,
}
