use crate::ParseResult;
use chrono::NaiveDate;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Program {
    pub name: ParseResult<String>,
    pub date: ParseResult<NaiveDate>,
    pub author: ParseResult<String>,
    pub guide: Vec<ParseResult<String>>,
}
