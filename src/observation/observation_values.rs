use crate::{LabResult, ParseResult};

use chrono::NaiveDate;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub enum ObservationValues {
    // TODO: Implement accepted soil type enum
    // TODO: Ever further, if it's not on that enum, then that should be unofficial soil type..
    #[default]
    None,
    PA {
        depth: ParseResult<f32>,
        load: ParseResult<f32>,
        hits: ParseResult<i32>,
        half_turns: ParseResult<i32>,
        soil_type: ParseResult<String>,
    },
    PI {
        depth: ParseResult<f32>,
        soil_type: ParseResult<String>,
    },
    LY {
        depth: ParseResult<f32>,
        load: ParseResult<f32>,
        hits: ParseResult<i32>,
        soil_type: ParseResult<String>,
    },
    SI {
        depth: ParseResult<f32>,
        shear_str: ParseResult<f32>,
        disturb_shear_str: ParseResult<f32>,
        sensitivity: ParseResult<f32>,
        residual_str: ParseResult<f32>,
    },
    HE {
        depth: ParseResult<f32>,
        hits: ParseResult<i32>,
        soil_type: ParseResult<String>,
    },
    HK {
        depth: ParseResult<f32>,
        hits: ParseResult<i32>,
        torque: ParseResult<f32>,
        soil_type: ParseResult<String>,
    },
    PT {
        depth: ParseResult<f32>,
        soil_type: ParseResult<String>,
    },
    TR {
        depth: ParseResult<f32>,
        soil_type: ParseResult<String>,
    },
    PR {
        depth: ParseResult<f32>,
        total_resistance: ParseResult<f32>,
        sleeve_friction: ParseResult<f32>,
        soil_type: ParseResult<String>,
    },
    CP {
        depth: ParseResult<f32>,
        total_resistance: ParseResult<f32>,
        sleeve_friction: ParseResult<f32>,
        tip_resistance: ParseResult<f32>,
        soil_type: ParseResult<String>,
    },
    CU {
        depth: ParseResult<f32>,
        total_resistance: ParseResult<f32>,
        sleeve_friction: ParseResult<f32>,
        tip_resistance: ParseResult<f32>,
        pore_water_pressure: ParseResult<f32>,
        soil_type: ParseResult<String>,
    },
    HP {
        depth: ParseResult<f32>,
        hits: ParseResult<i32>,
        pressure: ParseResult<f32>,
        torque: ParseResult<f32>,
        mode: ParseResult<String>,
        soil_type: ParseResult<String>,
    },
    PO {
        depth: ParseResult<f32>,
        time: ParseResult<i32>,
        soil_type: ParseResult<String>,
    },
    MW {
        depth: ParseResult<f32>,
        advance_rate: ParseResult<f32>,
        compressive_force: ParseResult<f32>,
        flushing_pressure: ParseResult<f32>,
        water_consumption: ParseResult<f32>,
        torque: ParseResult<f32>,
        rotation_speed: ParseResult<f32>,
        hits: ParseResult<String>,
        soil_type: ParseResult<String>,
    },
    VP {
        surface_elev: ParseResult<f32>,
        date: ParseResult<NaiveDate>,
        pipe_top_elev: ParseResult<f32>,
        pipe_bot_elev: ParseResult<f32>,
        sieve_len: ParseResult<f32>,
        measurer: ParseResult<String>,
    },
    VO {
        surface_elev: ParseResult<f32>,
        date: ParseResult<NaiveDate>,
        pipe_top_elev: ParseResult<f32>,
        pipe_bot_elev: ParseResult<f32>,
        sieve_len: ParseResult<f32>,
        measurer: ParseResult<String>,
    },
    VK {
        surface_elev: ParseResult<f32>,
        date: ParseResult<NaiveDate>,
        water_type: ParseResult<String>,
    },
    VPK {
        surface_elev: ParseResult<f32>,
        date: ParseResult<NaiveDate>,
    },
    HV {
        depth: ParseResult<f32>,
        pressure: ParseResult<f32>,
        date: ParseResult<NaiveDate>,
        measurer: ParseResult<String>,
    },
    HU {
        surface_elev: ParseResult<f32>,
        date: ParseResult<NaiveDate>,
        pipe_top_elev: ParseResult<f32>,
        pipe_bot_elev: ParseResult<f32>,
        sieve_len: ParseResult<f32>,
        measurer: ParseResult<String>,
    },
    PS {
        depth: ParseResult<f32>,
        modulus: ParseResult<f32>,
        fail_pressure: ParseResult<f32>,
    },
    PM {
        elev: ParseResult<f32>,
        date: ParseResult<NaiveDate>,
        measurer: ParseResult<String>,
    },
    KO {
        depth: ParseResult<f32>,
        soil_type: ParseResult<String>,
        stones: ParseResult<f32>,
        boulders: ParseResult<i32>,
        max_width: ParseResult<f32>,
        min_width: ParseResult<f32>,
    },
    KE {
        start_depth: ParseResult<f32>,
        end_depth: ParseResult<f32>,
    },
    KR {
        start_depth: ParseResult<f32>,
        end_depth: ParseResult<f32>,
    },
    NO {
        start_depth: ParseResult<f32>,
        sample_id: ParseResult<String>,
        end_depth: ParseResult<f32>,
        soil_type: ParseResult<String>,
        lab_values: Vec<ParseResult<LabResult>>,
    },
    NE {
        start_depth: ParseResult<f32>,
        sample_id: ParseResult<String>,
        end_depth: ParseResult<f32>,
        soil_type: ParseResult<String>,
        lab_values: Vec<ParseResult<LabResult>>,
    },
}

impl ObservationValues {
    pub(crate) fn get_parsed_depth(&self) -> Option<f32> {
        match self {
            ObservationValues::PA { depth, .. }
            | ObservationValues::PI { depth, .. }
            | ObservationValues::LY { depth, .. }
            | ObservationValues::HE { depth, .. }
            | ObservationValues::HK { depth, .. }
            | ObservationValues::PT { depth, .. }
            | ObservationValues::TR { depth, .. }
            | ObservationValues::PR { depth, .. }
            | ObservationValues::CP { depth, .. }
            | ObservationValues::CU { depth, .. }
            | ObservationValues::HP { depth, .. }
            | ObservationValues::PO { depth, .. }
            | ObservationValues::MW { depth, .. }
            | ObservationValues::KO { depth, .. }
            | ObservationValues::PS { depth, .. } => {
                if let ParseResult::Parsed(value) = depth {
                    Some(*value)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub(crate) fn get_parsed_soil_type(&self) -> Option<&String> {
        match self {
            ObservationValues::PA { soil_type, .. }
            | ObservationValues::PI { soil_type, .. }
            | ObservationValues::LY { soil_type, .. }
            | ObservationValues::HE { soil_type, .. }
            | ObservationValues::HK { soil_type, .. }
            | ObservationValues::PT { soil_type, .. }
            | ObservationValues::TR { soil_type, .. }
            | ObservationValues::PR { soil_type, .. }
            | ObservationValues::CP { soil_type, .. }
            | ObservationValues::CU { soil_type, .. }
            | ObservationValues::HP { soil_type, .. }
            | ObservationValues::PO { soil_type, .. }
            | ObservationValues::MW { soil_type, .. }
            | ObservationValues::KO { soil_type, .. } => {
                if let ParseResult::Parsed(ref soil) = soil_type {
                    Some(soil)
                } else {
                    None
                }
            }

            // Variants without soil_type
            _ => None,
        }
    }
}
