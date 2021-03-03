use serde::Deserialize;
use crate::{
    Coordinate,
    Government,
    Allegiance,
    Economy,
    Faction,
    FactionInfo,
    FactionConflict,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct System {
    #[serde(rename = "SystemAddress")]
    pub address: u64,
    #[serde(rename = "StarPos")]
    pub pos: Coordinate,
    #[serde(rename = "StarSystem")]
    pub name: String,

    pub population: u64,
    #[serde(rename = "SystemSecurity")]
    pub security: Security,

    #[serde(rename = "SystemGovernment")]
    pub government: Government,
    #[serde(rename = "SystemAllegiance")]
    pub allegiance: Allegiance,
    #[serde(rename = "SystemEconomy")]
    pub economy: Economy,
    #[serde(rename = "SystemSecondEconomy")]
    pub second_economy: Economy,

    #[serde(rename = "SystemFaction")]
    pub controlling_faction: Option<Faction>,
    #[serde(default)]
    pub factions: Vec<FactionInfo>,
    #[serde(default)]
    pub conflicts: Vec<FactionConflict>,

    pub powers: Option<Vec<String>>,
    pub powerplay_state: Option<PowerplayState>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[serde(rename_all = "PascalCase")]
pub enum Security {
    #[serde(rename = "$SYSTEM_SECURITY_high;")]
    #[serde(alias = "$GAlAXY_MAP_INFO_state_high;")]
    High,
    #[serde(rename = "$SYSTEM_SECURITY_medium;")]
    #[serde(alias = "$GAlAXY_MAP_INFO_state_medium;")]
    Medium,
    #[serde(rename = "$SYSTEM_SECURITY_low;")]
    #[serde(alias = "$GAlAXY_MAP_INFO_state_low;")]
    Low,
    #[serde(rename = "$SYSTEM_SECURITY_anarchy;")]
    #[serde(alias = "$GAlAXY_MAP_INFO_state_anarchy;")]
    Anarchy,
    #[serde(rename = "")]
    None,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
pub enum PowerplayState {
    InPrepareRadius,
    Prepared,
    Exploited,
    Contested,
    Controlled,
    Turmoil,
    HomeSystem,
}
