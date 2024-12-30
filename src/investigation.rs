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
    Classification, Coordinates, DepthlessRockSample, Equipment, InitialBorehole, Line, Method, Observation,
    Organisations, ParsedValue, Program, Record, Spatial, Standpipe, Termination, Work,
};

use std::fmt;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Investigation {
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
    pub notes: Vec<ParsedValue<String>>,
    pub free_text: Vec<ParsedValue<String>>,
    pub hidden_text: Vec<ParsedValue<String>>,
    pub observations: Vec<Observation>,
    // Computed and additional properties
    pub spatial: Spatial,
    pub total_depth: Option<f32>,
}

impl Investigation {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn compute_properties(&mut self) {
        //for observation in &mut self.observations {
        // TODO: calculate total depth
        // TODO: calculate all soil layer thicknesses
        // TODO: update observations to include missing soils
        //}
    }
}

impl fmt::Display for Investigation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.organisations != Organisations::default() {
            writeln!(f, "{}", self.organisations)?;
        }

        if self.classification != Classification::default() {
            writeln!(f, "{}", self.classification)?;
        }

        // Work
        if self.work != Work::default() {
            writeln!(f, "{}", self.work)?;
        }

        // Record
        if self.record != Record::default() {
            writeln!(f, "{}", self.record)?;
        }

        // Method
        if self.method != Method::default() {
            writeln!(f, "{}", self.method)?;
        }

        // Equipment
        if self.equipment != Equipment::default() {
            writeln!(f, "{}", self.equipment)?;
        }

        // Coordinates
        if self.coordinates != Coordinates::default() {
            writeln!(f, "{}", self.coordinates)?;
        }

        // Line
        if self.line != Line::default() {
            writeln!(f, "{}", self.line)?;
        }

        // Termination
        if self.termination != Termination::default() {
            writeln!(f, "{}", self.termination)?;
        }

        // Program
        if self.program != Program::default() {
            writeln!(f, "{}", self.program)?;
        }

        // InitialBorehole
        if self.initial_borehole != InitialBorehole::default() {
            writeln!(f, "{}", self.initial_borehole)?;
        }

        // Standpipe
        if self.standpipe != Standpipe::default() {
            writeln!(f, "Standpipe:")?;
            writeln!(f, "{}", self.standpipe)?;
        }

        // DepthlessRockSample
        if self.depthless_rock_sample != DepthlessRockSample::default() {
            writeln!(f, "Depthless rock sample:")?;
            writeln!(f, "{}", self.depthless_rock_sample)?;
        }

        // Notes
        if !self.notes.is_empty() {
            writeln!(f, "Notes:")?;
            for (i, note) in self.notes.iter().enumerate() {
                if let Some(ref note_str) = note.format_display() {
                    writeln!(f, "  {}. {}", i + 1, note_str)?;
                }
            }
        }

        // Free Text
        if !self.free_text.is_empty() {
            writeln!(f, "Free Text:")?;
            for (i, text) in self.free_text.iter().enumerate() {
                if let Some(ref text_str) = text.format_display() {
                    writeln!(f, "  {}. {}", i + 1, text_str)?;
                }
            }
        }

        // Hidden Text
        if !self.hidden_text.is_empty() {
            writeln!(f, "Hidden Text:")?;
            for (i, text) in self.hidden_text.iter().enumerate() {
                if let Some(ref text_str) = text.format_display() {
                    writeln!(f, "  {}. {}", i + 1, text_str)?;
                }
            }
        }

        // Observations
        // if !self.observations.is_empty() {
        //     writeln!(f, "Observations:")?;
        //     for (i, observation) in self.observations.iter().enumerate() {
        //         writeln!(f, "  Observation {}:", i + 1)?;
        //         writeln!(f, "    {}", observation)?;
        //     }
        // }

        // Spatial
        // if self.spatial != Spatial::default() {
        //    writeln!(f, "Spatial:")?;
        //    writeln!(f, "{}", self.spatial)?;
        //}

        // Total Depth
        //if let Some(ref depth) = self.total_depth {
        //    writeln!(f, "Total Depth: {}", depth)?;
        //}

        Ok(())
    }
}
