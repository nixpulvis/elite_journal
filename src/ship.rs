use serde::{Deserialize, Serialize};

/// These are just the game's names, they don't really make sense since tritium is an isotope
/// of hydrogen.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
pub enum Fuel {
    /// When we enter for fleet carriers, not the event
    Tritium,
    /// Ship fuel from the [`crate::entry::incremental::travel::FsdJump`]
    Hydrogen,
}

impl Default for Fuel {
    fn default() -> Self {
        Fuel::Hydrogen
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct JumpCost {
    #[serde(skip)]
    pub ty: Fuel,
    #[serde(rename = "JumpDist")]
    pub distance: Option<f32>,
    #[serde(rename = "FuelUsed")]
    pub used: Option<f32>,
    #[serde(rename = "FuelLevel")]
    pub level: Option<f32>,
}
