use super::Author;
use crate::error::{
    AuthorFieldError, AuthorHasNoneField, CommitterHasNoneField, ConditionTextError,
};

#[derive(Debug)]
pub enum ReplaceFilter {
    AuthorOnly(Author),
    CommitterOnly(Author),
    AuthorOrCommitter { author: Author, committer: Author },
    AuthorAndCommitter { author: Author, committer: Author },
}

macro_rules! author_condition_format {
    ($name:expr, $email:expr) => {
        format!(
            r#""$GIT_AUTHOR_NAME" = "{}" -a "$GIT_AUTHOR_EMAIL" = "{}""#,
            $name, $email
        )
    };
}

macro_rules! committer_condition_format {
    ($name:expr, $email:expr) => {
        format!(
            r#""$GIT_COMMITTER_NAME" = "{}" -a "$GIT_COMMITTER_EMAIL" = "{}""#,
            $name, $email
        )
    };
}

impl ReplaceFilter {
    pub fn to_condition_text(&self) -> Result<String, ConditionTextError> {
        match self {
            Self::AuthorOnly(author) => match (author.name(), author.email()) {
                (Some(name), Some(email)) => Ok(author_condition_format!(name, email)),
                (_, _) => Err(AuthorHasNoneField(AuthorFieldError::new(author)?).into()),
            },

            Self::CommitterOnly(committer) => match (committer.name(), committer.email()) {
                (Some(name), Some(email)) => Ok(committer_condition_format!(name, email)),
                (_, _) => Err(CommitterHasNoneField(AuthorFieldError::new(committer)?).into()),
            },

            Self::AuthorOrCommitter { author, committer } => {
                if author.has_none_field() {
                    Err(AuthorHasNoneField(AuthorFieldError::new(author)?).into())
                } else if committer.has_none_field() {
                    Err(CommitterHasNoneField(AuthorFieldError::new(committer)?).into())
                } else if let (
                    Some(author_name),
                    Some(author_email),
                    Some(committer_name),
                    Some(committer_email),
                ) = (
                    author.name(),
                    author.email(),
                    committer.name(),
                    committer.email(),
                ) {
                    Ok(format!(
                        "{} -o {}",
                        author_condition_format!(author_name, author_email),
                        committer_condition_format!(committer_name, committer_email)
                    ))
                } else {
                    unreachable!()
                }
            }

            Self::AuthorAndCommitter { author, committer } => {
                if author.has_none_field() {
                    Err(AuthorHasNoneField(AuthorFieldError::new(author)?).into())
                } else if committer.has_none_field() {
                    Err(CommitterHasNoneField(AuthorFieldError::new(committer)?).into())
                } else if let (
                    Some(author_name),
                    Some(author_email),
                    Some(committer_name),
                    Some(committer_email),
                ) = (
                    author.name(),
                    author.email(),
                    committer.name(),
                    committer.email(),
                ) {
                    Ok(format!(
                        "{} -a {}",
                        author_condition_format!(author_name, author_email),
                        committer_condition_format!(committer_name, committer_email)
                    ))
                } else {
                    unreachable!()
                }
            }
        }
    }
}
