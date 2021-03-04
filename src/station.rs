use serde::Deserialize;
use crate::{
    Government,
    Allegiance,
    Economy,
    Faction,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Station {
    pub dist_from_star_ls: Option<f64>,
    pub station_name: String,
    pub station_type: String,  // TODO: Enum?
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_faction: Faction,
    pub station_government: Government,
    pub station_allegiance: Option<Allegiance>,
    pub station_services: Vec<String>,  // TODO: Enums??
    pub station_economies: Vec<EconomyShare>,
    // NOTE: Should really be Some(false) when parsed locally. EDDN filters this field.
    pub wanted: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EconomyShare {
    pub name: Economy,
    pub proportion: f64,
}
