use crate::{ParsedValue, TryParse};

use std::fmt;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Method {
    pub token: ParsedValue<MethodToken>,
    pub category: ParsedValue<i32>,
    pub id: ParsedValue<String>,
    pub standard: ParsedValue<String>,
    pub sampler: ParsedValue<Sampler>,
    pub specifier: ParsedValue<String>,
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

impl TryParse for MethodToken {
    fn try_parse(input: &str) -> Result<Self, String> {
        let method = match input.to_uppercase().as_str() {
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
        };

        if method == MethodToken::None {
            Err(input.to_string())
        } else {
            Ok(method)
        }
    }
}

impl TryParse for Sampler {
    fn try_parse(input: &str) -> Result<Self, String> {
        match input.trim().to_uppercase().as_str() {
            "K" => Ok(Sampler::K),
            "L" => Ok(Sampler::L),
            "PMK" => Ok(Sampler::PMK),
            "R" => Ok(Sampler::R),
            "ST50" | "ST-50" => Ok(Sampler::ST50),
            "ST60" | "ST-60" => Ok(Sampler::ST60),
            _ => Err(input.to_string()),
        }
    }
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
