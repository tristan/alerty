pub mod config;
mod error;
mod source_iter;
mod sources;
pub mod utils;

use std::collections::HashMap;

use error::AlertyError;
use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};
use source_iter::{AlertSource, DataType};

pub use crate::config::Config;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AlertData {
    pub id: String,
    pub thumbnail: Option<String>,
    pub title: Option<String>,
    pub text: Option<String>,
    pub link: Option<String>,
}

type ResultDatabaseData = HashMap<DataType, HashMap<String, Vec<AlertData>>>;
type ResultDatabaseErrors = Vec<(DataType, String, String)>;

#[derive(Default, Serialize, Deserialize)]
struct ResultDatabase {
    data: ResultDatabaseData,
    errors: ResultDatabaseErrors,
}

pub fn run(config: &Config, test_mode: bool) -> Result<Option<String>, AlertyError> {
    let mut env = Environment::new();
    env.add_filter("split", |s: String, sep: &str| -> Vec<String> {
        s.split(sep).map(String::from).collect()
    });
    env.add_function("current_date", |format: &str| -> String {
        format_current_datetime(format)
    });
    let output_template = if let Some(path) = &config.output_template_path {
        let template = std::fs::read_to_string(path)?;
        env.add_template_owned("user", template)?;
        env.get_template("user")?
    } else {
        // TODO: optional json output
        env.add_template("html", include_str!("default_output_template.html"))?;
        env.get_template("html")?
    };

    let mut database = config.load_database()?;

    let mut new_results: ResultDatabaseData = Default::default();
    let mut errors: Vec<(DataType, String, String)> = Vec::new();

    if test_mode {
        new_results.entry(DataType::Instagram).or_default().insert(
            String::from("username"),
            vec![AlertData {
                id: String::from("post-id"),
                thumbnail: None,
                link: Some(String::from("http://localhost/image.png")),
                title: Some(String::from("hello")),
                text: None,
            }],
        );
        errors.push((
            DataType::Bandwear,
            String::from("testing"),
            String::from("Empty data"),
        ));
    } else {
        for (datatype, source) in config.sources() {
            eprintln!("fetching {:?}:{}", datatype, source.0.id());
            let data = database.data.entry(datatype).or_default();
            let old_data = data.entry(source.0.id()).or_default();
            let new_data = match source.0.fetch() {
                Ok(new_data) => new_data,
                Err(e) => {
                    let e_string = e.to_string();
                    eprintln!("Error: {e_string}");
                    errors.push((datatype, source.0.id(), e_string));
                    continue;
                }
            };
            if new_data.is_empty() {
                errors.push((datatype, source.0.id(), String::from("Empty data")));
            } else {
                let diff = calculate_diff(&*source.0, old_data, new_data.clone());
                *old_data = new_data;
                if !diff.is_empty() {
                    let results = new_results.entry(datatype).or_default();
                    results.insert(source.0.id(), diff);
                }
            }
        }

        let new_errors = calculate_error_diff(&database.errors, errors.clone());
        database.errors = errors;
        errors = new_errors;
        config.save_database(&database)?;
    }

    if !new_results.is_empty() || !errors.is_empty() {
        Ok(Some(output_template.render(
            context!(results => new_results, errors => errors),
        )?))
    } else {
        Ok(None)
    }
}

fn calculate_diff(
    source: &dyn AlertSource,
    old_data: &[AlertData],
    mut new_data: Vec<AlertData>,
) -> Vec<AlertData> {
    new_data.retain(|new| {
        if let Some(old) = old_data.iter().find(|old| old.id == new.id) {
            source.diff(old, new) // we only care about this if there is something different
        } else {
            true
        }
    });
    new_data
}

fn calculate_error_diff(
    old_data: &ResultDatabaseErrors,
    mut new_data: ResultDatabaseErrors,
) -> ResultDatabaseErrors {
    new_data.retain(|new| !old_data.iter().any(|old| old == new));
    new_data
}

pub fn format_current_datetime(input: &str) -> String {
    chrono::Local::now().format(input).to_string()
}
