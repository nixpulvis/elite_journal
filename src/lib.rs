//! # Elite: Dangerous Player Journal(s)
//!
//! As documented in detail the [readthedocs.io reference](https://elite-journal.readthedocs.io/en/latest),
//! there are a number of files which the game itself updates and third-party tools sync through
//! [EDDN](https://eddn.edcd.io) and other tools.
//!
//! Every [`Entry`] in the Elite Dangerous journal and status files will have at least the
//! following fields:
//!
//! - `timestamp`
//! - `event`
//!
//! Matching on the `event` will determine the rest of the fields in the object. Files other than
//! the main incremental journal logs each only contain a single event type, and are therefor not
//! included in the broader [`Event`] `enum`.
//!
//! # Incremental Player Journal
//!
//! The `Journal.<datestamp>.<part>.log` files store a list of events in JSON lines format.
//! The means that each line is a complete JSON object.
//!
//! Each incremental journal file will begin with a [`Fileheader`][incremental::Event::Fileheader]
//! event which in addition to some other metadata, also contains the `part` of the log. This, in
//! addition to the ubiquitous `timestamp` makes parsing the filename unnecessary. For more
//! information on each [`Event`] read their individual documentation.
//!
//! # Status File(s)
//!
//! - `Status.json` TODO
//! - `Cargo.json` TODO
//! - `NavRoute.json`
//!
//! # Service and Market Files
//!
//! - `Market.json` TODO
//! - `Shipyard.json` TODO
//! - `Outfitting.json` TODO
//! - `ModulesInfo.json` TODO
//!

// https://github.com/launchbadge/sqlx/issues/657#issuecomment-774040177
#![allow(unused_braces)]

use serde::Deserialize;
use chrono::prelude::*;

/// `Journal.<timestamp>.<part>.log`
pub mod incremental;
pub use self::incremental::Event;

/// `NavRoute.json`
pub mod route;
pub use self::route::Route;

mod coordinate;
pub use self::coordinate::Coordinate;

mod system;
pub use self::system::{System, Security, PowerplayState};

mod faction;
pub use self::faction::{
    Faction,
    FactionInfo,
    FactionConflict,
    FactionConflictProgress,
    State,
    StateTrend,
};

mod station;
pub use self::station::{Station, EconomyShare};

/// Serde helper deserializers
pub mod de;

/// A single timestamped entry, containing an [`Event`], [`Route`], etc.
///
/// - Parse [`Event`]s from `Journal.<timestamp>.<part>.log` files with [`incremental::parse_file`]
/// - TODO `Status.json`
/// or [`incremental::parse_dir`].
/// - Parse [`Route`]s from `NavRoute.json` files with [`route::parse_file`].
/// - TODO `Market.json`
/// - TODO `Shipyard.json`
/// - TODO `Outfitting.json`
/// - TODO `ModulesInfo.json`
#[derive(Deserialize, Debug, PartialEq)]
pub struct Entry<E> {
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub event: E,
}

#[test]
fn entry() {
    #[derive(Deserialize)]
    enum Dumb { Foo }
    assert!(serde_json::from_str::<Entry<Dumb>>(r#"
        {
            "timestamp": "1970-01-01T00:00:00Z",
            "Foo": null
        }
    "#).is_ok());
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    #[allow(unused)]
    struct Dumber {
        key: (),
    }
    assert!(serde_json::from_str::<Entry<Dumber>>(r#"
        {
            "timestamp": "1970-01-01T00:00:00Z",
            "Key": null
        }
    "#).is_ok());
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    #[allow(unused)]
    struct Dumbest {
        key: Government,
    }
    assert!(serde_json::from_str::<Entry<Dumbest>>(r#"
        {
            "timestamp": "1970-01-01T00:00:00Z",
            "Key": "Anarchy"
        }
    "#).is_ok());
}


pub trait Nullable {
    fn is_null(&self) -> bool;
}


#[derive(Deserialize, Debug, Copy, Clone)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
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
    #[serde(alias = "Prison Colony")]
    PrisonColony,
    #[serde(alias = "$government_Theocracy;")]
    Theocracy,
    #[serde(alias = "$government_Engineer;")]
    Engineer,
    #[serde(alias = "$government_Carrier;")]
    Carrier,
    #[serde(alias = "")]
    #[serde(alias = "$government_None;")]
    None,
}

impl Nullable for Government {
    fn is_null(&self) -> bool {
        match self {
            Government::None => true,
            _ => false,
        }
    }
}

impl PartialEq for Government {
    fn eq(&self, other: &Self) -> bool {
        match (*other, *self) {
            (Government::None, Government::None) => false,
            (l, r) => l as u8 == r as u8,
        }
    }
}

#[test]
fn government() {
    let high_tech = serde_json::from_str(r#"
        "Prison Colony"
    "#).unwrap();
    assert_eq!(Government::PrisonColony, high_tech);
    let extraction = serde_json::from_str(r#"
        "$government_Dictatorship;"
    "#).unwrap();
    assert_eq!(Government::Dictatorship, extraction);
    assert!(Government::None != Government::None);
    assert!(serde_json::from_str::<Government>(r#""$government_None;""#).unwrap().is_null());
    assert!(serde_json::from_str::<Government>(r#""None""#).unwrap().is_null());
    assert!(serde_json::from_str::<Government>(r#""""#).unwrap().is_null());
}


#[derive(Deserialize, Debug, Copy, Clone)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[serde(rename_all = "PascalCase")]
pub enum Allegiance {
    Alliance,
    Empire,
    Federation,
    Guardian,
    Independent,
    #[serde(alias = "Pilots Federation")]
    PilotsFederation,
    PlayerPilots,
    Thargoid,
    #[serde(alias = "")]
    None,
}

impl Nullable for Allegiance {
    fn is_null(&self) -> bool {
        match self {
            Allegiance::None => true,
            _ => false,
        }
    }
}

impl PartialEq for Allegiance {
    fn eq(&self, other: &Self) -> bool {
        match (*other, *self) {
            (Allegiance::None, Allegiance::None) => false,
            (l, r) => l as u8 == r as u8,
        }
    }
}

#[test]
fn allegiance() {
    let pilots_federation = serde_json::from_str(r#"
        "Pilots Federation"
    "#).unwrap();
    assert_eq!(Allegiance::PilotsFederation, pilots_federation);
    let player_pilots = serde_json::from_str(r#"
        "PlayerPilots"
    "#).unwrap();
    assert_eq!(Allegiance::PlayerPilots, player_pilots);
    assert!(Allegiance::None != Allegiance::None);
    assert!(serde_json::from_str::<Allegiance>(r#""None""#).unwrap().is_null());
    assert!(serde_json::from_str::<Allegiance>(r#""""#).unwrap().is_null());
}


#[derive(Deserialize, Debug, Copy, Clone)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[serde(rename_all = "PascalCase")]
pub enum Economy {
    #[serde(alias = "$economy_Agri;")]
    Agriculture,
    #[serde(alias = "$economy_Colony;")]
    Colony,
    #[serde(alias = "$economy_Extraction;")]
    Extraction,
    #[serde(alias = "$economy_HighTech;")]
    #[serde(alias = "High Tech")]
    HighTech,
    #[serde(alias = "$economy_Industrial;")]
    Industrial,
    #[serde(alias = "$economy_Military;")]
    Military,
    #[serde(alias = "$economy_Refinery;")]
    Refinery,
    #[serde(alias = "$economy_Service;")]
    Service,
    #[serde(alias = "$economy_Terraforming;")]
    Terraforming,
    #[serde(alias = "$economy_Tourism;")]
    Tourism,
    #[serde(alias = "$economy_Carrier;")]
    Carrier,
    #[serde(alias = "$economy_Prison;")]
    Prison,
    #[serde(alias = "$economy_Undefined;")]
    Undefined,
    #[serde(alias = "")]
    #[serde(alias = "$economy_None;")]
    None,
}

impl Nullable for Economy {
    fn is_null(&self) -> bool {
        match self {
            Economy::None => true,
            _ => false,
        }
    }
}

impl PartialEq for Economy {
    fn eq(&self, other: &Self) -> bool {
        match (*other, *self) {
            (Economy::None, Economy::None) => false,
            (l, r) => l as u8 == r as u8,
        }
    }
}

#[test]
fn economy() {
    let high_tech = serde_json::from_str(r#"
        "High Tech"
    "#).unwrap();
    assert_eq!(Economy::HighTech, high_tech);
    let extraction = serde_json::from_str(r#"
        "$economy_Extraction;"
    "#).unwrap();
    assert_eq!(Economy::Extraction, extraction);
    assert!(Economy::None != Economy::None);
    assert!(serde_json::from_str::<Economy>(r#""$economy_None;""#).unwrap().is_null());
    assert!(serde_json::from_str::<Economy>(r#""None""#).unwrap().is_null());
    assert!(serde_json::from_str::<Economy>(r#""""#).unwrap().is_null());
}
