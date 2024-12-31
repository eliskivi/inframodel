pub(crate) mod file_info;
pub(crate) mod format;
pub(crate) mod parse;
pub(crate) mod spatial;

use crate::{FileInfo, Format, Investigation, MethodToken, ParseResult, Spatial};

use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct InfraFile {
    pub file: FileInfo,
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
            if let ParseResult::Parsed(token) = investigation.method.token {
                *counts.entry(token).or_insert(0) += 1;
            }
        }
        counts
    }
}
