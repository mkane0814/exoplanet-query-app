use serde::{Serialize, Deserialize};
use bson::Document;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Data {
    pub pl_name: String,
    pub hostname: String,
    pub pl_letter: String,
    pub hd_name: String,
    pub hip_name: String,
    pub tic_id: String,
    pub gaia_id: String,
    pub default_flag: String,
    pub sy_snum: String,
    pub sy_pnum: String,
    pub sy_mnum: String,
    pub cb_flag: String,
    pub discovery_method: Option<String>,
    pub disc_year: String,
    pub disc_refname: String,
    pub disc_pubdate: String,
}
