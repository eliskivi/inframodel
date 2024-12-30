use crate::ParsedValue;

use chrono::NaiveDate;
use std::fmt;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Coordinates {
    // TODO: Implement "-999999" as unknown for x and y coordinates
    pub x: ParsedValue<f32>,
    pub y: ParsedValue<f32>,
    pub start_elevation: ParsedValue<f32>,
    pub date: ParsedValue<NaiveDate>,
    pub point_id: ParsedValue<String>,
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
