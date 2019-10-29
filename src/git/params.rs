#[derive(Clone, Copy)]
pub enum User {
    Name,
    Email,
}

impl User {
    pub fn to_arg(self) -> String {
        use User::*;
        match &self {
            Name => "user.name".to_string(),
            Email => "user.email".to_string(),
        }
    }
}
