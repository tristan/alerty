use crate::{error::AlertyError, AlertData, Config};
use serde::{Deserialize, Serialize};

pub(crate) trait AlertSourceConfig {
    type Source: AlertSource;
    fn initialize_source(&self) -> Self::Source;
}

pub(crate) trait AlertSource {
    fn fetch(&self) -> Result<Vec<AlertData>, AlertyError>;
    fn id(&self) -> String;
}

pub(crate) struct BoxedSource(pub(crate) Box<dyn AlertSource>);

pub(crate) fn initialize_boxedsource<S: AlertSource + 'static>(
    config: &dyn AlertSourceConfig<Source = S>,
) -> BoxedSource {
    BoxedSource(Box::new(config.initialize_source()))
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DataType {
    Instagram,
    Bandwear,
}

pub(crate) struct SourceIter<'a> {
    pub(crate) this: &'a Config,
    pub(crate) datatype: DataType,
    pub(crate) idx: usize,
}

impl<'a> Iterator for SourceIter<'a> {
    type Item = (DataType, BoxedSource);

    fn next(&mut self) -> Option<Self::Item> {
        let this = self.this;
        let res = loop {
            match self.datatype {
                DataType::Instagram => {
                    if let Some(ig) = &this.instagram {
                        if let Some(cfg) = ig.get(self.idx) {
                            break initialize_boxedsource(cfg);
                        }
                    }
                    self.datatype = DataType::Bandwear;
                    self.idx = 0;
                }
                DataType::Bandwear => {
                    if let Some(bw) = &this.bandwear {
                        if let Some(cfg) = bw.get(self.idx) {
                            break initialize_boxedsource(cfg);
                        }
                    }
                    return None;
                }
            }
        };
        self.idx += 1;
        Some((self.datatype, res))
    }
}
