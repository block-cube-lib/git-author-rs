use email_address_parser::EmailAddress;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Author {
    name: Option<String>,
    email: Option<EmailAddress>,
}

impl Author {
    pub fn new(name: Option<&str>, email: Option<&str>) -> Result<Self, crate::error::InvalidEmailAddressError>
    {
        let v = name.map(|s| s.trim().to_string());
        let name = match &v {
            Some(s) if s.is_empty() => None,
            Some(s) => Some(s.to_string()),
            None => None,
        };

        match email {
            Some(email) if EmailAddress::is_valid(email, None) => Ok(Author {
                name,
                email: EmailAddress::parse(email, None),
            }),
            Some(email) => Err(crate::error::InvalidEmailAddressError::new(email)),
            None => Ok(Author { name, email: None }),
        }
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
