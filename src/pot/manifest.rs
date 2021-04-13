use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiPotMetadata {
    pub path: String,
    pub description: String,
    pub include_files: Option<Vec<String>>,
}

impl BonsaiPotMetadata {
    pub fn copy_files_to_destination(
        &self,
        base_dir: &Path,
        destination_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(files) = &self.include_files {
            for file in files {
                let mut source_file = base_dir.to_path_buf();
                source_file.push(file);

                let mut destination_file = destination_dir.to_path_buf();
                destination_file.push(file);

                if let Some(dest_dir) = destination_file.parent() {
                    create_dir_all(dest_dir)?;
                }

                fs::copy(source_file, destination_file)?;
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiPotManifest {
    pub bonsai_pots: Vec<BonsaiPotMetadata>,
}

impl BonsaiPotManifest {
    pub fn from_path(path: &Path) -> Result<BonsaiPotManifest, Box<dyn Error>> {
        let contents = std::fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&contents)?)
    }

    pub fn copy_support_files(
        &self,
        base_dir: &Path,
        destination_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        self.bonsai_pots
            .iter()
            .try_for_each(|p| p.copy_files_to_destination(base_dir, destination_dir))?;

        Ok(())
    }
}
