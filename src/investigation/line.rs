use crate::ParseResult;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Line {
    pub name: ParseResult<String>,
    pub stake: ParseResult<f32>,
    pub distance: ParseResult<f32>,
}
