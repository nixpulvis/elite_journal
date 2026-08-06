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
    /// The target body for the surface scan
    pub body: Body,
    /// Detected nearby signals
    pub signals: Vec<Signal>,
}
