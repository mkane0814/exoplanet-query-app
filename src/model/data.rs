use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub struct Data {
    pub planet_data: Vec<PlanetData>,
    pub last_id: i64,
    pub first_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct PlanetData {
    pub pl_name: String,
    pub hostname: String,
    pub pl_letter: String,
    pub sy_snum: String,
    pub sy_pnum: String,
    pub sy_mnum: String,
    pub cb_flag: u8,
    pub default_flag: bool,

    #[serde(rename = "discoverymethod")]
    pub discovery_method: Option<String>,

    pub disc_year: String,
    pub disc_refname: String,
    pub disc_pubdate: String,
    pub disc_refhref: String,
    pub caltech_href: String,
    pub disc_telescope: String,
    pub pl_orbper: Option<String>,
    pub pl_orbpererr1: Option<String>,
    pub pl_orbpererr2: Option<String>,
    pub pl_rade: Option<String>,
    pub pl_radeerr1: Option<String>,
    pub pl_radeerr2: Option<String>,
    pub pl_bmasse: Option<String>,
    pub pl_bmasseerr1: Option<String>,
    pub pl_bmasseerr2: Option<String>,
    pub pl_bmassprov: Option<String>,
    #[serde(rename = "releasedate")]
    pub release_date: String,
    pub st_spectype: Option<String>,
    pub disc_facility: String,
    pub id: i64,
}
