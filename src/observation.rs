pub(crate) mod lab_results;
pub(crate) mod observation_values;

use crate::{ObservationValues, ParsedValue};

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Observation {
    pub values: ObservationValues,
    pub notes: Vec<ParsedValue<String>>,
    pub free_text: Vec<ParsedValue<String>>,
    pub hidden_text: Vec<ParsedValue<String>>,
    pub unofficial_soil_type: Vec<ParsedValue<String>>,
    pub water_observed: ParsedValue<String>,
}

impl Observation {
    pub fn new() -> Self {
        Self::default()
    }
}
