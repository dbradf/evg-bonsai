use crate::landscape::landscape::BonsaiLandscape;
use chrono::Utc;
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_merge_keys::merge_keys;

pub mod landscape;
pub mod pot;

const SUPPORT_FILE_DIRECTORY: &str = "bonsai_files";

fn get_merged_yaml(yaml_contents: &str) -> Result<String, Box<dyn Error>> {
    let raw = YamlLoader::load_from_str(&yaml_contents)?.remove(0);
    let merged = merge_keys(raw)?;

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&merged)?;
    }

    Ok(out_str)
}

pub fn build_landscape(
    source_file: &Path,
    target_dir: &Path,
    target_filename: &str,
) -> Result<(), Box<dyn Error>> {
    if !target_dir.exists() {
        create_dir_all(target_dir)?;
    }
    let mut support_files_destination = target_dir.to_path_buf();
    support_files_destination.push(SUPPORT_FILE_DIRECTORY);

    let contents = read_to_string(source_file)?;
    let merged_contents = get_merged_yaml(&contents)?;
    let bonsai_project: BonsaiLandscape = serde_yaml::from_str(&merged_contents)?;
    let evergreen_project = bonsai_project.create_evg_project()?;
    bonsai_project.copy_remote_support_files(support_files_destination.as_path())?;

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
