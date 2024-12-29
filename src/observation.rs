use crate::parsed_value::ParsedValue;
use chrono::NaiveDate;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub enum Observation {
    #[default]
    None,
    PA {
        depth: ParsedValue<f32>,
        load: ParsedValue<f32>,
        hits: ParsedValue<i32>,
        half_turns: ParsedValue<i32>,
        soil_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    PI {
        depth: ParsedValue<f32>,
        soil_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    LY {
        depth: ParsedValue<f32>,
        load: ParsedValue<f32>,
        hits: ParsedValue<i32>,
        soil_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    SI {
        depth: ParsedValue<f32>,
        shear_str: ParsedValue<f32>,
        disturb_shear_str: ParsedValue<f32>,
        sensitivity: ParsedValue<f32>,
        residual_str: ParsedValue<f32>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    HE {
        depth: ParsedValue<f32>,
        hits: ParsedValue<i32>,
        soil_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    HK {
        depth: ParsedValue<f32>,
        hits: ParsedValue<i32>,
        torque: ParsedValue<f32>,
        soil_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    PT {
        depth: ParsedValue<f32>,
        soil_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    TR {
        depth: ParsedValue<f32>,
        soil_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    PR {
        depth: ParsedValue<f32>,
        total_resistance: ParsedValue<f32>,
        sleeve_friction: ParsedValue<f32>,
        soil_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    CP {
        depth: ParsedValue<f32>,
        total_resistance: ParsedValue<f32>,
        sleeve_friction: ParsedValue<f32>,
        tip_resistance: ParsedValue<f32>,
        soil_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    CU {
        depth: ParsedValue<f32>,
        total_resistance: ParsedValue<f32>,
        sleeve_friction: ParsedValue<f32>,
        tip_resistance: ParsedValue<f32>,
        pore_water_pressure: ParsedValue<f32>,
        soil_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    HP {
        depth: ParsedValue<f32>,
        hits: ParsedValue<i32>,
        pressure: ParsedValue<f32>,
        torque: ParsedValue<f32>,
        mode: ParsedValue<String>,
        soil_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: bool,
    },
    PO {
        depth: ParsedValue<f32>,
        time: ParsedValue<i32>,
        soil_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
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

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    VP {
        surface_elev: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
        pipe_top_elev: ParsedValue<f32>,
        pipe_bot_elev: ParsedValue<f32>,
        sieve_len: ParsedValue<f32>,
        measurer: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    VO {
        surface_elev: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
        pipe_top_elev: ParsedValue<f32>,
        pipe_bot_elev: ParsedValue<f32>,
        sieve_len: ParsedValue<f32>,
        measurer: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    VK {
        surface_elev: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
        water_type: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    VPK {
        surface_elev: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    HV {
        depth: ParsedValue<f32>,
        pressure: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
        measurer: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    HU {
        surface_elev: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
        pipe_top_elev: ParsedValue<f32>,
        pipe_bot_elev: ParsedValue<f32>,
        sieve_len: ParsedValue<f32>,
        measurer: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    PS {
        depth: ParsedValue<f32>,
        modulus: ParsedValue<f32>,
        fail_pressure: ParsedValue<f32>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    PM {
        elev: ParsedValue<f32>,
        date: ParsedValue<NaiveDate>,
        measurer: ParsedValue<String>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    KO {
        depth: ParsedValue<f32>,
        soil_type: ParsedValue<String>,
        stones: ParsedValue<f32>,
        boulders: ParsedValue<i32>,
        max_width: ParsedValue<f32>,
        min_width: ParsedValue<f32>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    KE {
        start_depth: ParsedValue<f32>,
        end_depth: ParsedValue<f32>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    KR {
        start_depth: ParsedValue<f32>,
        end_depth: ParsedValue<f32>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    NO {
        start_depth: ParsedValue<f32>,
        sample_id: ParsedValue<String>,
        end_depth: ParsedValue<f32>,
        soil_type: ParsedValue<String>,

        lab_sieve: Vec<ParsedValue<Observation>>,
        lab_other: Vec<ParsedValue<Observation>>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    NE {
        start_depth: ParsedValue<f32>,
        sample_id: ParsedValue<String>,
        end_depth: ParsedValue<f32>,
        soil_type: ParsedValue<String>,

        lab_sieve: Vec<ParsedValue<Observation>>,
        lab_other: Vec<ParsedValue<Observation>>,

        notes: Vec<ParsedValue<String>>,
        free_txt: Vec<ParsedValue<String>>,
        hidden_txt: Vec<ParsedValue<String>>,
        unoff_soil: Vec<ParsedValue<String>>,
        water_obs: ParsedValue<String>,
    },
    LB {
        attribute: ParsedValue<String>,
        result: ParsedValue<String>,
        unit: ParsedValue<String>,
    },
    RK {
        sieve_mm: ParsedValue<f32>,
        pass_percent: ParsedValue<f32>,
    },
}

impl Observation {
    pub fn new() -> Self {
        Self::default()
    }
}
