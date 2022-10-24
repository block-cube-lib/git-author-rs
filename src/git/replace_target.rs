use crate::error::{
    AuthorFieldError, AuthorHasNoneField, CommitterHasNoneField, ProccessingContentError,
    ReplaceTargetParseError,
};
use clap::ArgEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ArgEnum)]
pub enum ReplaceTarget {
    AuthorOnly,
    CommitterOnly,
    AuthorAndCommitter,
}

impl std::str::FromStr for ReplaceTarget {
    type Err = ReplaceTargetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReplaceTarget::*;
        match s {
            "author-only" => Ok(AuthorOnly),
            "committer-only" => Ok(CommitterOnly),
            "author-only" => Ok(AuthorAndCommitter),
            _ => Err(ReplaceTargetParseError),
        }
    }
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
        unreachable!();
        return Ok(("----".to_string()));
        //match self {
        //    Self::Author { new_author } => match (new_author.name(), new_author.email()) {
        //        (Some(name), Some(email)) => Ok(author_format!(name, email)),
        //        (_, _) => Err(AuthorHasNoneField(AuthorFieldError::new(new_author)?).into()),
        //    },

        //    Self::Committer { new_committer } => {
        //        match (new_committer.name(), new_committer.email()) {
        //            (Some(name), Some(email)) => Ok(committer_format!(name, email)),
        //            (_, _) => Err(AuthorHasNoneField(AuthorFieldError::new(new_committer)?).into()),
        //        }
        //    }

        //    Self::AuthorAndCommitter {
        //        new_author,
        //        new_committer,
        //    } => {
        //        if new_author.has_none_field() {
        //            Err(AuthorHasNoneField(AuthorFieldError::new(new_author)?).into())
        //        } else if new_committer.has_none_field() {
        //            Err(CommitterHasNoneField(AuthorFieldError::new(new_committer)?).into())
        //        } else if let (
        //            Some(author_name),
        //            Some(author_email),
        //            Some(committer_name),
        //            Some(committer_email),
        //        ) = (
        //            new_author.name(),
        //            new_author.email(),
        //            new_committer.name(),
        //            new_committer.email(),
        //        ) {
        //            Ok(format!(
        //                "{} {}",
        //                author_format!(author_name, author_email),
        //                committer_format!(committer_name, committer_email)
        //            ))
        //        } else {
        //            unreachable!()
        //        }
        //    }
        //}
    }
}
