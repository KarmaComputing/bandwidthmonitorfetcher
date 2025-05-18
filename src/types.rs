use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct VnStats {
    pub(crate) vnstatversion: String,
    pub(crate) jsonversion: String,
    pub(crate) interfaces: Vec<VnStatInterface>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct VnStatInterface {
    pub(crate) name: String,
    pub(crate) alias: String,
}
