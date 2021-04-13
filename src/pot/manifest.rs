use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiPotMetadata {
    pub path: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiPotManifest {
    pub bonsai_pots: HashMap<String, BonsaiPotMetadata>,
}

impl BonsaiPotManifest {
    pub fn from_path(path: &Path) -> Result<BonsaiPotManifest, Box<dyn Error>> {
        let contents = std::fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&contents)?)
    }
}
