use serde::Deserialize;
use crate::{
    de,
    Nullable,
    Government,
    Allegiance,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Faction {
    pub name: String,
    #[serde(rename = "FactionState")]
    #[serde(default)]
    #[serde(deserialize_with = "de::enum_is_null")]
    pub state: Option<State>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FactionInfo {
    pub name: String,
    #[serde(rename = "FactionState")]
    #[serde(deserialize_with = "de::enum_is_null")]
    pub state: Option<State>,
    pub government: Government,
    pub influence: f32,
    pub allegiance: Allegiance,
    #[serde(deserialize_with = "de::enum_is_null")]
    pub happiness: Option<Happiness>,
    #[serde(default)]
    pub pending_states: Vec<StateTrend>,
    #[serde(default)]
    pub active_states: Vec<StateTrend>,
    #[serde(default)]
    pub recovering_states: Vec<StateTrend>,
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
pub struct StateTrend {
    pub state: State,
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


#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[serde(rename_all = "PascalCase")]
pub enum State {
    Blight,
    Boom,
    Bust,
    #[serde(alias = "Civil Liberty")]
    CivilLiberty,
    #[serde(alias = "Civil Unrest")]
    CivilUnrest,
    #[serde(alias = "Civil War")]
    CivilWar,
    #[serde(alias = "Cold War")]
    ColdWar,
    Colonisation,
    Drought,
    Election,
    Expansion,
    Famine,
    #[serde(alias = "Historic Event")]
    HistoricEvent,
    #[serde(alias = "Infrastructure Failure")]
    InfrastructureFailure,
    Investment,
    Lockdown,
    #[serde(alias = "Natural Disaster")]
    NaturalDisaster,
    Outbreak,
    #[serde(alias = "Pirate Attack")]
    PirateAttack,
    #[serde(alias = "Public Holiday")]
    PublicHoliday,
    Retreat,
    Revolution,
    #[serde(alias = "Technological Leap")]
    TechnologicalLeap,
    #[serde(alias = "Terrorist Attack")]
    Terrorism,
    #[serde(alias = "Trade War")]
    TradeWar,
    War,
    #[serde(alias = "")]
    None
}

impl Nullable for State {
    fn is_null(&self) -> bool {
        match self {
            State::None => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
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

impl Nullable for Happiness {
    fn is_null(&self) -> bool {
        match self {
            Happiness::None => true,
            _ => false,
        }
    }
}
