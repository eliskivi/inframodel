use crate::{HasInvestigations, InfraFile, Investigation, InvestigationAggregator};

use rayon::prelude::*;
use walkdir::WalkDir;

use std::fs;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct InvestigationCollection {
    pub investigations: Vec<Investigation>,
}

impl InvestigationCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse_folder(folder_path: &str) -> Result<InvestigationCollection, String> {
        let metadata =
            fs::metadata(folder_path).map_err(|e| format!("Failed to access folder: {}", e))?;

        if !metadata.is_dir() {
            return Err("Provided path is not a directory".to_string());
        }

        let file_paths: Vec<String> = WalkDir::new(folder_path)
            .into_iter()
            .filter_map(|entry| match entry {
                Ok(e) => {
                    if e.file_type().is_file() {
                        Some(e.path().to_string_lossy().into_owned())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            })
            .collect();

        if file_paths.is_empty() {
            return Err("No files found in the specified directory".to_string());
        }

        let investigations: Vec<Investigation> = file_paths
            .par_iter()
            .filter_map(|file_path| match Self::parse_file(file_path) {
                Ok(collection) => Some(collection.investigations),
                Err(_) => None,
            })
            .flatten()
            .collect();

        if investigations.is_empty() {
            return Err("No investigations could be parsed from the folder".to_string());
        }

        Ok(InvestigationCollection { investigations })
    }

    pub fn parse_file(file_path: &str) -> Result<InvestigationCollection, String> {
        let mut infra =
            InfraFile::parse_file(file_path).map_err(|_| "Failed to parse file".to_string())?;

        let file_info = infra.file_info.clone();
        let spatial = infra.spatial.clone();

        for inv in &mut infra.investigations {
            inv.file_info = file_info.clone();
            inv.spatial = spatial.clone();
        }

        Ok(InvestigationCollection {
            investigations: infra.investigations,
        })
    }
}

impl HasInvestigations for InvestigationCollection {
    fn investigations(&self) -> &Vec<Investigation> {
        &self.investigations
    }
}

impl InvestigationAggregator for InvestigationCollection {}
