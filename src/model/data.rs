use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Data {
    pub pl_name: String,
    pub hostname: String,
    pub pl_letter: String,
    pub sy_snum: String,
    pub sy_pnum: String,
    pub sy_mnum: String,
    pub cb_flag: String,

    #[serde(rename = "discoverymethod")]
    pub discovery_method: Option<String>,

    pub disc_year: String,
    pub disc_refname: String,
    pub disc_pubdate: String,
    pub disc_refhref: String,
    pub caltech_href: String,
}
