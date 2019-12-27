use super::Author;
use crate::error::{
    AuthorFieldError, AuthorHasNoneField, CommitterHasNoneField, ProccessingContentError,
};

#[derive(Debug)]
pub enum ReplaceTarget {
    Author {
        new_author: Author,
    },
    Committer {
        new_committer: Author,
    },
    AuthorAndCommitter {
        new_author: Author,
        new_committer: Author,
    },
}

macro_rules! author_format {
    ($name: expr, $email: expr) => {
        format!("GIT_AUTHOR_NAME={} GIT_AUTHOR_EMAIL={};", $name, $email)
    };
}
macro_rules! committer_format {
    ($name: expr, $email: expr) => {
        format!(
            "GIT_COMMITTER_NAME={} GIT_COMMITTER_EMAIL={};",
            $name, $email
        )
    };
}

impl ReplaceTarget {
    pub fn to_proccessing_content_text(&self) -> Result<String, ProccessingContentError> {
        match self {
            Self::Author { new_author } => match (new_author.name(), new_author.email()) {
                (Some(name), Some(email)) => Ok(author_format!(name, email)),
                (_, _) => Err(AuthorHasNoneField(AuthorFieldError::new(new_author)?).into()),
            },

            Self::Committer { new_committer } => {
                match (new_committer.name(), new_committer.email()) {
                    (Some(name), Some(email)) => Ok(committer_format!(name, email)),
                    (_, _) => Err(AuthorHasNoneField(AuthorFieldError::new(new_committer)?).into()),
                }
            }

            Self::AuthorAndCommitter {
                new_author,
                new_committer,
            } => {
                if new_author.has_none_field() {
                    Err(AuthorHasNoneField(AuthorFieldError::new(new_author)?).into())
                } else if new_committer.has_none_field() {
                    Err(CommitterHasNoneField(AuthorFieldError::new(new_committer)?).into())
                } else if let (
                    Some(author_name),
                    Some(author_email),
                    Some(committer_name),
                    Some(committer_email),
                ) = (
                    new_author.name(),
                    new_author.email(),
                    new_committer.name(),
                    new_committer.email(),
                ) {
                    Ok(format!(
                        "{} {}",
                        author_format!(author_name, author_email),
                        committer_format!(committer_name, committer_email)
                    ))
                } else {
                    unreachable!()
                }
            }
        }
    }
}
