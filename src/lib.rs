//! # Elite: Dangerous Player Journal(s)
//!
//! As documented in detail in the [readthedocs.io reference][rtd], which is parsed from the
//! official "Journal Manual", there are a number of files which the game itself updates and
//! third-party tools consume. Most notable, [EDDN][eddn] syncs a subset of the game's journals
//! from players running client tools like [Elite: Dangerous Discovery][edd] or [Elite: Dangerous
//! Market Connector][edmc]. See our [`eddn` crate][eddn_crate] for more information.
//!
//! Every [`Entry`] in the Elite Dangerous journal and status files will have at least the
//! following fields:
//!
//! - `timestamp`
//! - `event`
//!
//! The parser matchs on the `event` to determine the rest of the fields in the object. Status
//! files other than the main incremental journals each only contain a single event type, and are
//! therefor not included in the broader [`entry::Event`] `enum`.
//!
//! - Use [`parse_journal_file`] and [`parse_journal_dir`] for `*.log` journal files
//! - Use [`parse_status_file`] for `*.json` status files
//!
//! [rtd]: https://elite-journal.readthedocs.io/en/latest
//! [eddn]: https://eddn.edcd.io
//! [eddn_crate]: https://github.com/ED-NEWP/eddn
//! [edd]: https://github.com/EDDiscovery/EDDiscovery
//! [edmc]: https://github.com/EDCD/EDMarketConnector
//
// https://github.com/launchbadge/sqlx/issues/657#issuecomment-774040177
#![allow(unused_braces)]

use serde::Deserialize;
use self::de::Nullable;

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

/// Journal and status file entries
pub mod entry;
pub use self::entry::{
    parse_status_file,
    parse_journal_file,
    parse_journal_dir,
    Entry
};

/// A star system, located in static 3D space
pub mod system;

/// Galaxtic factions who occupy systems and participate in the game's background simulation
pub mod faction;

// TODO:
// - Are all stations dockable?
// - Do all stations have market ids? What about one's without the market service?
// - What about all the other types in EDDB's station types dropdown?
pub mod station;

/// Serde helper deserializers
pub mod de;


/// System and faction's organizational structure
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
    #[serde(alias = "Prison colony")]
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


/// System and faction's alignment to the broader groups
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
