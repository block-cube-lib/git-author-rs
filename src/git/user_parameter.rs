#[derive(Clone, Copy)]
pub enum UserParameter {
    Name,
    Email,
}

impl UserParameter {
    pub fn to_arg(self) -> String {
        use UserParameter::*;
        match &self {
            Name => "user.name".to_string(),
            Email => "user.email".to_string(),
        }
    }
}
