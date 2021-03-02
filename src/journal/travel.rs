use serde::Deserialize;
use crate::Coordinate;

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
    pub allegiance: Allegiance,
    #[serde(rename = "SystemEconomy")]
    pub economy: EconomyKind,
    #[serde(rename = "SystemSecondEconomy")]
    pub second_economy: EconomyKind,
    #[serde(rename = "SystemGovernment")]
    pub government: Government,
    #[serde(rename = "SystemSecurity")]
    pub security: Security,
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
    pub station_government: Government,
    pub station_allegiance: Option<Allegiance>,
    pub station_services: Vec<String>,  // TODO: Enums??
    pub station_economies: Vec<Economy>,
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
    pub government: Government,
    pub influence: f32,
    pub allegiance: Allegiance,
    pub happiness: Happiness,
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
pub enum Happiness {
    #[serde(rename = "$Faction_HappinessBand1;")]
    Elated,
    #[serde(rename = "$Faction_HappinessBand2;")]
    Happy,
    #[serde(rename = "$Faction_HappinessBand3;")]
    Discontented,
    #[serde(rename = "$Faction_HappinessBand4;")]
    Unhappy,
    #[serde(rename = "$Faction_HappinessBand5;")]
    Despondent,
    #[serde(rename = "")]
    None,
}

#[derive(Deserialize, Debug)]
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
#[serde(rename_all = "PascalCase")]
pub struct Economy {
    pub name: EconomyKind,
    pub proportion: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum EconomyKind {
    #[serde(rename = "$economy_Agri;")]
    Agriculture,
    #[serde(rename = "$economy_Colony;")]
    Colony,
    #[serde(rename = "$economy_Extraction;")]
    Extraction,
    #[serde(rename = "$economy_HighTech;")]
    HighTech,
    #[serde(rename = "$economy_Industrial;")]
    Industrial,
    #[serde(rename = "$economy_Military;")]
    Military,
    #[serde(rename = "$economy_Refinery;")]
    Refinery,
    #[serde(rename = "$economy_Service;")]
    Service,
    #[serde(rename = "$economy_Terraforming;")]
    Terraforming,
    #[serde(rename = "$economy_Tourism;")]
    Tourism,
    #[serde(rename = "$economy_Carrier;")]
    Carrier,
    #[serde(rename = "$economy_Prison;")]
    Prison,
    #[serde(rename = "$economy_Undefined;")]
    Undefined,
    #[serde(rename = "")]
    #[serde(alias = "$economy_None;")]
    None,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum Government {
    #[serde(alias = "$government_Anarchy;")]
    Anarchy,
    #[serde(alias = "$government_Communism;")]
    Communism,
    #[serde(alias = "$government_Confederacy;")]
    Confederacy,
    #[serde(alias = "$government_Cooperative;")]
    Cooperative,
    #[serde(alias = "$government_Corporate;")]
    Corporate,
    #[serde(alias = "$government_Democracy;")]
    Democracy,
    #[serde(alias = "$government_Dictatorship;")]
    Dictatorship,
    #[serde(alias = "$government_Feudal;")]
    Feudal,
    #[serde(alias = "$government_Patronage;")]
    Patronage,
    #[serde(alias = "$government_Prison;")]
    Prison,
    #[serde(alias = "$government_PrisonColony;")]
    PrisonColony,
    #[serde(alias = "$government_Theocracy;")]
    Theocracy,
    #[serde(alias = "$government_Engineer;")]
    Engineer,
    #[serde(alias = "$government_Carrier;")]
    Carrier,
    #[serde(rename = "")]
    #[serde(alias = "$government_None;")]
    None,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum Allegiance {
    Alliance,
    Empire,
    Federation,
    Guardian,
    Independent,
    PilotsFederation,
    PlayerPilots,
    Thargoid,
    #[serde(rename = "")]
    None,
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
