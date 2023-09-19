use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub struct Data {
    pub planet_data: Vec<PlanetData>,
    pub last_id: i64,
    pub first_id: i64,
}

impl Data {
    pub fn build(mut data_vec: Vec<PlanetData>) -> Option<Data> {
        data_vec.sort_unstable_by(|a, b| a.id.cmp(&b.id));
        if let Some(last_entry) = data_vec.last() {
            if let Some(first_entry) = data_vec.first() {
                let last_id = last_entry.id;
                let first_id = first_entry.id;
                leptos::logging::log!("First ID: {}", first_id);
                leptos::logging::log!("Last ID: {}", last_id);
                let data = Data {
                    planet_data: data_vec,
                    last_id,
                    first_id,
                };
                Some(data)
            } else {
                None
            }
        } else {
            None
        }
    }
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
