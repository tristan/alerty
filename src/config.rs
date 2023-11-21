use std::env;
use std::path::{PathBuf, Path};

use serde::Deserialize;
use crate::ResultDatabase;
use crate::error::AlertyError;
use crate::source_iter::{SourceIter, DataType};
use crate::sources::bandwear::BandwearConfig;
use crate::sources::instagram::InstagramConfig;

fn default_database_path() -> PathBuf {
    let mut home_dir: PathBuf = env::var_os("HOME").map(PathBuf::from).unwrap();
    home_dir.push(".config");
    home_dir.push("alerty");
    home_dir.push("database.json");
    home_dir
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default="default_database_path")]
    pub(crate) database_path: PathBuf,
    pub(crate) instagram: Option<Vec<InstagramConfig>>,
    pub(crate) bandwear: Option<Vec<BandwearConfig>>,
}

impl Config {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, AlertyError> {
        let data = std::fs::read_to_string(path.as_ref())?;
        let config: Config = toml::from_str(&data)?;
        Ok(config)
    }

    pub(crate) fn sources<'a>(&'a self) -> SourceIter<'a> {
        SourceIter { this: self, datatype: DataType::Instagram, idx: 0 }
    }

    pub(crate) fn load_database(&self) -> Result<ResultDatabase, AlertyError> {
        let database_path = &self.database_path;
        let database = if database_path.exists() {
            let data = std::fs::read_to_string(&database_path)?;
            serde_json::from_str::<ResultDatabase>(&data)?
        } else {
            ResultDatabase::default()
        };
        Ok(database)
    }

    pub(crate) fn save_database(&self, database: &ResultDatabase) -> Result<(), AlertyError> {
        let contents = serde_json::to_string(&database)?;
        std::fs::write(&self.database_path, contents)?;
        Ok(())
    }
}
