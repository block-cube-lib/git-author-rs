/// Config file location
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFileLocation {
    /// use global config file
    Global,
    /// use repository config file
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
    pub const VARIANTRS: [Self; 2] = [Self::Global, Self::Local];

    pub fn to_arg(self) -> String {
        format!("--{}", self)
    }
}
