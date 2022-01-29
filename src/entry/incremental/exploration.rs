use serde::Deserialize;
use crate::prelude::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BuyExplorationData {
    /// System name of purchased data
    pub system: String,
    /// Cost for system data
    pub cost : u64,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Material {
    name: String,
    percent: f64,
}

pub enum ScanType {
    Basic,
    Detailed,
    NavBeacon,
    NavBeaconDetail,
    AutoScan
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Scan {
    pub scan_type: String,
    #[serde(flatten)]
    pub body: Body,
    pub materials: Option<Vec<Material>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SAASignalsFound {
    /// The target body for the surface scan
    pub body: Body,
    /// Detected nearby signals
    pub signals: Vec<Signal>
}
