use serde::Deserialize;
use crate::{System, Station};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FsdTarget {
    #[serde(rename = "SystemAddress")]
    address: u64,
    name: String,
    star_class: String,  // TODO: Enum?
    #[serde(rename = "RemainingJumpsInRoute")]
    remaining: Option<u16>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FsdJump {
    #[serde(flatten)]
    pub system: System,
    // EDDN optional only?
    pub jump_dist: Option<f64>,
    // EDDN optional only?
    pub fuel_used: Option<f64>,
    pub fuel_level: Option<f64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    #[serde(flatten)]
    pub system: System,
    pub body: String,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub body_type: String,  // TODO: Enum?
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub docked: bool,
    #[serde(flatten)]
    pub station: Option<Station>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Docked {
    // NOTE: Should really be Some(false) when parsed locally. EDDN filters this field.
    pub active_fine: Option<bool>,
    #[serde(flatten)]
    pub station: Station,
}
