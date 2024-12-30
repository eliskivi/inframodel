mod infra_file;
mod investigation;
mod observation;
mod parsed_value;

pub use infra_file::{CoordinateSystem, ElevationSystem, File, Format, InfraFile, Spatial};
pub use investigation::{
    Classification, ClassificationName, Coordinates, DepthlessRockSample, Equipment, InitialBoreToken, InitialBorehole,
    Investigation, Line, Method, MethodToken, Organisations, Program, Record, Sampler, Standpipe, Termination,
    TerminationToken, Work,
};
pub use observation::{LabResult, Observation, ObservationValues};
pub use parsed_value::ParsedValue;

#[cfg(test)]
mod tests {
    #[test]
    fn parser_test() {
        let file_path = "C:/PT/3.txt";
        let tek = crate::InfraFile::parse_file(file_path);
        if let Ok(tek) = tek {
            //println!("{}", tek);
            let x = tek.count_investigations();
            println!("{:#?}", x);
        };
    }
}
