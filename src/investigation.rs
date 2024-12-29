use crate::observation::Observation;
use crate::parsed_value::ParsedValue;
use chrono::NaiveDate;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Investigation {
    pub owner_organisation: OwnerOrganisation,
    pub soil_classification: SoilClassification,
    pub investigator_organisation: InvestigatorOrganisation,
    pub work: Work,
    pub record: Record,
    pub investigation_method: InvestigationMethod,
    pub equipment: Equipment,
    pub coordinates: Coordinates,
    pub line: Line,
    pub termination: Termination,
    pub investigation_program: InvestigationProgram,
    pub depthless_rock_sample: DepthlessRockSample,
    pub initial_borehole: InitialBorehole,
    pub standpipe: Standpipe,

    pub observations: Vec<Observation>,

    pub notes: Vec<ParsedValue<String>>,
    pub free_text: Vec<ParsedValue<String>>,
    pub hidden_text: Vec<ParsedValue<String>>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct OwnerOrganisation {
    pub name: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct SoilClassification {
    pub name: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct InvestigatorOrganisation {
    pub name: ParsedValue<String>,
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
pub struct InvestigationMethod {
    pub token: ParsedValue<InvestigationToken>,
    pub category: ParsedValue<i32>,
    pub id: ParsedValue<String>,
    pub standard: ParsedValue<String>,
    pub sampler: ParsedValue<String>,
    pub specifier: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Equipment {
    pub number: ParsedValue<i32>,
    pub description: ParsedValue<String>,
    pub cone_size: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Coordinates {
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
    pub token: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct InvestigationProgram {
    pub name: ParsedValue<String>,
    pub date: ParsedValue<NaiveDate>,
    pub author: ParsedValue<String>,
    pub guide: Vec<ParsedValue<String>>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct DepthlessRockSample {
    pub attribute: ParsedValue<String>,
    pub value: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct InitialBorehole {
    pub depth: ParsedValue<f32>,
    pub method: ParsedValue<String>,
    pub soil_type: ParsedValue<String>,
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
pub enum InvestigationToken {
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

impl InvestigationToken {
    pub fn from_string(input: &str) -> Self {
        match input.to_uppercase().as_str() {
            "PA" | "WST" => InvestigationToken::PA,
            "PI" => InvestigationToken::PI,
            "LY" => InvestigationToken::LY,
            "SI" | "FVT" => InvestigationToken::SI,
            "HE" | "DP" => InvestigationToken::HE,
            "HK" => InvestigationToken::HK,
            "PT" => InvestigationToken::PT,
            "TR" => InvestigationToken::TR,
            "PR" => InvestigationToken::PR,
            "CP" | "CPT" => InvestigationToken::CP,
            "CU" | "CPTU" => InvestigationToken::CU,
            "HP" => InvestigationToken::HP,
            "PO" => InvestigationToken::PO,
            "MW" => InvestigationToken::MW,
            "VP" => InvestigationToken::VP,
            "VO" => InvestigationToken::VO,
            "VK" => InvestigationToken::VK,
            "VPK" => InvestigationToken::VPK,
            "HV" => InvestigationToken::HV,
            "HU" => InvestigationToken::HU,
            "PS" | "PMT" => InvestigationToken::PS,
            "PM" => InvestigationToken::PM,
            "KO" => InvestigationToken::KO,
            "KE" => InvestigationToken::KE,
            "KR" => InvestigationToken::KR,
            "NO" => InvestigationToken::NO,
            "NE" => InvestigationToken::NE,
            _ => InvestigationToken::None,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Digitized {
    #[default]
    No,
    Yes,
}
