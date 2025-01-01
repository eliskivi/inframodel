pub(crate) mod classification;
pub(crate) mod coordinates;
pub(crate) mod depthless_rock_sample;
pub(crate) mod equipment;
pub(crate) mod initial_borehole;
pub(crate) mod line;
pub(crate) mod method;
pub(crate) mod organisations;
pub(crate) mod program;
pub(crate) mod record;
pub(crate) mod standpipe;
pub(crate) mod termination;
pub(crate) mod work;

use crate::{
    Classification, Coordinates, DepthlessRockSample, Equipment, FileInfo, InitialBorehole, Line,
    Method, Observation, ObservationValues, Organisations, ParseResult, Program, Record, Spatial,
    Standpipe, Termination, Work,
};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Investigation {
    // File-level properties
    pub file_info: FileInfo,
    pub spatial: Spatial,

    // Parsed properties
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
    pub notes: Vec<ParseResult<String>>,
    pub free_text: Vec<ParseResult<String>>,
    pub hidden_text: Vec<ParseResult<String>>,
    pub observations: Vec<Observation>,

    // Computed and additional properties
    pub total_depth: Option<f32>,
    pub soil_layers: Vec<SoilLayer>,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct SoilLayer {
    pub soil_type: String,
    pub thickness: f32,
}

impl Investigation {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn compute_properties(&mut self) {
        self.add_missing_soil_types();
        self.calculate_total_depth();
        self.calculate_soil_layer_thicknesses();
    }

    pub fn add_missing_soil_types(&mut self) {
        let mut last_soil_type: Option<String> = None;

        for observation in &mut self.observations {
            match &mut observation.values {
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
                        last_soil_type = Some(soil.clone());
                    } else if let Some(ref soil) = last_soil_type {
                        *soil_type = ParseResult::Parsed(soil.clone());
                    }
                }
                _ => {}
            }
        }
    }

    fn calculate_total_depth(&mut self) {
        self.total_depth = self.observations.last().and_then(|last_observation| {
            let depth_result = match &last_observation.values {
                ObservationValues::PA { depth, .. }
                | ObservationValues::PI { depth, .. }
                | ObservationValues::LY { depth, .. }
                | ObservationValues::SI { depth, .. }
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
                | ObservationValues::HV { depth, .. }
                | ObservationValues::KO { depth, .. }
                | ObservationValues::PS { depth, .. } => depth,
                _ => return None,
            };

            match depth_result {
                ParseResult::Parsed(x) => Some(*x),
                _ => None,
            }
        });
    }

    pub fn calculate_soil_layer_thicknesses(&mut self) {
        let mut merged_layers: Vec<SoilLayer> = Vec::new();
        let mut previous_depth = 0.0;

        for observation in &self.observations {
            let current_depth = match observation.values.get_parsed_depth() {
                Some(d) => d,
                None => continue,
            };

            let soil_type = match observation.values.get_parsed_soil_type() {
                Some(s) => s.clone(),
                None => continue,
            };

            let thickness = current_depth - previous_depth;

            if thickness < 0.0 {
                continue;
            }

            let new_layer = SoilLayer {
                soil_type: soil_type.clone(),
                thickness,
            };

            if let Some(last_layer) = merged_layers.last_mut() {
                if last_layer.soil_type == new_layer.soil_type {
                    last_layer.thickness += new_layer.thickness;
                } else {
                    merged_layers.push(new_layer);
                }
            } else {
                merged_layers.push(new_layer);
            }

            previous_depth = current_depth;
        }

        self.soil_layers = merged_layers;
    }
}
