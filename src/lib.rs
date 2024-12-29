mod infra_file;
mod investigation;
mod observation;
mod parsed_value;

pub use infra_file::{File, Format, InfraFile, Spatial};
pub use investigation::{
    Classification, Coordinates, DepthlessRockSample, Equipment, InitialBorehole, Investigation, Line, Method, Organisations, Program,
    Record, Standpipe, Termination, Work,
};
pub use observation::Observation;
pub use parsed_value::ParsedValue;

#[cfg(test)]
mod tests {
    #[test]
    fn parser_test() {
        let file_path = "C:/PT/1.txt";
        let tek = crate::InfraFile::parse_file(file_path);
        println!("{:#?}", tek);
    }
}
