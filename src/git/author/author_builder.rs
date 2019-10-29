use super::{Author, Email, Name};
use crate::error::Error;

pub struct AuthorBuilder {
    name: Option<String>,
    email: Option<String>,
}

impl AuthorBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> AuthorBuilder {
        AuthorBuilder {
            name: None,
            email: None,
        }
    }

    pub fn name(self, name: &str) -> AuthorBuilder {
        AuthorBuilder {
            name: Some(name.into()),
            email: self.email,
        }
    }

    pub fn email(self, email: &str) -> AuthorBuilder {
        AuthorBuilder {
            name: self.name,
            email: Some(email.into()),
        }
    }

    pub fn build(self) -> Result<Author, Error> {
        match (self.name, self.email) {
            (Some(name), Some(email)) => Ok(Author {
                name: Name(name),
                email: Email(email),
            }),
            (Some(_), None) => Err(Error::Other {
                reason: "email is empty".into(),
            }),
            (None, Some(_)) => Err(Error::Other {
                reason: "name is empty.".into(),
            }),
            _ => Err(Error::Other {
                reason: "name and email is empty.".into(),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn biuld() {
        let author = AuthorBuilder::new()
            .name("hoge")
            .email("fuga@foo.com")
            .build()
            .unwrap();
        assert_eq!(
            author,
            Author {
                name: Name("hoge".into()),
                email: Email("fuga@foo.com".into())
            }
        );
    }

    #[test]
    #[should_panic]
    fn build_empty_name() {
        let _ = AuthorBuilder::new().email("fuga@foo.com").build().unwrap();
    }

    #[test]
    #[should_panic]
    fn build_empty_email() {
        let _ = AuthorBuilder::new().name("hoge").build().unwrap();
    }

    #[test]
    #[should_panic]
    fn build_empty_params() {
        let _ = AuthorBuilder::new().build().unwrap();
    }
}
