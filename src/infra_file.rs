pub(crate) mod file;
pub(crate) mod format;
pub(crate) mod parse;
pub(crate) mod spatial;

use crate::{File, Format, Investigation, MethodToken, ParsedValue, Spatial};

use std::collections::HashMap;
use std::fmt;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct InfraFile {
    pub file: File,
    pub format: Format,
    pub spatial: Spatial,
    pub investigations: Vec<Investigation>,
}

impl InfraFile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count_investigations(&self) -> HashMap<MethodToken, usize> {
        let mut counts = HashMap::new();
        for investigation in &self.investigations {
            if let ParsedValue::Some(token) = investigation.method.token {
                *counts.entry(token).or_insert(0) += 1;
            }
        }
        counts
    }

    pub fn print_investigations_count(&self) {
        let counts = self.count_investigations();
        let mut sorted_counts: Vec<(&MethodToken, &usize)> = counts.iter().collect();
        sorted_counts.sort_by(|a, b| b.1.cmp(a.1));

        if counts.is_empty() {
            println!("No investigations found.");
            return;
        }

        println!("{:-<10}-+-{:-<50}", "", "",);
        println!("{:^10} | {:<50}", "Count", "Investigation",);
        println!("{:-<10}-+-{:-<50}", "", "",);

        for (token, count) in sorted_counts {
            println!("{:^10} | {:<50}", count, token);
        }

        println!("{:-<10}-+-{:-<50}", "", "",);
    }
}

impl fmt::Display for InfraFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "----------------------------------------------------------------")?;
        writeln!(f, "  INFRA FILE")?;
        writeln!(f, "----------------------------------------------------------------")?;
        if let Some(ref path) = self.file.path {
            writeln!(f, "File path: {}", path)?;
        }
        if let Some(ref encoding) = self.file.encoding {
            writeln!(f, "File encoding: {}", encoding)?;
        }
        // if let Some(ref text) = self.file.text {
        //     writeln!(f, "Decoded file text: {}", text)?;
        // }

        if let ParsedValue::Some(ref version) = self.format.version {
            writeln!(f, "Format version: {}", version)?;
        } else if let ParsedValue::Fallback(ref version) = self.format.version {
            writeln!(f, "Format version: {} (fallback)", version)?;
        }
        if let ParsedValue::Some(ref used_software) = self.format.used_software {
            writeln!(f, "Software used: {}", used_software)?;
        } else if let ParsedValue::Fallback(ref used_software) = self.format.used_software {
            writeln!(f, "Software used: {} (fallback)", used_software)?;
        }
        if let ParsedValue::Some(ref software_version) = self.format.software_version {
            writeln!(f, "Software version: {}", software_version)?;
        } else if let ParsedValue::Fallback(ref software_version) = self.format.software_version {
            writeln!(f, "Software version: {} (fallback)", software_version)?;
        }

        // Handle Spatial Fields (ParsedValue<Enum>)
        if let ParsedValue::Some(ref coord_sys) = self.spatial.coordinate_system {
            writeln!(f, "Coordinate system: {}", coord_sys)?;
        } else if let ParsedValue::Fallback(ref coord_sys) = self.spatial.coordinate_system {
            writeln!(f, "Coordinate system: {} (fallback)", coord_sys)?;
        }

        if let ParsedValue::Some(ref elev_sys) = self.spatial.elevation_system {
            writeln!(f, "Elevation system: {}", elev_sys)?;
        } else if let ParsedValue::Fallback(ref elev_sys) = self.spatial.elevation_system {
            writeln!(f, "Elevation system: {} (fallback)", elev_sys)?;
        }

        if !self.investigations.is_empty() {
            for (i, investigation) in self.investigations.iter().enumerate() {
                writeln!(f, "----------------------------------------------------------------")?;
                writeln!(f, "  INVESTIGATION {}:", i + 1)?;
                writeln!(f, "----------------------------------------------------------------")?;
                writeln!(f, "{}", investigation)?;
            }
        }

        Ok(())
    }
}
