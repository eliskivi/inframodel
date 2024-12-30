use crate::ParseResult;
use chrono::NaiveDate;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Coordinates {
    // TODO: Implement "-999999" as unknown for x and y coordinates
    pub x: ParseResult<f32>,
    pub y: ParseResult<f32>,
    pub start_elevation: ParseResult<f32>,
    pub date: ParseResult<NaiveDate>,
    pub point_id: ParseResult<String>,
}
