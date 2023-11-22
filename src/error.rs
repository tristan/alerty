#[derive(Debug)]
pub enum AlertyError {
    UreqError(Box<ureq::Error>),
    IoError(std::io::Error),
    TomlError(toml::de::Error),
    JsonError(serde_json::Error),
    Other(String),
}

impl AlertyError {
    pub fn other<S>(error: S) -> Self
    where
        S: ToString,
    {
        AlertyError::Other(error.to_string())
    }
}

impl From<ureq::Error> for AlertyError {
    fn from(value: ureq::Error) -> Self {
        AlertyError::UreqError(Box::new(value))
    }
}

impl From<std::io::Error> for AlertyError {
    fn from(value: std::io::Error) -> Self {
        AlertyError::IoError(value)
    }
}

impl From<toml::de::Error> for AlertyError {
    fn from(value: toml::de::Error) -> Self {
        AlertyError::TomlError(value)
    }
}

impl From<serde_json::Error> for AlertyError {
    fn from(value: serde_json::Error) -> Self {
        AlertyError::JsonError(value)
    }
}

impl std::fmt::Display for AlertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertyError::UreqError(e) => write!(f, "{}", e),
            AlertyError::IoError(e) => write!(f, "{}", e),
            AlertyError::TomlError(e) => write!(f, "{}", e),
            AlertyError::JsonError(e) => write!(f, "{}", e),
            AlertyError::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for AlertyError {}
