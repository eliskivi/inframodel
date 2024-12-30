mod infra_file;
mod investigation;
mod observation;
mod parse_result;

pub use infra_file::file_info::FileInfo;
pub use infra_file::format::Format;
pub use infra_file::spatial::{CoordinateSystem, ElevationSystem, Spatial};
pub use infra_file::InfraFile;

pub use investigation::classification::{Classification, ClassificationName};
pub use investigation::coordinates::Coordinates;
pub use investigation::depthless_rock_sample::DepthlessRockSample;
pub use investigation::equipment::Equipment;
pub use investigation::initial_borehole::{InitialBoreToken, InitialBorehole};
pub use investigation::line::Line;
pub use investigation::method::{Method, MethodToken, Sampler};
pub use investigation::organisations::Organisations;
pub use investigation::program::Program;
pub use investigation::record::{Digitized, Record};
pub use investigation::standpipe::Standpipe;
pub use investigation::termination::{Termination, TerminationToken};
pub use investigation::work::Work;
pub use investigation::Investigation;

pub use observation::lab_results::LabResult;
pub use observation::observation_values::ObservationValues;
pub use observation::Observation;

pub use parse_result::{ParseResult, TryParse};

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let file_path = "C:/PT/1.txt";
        let tek = crate::InfraFile::parse_file(file_path);
        if let Ok(tek) = tek {
            for inv in tek.investigations {
                println!("--------------------------------");
                println!("{:#?}", inv.soil_layers);
            }
        };
    }
}
