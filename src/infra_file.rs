use crate::investigation::{Digitized, Investigation, MethodToken};
use crate::observation::Observation;
use crate::parsed_value::{ParsedValue, TryParse};

use chardetng::EncodingDetector;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref FLOAT_RE: Regex = Regex::new(r"^[+-]?[0-9]+([.,][0-9]+)?$").unwrap();
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct InfraFile {
    pub file: File,
    pub format: Format,
    pub spatial: Spatial,
    pub investigations: Vec<Investigation>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct File {
    pub path: ParsedValue<String>,
    pub encoding: ParsedValue<String>,
    pub text: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Format {
    pub version: ParsedValue<String>,
    pub used_software: ParsedValue<String>,
    pub software_version: ParsedValue<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Spatial {
    // TODO: Add common Finnish coordinate and elevation systems into an enum
    // TODO: Implement Unknown coordinate/elevation system marked with "?"
    pub coordinate_system: ParsedValue<String>,
    pub elevation_system: ParsedValue<String>,
}

impl InfraFile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn debug_print(&self) {
        println!("{:#?}", self);
    }

    // TODO: Implement other building and re-building methods
    // TODO: Implement Result<InfraFile, error_string>
    pub fn parse_file(file_path: &str) -> InfraFile {
        let buffer = std::fs::read(file_path).expect("File not found");
        let mut detector = EncodingDetector::new();
        detector.feed(&buffer, true);
        let encoding = detector.guess(None, true);

        let (decoded, _, _) = encoding.decode(&buffer);
        let lines: Vec<&str> = decoded.lines().collect();

        let mut infra = InfraFile {
            file: File {
                path: ParsedValue::Some(file_path.to_string()),
                encoding: ParsedValue::Some(encoding.name().to_string()),
                text: ParsedValue::Some(decoded.parse().unwrap()),
            },
            ..Default::default()
        };

        let mut inv = Investigation::default();

        for line in lines.iter() {
            if line.trim().is_empty() {
                continue;
            }

            let params: &[&str] = &line.split_whitespace().collect::<Vec<&str>>();
            if let Some((token, rest)) = params.split_first() {
                match *token {
                    "FO" => Self::parse_fo(&mut infra, rest),
                    "KJ" => Self::parse_kj(&mut infra, rest),
                    "OM" => Self::parse_om(&mut inv, rest),
                    "ML" => Self::parse_ml(&mut inv, rest),
                    "OR" => Self::parse_or(&mut inv, rest),
                    "TY" => Self::parse_ty(&mut inv, rest),
                    "PK" => Self::parse_pk(&mut inv, rest),
                    "TT" => Self::parse_tt(&mut inv, rest),
                    "LA" => Self::parse_la(&mut inv, rest),
                    "XY" => Self::parse_xy(&mut inv, rest),
                    "LN" => Self::parse_ln(&mut inv, rest),
                    "-1" => Self::parse_end(&mut infra, &mut inv, rest),
                    "GR" => Self::parse_gr(&mut inv, rest),
                    "GL" => Self::parse_gl(&mut inv, rest),
                    "AT" => Self::parse_at(&mut inv, rest),
                    "AL" => Self::parse_al(&mut inv, rest),
                    "ZP" => Self::parse_zp(&mut inv, rest),
                    "TP" => Self::parse_tp(&mut inv, rest),
                    "LP" => Self::parse_lp(&mut inv, rest),
                    "HM" => Self::parse_hm(&mut inv, rest),
                    "TX" => Self::parse_tx(&mut inv, rest),
                    "HT" => Self::parse_ht(&mut inv, rest),
                    "EM" => Self::parse_em(&mut inv, rest),
                    "LB" => Self::parse_lb(&mut inv, rest),
                    "RK" => Self::parse_rk(&mut inv, rest),
                    _ if FLOAT_RE.is_match(token) => match &inv.investigation_method.token {
                        ParsedValue::Some(method) => match method {
                            MethodToken::PA => Self::parse_pa(&mut inv, params),
                            MethodToken::PI => Self::parse_pi(&mut inv, params),
                            MethodToken::LY => Self::parse_ly(&mut inv, params),
                            MethodToken::SI => Self::parse_si(&mut inv, params),
                            MethodToken::HE => Self::parse_he(&mut inv, params),
                            MethodToken::HK => Self::parse_hk(&mut inv, params),
                            MethodToken::PT => Self::parse_pt(&mut inv, params),
                            MethodToken::TR => Self::parse_tr(&mut inv, params),
                            MethodToken::PR => Self::parse_pr(&mut inv, params),
                            MethodToken::CP => Self::parse_cp(&mut inv, params),
                            MethodToken::CU => Self::parse_cu(&mut inv, params),
                            MethodToken::HP => Self::parse_hp(&mut inv, params),
                            MethodToken::PO => Self::parse_po(&mut inv, params),
                            MethodToken::MW => Self::parse_mw(&mut inv, params),
                            MethodToken::VP => Self::parse_vp(&mut inv, params),
                            MethodToken::VO => Self::parse_vo(&mut inv, params),
                            MethodToken::VK => Self::parse_vk(&mut inv, params),
                            MethodToken::VPK => Self::parse_vpk(&mut inv, params),
                            MethodToken::HV => Self::parse_hv(&mut inv, params),
                            MethodToken::HU => Self::parse_hu(&mut inv, params),
                            MethodToken::PS => Self::parse_ps(&mut inv, params),
                            MethodToken::PM => Self::parse_pm(&mut inv, params),
                            MethodToken::KO => Self::parse_ko(&mut inv, params),
                            MethodToken::KE => Self::parse_ke(&mut inv, params),
                            MethodToken::KR => Self::parse_kr(&mut inv, params),
                            MethodToken::NO => Self::parse_no(&mut inv, params),
                            MethodToken::NE => Self::parse_ne(&mut inv, params),
                            MethodToken::None => unreachable!("Unknown method: {}", token),
                        },
                        ParsedValue::None => {
                            panic!("No method was specified, got: {}", token);
                        }
                        ParsedValue::Fallback(original) => {
                            panic!("Invalid method: {}", original);
                        }
                    },
                    _ => {}
                }
            }
        }

        infra
    }

    fn parse_value<T: TryParse>(params: &[&str], index: usize) -> ParsedValue<T> {
        if let Some(&raw) = params.get(index) {
            ParsedValue::parse(raw)
        } else {
            ParsedValue::None
        }
    }

    fn parse_fo(infra: &mut InfraFile, params: &[&str]) {
        infra.format.version = Self::parse_value::<String>(params, 0);
        infra.format.used_software = Self::parse_value::<String>(params, 1);
        infra.format.software_version = Self::parse_value::<String>(params, 2);
    }

    fn parse_kj(infra: &mut InfraFile, params: &[&str]) {
        infra.spatial.coordinate_system = Self::parse_value::<String>(params, 0);
        infra.spatial.elevation_system = Self::parse_value::<String>(params, 1);
    }

    fn parse_om(inv: &mut Investigation, params: &[&str]) {
        inv.organisations.owner_name = Self::parse_value::<String>(params, 0);
    }

    fn parse_ml(inv: &mut Investigation, params: &[&str]) {
        inv.soil_classification.name = Self::parse_value::<String>(params, 0);
    }

    fn parse_or(inv: &mut Investigation, params: &[&str]) {
        inv.organisations.investigator_name = Self::parse_value::<String>(params, 0);
    }

    fn parse_ty(inv: &mut Investigation, params: &[&str]) {
        inv.work.id = Self::parse_value::<String>(params, 0);
        inv.work.name = Self::parse_value::<String>(params, 1);
    }

    fn parse_pk(inv: &mut Investigation, params: &[&str]) {
        inv.record.number = Self::parse_value::<i32>(params, 0);
        inv.record.driller = Self::parse_value::<String>(params, 1);
        inv.record.inspector = Self::parse_value::<String>(params, 2);
        inv.record.processor = Self::parse_value::<String>(params, 3);
        inv.record.digitalized = Self::parse_value::<Digitized>(params, 4);
        inv.record.condition = Self::parse_value::<String>(params, 5);
    }

    fn parse_tt(inv: &mut Investigation, params: &[&str]) {
        inv.investigation_method.token = Self::parse_value::<MethodToken>(params, 0);
        inv.investigation_method.category = Self::parse_value::<i32>(params, 1);
        inv.investigation_method.id = Self::parse_value::<String>(params, 2);
        inv.investigation_method.standard = Self::parse_value::<String>(params, 3);
        inv.investigation_method.sampler = Self::parse_value::<String>(params, 4);
        inv.investigation_method.specifier = Self::parse_value::<String>(params, 5);
    }

    fn parse_la(inv: &mut Investigation, params: &[&str]) {
        inv.equipment.number = Self::parse_value::<i32>(params, 0);
        inv.equipment.description = Self::parse_value::<String>(params, 1);
        inv.equipment.cone_size = Self::parse_value::<String>(params, 2);
    }

    fn parse_xy(inv: &mut Investigation, params: &[&str]) {
        inv.coordinates.x = Self::parse_value::<f32>(params, 0);
        inv.coordinates.y = Self::parse_value::<f32>(params, 1);
        inv.coordinates.start_elevation = Self::parse_value::<f32>(params, 2);
        inv.coordinates.date = Self::parse_value::<NaiveDate>(params, 3);
        inv.coordinates.point_id = Self::parse_value::<String>(params, 4);
    }

    fn parse_ln(inv: &mut Investigation, params: &[&str]) {
        inv.line.name = Self::parse_value::<String>(params, 0);
        inv.line.stake = Self::parse_value::<f32>(params, 1);
        inv.line.distance = Self::parse_value::<f32>(params, 2);
    }

    fn parse_end(infra: &mut InfraFile, inv: &mut Investigation, params: &[&str]) {
        inv.termination.token = Self::parse_value::<String>(params, 0);
        infra.investigations.push(std::mem::take(inv));
    }

    fn parse_gr(inv: &mut Investigation, params: &[&str]) {
        inv.investigation_program.name = Self::parse_value::<String>(params, 0);
        inv.investigation_program.date = Self::parse_value::<NaiveDate>(params, 1);
        inv.investigation_program.author = Self::parse_value::<String>(params, 2);
    }

    fn parse_gl(inv: &mut Investigation, params: &[&str]) {
        inv.investigation_program
            .guide
            .push(Self::parse_value::<String>(params, 0));
    }

    fn parse_at(inv: &mut Investigation, params: &[&str]) {
        inv.depthless_rock_sample.attribute = Self::parse_value::<String>(params, 0);
        inv.depthless_rock_sample.value = Self::parse_value::<String>(params, 1);
    }

    fn parse_al(inv: &mut Investigation, params: &[&str]) {
        inv.initial_borehole.depth = Self::parse_value::<f32>(params, 0);
        inv.initial_borehole.method = Self::parse_value::<String>(params, 1);
        inv.initial_borehole.soil_type = Self::parse_value::<String>(params, 2);
    }

    fn parse_zp(inv: &mut Investigation, params: &[&str]) {
        inv.standpipe.top_elevation = Self::parse_value::<f32>(params, 0);
        inv.standpipe.ground_elevation = Self::parse_value::<f32>(params, 1);
        inv.standpipe.protection_top_elevation = Self::parse_value::<f32>(params, 2);
        inv.standpipe.cover_elevation = Self::parse_value::<f32>(params, 3);
        inv.standpipe.sieve_bottom_elevation = Self::parse_value::<f32>(params, 4);
    }

    fn parse_tp(inv: &mut Investigation, params: &[&str]) {
        inv.standpipe.upper_structure = Self::parse_value::<String>(params, 0);
        inv.standpipe.sieve_length = Self::parse_value::<f32>(params, 1);
        inv.standpipe.sieve_type = Self::parse_value::<String>(params, 2);
        inv.standpipe.diameter = Self::parse_value::<f32>(params, 3);
        inv.standpipe.material = Self::parse_value::<String>(params, 4);
    }

    fn parse_lp(inv: &mut Investigation, params: &[&str]) {
        inv.standpipe.measure_point = Self::parse_value::<String>(params, 0);
        inv.standpipe.details = Self::parse_value::<String>(params, 1);
        inv.standpipe.locked = Self::parse_value::<String>(params, 2);
        inv.standpipe.lock_owner = Self::parse_value::<String>(params, 3);
        inv.standpipe.installer = Self::parse_value::<String>(params, 4);
    }

    fn parse_hm(inv: &mut Investigation, params: &[&str]) {
        let combined = params.join(" ");

        if inv.observations.is_empty() {
            inv.notes.push(ParsedValue::Some(combined));
        } else if let Some(last_obs) = inv.observations.last_mut() {
            match last_obs {
                Observation::PA { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::PI { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::LY { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::SI { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::HE { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::HK { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::PT { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::TR { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::PR { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::CP { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::CU { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::HP { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::PO { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::MW { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::VP { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::VO { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::VK { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::VPK { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::HV { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::HU { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::PS { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::PM { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::KO { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::KE { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::KR { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::NO { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                Observation::NE { notes, .. } => {
                    notes.push(ParsedValue::Some(combined));
                }
                _ => {
                    panic!("No observations available to add hidden text.");
                }
            }
        }
    }

    fn parse_tx(inv: &mut Investigation, params: &[&str]) {
        let combined = params.join(" ");

        if inv.observations.is_empty() {
            inv.free_text.push(ParsedValue::Some(combined));
        } else if let Some(last_obs) = inv.observations.last_mut() {
            match last_obs {
                Observation::PA { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::PI { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::LY { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::SI { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::HE { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::HK { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::PT { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::TR { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::PR { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::CP { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::CU { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::HP { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::PO { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::MW { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::VP { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::VO { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::VK { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::VPK { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::HV { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::HU { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::PS { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::PM { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::KO { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::KE { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::KR { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::NO { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                Observation::NE { free_text, .. } => {
                    free_text.push(ParsedValue::Some(combined));
                }
                _ => {
                    panic!("No observations available to add hidden text.");
                }
            }
        }
    }

    fn parse_ht(inv: &mut Investigation, params: &[&str]) {
        let combined = params.join(" ");

        if inv.observations.is_empty() {
            inv.hidden_text.push(ParsedValue::Some(combined));
        } else if let Some(last_obs) = inv.observations.last_mut() {
            match last_obs {
                Observation::PA { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::PI { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::LY { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::SI { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::HE { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::HK { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::PT { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::TR { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::PR { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::CP { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::CU { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::HP { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::PO { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::MW { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::VP { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::VO { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::VK { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::VPK { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::HV { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::HU { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::PS { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::PM { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::KO { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::KE { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::KR { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::NO { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                Observation::NE { hidden_text, .. } => {
                    hidden_text.push(ParsedValue::Some(combined));
                }
                _ => {
                    panic!("No observations available to add hidden text.");
                }
            }
        }
    }

    fn parse_em(inv: &mut Investigation, params: &[&str]) {
        let combined = params.join(" ");

        if let Some(last_obs) = inv.observations.last_mut() {
            match last_obs {
                Observation::PA {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::PI {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::LY {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::SI {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::HE {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::HK {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::PT {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::TR {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::PR {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::CP {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::CU {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::HP {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::PO {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::MW {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::VP {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::VO {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::VK {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::VPK {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::HV {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::HU {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::PS {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::PM {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::KO {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::KE {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::KR {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::NO {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                Observation::NE {
                    unofficial_soil_type,
                    ..
                } => {
                    unofficial_soil_type.push(ParsedValue::Some(combined));
                }
                _ => {
                    panic!("No observations available to add hidden text.");
                }
            }
        }
    }

    fn parse_lb(inv: &mut Investigation, params: &[&str]) {
        let lb_value = Observation::LB {
            attribute: Self::parse_value::<String>(params, 0),
            result: Self::parse_value::<String>(params, 1),
            unit: Self::parse_value::<String>(params, 2),
        };

        if let Some(last_obs) = inv.observations.last_mut() {
            match last_obs {
                Observation::NO { lab_other, .. } => {
                    lab_other.push(ParsedValue::Some(lb_value));
                }
                Observation::NE { lab_other, .. } => {
                    lab_other.push(ParsedValue::Some(lb_value));
                }
                _ => {
                    panic!("Cannot add LB value to a non-NO/NE observation");
                }
            }
        } else {
            panic!("No observation in the list to store LB value");
        }
    }

    fn parse_rk(inv: &mut Investigation, params: &[&str]) {
        let lb_value = Observation::RK {
            sieve_mm: Self::parse_value::<f32>(params, 0),
            pass_percent: Self::parse_value::<f32>(params, 1),
        };

        if let Some(last_obs) = inv.observations.last_mut() {
            match last_obs {
                Observation::NO { lab_sieve, .. } => {
                    lab_sieve.push(ParsedValue::Some(lb_value));
                }
                Observation::NE { lab_sieve, .. } => {
                    lab_sieve.push(ParsedValue::Some(lb_value));
                }
                _ => {
                    panic!("Cannot add RK value to a non-NO/NE observation");
                }
            }
        } else {
            panic!("No observation in the list to store LB value");
        }
    }

    fn parse_pa(inv: &mut Investigation, params: &[&str]) {
        let mut load = ParsedValue::None;
        let mut hits = ParsedValue::None;

        if let ParsedValue::Some(val) = Self::parse_value::<f32>(params, 1) {
            if val >= 0.0 {
                load = ParsedValue::Some(val);
            } else {
                hits = ParsedValue::Some(val.abs() as i32);
            }
        }

        let obs = Observation::PA {
            depth: Self::parse_value::<f32>(params, 0),
            load,
            half_turns: Self::parse_value::<i32>(params, 2),
            hits,
            soil_type: Self::parse_value::<String>(params, 3),

            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: ParsedValue::None,
        };

        inv.observations.push(obs);
    }

    fn parse_pi(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::PI {
            depth: Self::parse_value::<f32>(params, 0),
            soil_type: Self::parse_value::<String>(params, 1),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_ly(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::LY {
            depth: Self::parse_value::<f32>(params, 0),
            load: Self::parse_value::<f32>(params, 1),
            hits: Self::parse_value::<i32>(params, 2),
            soil_type: Self::parse_value::<String>(params, 3),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_si(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::SI {
            depth: Self::parse_value::<f32>(params, 0),
            shear_str: Self::parse_value::<f32>(params, 1),
            disturb_shear_str: Self::parse_value::<f32>(params, 2),
            sensitivity: Self::parse_value::<f32>(params, 3),
            residual_str: Self::parse_value::<f32>(params, 4),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_he(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::HE {
            depth: Self::parse_value::<f32>(params, 0),
            hits: Self::parse_value::<i32>(params, 1),
            soil_type: Self::parse_value::<String>(params, 2),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_hk(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::HK {
            depth: Self::parse_value::<f32>(params, 0),
            hits: Self::parse_value::<i32>(params, 1),
            torque: Self::parse_value::<f32>(params, 2),
            soil_type: Self::parse_value::<String>(params, 3),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_pt(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::PT {
            depth: Self::parse_value::<f32>(params, 0),
            soil_type: Self::parse_value::<String>(params, 1),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_tr(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::TR {
            depth: Self::parse_value::<f32>(params, 0),
            soil_type: Self::parse_value::<String>(params, 1),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_pr(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::PR {
            depth: Self::parse_value::<f32>(params, 0),
            total_resistance: Self::parse_value::<f32>(params, 1),
            sleeve_friction: Self::parse_value::<f32>(params, 2),
            soil_type: Self::parse_value::<String>(params, 3),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_cp(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::CP {
            depth: Self::parse_value::<f32>(params, 0),
            total_resistance: Self::parse_value::<f32>(params, 1),
            sleeve_friction: Self::parse_value::<f32>(params, 2),
            tip_resistance: Self::parse_value::<f32>(params, 3),
            soil_type: Self::parse_value::<String>(params, 4),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_cu(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::CU {
            depth: Self::parse_value::<f32>(params, 0),
            total_resistance: Self::parse_value::<f32>(params, 1),
            sleeve_friction: Self::parse_value::<f32>(params, 2),
            tip_resistance: Self::parse_value::<f32>(params, 3),
            pore_water_pressure: Self::parse_value::<f32>(params, 4),
            soil_type: Self::parse_value::<String>(params, 5),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_hp(inv: &mut Investigation, params: &[&str]) {
        let mut hits = ParsedValue::None;
        let mut pressure = ParsedValue::None;

        if let Some(m) = params.get(3) {
            match *m {
                "H" => hits = Self::parse_value::<i32>(params, 1),
                "P" => pressure = Self::parse_value::<f32>(params, 1),
                _ => {}
            }
            let obs = Observation::HP {
                depth: Self::parse_value::<f32>(params, 0),
                hits,
                pressure,
                torque: Self::parse_value::<f32>(params, 2),
                mode: Self::parse_value::<String>(params, 3),
                soil_type: Self::parse_value::<String>(params, 4),
                notes: vec![],
                free_text: vec![],
                hidden_text: vec![],
                unofficial_soil_type: vec![],
                water_observed: false,
            };

            inv.observations.push(obs);
        }
    }

    fn parse_po(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::PO {
            depth: Self::parse_value::<f32>(params, 0),
            time: Self::parse_value::<i32>(params, 1),
            soil_type: Self::parse_value::<String>(params, 2),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_mw(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::MW {
            depth: Self::parse_value::<f32>(params, 0),
            advance_rate: Self::parse_value::<f32>(params, 1),
            compressive_force: Self::parse_value::<f32>(params, 2),
            flushing_pressure: Self::parse_value::<f32>(params, 3),
            water_consumption: Self::parse_value::<f32>(params, 4),
            torque: Self::parse_value::<f32>(params, 5),
            rotation_speed: Self::parse_value::<f32>(params, 6),
            // TODO Implement hits here / bool value
            hits: Self::parse_value::<String>(params, 7),
            soil_type: Self::parse_value::<String>(params, 8),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_vp(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::VP {
            surface_elev: Self::parse_value::<f32>(params, 0),
            date: Self::parse_value::<NaiveDate>(params, 1),
            pipe_top_elev: Self::parse_value::<f32>(params, 2),
            pipe_bot_elev: Self::parse_value::<f32>(params, 3),
            sieve_len: Self::parse_value::<f32>(params, 4),
            measurer: Self::parse_value::<String>(params, 5),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_vo(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::VO {
            surface_elev: Self::parse_value::<f32>(params, 0),
            date: Self::parse_value::<NaiveDate>(params, 1),
            pipe_top_elev: Self::parse_value::<f32>(params, 2),
            pipe_bot_elev: Self::parse_value::<f32>(params, 3),
            sieve_len: Self::parse_value::<f32>(params, 4),
            measurer: Self::parse_value::<String>(params, 5),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_vk(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::VK {
            surface_elev: Self::parse_value::<f32>(params, 0),
            date: Self::parse_value::<NaiveDate>(params, 1),
            // TODO Implement bool enum here for water type
            water_type: Self::parse_value::<String>(params, 2),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_vpk(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::VPK {
            surface_elev: Self::parse_value::<f32>(params, 0),
            date: Self::parse_value::<NaiveDate>(params, 1),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_hv(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::HV {
            depth: Self::parse_value::<f32>(params, 0),
            pressure: Self::parse_value::<f32>(params, 1),
            date: Self::parse_value::<NaiveDate>(params, 2),
            measurer: Self::parse_value::<String>(params, 3),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_hu(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::HU {
            surface_elev: Self::parse_value::<f32>(params, 0),
            date: Self::parse_value::<NaiveDate>(params, 1),
            pipe_top_elev: Self::parse_value::<f32>(params, 2),
            pipe_bot_elev: Self::parse_value::<f32>(params, 3),
            sieve_len: Self::parse_value::<f32>(params, 4),
            measurer: Self::parse_value::<String>(params, 5),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_ps(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::PS {
            depth: Self::parse_value::<f32>(params, 0),
            modulus: Self::parse_value::<f32>(params, 1),
            fail_pressure: Self::parse_value::<f32>(params, 2),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_pm(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::PM {
            elev: Self::parse_value::<f32>(params, 0),
            date: Self::parse_value::<NaiveDate>(params, 1),
            measurer: Self::parse_value::<String>(params, 2),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_ko(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::KO {
            depth: Self::parse_value::<f32>(params, 0),
            soil_type: Self::parse_value::<String>(params, 1),
            stones: Self::parse_value::<f32>(params, 2),
            boulders: Self::parse_value::<i32>(params, 3),
            max_width: Self::parse_value::<f32>(params, 4),
            min_width: Self::parse_value::<f32>(params, 5),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_ke(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::KE {
            start_depth: Self::parse_value::<f32>(params, 0),
            end_depth: Self::parse_value::<f32>(params, 1),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_kr(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::KR {
            start_depth: Self::parse_value::<f32>(params, 0),
            end_depth: Self::parse_value::<f32>(params, 1),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_no(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::NO {
            start_depth: Self::parse_value::<f32>(params, 0),
            sample_id: Self::parse_value::<String>(params, 1),
            end_depth: Self::parse_value::<f32>(params, 2),
            soil_type: Self::parse_value::<String>(params, 3),
            lab_sieve: Vec::new(),
            lab_other: Vec::new(),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }

    fn parse_ne(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation::NE {
            start_depth: Self::parse_value::<f32>(params, 0),
            sample_id: Self::parse_value::<String>(params, 1),
            end_depth: Self::parse_value::<f32>(params, 2),
            soil_type: Self::parse_value::<String>(params, 3),
            lab_sieve: Vec::new(),
            lab_other: Vec::new(),
            notes: vec![],
            free_text: vec![],
            hidden_text: vec![],
            unofficial_soil_type: vec![],
            water_observed: Default::default(),
        };

        inv.observations.push(obs);
    }
}
