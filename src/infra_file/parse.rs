use crate::{
    ClassificationName, CoordinateSystem, Digitized, ElevationSystem, FileInfo, InfraFile,
    InitialBoreToken, Investigation, LabResult, MethodToken, Observation, ObservationValues,
    ParseResult, Sampler, TerminationToken, TryParse,
};

use chardetng::EncodingDetector;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref FLOAT_RE: Regex = Regex::new(r"^[+-]?[0-9]+([.,][0-9]+)?$").unwrap();
}

impl InfraFile {
    pub fn parse_file(file_path: &str) -> Result<InfraFile, String> {
        let buffer = match std::fs::read(file_path) {
            Ok(data) => data,
            Err(e) => return Err(format!("Failed to read file '{}': {}", file_path, e)),
        };

        let mut detector = EncodingDetector::new();
        detector.feed(&buffer, true);
        let encoding = detector.guess(None, true);
        let (decoded, _, _) = encoding.decode(&buffer);
        let lines: Vec<&str> = decoded.lines().collect();

        let mut infra = InfraFile {
            file_info: FileInfo {
                path: Some(file_path.to_string()),
                encoding: Some(encoding.name().to_string()),
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
                    // TODO: Implement KK token parsing correctly
                    _ if FLOAT_RE.is_match(token) => match &inv.method.token {
                        ParseResult::Parsed(method) => match method {
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
                            MethodToken::None => {
                                unreachable!("Unknown method: {}", token)
                            }
                        },
                        ParseResult::None => {
                            // panic!("No method was specified, got: {}", token);
                        }
                        ParseResult::Fallback(_) => {
                            // panic!("Invalid method: {}", original);
                        }
                    },
                    _ => {}
                }
            }
        }

        Self::compute_properties(&mut infra);
        Ok(infra)
    }

    fn compute_properties(infra: &mut InfraFile) {
        for investigation in &mut infra.investigations {
            investigation.file_info = infra.file_info.clone();
            investigation.spatial = infra.spatial.clone();
            investigation.compute_properties();
        }
    }

    fn parse_value<T: TryParse>(params: &[&str], index: usize) -> ParseResult<T> {
        if let Some(&raw) = params.get(index) {
            ParseResult::parse(raw)
        } else {
            ParseResult::None
        }
    }

    fn parse_fo(infra: &mut InfraFile, params: &[&str]) {
        infra.format.version = Self::parse_value::<String>(params, 0);
        infra.format.used_software = Self::parse_value::<String>(params, 1);
        infra.format.software_version = Self::parse_value::<String>(params, 2);
    }

    fn parse_kj(infra: &mut InfraFile, params: &[&str]) {
        infra.spatial.coordinate_system = Self::parse_value::<CoordinateSystem>(params, 0);
        infra.spatial.elevation_system = Self::parse_value::<ElevationSystem>(params, 1);
    }

    fn parse_om(inv: &mut Investigation, params: &[&str]) {
        inv.organisations.owner_name = Self::parse_value::<String>(params, 0);
    }

    fn parse_ml(inv: &mut Investigation, params: &[&str]) {
        inv.classification.name = Self::parse_value::<ClassificationName>(params, 0);
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
        inv.method.token = Self::parse_value::<MethodToken>(params, 0);
        inv.method.category = Self::parse_value::<i32>(params, 1);
        inv.method.id = Self::parse_value::<String>(params, 2);
        inv.method.standard = Self::parse_value::<String>(params, 3);
        inv.method.sampler = Self::parse_value::<Sampler>(params, 4);
        inv.method.specifier = Self::parse_value::<String>(params, 5);
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
        inv.termination.token = Self::parse_value::<TerminationToken>(params, 0);
        infra.investigations.push(std::mem::take(inv));
    }

    fn parse_gr(inv: &mut Investigation, params: &[&str]) {
        inv.program.name = Self::parse_value::<String>(params, 0);
        inv.program.date = Self::parse_value::<NaiveDate>(params, 1);
        inv.program.author = Self::parse_value::<String>(params, 2);
    }

    fn parse_gl(inv: &mut Investigation, params: &[&str]) {
        inv.program
            .guide
            .push(Self::parse_value::<String>(params, 0));
    }

    fn parse_at(inv: &mut Investigation, params: &[&str]) {
        inv.depthless_rock_sample.attribute = Self::parse_value::<String>(params, 0);
        inv.depthless_rock_sample.value = Self::parse_value::<String>(params, 1);
    }

    fn parse_al(inv: &mut Investigation, params: &[&str]) {
        inv.initial_borehole.depth = Self::parse_value::<f32>(params, 0);
        inv.initial_borehole.method = Self::parse_value::<InitialBoreToken>(params, 1);
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

        match inv.observations.last_mut() {
            Some(last_obs) => {
                last_obs.notes.push(ParseResult::Parsed(combined));
            }
            None => {
                inv.notes.push(ParseResult::Parsed(combined));
            }
        }
    }

    fn parse_tx(inv: &mut Investigation, params: &[&str]) {
        let combined = params.join(" ");

        match inv.observations.last_mut() {
            Some(last_obs) => {
                last_obs.free_text.push(ParseResult::Parsed(combined));
            }
            None => {
                inv.free_text.push(ParseResult::Parsed(combined));
            }
        }
    }

    fn parse_ht(inv: &mut Investigation, params: &[&str]) {
        let combined = params.join(" ");

        match inv.observations.last_mut() {
            Some(last_obs) => {
                last_obs.hidden_text.push(ParseResult::Parsed(combined));
            }
            None => {
                inv.hidden_text.push(ParseResult::Parsed(combined));
            }
        }
    }

    fn parse_em(inv: &mut Investigation, params: &[&str]) {
        let combined = params.join(" ");

        if let Some(last_obs) = inv.observations.last_mut() {
            last_obs
                .unofficial_soil_type
                .push(ParseResult::Parsed(combined));
        }
    }

    fn parse_lb(inv: &mut Investigation, params: &[&str]) {
        // TODO: Implement common lab types
        let lab_other = LabResult::Other {
            attribute: Self::parse_value::<String>(params, 0),
            result: Self::parse_value::<String>(params, 1),
            unit: Self::parse_value::<String>(params, 2),
        };

        if let Some(last_obs) = inv.observations.last_mut() {
            match &mut last_obs.values {
                ObservationValues::NO { lab_values, .. } => {
                    lab_values.push(ParseResult::Parsed(lab_other));
                }
                ObservationValues::NE { lab_values, .. } => {
                    lab_values.push(ParseResult::Parsed(lab_other));
                }
                _ => {
                    // panic!("Cannot add RK value to a non-NO/NE observation");
                }
            }
        } else {
            // panic!("No observation in the list to store LB value");
        }
    }

    fn parse_rk(inv: &mut Investigation, params: &[&str]) {
        let lab_sieve = LabResult::GrainSize {
            grain_mm: Self::parse_value::<f32>(params, 0),
            pass_percent: Self::parse_value::<f32>(params, 1),
        };

        if let Some(last_obs) = inv.observations.last_mut() {
            match &mut last_obs.values {
                ObservationValues::NO { lab_values, .. } => {
                    lab_values.push(ParseResult::Parsed(lab_sieve));
                }
                ObservationValues::NE { lab_values, .. } => {
                    lab_values.push(ParseResult::Parsed(lab_sieve));
                }
                _ => {
                    // panic!("Cannot add RK value to a non-NO/NE observation");
                }
            }
        } else {
            // panic!("No observation in the list to store LB value");
        }
    }

    fn parse_pa(inv: &mut Investigation, params: &[&str]) {
        let mut load = ParseResult::None;
        let mut hits = ParseResult::None;

        if let ParseResult::Parsed(val) = Self::parse_value::<f32>(params, 1) {
            if val >= 0.0 {
                load = ParseResult::Parsed(val);
            } else {
                hits = ParseResult::Parsed(val.abs() as i32);
            }
        }

        let obs = Observation {
            values: ObservationValues::PA {
                depth: Self::parse_value::<f32>(params, 0),
                load,
                half_turns: Self::parse_value::<i32>(params, 2),
                hits,
                soil_type: Self::parse_value::<String>(params, 3),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_pi(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::PI {
                depth: Self::parse_value::<f32>(params, 0),
                soil_type: Self::parse_value::<String>(params, 1),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_ly(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::LY {
                depth: Self::parse_value::<f32>(params, 0),
                load: Self::parse_value::<f32>(params, 1),
                hits: Self::parse_value::<i32>(params, 2),
                soil_type: Self::parse_value::<String>(params, 3),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_si(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::SI {
                depth: Self::parse_value::<f32>(params, 0),
                shear_str: Self::parse_value::<f32>(params, 1),
                disturb_shear_str: Self::parse_value::<f32>(params, 2),
                sensitivity: Self::parse_value::<f32>(params, 3),
                residual_str: Self::parse_value::<f32>(params, 4),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_he(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::HE {
                depth: Self::parse_value::<f32>(params, 0),
                hits: Self::parse_value::<i32>(params, 1),
                soil_type: Self::parse_value::<String>(params, 2),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_hk(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::HK {
                depth: Self::parse_value::<f32>(params, 0),
                hits: Self::parse_value::<i32>(params, 1),
                torque: Self::parse_value::<f32>(params, 2),
                soil_type: Self::parse_value::<String>(params, 3),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_pt(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::PT {
                depth: Self::parse_value::<f32>(params, 0),
                soil_type: Self::parse_value::<String>(params, 1),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_tr(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::TR {
                depth: Self::parse_value::<f32>(params, 0),
                soil_type: Self::parse_value::<String>(params, 1),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_pr(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::PR {
                depth: Self::parse_value::<f32>(params, 0),
                total_resistance: Self::parse_value::<f32>(params, 1),
                sleeve_friction: Self::parse_value::<f32>(params, 2),
                soil_type: Self::parse_value::<String>(params, 3),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_cp(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::CP {
                depth: Self::parse_value::<f32>(params, 0),
                total_resistance: Self::parse_value::<f32>(params, 1),
                sleeve_friction: Self::parse_value::<f32>(params, 2),
                tip_resistance: Self::parse_value::<f32>(params, 3),
                soil_type: Self::parse_value::<String>(params, 4),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_cu(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::CU {
                depth: Self::parse_value::<f32>(params, 0),
                total_resistance: Self::parse_value::<f32>(params, 1),
                sleeve_friction: Self::parse_value::<f32>(params, 2),
                tip_resistance: Self::parse_value::<f32>(params, 3),
                pore_water_pressure: Self::parse_value::<f32>(params, 4),
                soil_type: Self::parse_value::<String>(params, 5),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_hp(inv: &mut Investigation, params: &[&str]) {
        let mut hits = ParseResult::None;
        let mut pressure = ParseResult::None;

        if let Some(m) = params.get(3) {
            match *m {
                "H" => hits = Self::parse_value::<i32>(params, 1),
                "P" => pressure = Self::parse_value::<f32>(params, 1),
                _ => {}
            }

            let obs = Observation {
                values: ObservationValues::HP {
                    depth: Self::parse_value::<f32>(params, 0),
                    hits,
                    pressure,
                    torque: Self::parse_value::<f32>(params, 2),
                    // TODO: Implement enum holding the mode
                    mode: Self::parse_value::<String>(params, 3),
                    soil_type: Self::parse_value::<String>(params, 4),
                },
                ..Default::default()
            };

            inv.observations.push(obs);
        }
    }

    fn parse_po(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::PO {
                depth: Self::parse_value::<f32>(params, 0),
                time: Self::parse_value::<i32>(params, 1),
                soil_type: Self::parse_value::<String>(params, 2),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_mw(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::MW {
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
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_vp(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::VP {
                surface_elev: Self::parse_value::<f32>(params, 0),
                date: Self::parse_value::<NaiveDate>(params, 1),
                pipe_top_elev: Self::parse_value::<f32>(params, 2),
                pipe_bot_elev: Self::parse_value::<f32>(params, 3),
                sieve_len: Self::parse_value::<f32>(params, 4),
                measurer: Self::parse_value::<String>(params, 5),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_vo(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::VO {
                surface_elev: Self::parse_value::<f32>(params, 0),
                date: Self::parse_value::<NaiveDate>(params, 1),
                pipe_top_elev: Self::parse_value::<f32>(params, 2),
                pipe_bot_elev: Self::parse_value::<f32>(params, 3),
                sieve_len: Self::parse_value::<f32>(params, 4),
                measurer: Self::parse_value::<String>(params, 5),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_vk(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::VK {
                surface_elev: Self::parse_value::<f32>(params, 0),
                date: Self::parse_value::<NaiveDate>(params, 1),
                // TODO Implement bool enum here for water type
                water_type: Self::parse_value::<String>(params, 2),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_vpk(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::VPK {
                surface_elev: Self::parse_value::<f32>(params, 0),
                date: Self::parse_value::<NaiveDate>(params, 1),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_hv(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::HV {
                depth: Self::parse_value::<f32>(params, 0),
                pressure: Self::parse_value::<f32>(params, 1),
                date: Self::parse_value::<NaiveDate>(params, 2),
                measurer: Self::parse_value::<String>(params, 3),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_hu(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::HU {
                surface_elev: Self::parse_value::<f32>(params, 0),
                date: Self::parse_value::<NaiveDate>(params, 1),
                pipe_top_elev: Self::parse_value::<f32>(params, 2),
                pipe_bot_elev: Self::parse_value::<f32>(params, 3),
                sieve_len: Self::parse_value::<f32>(params, 4),
                measurer: Self::parse_value::<String>(params, 5),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_ps(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::PS {
                depth: Self::parse_value::<f32>(params, 0),
                modulus: Self::parse_value::<f32>(params, 1),
                fail_pressure: Self::parse_value::<f32>(params, 2),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_pm(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::PM {
                elev: Self::parse_value::<f32>(params, 0),
                date: Self::parse_value::<NaiveDate>(params, 1),
                measurer: Self::parse_value::<String>(params, 2),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_ko(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::KO {
                depth: Self::parse_value::<f32>(params, 0),
                soil_type: Self::parse_value::<String>(params, 1),
                stones: Self::parse_value::<f32>(params, 2),
                boulders: Self::parse_value::<i32>(params, 3),
                max_width: Self::parse_value::<f32>(params, 4),
                min_width: Self::parse_value::<f32>(params, 5),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_ke(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::KE {
                start_depth: Self::parse_value::<f32>(params, 0),
                end_depth: Self::parse_value::<f32>(params, 1),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_kr(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::KR {
                start_depth: Self::parse_value::<f32>(params, 0),
                end_depth: Self::parse_value::<f32>(params, 1),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_no(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::NO {
                start_depth: Self::parse_value::<f32>(params, 0),
                sample_id: Self::parse_value::<String>(params, 1),
                end_depth: Self::parse_value::<f32>(params, 2),
                soil_type: Self::parse_value::<String>(params, 3),
                lab_values: Vec::new(),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }

    fn parse_ne(inv: &mut Investigation, params: &[&str]) {
        let obs = Observation {
            values: ObservationValues::NE {
                start_depth: Self::parse_value::<f32>(params, 0),
                sample_id: Self::parse_value::<String>(params, 1),
                end_depth: Self::parse_value::<f32>(params, 2),
                soil_type: Self::parse_value::<String>(params, 3),
                lab_values: Vec::new(),
            },
            ..Default::default()
        };

        inv.observations.push(obs);
    }
}
