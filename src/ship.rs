use serde::{Serialize, Deserialize};

/// These are just the game's names, they don't really make sense since tritium is an isotope
/// of hydrogen.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(all(unix, feature = "with-sqlx"), derive(sqlx::Type))]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct JumpCost {
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
