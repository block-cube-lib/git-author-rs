use crate::error::InvalidEmailAddressError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailAddress(String);

impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmailAddress {
    type Err = InvalidEmailAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(_) = email_address_parser::EmailAddress::parse(s, None) {
            Ok(EmailAddress(s.to_string()))
        } else {
            Err(InvalidEmailAddressError {
                address: s.to_string(),
            })
        }
    }
}
