use std::cmp::Ordering;
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

    #[serde(deserialize_with = "de::zero_is_none")]
    pub population: Option<u64>,
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

#[test]
fn system() {
    let system = serde_json::from_str::<System>(r#"
        {
            "StarPos": [123.321, 1337.42, 0.0],
            "StarSystem": "Somewhere",
            "SystemAddress": 1928374650,
            "Population": 0,
            "SystemSecurity": "",
            "SystemGovernment": "",
            "SystemAllegiance": "",
            "SystemEconomy": "",
            "SystemSecondEconomy": ""
        }
    "#).unwrap();
    assert_eq!(0., system.pos.z);
    assert_eq!(None, system.population);
    assert_eq!(None, system.security);
    assert_eq!(None, system.government);
    assert_eq!(None, system.allegiance);
    assert_eq!(None, system.economy);
    assert_eq!(None, system.second_economy);
}


#[derive(Deserialize, Debug, Copy, Clone)]
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

impl PartialEq for Security {
    fn eq(&self, other: &Self) -> bool {
        match (*other, *self) {
            (Security::None, Security::None) => false,
            (l, r) => l as u8 == r as u8,
        }
    }
}

impl PartialOrd for Security {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*other, *self) {
            (_, Security::None) | (Security::None, _) => None,
            (l, r) => (l as u8).partial_cmp(&(r as u8))
        }
    }
}

#[test]
fn security() {
    let high = serde_json::from_str(r#"
        "$SYSTEM_SECURITY_high;"
    "#).unwrap();
    assert_eq!(Security::High, high);
    let low = serde_json::from_str(r#"
        "$GAlAXY_MAP_INFO_state_low;"
    "#).unwrap();
    assert_eq!(Security::Low, low);
    let anarchy = serde_json::from_str(r#"
        "Anarchy"
    "#).unwrap();
    assert_eq!(Security::Anarchy, anarchy);
    assert!(anarchy.is_null());
    let none = serde_json::from_str(r#""""#).unwrap();
    assert!(anarchy != none);
    assert!(Security::None != none);
    assert!(none.is_null());
    assert!(high > low);
    assert!(low > anarchy);
    assert!(!(anarchy > none));
    assert!(!(anarchy < none));
}

// TODO: test
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
