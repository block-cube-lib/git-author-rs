use crate::EmailAddress;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Author {
    name: Option<String>,
    email: Option<EmailAddress>,
}

impl Author {
    pub fn new(name: Option<&str>, email: Option<EmailAddress>) -> Self {
        let v = name.map(|s| s.trim().to_string());
        let name = match &v {
            Some(s) if s.is_empty() => None,
            Some(s) => Some(s.to_string()),
            None => None,
        };
        Author { name, email }
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn email(&self) -> &Option<EmailAddress> {
        &self.email
    }

    pub fn has_none_field(&self) -> bool {
        self.name().is_none() || self.email().is_none()
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
