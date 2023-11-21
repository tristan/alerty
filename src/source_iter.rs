use serde::{Deserialize, Serialize};
use crate::{sources::{instagram, bandwear}, AlertData, error::AlertyError, Config};

// TODO: move initialize and type to AlertSourceConfig and see if things can be simplified a bit (e.g. maybe we wont need all the type erasure?)
pub(crate) trait AlertSourceConfig {}

pub(crate) trait AlertSource {
    type Config: AlertSourceConfig;
    fn initialize(config: &Self::Config) -> Self;
    fn fetch(&self) -> Result<Vec<AlertData>, AlertyError>;
    fn id(&self) -> String;
}

pub(crate) trait SourceDataType {
    fn datatype(&self) -> DataType;
}

impl SourceDataType for instagram::Instagram {
    fn datatype(&self) -> DataType {
        DataType::Instagram
    }
}

impl SourceDataType for bandwear::Bandwear {
    fn datatype(&self) -> DataType {
        DataType::Bandwear
    }
}

pub trait DynAlertSource: SourceDataType {
    fn fetch(&self) -> Result<Vec<AlertData>, AlertyError>;
    fn id(&self) -> String;
}

impl<S> DynAlertSource for S
where S: ?Sized + AlertSource + SourceDataType,
{
    fn fetch(&self) -> Result<Vec<AlertData>, AlertyError> {
        self.fetch()
    }
    fn id(&self) -> String {
        self.id()
    }
}

pub(crate) struct BoxedSource(pub(crate) Box<dyn DynAlertSource>);

fn initialize_alertsource<S: AlertSource + SourceDataType + 'static>(config: &S::Config) -> BoxedSource {
    BoxedSource(Box::new(S::initialize(config)))
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[serde(rename_all="snake_case")]
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
    type Item = BoxedSource;

    fn next(&mut self) -> Option<Self::Item> {
        let this = self.this;
        let res = loop {
            match self.datatype {
                DataType::Instagram => {
                    if let Some(ig) = &this.instagram {
                        if let Some(cfg) = ig.get(self.idx) {
                            break initialize_alertsource::<instagram::Instagram>(cfg);
                        }
                    }
                    self.datatype = DataType::Bandwear;
                    self.idx = 0;
                }
                DataType::Bandwear => {
                    if let Some(bw) = &this.bandwear {
                        if let Some(cfg) = bw.get(self.idx) {
                            break initialize_alertsource::<bandwear::Bandwear>(cfg);
                        }
                    }
                    return None;
                }
            }
        };
        self.idx += 1;
        Some(res)
    }
}
