use crate::ParseResult;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Organisations {
    pub owner_name: ParseResult<String>,
    pub investigator_name: ParseResult<String>,
}
