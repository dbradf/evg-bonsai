use crate::landscape::pot::{BonsaiPot, GithubSourceDesc};
use crate::pot::github_service::get_repository;
use crate::pot::manifest::BonsaiPotManifest;
use simple_error::bail;
use std::error::Error;

const MANIFEST_FILE: &str = "bonsai.manifest.yml";

pub fn get_remote_pot(github_source: &GithubSourceDesc) -> Result<BonsaiPot, Box<dyn Error>> {
    let repo_path = get_repository(
        &github_source.owner,
        &github_source.repo,
        github_source.revision.as_deref(),
    )?;
    let mut manifest_path = repo_path.clone();
    manifest_path.push(MANIFEST_FILE);

    if !manifest_path.exists() {
        bail!(format!(
            "Could not find manifest in repo {}/{}",
            github_source.owner, github_source.repo
        ));
    }

    let manifest = BonsaiPotManifest::from_path(manifest_path.as_path())?;
    let pot_name = &github_source.pot_name;
    if let Some(metadata) = manifest.bonsai_pots.get(pot_name) {
        let mut pot_path = repo_path.clone();
        pot_path.push(&metadata.path);
        BonsaiPot::from_path(pot_path.as_path())
    } else {
        bail!(format!("Could not find pot({}) in manifest", pot_name))
    }
}
