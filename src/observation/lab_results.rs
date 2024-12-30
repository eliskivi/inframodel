use crate::ParsedValue;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub enum LabResult {
    #[default]
    None,
    GrainSize {
        grain_mm: ParsedValue<f32>,
        pass_percent: ParsedValue<f32>,
    },
    WaterContent {
        water_content: ParsedValue<f32>,
    },
    Other {
        attribute: ParsedValue<String>,
        result: ParsedValue<String>,
        unit: ParsedValue<String>,
    },
}
