//! A module that defines functions and structures for calling git commands.

mod author;
mod config_file_location;
mod user_parameter;

use crate::error::Error;
pub use author::Author;
pub use config_file_location::ConfigFileLocation;
use user_parameter::UserParameter;

fn output_to_result(output: std::process::Output) -> Result<String, Error> {
    if output.status.success() {
        let stdout = output.stdout;
        String::from_utf8(stdout).map_err(|e| e.into())
    } else {
        let description = String::from_utf8(output.stderr)?;
        Err(Error::CommandError(description))
    }
}

/// get user.name or user.email
fn get_git_user_param(
    location: Option<ConfigFileLocation>,
    user_parameter: UserParameter,
) -> Result<Option<String>, Error> {
    let output = if let Some(location) = location {
        std::process::Command::new("git")
            .arg("config")
            .arg(location.to_arg())
            .arg(user_parameter.to_arg())
            .output()?
    } else {
        std::process::Command::new("git")
            .arg("config")
            .arg(user_parameter.to_arg())
            .output()?
    };

    Ok(match output_to_result(output) {
        Ok(s) => {
            let s = s.trim().to_string();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        }
        Err(_) => None,
    })
}

/// get author(user.name and email)
pub fn get_author(location: Option<ConfigFileLocation>) -> Result<Author, Error> {
    let name = get_git_user_param(location, UserParameter::Name)?;
    let email = get_git_user_param(location, UserParameter::Email)?;
    let author = Author::new(name, email)?;
    Ok(author)
}

/// get user.name or user.email
fn set_git_user_param(
    location: ConfigFileLocation,
    user_parameter: UserParameter,
    value: &str,
) -> Result<String, Error> {
    let output = std::process::Command::new("git")
        .arg("config")
        .arg(location.to_arg())
        .arg(user_parameter.to_arg())
        .arg(value)
        .output()?;
    output_to_result(output)
}

/// Set git author
pub fn set_author(location: ConfigFileLocation, author: Author) -> Result<(), Error> {
    match (author.name(), author.email()) {
        (Some(name), Some(email)) => {
            set_git_user_param(location, UserParameter::Name, name)?;
            set_git_user_param(location, UserParameter::Email, email)?;
            Ok(())
        }
        (Some(_), None) => Err(Error::InvalidArguments("email is empty".to_string())),
        (None, Some(_)) => Err(Error::InvalidArguments("name is empty".to_string())),
        (None, None) => Err(Error::InvalidArguments(
            "name and email are empty".to_string(),
        )),
    }
}

/// Unset the author parameters.
pub fn unset_author(location: ConfigFileLocation) -> Result<(), Error> {
    let unset_user_param = |user_param: &str| {
        std::process::Command::new("git")
            .arg("config")
            .arg(location.to_arg())
            .arg("--unset")
            .arg(user_param)
            .output()
    };

    // Do not check output.status.
    unset_user_param("user.name")?;
    unset_user_param("user.email")?;
    Ok(())
}

/// Replaces committer and author from past commits in the current branch.
pub fn replace(old_author: Author, new_author: Author) -> Result<String, Error> {
    let values = [
        old_author.name(),
        old_author.email(),
        new_author.name(),
        new_author.email(),
    ];
    if values.iter().any(|&e| e.is_none()) {
        return Err(Error::InvalidArguments(
            "Argument has empty value.".to_string(),
        ));
    }

    let output = std::process::Command::new("git")
        .arg("filter-branch")
        .arg("-f")
        .arg("--env-filter")
        .arg(format!(r#"
        if [ "$GIT_AUTHOR_NAME" = "{old_name}" -a "$GIT_AUTHOR_EMAIL" = "{old_email}" -o "$GIT_COMMITTER_NAME" = "{old_name}" -a "$GIT_COMMITTER_EMAIL" = "{old_email}" ]; then
            GIT_AUTHOR_NAME={new_name} GIT_AUTHOR_EMAIL={new_email}; GIT_COMMITTER_NAME={new_name}; GIT_COMMITTER_EMAIL={new_email};
        fi
        HEAD;"#,
                new_name=&new_author.name().as_ref().unwrap(),
                new_email=&new_author.email().as_ref().unwrap(),
                old_name=&old_author.name().as_ref().unwrap(),
                old_email=&old_author.email().as_ref().unwrap()))
        .output()?;
    output_to_result(output)
}
