use crate::ParsedValue;

use std::fmt;

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
