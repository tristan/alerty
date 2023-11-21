mod sources;
pub mod utils;
mod config;
mod source_iter;
mod error;

use std::{collections::HashMap};

use error::AlertyError;
use serde::{Deserialize, Serialize};
use source_iter::DataType;

pub use crate::config::Config;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AlertData {
    pub id: String,
    pub thumbnail: Option<String>,
    pub title: Option<String>,
    pub text: Option<String>,
    pub link: Option<String>,
}

type ResultDatabase = HashMap<DataType, HashMap<String, Vec<AlertData>>>;

pub fn run(config: &Config) -> Result<(), AlertyError> {

    let mut database = config.load_database()?;

    let mut new_results: ResultDatabase = HashMap::new();
    let mut errors: Vec<(DataType, String, AlertyError)> = Vec::new();

    for source in config.sources() {
        let datatype = source.0.datatype();
        println!("fetching {:?}:{}", datatype, source.0.id());
        let data = database.entry(datatype).or_default();
        let old_data = data.entry(source.0.id()).or_default();
        let new_data = match source.0.fetch() {
            Ok(new_data) => new_data,
            Err(e) => {
                errors.push((datatype, source.0.id(), e));
                continue;
            }
        };
        let diff = calculate_diff(old_data, new_data.clone());
        *old_data = new_data;
        if !diff.is_empty() {
            let results = new_results.entry(datatype).or_default();
            results.insert(source.0.id(), diff);
        }
    }

    if !new_results.is_empty() {
        // TODO:
        // record new_results in file with timestamp
        // render as html
        // send as email
    }

    config.save_database(&database)?;

    Ok(())
}

fn calculate_diff(
    old_data: &[AlertData],
    mut new_data: Vec<AlertData>,
) -> Vec<AlertData> {
    new_data.retain(|new| {
        if let Some(old) = old_data.iter().find(|old| old.id == new.id) {
            old != new // we only care about this if there is something different
        } else {
            true
        }
    });
    new_data
}
