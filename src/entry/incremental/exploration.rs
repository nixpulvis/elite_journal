use crate::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BuyExplorationData {
    /// System name of purchased data
    pub system: String,
    /// Cost for system data
    pub cost: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SellExplorationData {
    /// List of system names which were discovered
    pub systems: Vec<String>,
    /// List of **body** names which were discovered
    pub discovered: Vec<String>,
    /// Credit value for the discoveries
    pub base_value: u64,
    /// Credit bonus for efficency
    /// TODO: Are there other ways to get a bonus?
    pub bonus: u64,
    /// Total credit value, `base_value` + `bonus` + other factors
    ///
    /// Other factors are things like the 200% Li Yong Rui bonus.
    pub total_earnings: u64,
}

pub enum ScanType {
    Basic,
    Detailed,
    NavBeacon,
    NavBeaconDetail,
    AutoScan,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ScanTarget {
    Star(Star),
    Body(Body),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Scan {
    pub scan_type: String,
    pub star_system: String,
    pub star_pos: Coordinate,
    pub system_address: i64,
    #[serde(flatten)]
    pub target: ScanTarget,

    #[serde(flatten)]
    pub other: serde_json::Value,
}

/// The honk: what a system holds, counted before any of it is identified
///
/// The first thing done on arriving somewhere, and the only event that says
/// how much there is to find. Everything else describes what has been found.
///
/// Names the system `SystemName` where nearly every other event calls it
/// `StarSystem`. It is not a mistake in the schema, and a struct that assumes
/// otherwise reads nothing.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FssDiscoveryScan {
    #[serde(rename = "SystemName")]
    pub system_name: String,
    pub star_pos: Coordinate,
    pub system_address: i64,

    /// Bodies in the system: stars, planets, moons
    pub body_count: i32,
    /// Everything else the honk finds, being belts and rings
    ///
    /// Never appears in a body table, here or in the game's own, because
    /// none of it is a body.
    pub non_body_count: i32,
}

/// Every body in a system found, which fixes the count as certain
///
/// Says the same thing [`FssDiscoveryScan`] does and says it having finished:
/// the honk's count is what the sensors made of the system on arrival, this
/// is the tally once every one of them has been resolved.
///
/// Names the system `SystemName`, as [`FssDiscoveryScan`] does.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FssAllBodiesFound {
    #[serde(rename = "SystemName")]
    pub system_name: String,
    pub star_pos: Coordinate,
    pub system_address: i64,

    /// Bodies in the system, all of them now accounted for
    pub count: i32,
}

/// A nav beacon read, which hands over the system's body count for free
///
/// The count is the same quantity [`FssDiscoveryScan`] reports, arrived at by
/// reading a beacon rather than by honking. Unlike those two this event names
/// the system `StarSystem`, as most events do.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NavBeaconScan {
    pub star_system: String,
    pub star_pos: Coordinate,
    pub system_address: i64,

    /// Bodies in the system
    pub num_bodies: i32,
}

/// The center of mass a close pair goes round, scanned as a body in its own
/// right
///
/// [`None`] for the orbit where the barycenter goes round nothing, which is
/// what the one at the root of a multi-star system comes back as.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ScanBaryCentre {
    pub star_system: String,
    pub star_pos: Coordinate,
    pub system_address: i64,
    #[serde(rename = "BodyID")]
    pub body_id: i16,
    #[serde(flatten)]
    pub orbit: Option<Orbit>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SAASignalsFound {
    /// The body the surface scan was of, named and numbered
    ///
    /// Not a [`Body`]. A scan of a body says what it is made of and how it
    /// moves; this says which body signals were found on and nothing else
    /// about it, so there is no body here to describe. Asking for one is
    /// what stopped this event from being read at all: every message failed
    /// on a missing `Body` field that the game has never sent.
    #[serde(rename = "BodyName")]
    pub body_name: String,
    #[serde(rename = "BodyID")]
    pub body_id: i16,

    pub star_system: String,
    pub star_pos: Coordinate,
    pub system_address: i64,

    /// Detected nearby signals
    pub signals: Vec<Signal>,
}
