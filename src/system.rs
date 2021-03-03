use serde::Deserialize;
use crate::{
    de,
    Nullable,
    Coordinate,
    Government,
    Allegiance,
    Economy,
    Faction,
    FactionInfo,
    Conflict,
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
    #[serde(deserialize_with = "de::enum_is_null")]
    pub security: Option<Security>,

    #[serde(rename = "SystemGovernment")]
    #[serde(deserialize_with = "de::enum_is_null")]
    pub government: Option<Government>,
    #[serde(rename = "SystemAllegiance")]
    #[serde(deserialize_with = "de::enum_is_null")]
    pub allegiance: Option<Allegiance>,
    #[serde(rename = "SystemEconomy")]
    #[serde(deserialize_with = "de::enum_is_null")]
    pub economy: Option<Economy>,
    #[serde(rename = "SystemSecondEconomy")]
    #[serde(deserialize_with = "de::enum_is_null")]
    pub second_economy: Option<Economy>,

    #[serde(rename = "SystemFaction")]
    pub controlling_faction: Option<Faction>,
    #[serde(default)]
    pub factions: Vec<FactionInfo>,
    #[serde(default)]
    pub conflicts: Vec<Conflict>,

    // TODO: Should this even be an enum?
    pub powers: Option<Vec<String>>,
    pub powerplay_state: Option<PowerplayState>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[serde(rename_all = "PascalCase")]
pub enum Security {
    #[serde(alias = "$SYSTEM_SECURITY_high;")]
    #[serde(alias = "$GAlAXY_MAP_INFO_state_high;")]
    High,
    #[serde(alias = "$SYSTEM_SECURITY_medium;")]
    #[serde(alias = "$GAlAXY_MAP_INFO_state_medium;")]
    Medium,
    #[serde(alias = "$SYSTEM_SECURITY_low;")]
    #[serde(alias = "$GAlAXY_MAP_INFO_state_low;")]
    Low,
    #[serde(alias = "$SYSTEM_SECURITY_anarchy;")]
    #[serde(alias = "$GAlAXY_MAP_INFO_state_anarchy;")]
    Anarchy,
    #[serde(alias = "")]
    None,
}

impl Nullable for Security {
    fn is_null(&self) -> bool {
        match self {
            Security::None => true,
            Security::Anarchy => true,
            _ => false,
        }
    }
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
