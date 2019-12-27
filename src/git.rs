//! A module that defines functions and structures for calling git commands.

mod author;
mod config_file_location;
mod replace_filter;
mod replace_target;
mod user_parameter;

use crate::error::*;
pub use author::Author;
pub use config_file_location::ConfigFileLocation;
pub use replace_filter::ReplaceFilter;
pub use replace_target::ReplaceTarget;
use user_parameter::UserParameter;

fn output_to_result(output: std::process::Output) -> Result<String, OutputError> {
    if output.status.success() {
        let stdout = output.stdout;
        String::from_utf8(stdout).map_err(|e| e.into())
    } else {
        let description = String::from_utf8(output.stderr)?;
        Err(CommandExecuteError(description).into())
    }
}

/// get user.name or user.email
fn get_git_user_param(
    location: Option<ConfigFileLocation>,
    user_parameter: UserParameter,
) -> Result<Option<String>, OutputError> {
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
    if output.status.success() {
        let s = String::from_utf8(output.stdout)?;
        Ok(Some(s.trim_end_matches('\n').to_string()))
    } else if output.stderr.is_empty() {
        Ok(None)
    } else {
        Err(CommandExecuteError(String::from_utf8(output.stderr)?).into())
    }
}

/// get author(user.name and email)
pub fn get_author(location: Option<ConfigFileLocation>) -> Result<Author, GetError> {
    let name = get_git_user_param(location, UserParameter::Name)?;
    let email = get_git_user_param(location, UserParameter::Email)?;
    let author = Author::new(name, email)?;
    Ok(author)
}

/// set user.name or user.email
fn set_git_user_param(
    location: ConfigFileLocation,
    user_parameter: UserParameter,
    value: &str,
) -> Result<String, OutputError> {
    let output = std::process::Command::new("git")
        .arg("config")
        .arg(location.to_arg())
        .arg(user_parameter.to_arg())
        .arg(value)
        .output()?;

    if output.status.success() {
        let stdout = output.stdout;
        String::from_utf8(stdout).map_err(|e| e.into())
    } else {
        let description = String::from_utf8(output.stderr)?;
        Err(CommandExecuteError(description).into())
    }
}

/// Set git author
pub fn set_author(location: ConfigFileLocation, author: &Author) -> Result<(), SetError> {
    match (author.name(), author.email()) {
        (Some(name), Some(email)) => {
            set_git_user_param(location, UserParameter::Name, name)?;
            set_git_user_param(location, UserParameter::Email, email)?;
            Ok(())
        }
        _ => Err(AuthorFieldError::new(&author).unwrap().into()),
    }
}

/// unset user.name or user.email
fn unset_git_user_param(
    location: Option<ConfigFileLocation>,
    user_parameter: UserParameter,
) -> Result<(), OutputError> {
    let output = if let Some(location) = location {
        std::process::Command::new("git")
            .arg("config")
            .arg(location.to_arg())
            .arg("--unset")
            .arg(user_parameter.to_arg())
            .output()?
    } else {
        std::process::Command::new("git")
            .arg("config")
            .arg("--unset")
            .arg(user_parameter.to_arg())
            .output()?
    };

    if output.status.success() || output.stderr.is_empty() {
        Ok(())
    } else {
        let description = String::from_utf8(output.stderr)?;
        Err(CommandExecuteError(description).into())
    }
}

/// Unset the author parameters.
pub fn unset_author(location: Option<ConfigFileLocation>) -> Result<(), UnsetError> {
    unset_git_user_param(location, UserParameter::Name)?;
    unset_git_user_param(location, UserParameter::Email)?;
    Ok(())
}

/// Replaces committer and author from past commits in the current branch.
/// options
/// --author-only
/// --committer-only
pub fn replace(filter: ReplaceFilter, target: ReplaceTarget) -> Result<(), ReplaceError> {
    let condition_arg = format!(
        r#"
        if [ {} ]; then
            {}
        fi
        HEAD;"#,
        filter.to_condition_text()?,
        target.to_proccessing_content_text()?
    );

    replace_impl(&condition_arg)
}

/// Replaces committer and author from past commits in the current branch.
///
/// # Arguments
/// * `target` - Old AUTHOR or COMMITTER
/// * `new_author` - New AUTHOR or COMMITTER
pub fn replace_simple(target: Author, new_author: Author) -> Result<(), ReplaceError> {
    let condition_arg = format!(
        r#"
        if [ {} ]; then
            {}
        elif [ {} ]; then
            {}
        elif [ {} ]; then
            {}
        fi
        HEAD;"#,
        ReplaceFilter::AuthorAndCommitter {
            author: target.clone(),
            committer: target.clone()
        }
        .to_condition_text()?,
        ReplaceTarget::AuthorAndCommitter {
            new_author: new_author.clone(),
            new_committer: new_author.clone()
        }
        .to_proccessing_content_text()?,
        ReplaceFilter::AuthorOnly(target.clone()).to_condition_text()?,
        ReplaceTarget::Author {
            new_author: new_author.clone()
        }
        .to_proccessing_content_text()?,
        ReplaceFilter::CommitterOnly(target).to_condition_text()?,
        ReplaceTarget::Committer {
            new_committer: new_author
        }
        .to_proccessing_content_text()?,
    );

    replace_impl(&condition_arg)
}

/// impl replace
fn replace_impl(condition_arg: &str) -> Result<(), ReplaceError> {
    let output = std::process::Command::new("git")
        .arg("filter-branch")
        .arg("-f")
        .arg("--env-filter")
        .arg(condition_arg)
        .output()?;
    output_to_result(output)?;

    Ok(())
}
