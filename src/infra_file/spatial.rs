use crate::{ParsedValue, TryParse};

use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Spatial {
    pub coordinate_system: ParsedValue<CoordinateSystem>,
    pub elevation_system: ParsedValue<ElevationSystem>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum CoordinateSystem {
    #[default]
    Unknown,
    WGS84,
    HKI,
    VANTAA,
    ESPOO,
    KKJ0,
    KKJ1,
    KKJ2,
    KKJ3,
    KKJ4,
    KKJ5,
    YKJ,
    GK19,
    GK20,
    GK21,
    GK22,
    GK23,
    GK24,
    GK25,
    GK26,
    GK27,
    GK28,
    GK29,
    GK30,
    GK31,
    TM34,
    TM35,
    TM36,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum ElevationSystem {
    #[default]
    Unknown,
    N2000,
    N60,
    N43,
    NN,
    LN,
}

impl TryParse for CoordinateSystem {
    fn try_parse(input: &str) -> Result<Self, String> {
        match input.trim().to_uppercase().as_str() {
            "WGS84" | "WGS" => Ok(CoordinateSystem::WGS84),
            "HKI" => Ok(CoordinateSystem::HKI),
            "VANTAA" => Ok(CoordinateSystem::VANTAA),
            "ESPOO" => Ok(CoordinateSystem::ESPOO),
            "KKJ0" => Ok(CoordinateSystem::KKJ0),
            "KKJ1" => Ok(CoordinateSystem::KKJ1),
            "KKJ2" => Ok(CoordinateSystem::KKJ2),
            "KKJ3" => Ok(CoordinateSystem::KKJ3),
            "KKJ4" => Ok(CoordinateSystem::KKJ4),
            "KKJ5" => Ok(CoordinateSystem::KKJ5),
            "YKJ" => Ok(CoordinateSystem::YKJ),
            "GK19" | "ETRSGK19" | "ETRS-GK19" => Ok(CoordinateSystem::GK19),
            "GK20" | "ETRSGK20" | "ETRS-GK20" => Ok(CoordinateSystem::GK20),
            "GK21" | "ETRSGK21" | "ETRS-GK21" => Ok(CoordinateSystem::GK21),
            "GK22" | "ETRSGK22" | "ETRS-GK22" => Ok(CoordinateSystem::GK22),
            "GK23" | "ETRSGK23" | "ETRS-GK23" => Ok(CoordinateSystem::GK23),
            "GK24" | "ETRSGK24" | "ETRS-GK24" => Ok(CoordinateSystem::GK24),
            "GK25" | "ETRSGK25" | "ETRS-GK25" => Ok(CoordinateSystem::GK25),
            "GK26" | "ETRSGK26" | "ETRS-GK26" => Ok(CoordinateSystem::GK26),
            "GK27" | "ETRSGK27" | "ETRS-GK27" => Ok(CoordinateSystem::GK27),
            "GK28" | "ETRSGK28" | "ETRS-GK28" => Ok(CoordinateSystem::GK28),
            "GK29" | "ETRSGK29" | "ETRS-GK29" => Ok(CoordinateSystem::GK29),
            "GK30" | "ETRSGK30" | "ETRS-GK30" => Ok(CoordinateSystem::GK30),
            "GK31" | "ETRSGK31" | "ETRS-GK31" => Ok(CoordinateSystem::GK31),
            "TM35" | "ETRSTM35" | "ETRS-TM35" | "TM35FIN" | "ETRSTM35FIN" | "ETRS-TM35FIN" => {
                Ok(CoordinateSystem::TM35)
            },
            _ => Err(input.to_string()),
        }
    }
}

impl TryParse for ElevationSystem {
    fn try_parse(input: &str) -> Result<Self, String> {
        match input.trim().to_uppercase().as_str() {
            "N2000" => Ok(ElevationSystem::N2000),
            "N60" => Ok(ElevationSystem::N60),
            "N43" => Ok(ElevationSystem::N43),
            "NN" => Ok(ElevationSystem::NN),
            "LN" => Ok(ElevationSystem::LN),
            _ => Err(input.to_string()),
        }
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

impl fmt::Display for CoordinateSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cs_str = match self {
            CoordinateSystem::Unknown => "Unknown",
            CoordinateSystem::WGS84 => "WGS84",
            CoordinateSystem::HKI => "HKI",
            CoordinateSystem::VANTAA => "VANTAA",
            CoordinateSystem::ESPOO => "ESPOO",
            CoordinateSystem::KKJ0 => "KKJ0",
            CoordinateSystem::KKJ1 => "KKJ1",
            CoordinateSystem::KKJ2 => "KKJ2",
            CoordinateSystem::KKJ3 => "KKJ3",
            CoordinateSystem::KKJ4 => "KKJ4",
            CoordinateSystem::KKJ5 => "KKJ5",
            CoordinateSystem::YKJ => "YKJ",
            CoordinateSystem::GK19 => "ETRS-GK19",
            CoordinateSystem::GK20 => "ETRS-GK20",
            CoordinateSystem::GK21 => "ETRS-GK21",
            CoordinateSystem::GK22 => "ETRS-GK22",
            CoordinateSystem::GK23 => "ETRS-GK23",
            CoordinateSystem::GK24 => "ETRS-GK24",
            CoordinateSystem::GK25 => "ETRS-GK25",
            CoordinateSystem::GK26 => "ETRS-GK26",
            CoordinateSystem::GK27 => "ETRS-GK27",
            CoordinateSystem::GK28 => "ETRS-GK28",
            CoordinateSystem::GK29 => "ETRS-GK29",
            CoordinateSystem::GK30 => "ETRS-GK30",
            CoordinateSystem::GK31 => "ETRS-GK31",
            CoordinateSystem::TM34 => "ETRS-TM34",
            CoordinateSystem::TM35 => "ETRS-TM35",
            CoordinateSystem::TM36 => "ETRS-TM36",
        };
        write!(f, "{}", cs_str)
    }
}

impl fmt::Display for ElevationSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let es_str = match self {
            ElevationSystem::Unknown => "Unknown",
            ElevationSystem::N2000 => "N2000",
            ElevationSystem::N60 => "N60",
            ElevationSystem::N43 => "N43",
            ElevationSystem::NN => "NN",
            ElevationSystem::LN => "LN",
        };
        write!(f, "{}", es_str)
    }
}
