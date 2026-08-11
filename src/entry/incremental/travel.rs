use crate::prelude::*;
use serde::Deserialize;

/// Coming up on a settlement, which is a station on a planet's surface
///
/// The only station-bearing event that says where on the body the station is.
/// Everything else in orbit needs no such thing, and a settlement is not
/// findable without it.
///
/// Names the settlement `Name` rather than `StationName`, which is why the
/// station cannot simply be flattened in the way [`Docked`] and [`Location`]
/// flatten theirs.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ApproachSettlement {
    pub name: String,
    #[serde(rename = "MarketID")]
    pub market_id: Option<i64>,

    #[serde(rename = "StarSystem")]
    pub system_name: String,
    pub star_pos: Coordinate,
    pub system_address: i64,

    #[serde(rename = "BodyID")]
    pub body_id: i16,
    pub body_name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,

    #[serde(rename = "StationFaction")]
    pub faction: Option<Faction>,
    #[serde(rename = "StationGovernment")]
    pub government: Option<Government>,
    #[serde(rename = "StationAllegiance")]
    pub allegiance: Option<Allegiance>,
    #[serde(rename = "StationServices")]
    pub services: Option<Vec<Service>>,
    #[serde(rename = "StationEconomies")]
    pub economies: Option<Vec<EconomyShare>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ApproachBody {
    // TODO: Unify names in simply syntax/format.
    #[serde(rename = "StarSystem")]
    pub system_name: String,
    #[serde(rename = "Body")]
    pub name: String,
}

/// The market a docking event is about
///
/// `MarketID`, and PascalCase makes `MarketId`, which is a different field
/// and is never sent. Every docking event below named it that way and so
/// read none of the messages it was written for; [`Docked`] escaped only
/// because it flattens a [`Station`], where the name is spelled out by hand.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DockingRequested {
    pub station_name: String,
    pub station_type: Option<String>,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    /// How many pads of each size the station has
    ///
    /// A count per size, not one size. `PadSize` was what this asked for and
    /// the game has never sent it: the field is `{"Small":19,"Medium":21,
    /// "Large":9}`, which is what [`LandingPads`] is. Optional because the
    /// game did not always send it at all.
    pub landing_pads: Option<LandingPads>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DockingGranted {
    pub station_name: String,
    /// Optional in the schema, and sent by most but not all
    pub station_type: Option<String>,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub landing_pad: Option<u8>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DockingDenied {
    pub station_name: String,
    /// Optional in the schema, and sent by most but not all
    pub station_type: Option<String>,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub reason: DockingDeniedReason,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DockingCancelled {
    pub station_name: String,
    pub station_type: Option<String>,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DockingTimeout {
    pub station_name: String,
    pub station_type: Option<String>,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Docked {
    // NOTE: Should really be Some(false) when parsed locally. EDDN filters this field.
    pub active_fine: Option<bool>,
    pub system_address: i64,
    #[serde(rename = "StarSystem")]
    pub system_name: String,
    #[serde(flatten)]
    pub station: Station,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Undocked {
    pub station_name: String,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FsdTarget {
    pub system_address: i64,
    pub name: String,
    pub star_class: String, // TODO: Enum?
    #[serde(rename = "RemainingJumpsInRoute")]
    pub remaining: Option<u16>,
}

#[derive(Deserialize, Debug)]
pub enum FsdJumpType {
    Hyperspace,
    Supercruise,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StartJump {
    #[serde(rename = "JumpType")]
    pub ty: FsdJumpType,
    pub system_address: String,
    #[serde(rename = "StarSystem")]
    pub system_name: String,
    pub star_class: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseEntry {
    #[serde(rename = "StarSystem")]
    pub system_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseExit {
    #[serde(rename = "StarSystem")]
    pub system_name: String,
    pub body_id: i16,
    pub body: String,
    pub body_type: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FsdJump {
    #[serde(flatten)]
    pub system: System,
    #[serde(flatten)]
    pub cost: Option<JumpCost>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LeaveBody {
    // TODO: Unify names in simply syntax/format.
    #[serde(rename = "StarSystem")]
    pub system_name: String,
    #[serde(rename = "Body")]
    pub name: String,

    pub system_address: i64,
    pub body_id: i16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Liftoff {
    #[serde(rename = "StarSystem")]
    pub system_name: String,
    #[serde(rename = "Body")]
    pub body_name: String,
    pub body_id: i16,
    pub latitude: f64,
    pub longitude: f64,
    pub on_station: bool,
    pub on_planet: bool,
    pub nearest_destination: String,
    pub player_controlled: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Touchdown {
    pub system_address: i64,
    #[serde(rename = "StarSystem")]
    pub system_name: String,
    pub body_name: String,
    #[serde(rename = "BodyID")]
    pub body_id: i16,
    pub latitude: f64,
    pub longitude: f64,
    pub on_station: bool,
    pub on_planet: bool,
    pub nearest_destination: String,
    pub player_controlled: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    #[serde(flatten)]
    pub system: System,
    #[serde(flatten)]
    pub body: Option<Body>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub docked: bool,
    #[serde(flatten)]
    pub station: Option<Station>,
}

/// Arriving in a system aboard a fleet carrier
///
/// A carrier jump is a system visit and says everything about the system that
/// arriving under your own power does: where it is, who holds it, what it
/// trades in, which factions are at each other's throats. The carrier is a
/// station and the commander is docked at it the whole way, so a station comes
/// with it too.
///
/// Told apart from [`Location`] only by `docked`, which the game began sending
/// partway through the event's life and which nothing here needs. A carrier
/// jump that predates it is still a carrier jump.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierJump {
    #[serde(flatten)]
    pub system: System,
    #[serde(flatten)]
    pub body: Option<Body>,
    pub docked: Option<bool>,
    #[serde(flatten)]
    pub station: Option<Station>,
}
