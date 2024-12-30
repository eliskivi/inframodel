use crate::ParseResult;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub enum LabResult {
    #[default]
    None,
    GrainSize {
        grain_mm: ParseResult<f32>,
        pass_percent: ParseResult<f32>,
    },
    WaterContent {
        water_content: ParseResult<f32>,
    },
    Other {
        attribute: ParseResult<String>,
        result: ParseResult<String>,
        unit: ParseResult<String>,
    },
}
