use crate::error::Error;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(pub String);

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.trim();
        if !name.is_empty() {
            Ok(Name(name.into()))
        } else {
            Err(Error::ParseError)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(pub String);

impl FromStr for Email {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let email: addr::Email = s.parse()?;
        let email = Email(format!("{}", email));
        Ok(email)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Author {
    name: Name,
    email: Email,
}

impl Author {
    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn email(&self) -> &Email {
        &self.email
    }
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} <{}>", self.name.0, self.email.0)
    }
}

impl FromStr for Author {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"(.+)<(.+)>"#)?;

        if let Some(cap) = re.captures(s) {
            let name = Name::from_str(&cap[1])?;
            let email = Email::from_str(&cap[2])?;
            Ok(Author { name, email })
        } else {
            Err(Error::ParseError)
        }
    }
}

pub mod author_builder;
pub use author_builder::AuthorBuilder;
