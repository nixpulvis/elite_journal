use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FsdJump {
    pub star_system: String,
    pub system_address: u64,
    pub star_pos: Coordinate,
    pub system_allegiance: String,
    pub system_economy: String,
    pub system_second_economy: String,
    pub system_government: String,
    pub system_security: String,
    pub population: u64,
    // EDDN optional only?
    pub jump_dist: Option<f64>,
    // EDDN optional only?
    pub fuel_used: Option<f64>,
    pub fuel_level: Option<f64>,
    pub system_faction: Option<SystemFaction>,
    #[serde(default)]
    pub factions: Vec<Faction>,
    #[serde(default)]
    pub conflicts: Vec<Conflict>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SystemFaction {
    pub name: String,
    #[serde(rename = "FactionState")]
    pub state: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Faction {
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

    // personal fields

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
pub struct Conflict {
    pub war_type: String,
    pub status: String,
    pub faction_1: ConflictProgress,
    pub faction_2: ConflictProgress,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ConflictProgress {
    pub name: String,
    pub stake: String,
    pub won_days: u8,
}
