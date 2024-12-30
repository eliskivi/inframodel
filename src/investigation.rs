use crate::observation::Observation;
use crate::parsed_value::ParsedValue;
use chrono::NaiveDate;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Investigation {
    pub organisations: Organisations,
    pub classification: Classification,
    pub work: Work,
    pub record: Record,
    pub method: Method,
    pub equipment: Equipment,
    pub coordinates: Coordinates,
    pub line: Line,
    pub termination: Termination,
    pub program: Program,
    pub depthless_rock_sample: DepthlessRockSample,
    pub initial_borehole: InitialBorehole,
    pub standpipe: Standpipe,

    pub observations: Vec<Observation>,
    pub obs_holder: Vec<Observation>,

    pub notes: Vec<ParsedValue<String>>,
    pub free_text: Vec<ParsedValue<String>>,
    pub hidden_text: Vec<ParsedValue<String>>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Organisations {
    pub owner_name: ParsedValue<String>,
    pub investigator_name: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Classification {
    pub name: ParsedValue<ClassificationName>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub enum ClassificationName {
    #[default]
    GEO,
    ISO,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Work {
    pub id: ParsedValue<String>,
    pub name: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Record {
    pub number: ParsedValue<i32>,
    pub driller: ParsedValue<String>,
    pub inspector: ParsedValue<String>,
    pub processor: ParsedValue<String>,
    pub digitalized: ParsedValue<Digitized>,
    pub condition: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Method {
    pub token: ParsedValue<MethodToken>,
    pub category: ParsedValue<i32>,
    pub id: ParsedValue<String>,
    pub standard: ParsedValue<String>,
    pub sampler: ParsedValue<Sampler>,
    pub specifier: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub enum Sampler {
    #[default]
    Unknown,
    K,
    L,
    PMK,
    R,
    ST50,
    ST60,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Equipment {
    pub number: ParsedValue<i32>,
    pub description: ParsedValue<String>,
    pub cone_size: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Coordinates {
    // TODO: Implement "-999999" as unknown for x and y coordinates
    pub x: ParsedValue<f32>,
    pub y: ParsedValue<f32>,
    pub start_elevation: ParsedValue<f32>,
    pub date: ParsedValue<NaiveDate>,
    pub point_id: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Line {
    pub name: ParsedValue<String>,
    pub stake: ParsedValue<f32>,
    pub distance: ParsedValue<f32>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Termination {
    pub token: ParsedValue<TerminationToken>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub enum TerminationToken {
    #[default]
    Unknown,
    TM,
    KI,
    KL,
    KA,
    KK,
    MS,
    KN,
    JA,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Program {
    pub name: ParsedValue<String>,
    pub date: ParsedValue<NaiveDate>,
    pub author: ParsedValue<String>,
    pub guide: Vec<ParsedValue<String>>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct DepthlessRockSample {
    // TODO: Implement known attributes (attachment 3)
    pub attribute: ParsedValue<String>,
    pub value: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct InitialBorehole {
    pub depth: ParsedValue<f32>,
    pub method: ParsedValue<InitialBoreToken>,
    pub soil_type: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub enum InitialBoreToken {
    #[default]
    Unknown,
    SI,
    LK,
    AP,
    LY,
    VA,
    JA,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Standpipe {
    // ZP token
    pub top_elevation: ParsedValue<f32>,
    pub ground_elevation: ParsedValue<f32>,
    pub protection_top_elevation: ParsedValue<f32>,
    pub cover_elevation: ParsedValue<f32>,
    pub sieve_bottom_elevation: ParsedValue<f32>,
    // TP token
    pub upper_structure: ParsedValue<String>,
    pub sieve_length: ParsedValue<f32>,
    pub sieve_type: ParsedValue<String>,
    pub diameter: ParsedValue<f32>,
    pub material: ParsedValue<String>,
    // LP token
    pub measure_point: ParsedValue<String>,
    pub details: ParsedValue<String>,
    pub locked: ParsedValue<String>,
    pub lock_owner: ParsedValue<String>,
    pub installer: ParsedValue<String>,
}

impl Investigation {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum MethodToken {
    #[default]
    None,
    PA,
    PI,
    LY,
    SI,
    HE,
    HK,
    PT,
    TR,
    PR,
    CP,
    CU,
    HP,
    PO,
    MW,
    VP,
    VO,
    VK,
    VPK,
    HV,
    HU,
    PS,
    PM,
    KO,
    KE,
    KR,
    NO,
    NE,
}

impl MethodToken {
    pub fn from_string(input: &str) -> Self {
        match input.to_uppercase().as_str() {
            "PA" | "WST" => MethodToken::PA,
            "PI" => MethodToken::PI,
            "LY" => MethodToken::LY,
            "SI" | "FVT" => MethodToken::SI,
            "HE" | "DP" => MethodToken::HE,
            "HK" => MethodToken::HK,
            "PT" => MethodToken::PT,
            "TR" => MethodToken::TR,
            "PR" => MethodToken::PR,
            "CP" | "CPT" => MethodToken::CP,
            "CU" | "CPTU" => MethodToken::CU,
            "HP" => MethodToken::HP,
            "PO" => MethodToken::PO,
            "MW" => MethodToken::MW,
            "VP" => MethodToken::VP,
            "VO" => MethodToken::VO,
            "VK" => MethodToken::VK,
            "VPK" => MethodToken::VPK,
            "HV" => MethodToken::HV,
            "HU" => MethodToken::HU,
            "PS" | "PMT" => MethodToken::PS,
            "PM" => MethodToken::PM,
            "KO" => MethodToken::KO,
            "KE" => MethodToken::KE,
            "KR" => MethodToken::KR,
            "NO" => MethodToken::NO,
            "NE" => MethodToken::NE,
            _ => MethodToken::None,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Digitized {
    #[default]
    No,
    Yes,
}
