use crate::{de::*, prelude::*};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Faction {
    pub name: String,
    #[serde(rename = "FactionState")]
    #[serde(default)]
    #[serde(deserialize_with = "enum_is_null")]
    pub state: Option<State>,
}

#[test]
fn faction() {
    assert!(serde_json::from_str::<Faction>(
        r#"
        { "Name": "Faction A" }
    "#
    )
    .is_ok());
    let faction = serde_json::from_str::<Faction>(
        r#"
        {
            "Name": "Faction A",
            "FactionState": ""
        }
    "#,
    )
    .unwrap();
    assert_eq!(None, faction.state);
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FactionInfo {
    pub name: String,
    #[serde(rename = "FactionState")]
    #[serde(deserialize_with = "enum_is_null")]
    pub state: Option<State>,
    pub government: Government,
    pub influence: f32,
    pub allegiance: Allegiance,
    #[serde(deserialize_with = "enum_is_null")]
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

#[test]
fn faction_info() {
    let info = serde_json::from_str::<FactionInfo>(
        r#"
        {
            "Name": "Faction A",
            "FactionState": "None",
            "Government": "Democracy",
            "Influence": 77.66,
            "Allegiance": "Pilots Federation",
            "Happiness": "",
            "PendingStates": [],
            "ActiveStates": [],
            "RecoveringStates": []
        }
    "#,
    )
    .unwrap();
    assert_eq!(None, info.state);
    assert_eq!(None, info.happiness);
    assert!(!info.squadron_faction);
    assert!(!info.home_system);
    assert!(!info.happiest_system);
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[serde(rename_all = "PascalCase")]
pub enum State {
    Blight,
    Boom,
    Bust,
    #[serde(alias = "Civil Liberty")]
    #[serde(alias = "Civil liberty")]
    CivilLiberty,
    #[serde(alias = "Civil Unrest")]
    #[serde(alias = "Civil unrest")]
    CivilUnrest,
    #[serde(alias = "Civil War")]
    #[serde(alias = "Civil war")]
    CivilWar,
    #[serde(alias = "Cold War")]
    #[serde(alias = "Cold war")]
    ColdWar,
    Colonisation,
    Drought,
    Election,
    Expansion,
    Famine,
    #[serde(alias = "Historic Event")]
    #[serde(alias = "Historic event")]
    HistoricEvent,
    #[serde(alias = "Infrastructure Failure")]
    #[serde(alias = "Infrastructure failure")]
    InfrastructureFailure,
    Investment,
    Lockdown,
    #[serde(alias = "Natural Disaster")]
    #[serde(alias = "Natural disaster")]
    NaturalDisaster,
    Outbreak,
    #[serde(alias = "Pirate Attack")]
    #[serde(alias = "Pirate attack")]
    PirateAttack,
    #[serde(alias = "Public Holiday")]
    #[serde(alias = "Public holiday")]
    PublicHoliday,
    Retreat,
    Revolution,
    #[serde(alias = "Technological Leap")]
    #[serde(alias = "Technological leap")]
    TechnologicalLeap,
    #[serde(alias = "Terrorist Attack")]
    #[serde(alias = "Terrorist attack")]
    Terrorism,
    #[serde(alias = "Trade War")]
    #[serde(alias = "Trade war")]
    TradeWar,
    War,
    #[serde(alias = "")]
    None,
}

impl Nullable for State {
    fn is_null(&self) -> bool {
        match self {
            State::None => true,
            _ => false,
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        match (*other, *self) {
            (State::None, State::None) => false,
            (l, r) => l as u8 == r as u8,
        }
    }
}

#[test]
fn state() {
    let revolution = serde_json::from_str(
        r#"
        "Revolution"
    "#,
    )
    .unwrap();
    let none = serde_json::from_str(
        r#"
        ""
    "#,
    )
    .unwrap();
    assert_eq!(State::Revolution, revolution);
    assert!(State::None != none);
    assert!(none.is_null());
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "")]
    Recovering,
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (*other as u8).partial_cmp(&(*self as u8))
    }
}

#[test]
fn status() {
    let active = serde_json::from_str(r#""active""#).unwrap();
    assert_eq!(Status::Active, active);
    let pending = serde_json::from_str(r#""pending""#).unwrap();
    assert_eq!(Status::Pending, pending);
    let recovering = serde_json::from_str(r#""""#).unwrap();
    assert_eq!(Status::Recovering, recovering);
    assert!(active > pending && pending > recovering);
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StateTrend {
    pub state: State,
    // TODO: When is this ever not 0?
    #[serde(default)]
    #[serde(deserialize_with = "zero_is_none")]
    pub trend: Option<u64>,
}

#[test]
fn state_trend() {
    assert!(serde_json::from_str::<StateTrend>(
        r#"
        { "State": "Expansion" }
    "#
    )
    .is_ok());
    assert!(serde_json::from_str::<StateTrend>(
        r#"
        { "State": "Expansion", "Trend": null }
    "#
    )
    .is_ok());
    assert_eq!(
        serde_json::from_str::<StateTrend>(
            r#"
        { "State": "Expansion", "Trend": 0 }
    "#
        )
        .unwrap()
        .trend,
        None
    );
    assert_eq!(
        serde_json::from_str::<StateTrend>(
            r#"
        { "State": "Expansion", "Trend": 1 }
    "#
        )
        .unwrap()
        .trend,
        Some(1)
    );
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FactionConflict {
    /// A conflict's type determines what triggers it and the way it is resolved.
    #[serde(rename = "WarType")]
    pub ty: FactionConflictType,
    /// Conflicts (like any other state) have a countdown and cooldown period before and after they
    /// are active.
    ///
    // TODO: Does this match the faction's states vector?
    pub status: Option<Status>,
    /// The defending faction, which had the higher influence before the conflict.
    pub faction_1: FactionConflictProgress,
    /// The attacking faction, which had the lower influence before the conflict.
    pub faction_2: FactionConflictProgress,
}

#[test]
fn conflict() {
    assert!(serde_json::from_str::<FactionConflict>(
        r#"
        {
            "WarType": "civilwar",
            "Status": "active",
            "Faction1": { "Name": "Faction A", "Stake": "Installation X", "WonDays": 2 },
            "Faction2": { "Name": "Faction A", "Stake": "Installation X", "WonDays": 2 }
        }
    "#
    )
    .is_ok());
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "with-sqlx", sqlx(type_name = "Conflict"))]
pub enum FactionConflictType {
    #[serde(rename = "war")]
    War,
    #[serde(rename = "civilwar")]
    CivilWar,
    #[serde(rename = "election")]
    Election,
}

#[test]
fn conflict_type() {
    let war = serde_json::from_str(r#""war""#).unwrap();
    let civil_war = serde_json::from_str(r#""civilwar""#).unwrap();
    let election = serde_json::from_str(r#""election""#).unwrap();
    assert_eq!(FactionConflictType::War, war);
    assert_eq!(FactionConflictType::CivilWar, civil_war);
    assert_eq!(FactionConflictType::Election, election);
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FactionConflictProgress {
    pub name: String,
    pub stake: String,
    pub won_days: u8,
}

#[test]
fn conflict_progress() {
    assert!(serde_json::from_str::<FactionConflictProgress>(
        r#"
        { "Name": "Faction A", "Stake": "Installation X", "WonDays": 2 }
    "#
    )
    .is_ok());
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
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

impl PartialEq for Happiness {
    fn eq(&self, other: &Self) -> bool {
        match (*other, *self) {
            (Happiness::None, Happiness::None) => false,
            (l, r) => l as u8 == r as u8,
        }
    }
}

impl PartialOrd for Happiness {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_null() || other.is_null() {
            return None;
        }
        (*other as u8).partial_cmp(&(*self as u8))
    }
}

#[test]
fn happiness() {
    let elated = serde_json::from_str(
        r#"
        "$Faction_HappinessBand1;"
    "#,
    )
    .unwrap();
    assert_eq!(Happiness::Elated, elated);
    let unhappy = serde_json::from_str(
        r#"
        "$Faction_HappinessBand4;"
    "#,
    )
    .unwrap();
    let none = serde_json::from_str(r#""""#).unwrap();
    assert_eq!(Happiness::Unhappy, unhappy);
    assert!(elated > unhappy);
    assert!(Happiness::None != none);
    assert!(!(elated > Happiness::None));
    assert!(!(elated < Happiness::None));
    assert!(none.is_null());
}
