// automake-rs-core: Oracle profile management
//
// Load and validate the oracle profile for subordinate oracle queries.

use std::path::Path;

#[derive(Debug, Clone)]
pub struct Profile {
    pub automake_version: Option<String>,
    pub aclocal_version: Option<String>,
}

impl Profile {
    pub fn load(path: &Path) -> Result<Self, ProfileError> {
        let data = std::fs::read_to_string(path).map_err(ProfileError::Io)?;
        let profile: serde_json::Value =
            serde_json::from_str(&data).map_err(|e| ProfileError::Parse(e.to_string()))?;

        let am_version = profile
            .get("binaries")
            .and_then(|b| b.get("automake"))
            .and_then(|am| am.get("version_output"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let ac_version = profile
            .get("binaries")
            .and_then(|b| b.get("aclocal"))
            .and_then(|ac| ac.get("version_output"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(Self {
            automake_version: am_version,
            aclocal_version: ac_version,
        })
    }
}

#[derive(Debug)]
pub enum ProfileError {
    Io(std::io::Error),
    Parse(String),
    Missing,
}

impl std::fmt::Display for ProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O: {}", e),
            Self::Parse(s) => write!(f, "parse: {}", s),
            Self::Missing => write!(f, "profile missing"),
        }
    }
}

impl std::error::Error for ProfileError {}
