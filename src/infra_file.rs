pub(crate) mod file_info;
pub(crate) mod format;
pub(crate) mod parse;
pub(crate) mod spatial;

use crate::{FileInfo, Format, HasInvestigations, Investigation, InvestigationAggregator, Spatial};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct InfraFile {
    pub file_info: FileInfo,
    pub format: Format,
    pub spatial: Spatial,
    pub investigations: Vec<Investigation>,
}

impl InfraFile {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HasInvestigations for InfraFile {
    fn investigations(&self) -> &Vec<Investigation> {
        &self.investigations
    }
}

impl InvestigationAggregator for InfraFile {}
