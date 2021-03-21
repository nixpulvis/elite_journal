use serde::Deserialize;
use crate::prelude::*;

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

/// These are just the game's names, they don't really make sense since tritium is an isotope
/// of hydrogen.
#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
pub enum Fuel {
    /// When we enter for fleet carriers, not the event
    Tritium,
    /// Ship fuel from the [`elite_journal::entry::incremental::travel::FsdJump`]
    Hydrogen,
}

impl Default for Fuel {
    fn default() -> Self {
        Fuel::Hydrogen
    }
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Cost {
    #[serde(skip)]
    pub ty: Fuel,
    // EDDN optional only?
    #[serde(rename = "JumpDist")]
    pub distance: f32,
    // EDDN optional only?
    #[serde(rename = "FuelUsed")]
    pub used: f32,
    #[serde(rename = "FuelLevel")]
    pub level: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FsdJump {
    #[serde(flatten)]
    pub system: System,
    #[serde(flatten)]
    pub cost: Option<Cost>,
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
