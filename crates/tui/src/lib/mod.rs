//! ngit: a git+nostr command line utility and library
/// ngit::cli_interactor
pub mod cli_interactor;
/// ngit::client
pub mod client;
/// ngit::git
pub mod git;
/// ngit::git_events
pub mod git_events;
/// ngit::login
pub mod login;
/// ngit::repo_ref
pub mod repo_ref;
/// ngit::repo_state
pub mod repo_state;

use anyhow::{anyhow, Result};
use directories::ProjectDirs;

pub fn get_dirs() -> Result<ProjectDirs> {
	ProjectDirs::from("", "", "ngit").ok_or(anyhow!(
        "should find operating system home directories with rust-directories crate"
    ))
}
