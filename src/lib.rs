//! # Elite: Dangerous Player Journal(s)
//!
//! As documented in detail the [readthedocs.io reference](https://elite-journal.readthedocs.io/en/latest),
//! there are a number of files which the game itself updates and third-party tools sync through
//! [EDDN](https://eddn.edcd.io) and other tools.
//!
//! - `Journal.<datestamp>.<part>.log` - [Incremental Player Journal](#incremental-player-journal)
//! - `Status.json` TODO
//! - `NavRoute.json` TODO
//! - `Market.json` TODO
//! - `Shipyard.json` TODO
//! - `Outfitting.json` TODO
//! - `ModulesInfo.json` TODO
//!
//! # Incremental Player Journal
//!
//! The `Journal.<datestamp>.<part>.log` files store a list of events in JSON lines format.
//! The means that each line is a complete JSON object. Specifically, each object will have at
//! least the following fields:
//!
//! - `timestamp`
//! - `event`
//!
//! Matching on the `event` will determain the rest of the fields in the object.
//!
//! Each journal file will begin with a `Fileheader` event which in addition to some other
//! metadata, also contains the `part` of the log. This, in addition to the ubiquitous `timestamp`
//! makes parsing the filename unnecessary.

use serde::{Deserialize, Deserializer};
use chrono::prelude::*;

/// A single log entry, containing an [`Event`]
#[derive(Deserialize, Debug)]
pub struct Entry<E> {
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub event: E,
}

#[derive(Deserialize, Debug)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

mod journal;
pub use self::journal::{
    parse_file,
    parse_dir,
    travel,
    startup,
    Event};

pub mod nav_route;

fn string_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() || s.ends_with("_None;") {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}
