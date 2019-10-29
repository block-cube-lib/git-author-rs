#[derive(Debug, Clone, Copy)]
pub enum ConfigFileLocation {
    Global,
    Local,
}

impl std::fmt::Display for ConfigFileLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use ConfigFileLocation::*;
        match &self {
            Global => write!(f, "global"),
            Local => write!(f, "local"),
        }
    }
}

impl ConfigFileLocation {
    pub fn to_arg(self) -> String {
        use ConfigFileLocation::*;
        match &self {
            Global => "--global".to_string(),
            Local => "--local".to_string(),
        }
    }
}
