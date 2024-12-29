pub mod infra_file;
pub mod investigation;
pub mod observation;
pub mod parsed_value;

pub use infra_file::{File, Format, InfraFile, Spatial};
pub use investigation::{
    Coordinates, DepthlessRockSample, Equipment, InitialBorehole, Investigation,
    InvestigationMethod, InvestigationProgram, InvestigatorOrganisation, Line, OwnerOrganisation,
    Record, SoilClassification, Standpipe, Termination, Work,
};
pub use observation::Observation;
pub use parsed_value::ParsedValue;

#[cfg(test)]
mod tests {
    #[test]
    fn parser_test() {
        let file_path = "C:/PT/test1.txt";
        let tek = crate::infra_file::InfraFile::parse_file(file_path);
        tek.debug_print();
    }
}
