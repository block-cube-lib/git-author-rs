pub mod author;
mod config_file_location;
mod params;

use crate::error::Error;
use author::*;
pub use config_file_location::ConfigFileLocation;

fn get_git_user_param(location: ConfigFileLocation, user: params::User) -> Result<String, Error> {
    let output = std::process::Command::new("git")
        .arg("config")
        .arg(location.to_arg())
        .arg(user.to_arg())
        .output()?;
    let output = String::from_utf8(output.stdout)?;
    let output = output.trim_end_matches('\n');
    Ok(output.into())
}

pub fn get_author(location: ConfigFileLocation) -> Result<Author, Error> {
    let author = AuthorBuilder::new()
        .name(&get_git_user_param(location, params::User::Name)?)
        .email(&get_git_user_param(location, params::User::Email)?)
        .build()?;
    Ok(author)
}

pub fn get_local_author() -> Result<Author, Error> {
    Ok(get_author(ConfigFileLocation::Local)?)
}

fn set_git_user_param(location: ConfigFileLocation, user: params::User, value: &str) -> Result<(), Error> {
    let _output = std::process::Command::new("git")
        .arg("config")
        .arg(location.to_arg())
        .arg(user.to_arg())
        .arg(value)
        .output()?;
    Ok(())
}

pub fn set_author(location: ConfigFileLocation, author: Author) -> Result<(), Error> {
    set_git_user_param(location, params::User::Name, &author.name().0)?;
    set_git_user_param(location, params::User::Email, &author.email().0)?;
    Ok(())
}

/// ## Replace commiter information in the current branch with author.
pub fn replace(commiter: Author, author: Author) -> Result<(), Error> {
    let _output = std::process::Command::new("git")
        .arg("filter-branch")
        .arg("-f")
        .arg("--env-filter")
        .arg(format!(
                "\"GIT_AUTHOR_email={}; GIT_AUTHOR_EMAIL={}; GIT_COMMITTER_NAME={}; GIT_COMMITTER_EMAIL={};\"",
                author.name().0, author.email().0, commiter.name().0, commiter.email().0))
        .arg("HEAD;")
        .output()?;
    Ok(())
}
