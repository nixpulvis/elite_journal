use crate::body::Discovery;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer};
use std::collections::BTreeMap as Map;

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

/// What a `Scan` turned out to be about
#[derive(Debug)]
pub enum ScanTarget {
    Star(Star),
    Body(Body),
    Cluster(Cluster),
    Ring(Ring),
}

impl<'de> Deserialize<'de> for ScanTarget {
    /// Read the field that tells the four apart, then read that one variant
    ///
    /// `#[serde(untagged)]` instead tries each in turn and, when none fits,
    /// reports only that none fitted. Which field of which variant was wrong
    /// it does not say, and a scan is thirty fields wide.
    ///
    /// It is also unsound here. A cluster asks for the little that every scan
    /// carries, so under `untagged` it would accept a star that had failed its
    /// own variant over a single missing field, and the star would be filed as
    /// a stretch of belt.
    ///
    /// Each of the four is asked for by something it has rather than by
    /// something it lacks: a star carries `StarType`, a planet `PlanetClass`, a
    /// cluster lies in a ring and names it as the nearest of its parents, and a
    /// ring carries an orbit and nothing of substance. A scan answering to none
    /// of them is reported, since a shape nobody has modelled stored as the
    /// nearest thing to hand is worse than a shape nobody has modelled said out
    /// loud.
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let scan = serde_json::Value::deserialize(de)?;

        if scan.get("StarType").is_some() {
            Star::deserialize(scan).map(ScanTarget::Star)
        } else if scan.get("PlanetClass").is_some() {
            Body::deserialize(scan).map(ScanTarget::Body)
        } else if lies_in_a_ring(&scan) {
            Cluster::deserialize(scan).map(ScanTarget::Cluster)
        } else if carries_only_an_orbit(&scan) {
            Ring::deserialize(scan).map(ScanTarget::Ring)
        } else {
            return Err(de::Error::custom(format!(
                "a scan of no kind read here: {}",
                scan.get("BodyName")
                    .and_then(|name| name.as_str())
                    .unwrap_or("something unnamed")
            )));
        }
        .map_err(de::Error::custom)
    }
}

/// Whether the nearest thing a scan hangs off is a ring
///
/// What a belt cluster is: a stretch of one of the rings a star or a planet
/// carries. The game says so in the first of its parents, nearest first, and
/// says it of nothing else.
fn lies_in_a_ring(scan: &serde_json::Value) -> bool {
    scan.get("Parents")
        .and_then(|parents| parents.get(0))
        .is_some_and(|nearest| nearest.get("Ring").is_some())
}

/// Whether a scan says where a thing goes and nothing about the thing
///
/// What a ring is: a name, an id, what it goes round, and the path. A star and
/// a planet each carry what they are made of as well, and of that a radius and
/// a rotation are the two every one of them has. So a scan carrying either is
/// not a ring however much else it is missing, and a planet that arrived
/// without its class is reported rather than filed as a path.
fn carries_only_an_orbit(scan: &serde_json::Value) -> bool {
    scan.get("SemiMajorAxis").is_some()
        && scan.get("Radius").is_none()
        && scan.get("RotationPeriod").is_none()
}

/// A ring, scanned in its own right rather than as something a body carries
///
/// A body's own scan lists the rings it has, with what they are made of and how
/// wide they are. This is the other way the game reports one: as a body in the
/// numbering, going round the body it belongs to, carrying an orbit and nothing
/// else. It is the orbit that tells it from a belt cluster, which lies in a ring
/// and has none.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Ring {
    #[serde(rename = "BodyName")]
    pub name: String,
    #[serde(rename = "BodyID")]
    pub id: i16,
    /// The body it goes round, nearest first
    #[serde(default)]
    pub parents: Vec<Map<String, i16>>,
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival: Option<f32>,
    #[serde(flatten)]
    pub orbit: Orbit,
    #[serde(flatten)]
    pub discovery: Discovery,
}

/// A belt cluster, which is scanned as a body and has none of a body's figures
///
/// A quarter of the scans EDDN carries are these. No class, mass, radius or
/// temperature, because there is no single object there to measure: it is a
/// stretch of a belt, named for the ring it belongs to and numbered among the
/// system's bodies.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Cluster {
    #[serde(rename = "BodyName")]
    pub name: String,
    #[serde(rename = "BodyID")]
    pub id: i16,
    /// The ring it lies in, and what that ring goes round
    #[serde(default)]
    pub parents: Vec<Map<String, i16>>,
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival: Option<f32>,
    #[serde(flatten)]
    pub discovery: Discovery,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Scan {
    /// How close a look was taken, where the sender says
    ///
    /// [`None`] because not every uploader sends it, and nothing here reads it.
    pub scan_type: Option<String>,
    pub star_system: String,
    pub star_pos: Coordinate,
    pub system_address: i64,
    #[serde(flatten)]
    pub target: ScanTarget,

    #[serde(flatten)]
    pub other: serde_json::Value,
}

/// Signals read off a body from orbit, which the honk finds
///
/// The same kinds and counts [`SAASignalsFound`] reports, seen from further
/// off: the honk finds them, a surface scan is what maps them. Either may
/// arrive first, and either may arrive for a body nothing has scanned.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FssBodySignals {
    #[serde(rename = "BodyName")]
    pub body_name: Option<String>,
    #[serde(rename = "BodyID")]
    pub body_id: i16,

    pub star_system: String,
    pub star_pos: Coordinate,
    pub system_address: i64,

    pub signals: Vec<Signal>,
}

/// Everything hanging in a system that is not a body
///
/// Stations, megaships, installations, beacons, and the unidentified sources
/// that come and go. Sent in batches: the game emits one of these per signal
/// and EDDN gathers a system's worth into a single message, so the outer
/// timestamp is the first signal's and each signal carries its own.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FssSignalDiscovered {
    pub star_system: Option<String>,
    pub star_pos: Option<Coordinate>,
    pub system_address: i64,

    #[serde(rename = "signals")]
    pub signals: Vec<SystemSignal>,
}

/// One signal out of an [`FssSignalDiscovered`] batch
///
/// Only the name is certain. What kind of thing it is, who spawned it and how
/// dangerous it is are all told where the game bothered to say, which depends
/// on what the signal is.
///
/// How long it has left is never told. The journal carries it and the schema
/// disallows it, so a signal that has since despawned is indistinguishable
/// from one still there apart from how long ago this was sent.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SystemSignal {
    /// Spelled lowercase, alone among the fields of this event
    ///
    /// Which is what the schema says, and the rest of the event is
    /// PascalCase, so it has to be held out of the renaming by hand.
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    pub signal_name: String,
    pub signal_type: Option<String>,
    /// Permanent where [`Some(true)`], which is as near an expiry as there is
    pub is_station: Option<bool>,
    #[serde(rename = "USSType")]
    pub uss_type: Option<String>,
    pub spawning_state: Option<String>,
    pub spawning_faction: Option<String>,
    pub spawning_power: Option<String>,
    pub opposing_power: Option<String>,
    pub threat_level: Option<i32>,
}

/// A codex sighting: a kind of thing, found somewhere
///
/// Names the system `System`, which no other event does. Whether the sender
/// was first to it is not here and cannot be -- the schema disallows it as
/// personal data -- so this says a thing was found, not that it was
/// discovered.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CodexEntry {
    #[serde(rename = "System")]
    pub system_name: String,
    pub star_pos: Coordinate,
    pub system_address: i64,

    #[serde(rename = "EntryID")]
    pub entry_id: i64,
    /// Not required by the schema, though always sent in practice
    pub name: Option<String>,
    pub category: Option<String>,
    pub sub_category: Option<String>,
    pub region: Option<String>,

    #[serde(rename = "BodyID")]
    pub body_id: Option<i16>,
    pub body_name: Option<String>,
    pub nearest_destination: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
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
