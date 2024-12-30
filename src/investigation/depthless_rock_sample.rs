use crate::ParseResult;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct DepthlessRockSample {
    // TODO: Implement known attributes (attachment 3)
    pub attribute: ParseResult<String>,
    pub value: ParseResult<String>,
}
