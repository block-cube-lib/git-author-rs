use crate::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Author {
    name: Option<String>,
    email: Option<String>,
}

impl Author {
    pub fn new<S1, S2>(name: Option<S1>, email: Option<S2>) -> Result<Self, Error>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let v = name.map(|s| s.as_ref().trim().to_string());
        let name = match &v {
            Some(s) if s.is_empty() => None,
            Some(s) => Some(s.clone()),
            None => None,
        };

        let email = if let Some(email) = email {
            let parsed: addr::Email = email.as_ref().trim().parse()?;
            Some(format!("{}", parsed))
        } else {
            None
        };

        Ok(Author { name, email })
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn email(&self) -> &Option<String> {
        &self.email
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.name, &self.email) {
            (Some(name), Some(email)) => write!(f, "{} <{}>", name, email),
            (Some(name), None) => write!(f, "{}", name),
            (None, Some(email)) => write!(f, "<{}>", email),
            (None, None) => write!(f, ""),
        }
    }
}
