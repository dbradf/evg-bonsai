use crate::landscape::landscape::BonsaiLandscape;
use chrono::Utc;
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::{BufWriter, Write};
use std::path::Path;

pub mod landscape;
pub mod pot;

pub fn build_landscape(
    source_file: &Path,
    target_dir: &Path,
    target_filename: &str,
) -> Result<(), Box<dyn Error>> {
    if !target_dir.exists() {
        create_dir_all(target_dir)?;
    }
    let contents = read_to_string(source_file)?;
    let bonsai_project: BonsaiLandscape = serde_yaml::from_str(&contents)?;
    let evergreen_project = bonsai_project.create_evg_project()?;

    let project_config = serde_yaml::to_string(&evergreen_project)?;
    let now = Utc::now();

    let mut target_file = target_dir.to_path_buf();
    target_file.push(target_filename);
    let f = File::create(target_file)?;
    let mut writer = BufWriter::new(f);

    writer.write_all("# Generated from bonsai\n".as_bytes())?;
    writer.write_all(format!("# Generated at: {}\n", now).as_bytes())?;
    writer.write_all(project_config.as_bytes())?;

    Ok(())
}
