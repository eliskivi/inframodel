use crate::parsed_value::ParsedValue;
use chrono::NaiveDate;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Observation {
    pub values: ObservationValues,
    pub notes: Vec<ParsedValue<String>>,
    pub free_text: Vec<ParsedValue<String>>,
    pub hidden_text: Vec<ParsedValue<String>>,
    pub unofficial_soil_type: Vec<ParsedValue<String>>,
    pub water_observed: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub enum ObservationValues {
    // TODO: Implement accepted soil type enum
    // TODO: Ever further, if it's not on that enum, then that should be unofficial soil type..
    #[default]
    None,
    PA {
        depth: ParsedValue<f32>,
        load: ParsedValue<f32>,
        hits: ParsedValue<i32>,
        half_turns: ParsedValue<i32>,
        soil_type: ParsedValue<String>,
    },
    PI {
        depth: ParsedValue<f32>,
        soil_type: ParsedValue<String>,
    },
    LY {
        depth: ParsedValue<f32>,
        load: ParsedValue<f32>,
        hits: ParsedValue<i32>,
        soil_type: ParsedValue<String>,
    },
    SI {
        depth: ParsedValue<f32>,
        shear_str: ParsedValue<f32>,
        disturb_shear_str: ParsedValue<f32>,
        sensitivity: ParsedValue<f32>,
        residual_str: ParsedValue<f32>,
    },
    HE {
        depth: ParsedValue<f32>,
        hits: ParsedValue<i32>,
        soil_type: ParsedValue<String>,
    },
    HK {
        depth: ParsedValue<f32>,
        hits: ParsedValue<i32>,
        torque: ParsedValue<f32>,
        soil_type: ParsedValue<String>,
    },
    PT {
        depth: ParsedValue<f32>,
        soil_type: ParsedValue<String>,
    },
    TR {
        depth: ParsedValue<f32>,
        soil_type: ParsedValue<String>,
    },
    PR {
        depth: ParsedValue<f32>,
        total_resistance: ParsedValue<f32>,
        sleeve_friction: ParsedValue<f32>,
        soil_type: ParsedValue<String>,
    },
    CP {
        depth: ParsedValue<f32>,
        total_resistance: ParsedValue<f32>,
        sleeve_friction: ParsedValue<f32>,
        tip_resistance: ParsedValue<f32>,
        soil_type: ParsedValue<String>,
    },
    CU {
        depth: ParsedValue<f32>,
        total_resistance: ParsedValue<f32>,
        sleeve_friction: ParsedValue<f32>,
        tip_resistance: ParsedValue<f32>,
        pore_water_pressure: ParsedValue<f32>,
        soil_type: ParsedValue<String>,
    },
    HP {
        depth: ParsedValue<f32>,
        hits: ParsedValue<i32>,
        pressure: ParsedValue<f32>,
        torque: ParsedValue<f32>,
        mode: ParsedValue<String>,
        soil_type: ParsedValue<String>,
    },
    PO {
        depth: ParsedValue<f32>,
        time: ParsedValue<i32>,
        soil_type: ParsedValue<String>,
    },
    MW {
        depth: ParsedValue<f32>,
        advance_rate: ParsedValue<f32>,
        compressive_force: ParsedValue<f32>,
        flushing_pressure: ParsedValue<f32>,
        water_consumption: ParsedValue<f32>,
        torque: ParsedValue<f32>,
        rotation_speed: ParsedValue<f32>,
        hits: ParsedValue<String>,
        soil_type: ParsedValue<String>,
    },
    VP {
        surface_elev: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
        pipe_top_elev: ParsedValue<f32>,
        pipe_bot_elev: ParsedValue<f32>,
        sieve_len: ParsedValue<f32>,
        measurer: ParsedValue<String>,
    },
    VO {
        surface_elev: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
        pipe_top_elev: ParsedValue<f32>,
        pipe_bot_elev: ParsedValue<f32>,
        sieve_len: ParsedValue<f32>,
        measurer: ParsedValue<String>,
    },
    VK {
        surface_elev: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
        water_type: ParsedValue<String>,
    },
    VPK {
        surface_elev: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
    },
    HV {
        depth: ParsedValue<f32>,
        pressure: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
        measurer: ParsedValue<String>,
    },
    HU {
        surface_elev: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
        pipe_top_elev: ParsedValue<f32>,
        pipe_bot_elev: ParsedValue<f32>,
        sieve_len: ParsedValue<f32>,
        measurer: ParsedValue<String>,
    },
    PS {
        depth: ParsedValue<f32>,
        modulus: ParsedValue<f32>,
        fail_pressure: ParsedValue<f32>,
    },
    PM {
        elev: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
        measurer: ParsedValue<String>,
    },
    KO {
        depth: ParsedValue<f32>,
        soil_type: ParsedValue<String>,
        stones: ParsedValue<f32>,
        boulders: ParsedValue<i32>,
        max_width: ParsedValue<f32>,
        min_width: ParsedValue<f32>,
    },
    KE {
        start_depth: ParsedValue<f32>,
        end_depth: ParsedValue<f32>,
    },
    KR {
        start_depth: ParsedValue<f32>,
        end_depth: ParsedValue<f32>,
    },
    NO {
        start_depth: ParsedValue<f32>,
        sample_id: ParsedValue<String>,
        end_depth: ParsedValue<f32>,
        soil_type: ParsedValue<String>,
        lab_values: Vec<ParsedValue<LabResult>>,
    },
    NE {
        start_depth: ParsedValue<f32>,
        sample_id: ParsedValue<String>,
        end_depth: ParsedValue<f32>,
        soil_type: ParsedValue<String>,
        lab_values: Vec<ParsedValue<LabResult>>,
    },
}

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

impl Observation {
    pub fn new() -> Self {
        Self::default()
    }
}
