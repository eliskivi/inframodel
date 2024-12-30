pub(crate) mod lab_results;
pub(crate) mod observation_values;

use crate::{ObservationValues, ParseResult};

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Observation {
    pub values: ObservationValues,
    pub notes: Vec<ParseResult<String>>,
    pub free_text: Vec<ParseResult<String>>,
    pub hidden_text: Vec<ParseResult<String>>,
    pub unofficial_soil_type: Vec<ParseResult<String>>,
    pub water_observed: ParseResult<String>,
}

impl Observation {
    pub fn new() -> Self {
        Self::default()
    }
}
