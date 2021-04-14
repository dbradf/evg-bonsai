use crate::landscape::pot::GithubVersionSpec;
use directories_next::BaseDirs;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{AutotagOption, Cred, FetchOptions, Oid, RemoteCallbacks, Repository};
use simple_error::bail;
use std::env;
use std::error::Error;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

const APPLICATION_NAME: &str = "evg-bonsai";

fn get_cache_dir() -> Result<PathBuf, Box<dyn Error>> {
    if let Some(base_dirs) = BaseDirs::new() {
        let mut cache_dir = base_dirs.cache_dir().to_owned();
        cache_dir.push(APPLICATION_NAME);
        if !cache_dir.exists() {
            create_dir_all(&cache_dir)?;
        }

        Ok(cache_dir)
    } else {
        bail!("No cache directory found.")
    }
}

fn create_ssh_fetch_options() -> FetchOptions<'static> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);
    fo.download_tags(AutotagOption::All);

    fo
}

fn clone_via_ssh(url: &str, path: &Path) -> Result<Repository, Box<dyn Error>> {
    let fo = create_ssh_fetch_options();
    Ok(RepoBuilder::new().fetch_options(fo).clone(url, path)?)
}

fn format_spec(branch_name: &str) -> String {
    format!("refs/heads/{}", branch_name)
}

fn format_rm_spec(remote: &str, branch_name: &str) -> String {
    format!("refs/remotes/{}/{}", remote, branch_name)
}

fn fast_forward(repo: &Repository) -> Result<(), Box<dyn Error>> {
    let branch = "master";
    let mut fo = create_ssh_fetch_options();

    repo.find_remote("origin")?
        .fetch(&[] as &[&str], Some(&mut fo), None)?;
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    let analysis = repo.merge_analysis(&[&fetch_commit])?;
    if analysis.0.is_up_to_date() {
        Ok(())
    } else if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", branch);
        let mut reference = repo.find_reference(&refname)?;
        reference.set_target(fetch_commit.id(), "Fast-Forward")?;
        repo.set_head(&refname)?;
        Ok(repo.checkout_head(Some(CheckoutBuilder::default().force()))?)
    } else {
        bail!("Fast-forward only")
    }
}

fn checkout_revision(repo: &Repository, revision: &str) -> Result<(), Box<dyn Error>> {
    let revspec = format_spec(revision);

    let oid = Oid::from_str(revision)?;
    let commit = repo.find_commit(oid)?;
    let obj_result = repo.revparse_single(&revspec);
    if obj_result.is_err() {
        let _branch = repo.branch(revision, &commit, false)?;
    }
    let obj = repo.revparse_single(&revspec)?;

    repo.checkout_tree(&obj, None)?;
    repo.set_head_detached(oid)?;

    Ok(())
}

fn checkout_branch(repo: &Repository, branch_name: &str) -> Result<(), Box<dyn Error>> {
    let revspec = format_rm_spec("origin", branch_name);
    let obj = repo.revparse_single(&revspec)?;

    repo.checkout_tree(&obj, None)?;
    repo.set_head(&revspec)?;

    Ok(())
}

pub fn get_repository(
    owner: &str,
    repo: &str,
    maybe_version: &Option<GithubVersionSpec>,
) -> Result<PathBuf, Box<dyn Error>> {
    let base_dir = get_cache_dir()?;
    let mut repo_dir = base_dir;
    repo_dir.push(repo);
    let repo = if repo_dir.exists() {
        Repository::open(&repo_dir)?
    } else {
        let url = format!("git@github.com:{}/{}.git", owner, repo);
        clone_via_ssh(&url, &repo_dir.as_path())?
    };

    fast_forward(&repo)?;

    if let Some(version) = maybe_version {
        println!("Version {:?} specified", version);
        match version {
            GithubVersionSpec::Branch(branch) => checkout_branch(&repo, branch)?,
            GithubVersionSpec::Revision(rev) => checkout_revision(&repo, rev)?,
        }
    }

    Ok(repo_dir.as_path().to_owned())
}
