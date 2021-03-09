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
//! included in the broader [`entry::incremental::Event`] `enum`.
//
// https://github.com/launchbadge/sqlx/issues/657#issuecomment-774040177
#![allow(unused_braces)]

use serde::Deserialize;

/// All shared data models used throughout the events
///
/// To use this module effectivly add the following `use` statement to your files:
///
/// ```
/// use elite_journal::prelude::*;
/// ```
///
/// Often you'll also want to import the [`Entry`] and various [`entry`] types as well, for
/// example:
///
/// ```
/// use elite_journal::{prelude::*, entry::{Entry, Event}};
/// ```
pub mod prelude;

pub mod entry;
pub use self::entry::Entry;

pub mod system;
pub mod faction;
pub mod station;

/// Serde helper deserializers
pub mod de;


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
