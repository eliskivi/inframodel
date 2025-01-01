mod infra_file;
mod investigation;
mod investigation_aggregator;
mod investigation_collection;
mod observation;
mod parse_result;

pub use infra_file::{
    file_info::FileInfo,
    format::Format,
    spatial::{CoordinateSystem, ElevationSystem, Spatial},
    InfraFile,
};

pub use investigation::{
    classification::{Classification, ClassificationName},
    coordinates::Coordinates,
    depthless_rock_sample::DepthlessRockSample,
    equipment::Equipment,
    initial_borehole::{InitialBoreToken, InitialBorehole},
    line::Line,
    method::{Method, MethodToken, Sampler},
    organisations::Organisations,
    program::Program,
    record::{Digitized, Record},
    standpipe::Standpipe,
    termination::{Termination, TerminationToken},
    work::Work,
    Investigation,
};

pub use investigation_collection::InvestigationCollection;

pub use investigation_aggregator::{HasInvestigations, InvestigationAggregator};

pub use observation::{lab_results::LabResult, observation_values::ObservationValues, Observation};

pub use parse_result::{ParseResult, TryParse};

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse() {
        let folder_path_str = "C://PT/";

        match InvestigationCollection::parse_folder(folder_path_str) {
            Ok(tek) => {
                println!("{:#?}", tek.count_investigations());
            }
            Err(e) => {
                eprintln!("Failed to parse file: {}", e);
            }
        }

        let file_path_str = "C://PT/2.txt";

        match InfraFile::parse_file(file_path_str) {
            Ok(tek) => {
                println!("{:#?}", tek.count_investigations());
            }
            Err(e) => {
                eprintln!("Failed to parse file: {}", e);
            }
        }
    }
}
