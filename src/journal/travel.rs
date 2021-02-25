use serde::Deserialize;
use crate::{string_is_none, Coordinate};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct System {
    #[serde(rename = "StarSystem")]
    pub name: String,
    #[serde(rename = "SystemAddress")]
    pub address: u64,
    #[serde(rename = "StarPos")]
    pub pos: Coordinate,
    #[serde(rename = "SystemAllegiance")]
    #[serde(deserialize_with = "string_is_none")]
    pub allegiance: Option<String>,
    #[serde(rename = "SystemEconomy")]
    #[serde(deserialize_with = "string_is_none")]
    pub economy: Option<String>,
    #[serde(rename = "SystemSecondEconomy")]
    #[serde(deserialize_with = "string_is_none")]
    pub second_economy: Option<String>,
    #[serde(rename = "SystemGovernment")]
    #[serde(deserialize_with = "string_is_none")]
    pub government: Option<String>,
    #[serde(rename = "SystemSecurity")]
    #[serde(deserialize_with = "string_is_none")]
    pub security: Option<String>,
    pub population: u64,
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
#[serde(rename_all = "PascalCase")]
pub struct Station {
    pub dist_from_star_ls: Option<f64>,
    pub station_name: String,
    pub station_type: String,  // TODO: Enum?
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_faction: Faction,
    pub station_government: String,  // TODO: Enum?
    pub station_allegiance: Option<String>,  // TODO: Enum?
    pub station_services: Vec<String>,  // TODO: Enums??
    pub station_economies: Vec<Economy>,  // ???? (Array of (Name,Proportion) pairs )
    // NOTE: Should really be Some(false) when parsed locally. EDDN filters this field.
    pub wanted: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Docked {
    // NOTE: Should really be Some(false) when parsed locally. EDDN filters this field.
    pub active_fine: Option<bool>,
    #[serde(flatten)]
    pub station: Station,
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
pub struct Faction {
    pub name: String,
    #[serde(rename = "FactionState")]
    pub state: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FactionInfo {
    pub name: String,
    #[serde(rename = "FactionState")]
    pub state: String,
    pub government: String,
    pub influence: f64,
    pub allegiance: String,
    pub happiness: String,
    #[serde(default)]
    pub pending_states: Vec<FactionStateTrend>,
    #[serde(default)]
    pub active_states: Vec<FactionStateTrend>,
    #[serde(default)]
    pub recovering_states: Vec<FactionStateTrend>,
    // EDDN optional only?
    #[serde(rename = "MyReputation")]
    pub reputation: Option<f64>,
    #[serde(default)]
    pub squadron_faction: bool,
    #[serde(default)]
    pub home_system: bool,
    #[serde(default)]
    pub happiest_system: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionStateTrend {
    pub state: String,
    pub trend: Option<u8>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FactionConflict {
    pub war_type: String,
    pub status: String,
    pub faction_1: FactionConflictProgress,
    pub faction_2: FactionConflictProgress,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FactionConflictProgress {
    pub name: String,
    pub stake: String,
    pub won_days: u8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Economy {
    pub name: String,
    pub proportion: f64,
}

#[derive(Deserialize, Debug)]
pub enum PowerplayState {
    InPrepareRadius,
    Prepared,
    Exploited,
    Contested,
    Controlled,
    Turmoil,
    HomeSystem,
}
