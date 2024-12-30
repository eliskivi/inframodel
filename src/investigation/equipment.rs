use crate::ParseResult;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Equipment {
    pub number: ParseResult<i32>,
    pub description: ParseResult<String>,
    pub cone_size: ParseResult<String>,
}
