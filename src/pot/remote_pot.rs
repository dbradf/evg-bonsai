use crate::landscape::pot::{BonsaiPot, GithubSourceDesc};
use crate::pot::github_service::get_repository;
use crate::pot::manifest::BonsaiPotManifest;
use simple_error::bail;
use std::error::Error;
use std::path::Path;

const MANIFEST_FILE: &str = "bonsai.manifest.yml";

fn find_manifest(repo_path: &Path) -> Result<BonsaiPotManifest, Box<dyn Error>> {
    let mut manifest_path = repo_path.to_path_buf();
    manifest_path.push(MANIFEST_FILE);

    if !manifest_path.exists() {
        bail!("Could not find manifest in repo")
    }

    BonsaiPotManifest::from_path(&manifest_path)
}

pub fn get_remote_pots(github_source: &GithubSourceDesc) -> Result<Vec<BonsaiPot>, Box<dyn Error>> {
    let repo_path = get_repository(
        &github_source.owner,
        &github_source.repo,
        &github_source.version,
    )?;
    let manifest = find_manifest(repo_path.as_path())?;
    manifest
        .bonsai_pots
        .iter()
        .map(|pot_md| {
            let mut pot_path = repo_path.clone();
            pot_path.push(&pot_md.path);
            BonsaiPot::from_path(pot_path.as_path())
        })
        .collect()
}

pub fn copy_support_files(
    github_source: &GithubSourceDesc,
    destination_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    let repo_path = get_repository(
        &github_source.owner,
        &github_source.repo,
        &github_source.version,
    )?;
    let manifest = find_manifest(repo_path.as_path())?;
    manifest.copy_support_files(repo_path.as_path(), destination_dir)?;

    Ok(())
}
