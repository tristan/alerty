use std::env;
use std::path::{Path, PathBuf};

use crate::error::AlertyError;
use crate::source_iter::{DataType, SourceIter};
use crate::sources::bandwear::BandwearConfig;
use crate::sources::instagram::InstagramConfig;
use crate::{format_current_datetime, ResultDatabase};
use serde::de::Visitor;
use serde::Deserialize;

fn default_database_path() -> PathBuf {
    let mut home_dir: PathBuf = env::var_os("HOME").map(PathBuf::from).unwrap();
    home_dir.push(".config");
    home_dir.push("alerty");
    home_dir.push("database.json");
    home_dir
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_database_path")]
    pub(crate) database_path: PathBuf,
    pub(crate) output_template_path: Option<PathBuf>,
    pub(crate) instagram: Option<Vec<InstagramConfig>>,
    pub(crate) bandwear: Option<Vec<BandwearConfig>>,
    pub smtp: Option<SmtpConfig>,
    pub outputs: Option<Vec<OutputType>>,
}

#[derive(Debug)]
pub enum OutputType {
    StdOut,
    Email,
    File(PathBuf),
}

impl<'de> Deserialize<'de> for OutputType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct _Visitor;
        impl<'de> Visitor<'de> for _Visitor {
            type Value = OutputType;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "variant identifier")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let res = match v.to_lowercase().as_bytes() {
                    b"stdout" => OutputType::StdOut,
                    b"email" => OutputType::Email,
                    _ => {
                        let formatted = format_current_datetime(v);
                        OutputType::File(PathBuf::from(formatted))
                    }
                };
                Ok(res)
            }
        }
        deserializer.deserialize_str(_Visitor)
    }
}

#[derive(Deserialize)]
pub struct SmtpConfig {
    pub relay: String,
    pub username: String,
    pub password: String,
    pub from: String,
    pub to: String,
    pub subject: String,
}

impl Config {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, AlertyError> {
        let data = std::fs::read_to_string(path.as_ref())?;
        let config: Config = toml::from_str(&data)?;
        // validate
        if let Some(outputs) = &config.outputs {
            if outputs.iter().any(|o| matches!(o, OutputType::Email)) && config.smtp.is_none() {
                return Err(AlertyError::config(
                    "output type `email` requires smtp config",
                ));
            }
        }
        Ok(config)
    }

    pub(crate) fn sources(&self) -> SourceIter<'_> {
        SourceIter {
            this: self,
            datatype: DataType::Instagram,
            idx: 0,
        }
    }

    pub(crate) fn load_database(&self) -> Result<ResultDatabase, AlertyError> {
        let database_path = &self.database_path;
        let database = if database_path.exists() {
            let data = std::fs::read_to_string(database_path)?;
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
