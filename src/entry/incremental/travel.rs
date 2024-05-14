use crate::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ApproachBody {
    // TODO: Unify names in simply syntax/format.
    #[serde(rename = "StarSystem")]
    pub system_name: String,
    #[serde(rename = "Body")]
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DockingRequested {
    pub station_name: String,
    pub station_type: String,
    pub market_id: u64,
    pub landing_pads: PadSize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DockingGranted {
    pub station_name: String,
    pub station_type: String,
    pub market_id: u64,
    pub landing_pad: u8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DockingDenied {
    pub station_name: String,
    pub station_type: String,
    pub market_id: u64,
    pub reason: DockingDeniedReason,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DockingCancelled {
    pub station_name: String,
    pub station_type: String,
    pub market_id: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DockingTimeout {
    pub station_name: String,
    pub station_type: String,
    pub market_id: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Docked {
    // NOTE: Should really be Some(false) when parsed locally. EDDN filters this field.
    pub active_fine: Option<bool>,
    #[serde(flatten)]
    pub station: Station,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Undocked {
    pub station_name: String,
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
    pub body_id: i64,
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
