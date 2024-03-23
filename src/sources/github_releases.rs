use crate::{source_iter::{AlertSourceConfig, AlertSource}, AlertData, error::AlertyError};
use serde::Deserialize;

pub struct GithubReleases {
    project: String,
}

#[derive(Deserialize)]
pub struct GithubReleasesConfig {
    project: String,
}

#[derive(Deserialize)]
struct AtomFeed {
    #[serde(rename="entry")]
    entries: Vec<Entry>
}

#[derive(Deserialize)]
struct Entry {
    id: String,
    title: String,
    link: Link,
}

#[derive(Deserialize)]
struct Link {
    #[serde(rename="@href")]
    href: String
}

impl AlertSourceConfig for GithubReleasesConfig {
    type Source = GithubReleases;
    fn initialize_source(&self) -> Self::Source {
        Self::Source {
            project: self.project.clone(),
        }
    }
}

impl AlertSource for GithubReleases {
    fn id(&self) -> String {
        self.project.clone()
    }

    fn fetch(&self) -> Result<Vec<AlertData>, AlertyError> {
        let res = ureq::get(&format!(
            "https://github.com/{}/releases.atom", self.project
        ))
            .call();
        let res = match res {
            Ok(res) => res,
            Err(e) => {
                return Err(AlertyError::other(format!("ERROR FETCHING DATA: {e}")));
            }
        };
        let doc = res.into_string().unwrap();
        let feed: AtomFeed = match quick_xml::de::from_str(&doc) {
            Ok(res) => res,
            Err(e) => {
                return Err(AlertyError::other(format!("ERROR PARSING DATA: {e}")));
            }
        };
        feed.entries.into_iter().map(|e| {
            Ok(AlertData {
                id: e.id,
                title: Some(e.title),
                text: None,
                thumbnail: None,
                link: Some(e.link.href),
            })
        }).collect::<Result<Vec<AlertData>, AlertyError>>()
    }

}
