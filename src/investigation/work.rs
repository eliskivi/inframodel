use crate::ParseResult;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Work {
    pub id: ParseResult<String>,
    pub name: ParseResult<String>,
}
