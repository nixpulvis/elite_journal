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
pub use self::faction::{Faction, FactionInfo, FactionStateTrend, FactionConflict, FactionConflictProgress};

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
#[derive(Deserialize, Debug)]
pub struct Entry<E> {
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub event: E,
}

pub trait Nullable {
    fn is_null(&self) -> bool;
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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
