use crate::observation::Observation;
use crate::parsed_value::ParsedValue;
use crate::Spatial;
use chrono::NaiveDate;
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

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Organisations {
    pub owner_name: ParsedValue<String>,
    pub investigator_name: ParsedValue<String>,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Classification {
    pub name: ParsedValue<ClassificationName>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum ClassificationName {
    #[default]
    GEO,
    ISO,
}

impl fmt::Display for ClassificationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            ClassificationName::GEO => "GEO classification",
            ClassificationName::ISO => "SFS-EN ISO 14688-2",
        };
        write!(f, "{}", token_str)
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Work {
    pub id: ParsedValue<String>,
    pub name: ParsedValue<String>,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Record {
    pub number: ParsedValue<i32>,
    pub driller: ParsedValue<String>,
    pub inspector: ParsedValue<String>,
    pub processor: ParsedValue<String>,
    pub digitalized: ParsedValue<Digitized>,
    pub condition: ParsedValue<String>,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Method {
    pub token: ParsedValue<MethodToken>,
    pub category: ParsedValue<i32>,
    pub id: ParsedValue<String>,
    pub standard: ParsedValue<String>,
    pub sampler: ParsedValue<Sampler>,
    pub specifier: ParsedValue<String>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
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

impl fmt::Display for Sampler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            Sampler::K => "Auger drill", // Finnish: kierrekaira
            Sampler::L => "Shovel",
            Sampler::PMK => "Small piston drill-26", // Finnish: Pienoismäntäkaira-26
            Sampler::R => "Bagged sample",
            Sampler::ST50 => "St-50",
            Sampler::ST60 => "St-60",
            Sampler::Unknown => "Unknown sampler",
        };
        write!(f, "{}", token_str)
    }
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

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Termination {
    pub token: ParsedValue<TerminationToken>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
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

impl fmt::Display for TerminationToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            TerminationToken::TM => "Dense soil layer",
            TerminationToken::KI => "Estimated rock or boulder",
            TerminationToken::KL => "Rock, boulder or bedrock contact",
            TerminationToken::KA => "Bedrock contact (verified rock)",
            TerminationToken::KK => "Bedrock surface (verified with trial pit)",
            TerminationToken::MS => "Specified depth",
            TerminationToken::KN => "Bridging between stones and boulders",
            TerminationToken::JA => "Continues as another investigation",
            TerminationToken::Unknown => "Unknown reason",
        };
        write!(f, "{}", token_str)
    }
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

#[derive(Clone, PartialEq, Debug, Default)]
pub struct InitialBorehole {
    pub depth: ParsedValue<f32>,
    pub method: ParsedValue<InitialBoreToken>,
    pub soil_type: ParsedValue<String>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
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

impl fmt::Display for InitialBoreToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            InitialBoreToken::SI => "Through protective pipe",
            InitialBoreToken::LK => "Shovel pit",
            InitialBoreToken::AP => "Opening with drill",
            InitialBoreToken::LY => "Hammered",
            InitialBoreToken::VA => "Water initiation",
            InitialBoreToken::JA => "Continues previous investigation",
            InitialBoreToken::Unknown => "Unknown method",
        };
        write!(f, "{}", token_str)
    }
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Digitized {
    #[default]
    No,
    Yes,
}

impl fmt::Display for Digitized {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            Digitized::No => "Not digitized",
            Digitized::Yes => "Digitized",
        };
        write!(f, "{}", token_str)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum MethodToken {
    #[default]
    None,
    PA,  // Finnish: Painokairaus
    PI,  // Finnish: Pistokairaus
    LY,  // Finnish: Lyöntikairaus
    SI,  // Finnish: Siipikairaus
    HE,  // Finnish: Heijarikairaus
    HK,  // Finnish: Heijarikairaus vääntömomentilla
    PT,  // Finnish: Putkikairaus
    TR,  // Finnish: Tärykairaus
    PR,  // Finnish: Puristinkairaus
    CP,  // Finnish: Puristinkairaus (CPT)
    CU,  // Finnish: Huokospainekairaus (CPTU)
    HP,  // Finnish: Puristin-heijarikairaus
    PO,  // Finnish: Porakonekairaus
    MW,  // Finnish: MWD-kairaus
    VP,  // Finnish: Pohjaveden mittausputki
    VO,  // Finnish: Orsiveden mittausputki
    VK,  // Finnish: Vedenpinnan mittaus kaivosta
    VPK, // Finnish: Kalliopohjavesiputki
    HV,  // Finnish: Huokosveden paineen mittaus
    HU,  // Finnish: Huokosilmaputki
    PS,  // Finnish: Pressometrikoe
    PM,  // Finnish: Painumamittaus
    KO,  // Finnish: Koekuoppa
    KE,  // Finnish: Kallionäytekairaus laajennettu
    KR,  // Finnish: Kallionäytekairaus videoitu
    NO,  // Finnish: Häiritty näytteenotto
    NE,  // Finnish: Häiriintymätön näytteenotto
}

impl fmt::Display for MethodToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            MethodToken::None => "None",
            MethodToken::PA => "Weight sounding test", // Finnish: Painokairaus
            MethodToken::PI => "Stick drilling",       // Finnish: Pistokairaus
            MethodToken::LY => "Hammer drilling",      // Finnish: Lyöntikairaus
            MethodToken::SI => "Field vane test",      // Finnish: Siipikairaus
            MethodToken::HE => "Dynamic probing",      // Finnish: Heijarikairaus
            MethodToken::HK => "Dynamic probing with torque", // Finnish: Heijarikairaus vääntömomentilla
            // TODO: Verify english translation
            MethodToken::PT => "Pipe drilling", // Finnish: Putkikairaus
            // TODO: Verify english translation
            MethodToken::TR => "Pin drilling", // Finnish: Tärykairaus
            // TODO: Verify english translation
            MethodToken::PR => "Static penetration test", // Finnish: Puristinkairaus
            MethodToken::CP => "Cone penetration test (CPT)", // Finnish: Puristinkairaus (CPT)
            MethodToken::CU => "CPTU-sounding (CPTU)",    // Finnish: Huokospainekairaus (CPTU)
            MethodToken::HP => "Static dynamic penetration test", // Finnish: Puristin-heijarikairaus
            MethodToken::PO => "MWD quality class 3",     // Finnish: Porakonekairaus
            MethodToken::MW => "MWD-drilling",            // Finnish: MWD-kairaus
            MethodToken::VP => "Standpipe for groundwater table", // Finnish: Pohjaveden mittausputki
            MethodToken::VO => "Standpipe for perched water table", // Finnish: Orsiveden mittausputki
            MethodToken::VK => "Water table in well",     // Finnish: Vedenpinnan mittaus kaivosta
            MethodToken::VPK => "Standpipe for groundwater table (bedrock)", // Finnish: Kalliopohjavesiputki
            MethodToken::HV => "Piezometer measurement",  // Finnish: Huokosveden paineen mittaus
            // TODO: Verify english translation
            MethodToken::HU => "Air void pipe",          // Finnish: Huokosilmaputki
            MethodToken::PS => "Pressuremeter test",     // Finnish: Pressometrikoe
            MethodToken::PM => "Settlement measurement", // Finnish: Painumamittaus
            MethodToken::KO => "Test pit",               // Finnish: Koekuoppa
            // TODO: Verify english translation
            MethodToken::KE => "Core sampling (extended)", // Finnish: Kallionäytekairaus laajennettu
            // TODO: Verify english translation
            MethodToken::KR => "Core sampling (video)", // Finnish: Kallionäytekairaus videoitu
            MethodToken::NO => "Disturbed sampling",    // Finnish: Häiritty näytteenotto
            MethodToken::NE => "Undisturbed sampling",  // Finnish: Häiriintymätön näytteenotto
        };
        write!(f, "{}", token_str)
    }
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

impl<T: fmt::Display> ParsedValue<T> {
    fn format_display(&self) -> Option<String> {
        match self {
            ParsedValue::Some(ref value) => Some(format!("{}", value)),
            ParsedValue::Fallback(ref value) => Some(format!("{} (fallback)", value)),
            ParsedValue::None => None,
        }
    }
}

impl fmt::Display for Organisations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref owner) = self.owner_name.format_display() {
            writeln!(f, "Owner: {}", owner)?;
        }

        if let Some(ref investigator) = self.investigator_name.format_display() {
            writeln!(f, "Investigator: {}", investigator)?;
        }

        Ok(())
    }
}

impl fmt::Display for Classification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref name) = self.name.format_display() {
            writeln!(f, "Soil classification: {}", name)?;
        }
        Ok(())
    }
}

impl fmt::Display for Work {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref id) = self.id.format_display() {
            writeln!(f, "Work ID: {}", id)?;
        }

        if let Some(ref name) = self.name.format_display() {
            writeln!(f, "Work Name: {}", name)?;
        }

        Ok(())
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref number) = self.number.format_display() {
            writeln!(f, "Record number: {}", number)?;
        }

        if let Some(ref driller) = self.driller.format_display() {
            writeln!(f, "Driller: {}", driller)?;
        }

        if let Some(ref inspector) = self.inspector.format_display() {
            writeln!(f, "Inspector: {}", inspector)?;
        }

        if let Some(ref processor) = self.processor.format_display() {
            writeln!(f, "Processor: {}", processor)?;
        }

        if let Some(ref digitalized) = self.digitalized.format_display() {
            writeln!(f, "Digitalized: {}", digitalized)?;
        }

        if let Some(ref condition) = self.condition.format_display() {
            writeln!(f, "Condition: {}", condition)?;
        }

        Ok(())
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref token) = self.token.format_display() {
            writeln!(f, "Investigation type: {}", token)?;
        }

        if let Some(ref category) = self.category.format_display() {
            writeln!(f, "Category: {}", category)?;
        }

        if let Some(ref id) = self.id.format_display() {
            writeln!(f, "Method ID: {}", id)?;
        }

        if let Some(ref standard) = self.standard.format_display() {
            writeln!(f, "Standard: {}", standard)?;
        }

        if let Some(ref sampler) = self.sampler.format_display() {
            writeln!(f, "Sampler: {}", sampler)?;
        }

        if let Some(ref specifier) = self.specifier.format_display() {
            writeln!(f, "Specifier: {}", specifier)?;
        }

        Ok(())
    }
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref number) = self.number.format_display() {
            writeln!(f, "Equipment number: {}", number)?;
        }

        if let Some(ref description) = self.description.format_display() {
            writeln!(f, "Description: {}", description)?;
        }

        if let Some(ref cone_size) = self.cone_size.format_display() {
            writeln!(f, "Cone size: {}", cone_size)?;
        }

        Ok(())
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref x) = self.x.format_display() {
            writeln!(f, "X coordinate (N): {}", x)?;
        }

        if let Some(ref y) = self.y.format_display() {
            writeln!(f, "Y coordinate (E): {}", y)?;
        }

        if let Some(ref elevation) = self.start_elevation.format_display() {
            writeln!(f, "Start elevation: {}", elevation)?;
        }

        if let Some(ref date) = self.date.format_display() {
            writeln!(f, "Date: {}", date)?;
        }

        if let Some(ref point_id) = self.point_id.format_display() {
            writeln!(f, "Point ID: {}", point_id)?;
        }

        Ok(())
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref name) = self.name.format_display() {
            writeln!(f, "Line name: {}", name)?;
        }

        if let Some(ref stake) = self.stake.format_display() {
            writeln!(f, "Stake: {}", stake)?;
        }

        if let Some(ref distance) = self.distance.format_display() {
            writeln!(f, "Distance: {}", distance)?;
        }

        Ok(())
    }
}

impl fmt::Display for Termination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref token) = self.token.format_display() {
            writeln!(f, "Termination reason: {}", token)?;
        }
        Ok(())
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref name) = self.name.format_display() {
            writeln!(f, "Program name: {}", name)?;
        }

        if let Some(ref date) = self.date.format_display() {
            writeln!(f, "Program date: {}", date)?;
        }

        if let Some(ref author) = self.author.format_display() {
            writeln!(f, "Author: {}", author)?;
        }

        if !self.guide.is_empty() {
            writeln!(f, "Program guide:")?;
            for (i, guide) in self.guide.iter().enumerate() {
                if let Some(ref guide_str) = guide.format_display() {
                    writeln!(f, "  {}. {}", i + 1, guide_str)?;
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for DepthlessRockSample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref attribute) = self.attribute.format_display() {
            writeln!(f, "Attribute: {}", attribute)?;
        }

        if let Some(ref value) = self.value.format_display() {
            writeln!(f, "Value: {}", value)?;
        }

        Ok(())
    }
}

impl fmt::Display for InitialBorehole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref depth) = self.depth.format_display() {
            writeln!(f, "Initial boring depth: {}", depth)?;
        }

        if let Some(ref method) = self.method.format_display() {
            writeln!(f, "Initial boring method: {}", method)?;
        }

        if let Some(ref soil_type) = self.soil_type.format_display() {
            writeln!(f, "Initial soil type: {}", soil_type)?;
        }

        Ok(())
    }
}

impl fmt::Display for Standpipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref top_elev) = self.top_elevation.format_display() {
            writeln!(f, "Standpipe top elevation: {}", top_elev)?;
        }
        if let Some(ref ground_elev) = self.ground_elevation.format_display() {
            writeln!(f, "Standpipe ground elevation: {}", ground_elev)?;
        }
        if let Some(ref protection_top_elev) = self.protection_top_elevation.format_display() {
            writeln!(f, "Protection top elevation: {}", protection_top_elev)?;
        }
        if let Some(ref cover_elev) = self.cover_elevation.format_display() {
            writeln!(f, "Cover elevation: {}", cover_elev)?;
        }
        if let Some(ref sieve_bottom_elev) = self.sieve_bottom_elevation.format_display() {
            writeln!(f, "Sieve bottom elevation: {}", sieve_bottom_elev)?;
        }
        if let Some(ref upper_structure) = self.upper_structure.format_display() {
            writeln!(f, "Upper structure: {}", upper_structure)?;
        }
        if let Some(ref sieve_length) = self.sieve_length.format_display() {
            writeln!(f, "Sieve length: {}", sieve_length)?;
        }
        if let Some(ref sieve_type) = self.sieve_type.format_display() {
            writeln!(f, "Sieve type: {}", sieve_type)?;
        }
        if let Some(ref diameter) = self.diameter.format_display() {
            writeln!(f, "Diameter: {}", diameter)?;
        }
        if let Some(ref material) = self.material.format_display() {
            writeln!(f, "Material: {}", material)?;
        }
        if let Some(ref measure_point) = self.measure_point.format_display() {
            writeln!(f, "Measurement point: {}", measure_point)?;
        }
        if let Some(ref details) = self.details.format_display() {
            writeln!(f, "Details: {}", details)?;
        }
        if let Some(ref locked) = self.locked.format_display() {
            writeln!(f, "Locked: {}", locked)?;
        }
        if let Some(ref lock_owner) = self.lock_owner.format_display() {
            writeln!(f, "Lock owner: {}", lock_owner)?;
        }
        if let Some(ref installer) = self.installer.format_display() {
            writeln!(f, "Installer: {}", installer)?;
        }

        Ok(())
    }
}

impl fmt::Display for Spatial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref coord_sys) = self.coordinate_system.format_display() {
            writeln!(f, "Coordinate system: {}", coord_sys)?;
        }
        if let Some(ref elev_sys) = self.elevation_system.format_display() {
            writeln!(f, "Elevation system: {}", elev_sys)?;
        }

        Ok(())
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
