use std::fmt;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "with-sqlx")]
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

#[derive(Serialize, Deserialize, Debug)]
pub struct Station {
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: Option<f64>,
    #[serde(rename = "StationName")]
    pub name: String,
    #[serde(rename = "StationType")]
    pub ty: Option<StationType>,
    #[serde(rename = "MarketID")]
    pub market_id: Option<i64>,
    #[serde(rename = "LandingPads")]
    pub landing_pads: Option<LandingPads>,
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
    // NOTE: Should really be Some(false) when parsed locally. EDDN filters this field.
    pub wanted: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[serde(rename_all = "PascalCase")]
pub struct LandingPads {
    pub large: i16,
    pub medium: i16,
    pub small: i16,
}

impl fmt::Display for LandingPads {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Large: {}, Medium: {}, Small: {}", self.large, self.medium, self.small)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
pub enum StationType {
    AsteroidBase,
    Coriolis,
    CraterOutpost,
    CraterPort,
    FleetCarrier,
    MegaShip,
    Ocellus,
    Orbis,
    Outpost,
}

impl fmt::Display for StationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum PadSize {
    Small,
    Medium,
    Large,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DockingDeniedReason {
    NoSpace,
    TooLarge,
    Hostile,
    Offences,
    Distance,
    ActiveFighter,
    NoReason,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "with-sqlx", sqlx(no_pg_array))]
pub enum Service {
    #[serde(rename = "autodock")]
    Autodock,
    #[serde(rename = "blackmarket")]
    Blackmarket,
    #[serde(rename = "carrierfuel")]
    CarrierFuel,
    #[serde(rename = "carriermanagement")]
    CarrierManagement,
    #[serde(rename = "commodities")]
    Commodities,
    #[serde(rename = "contacts")]
    Contacts,
    #[serde(rename = "crewlounge")]
    CrewLounge,
    #[serde(rename = "dock")]
    Dock,
    #[serde(rename = "engineer")]
    Engineer,
    #[serde(rename = "exploration")]
    Exploration,
    #[serde(rename = "facilitator")]
    Facilitator,
    #[serde(rename = "flightcontroller")]
    FlightController,
    #[serde(rename = "initiatives")]
    Initiatives,
    #[serde(rename = "materialtrader")]
    MaterialTrader,
    #[serde(rename = "missions")]
    Missions,
    #[serde(rename = "missionsgenerated")]
    MissionsGenerated,
    #[serde(rename = "modulepacks")]
    Modulepacks,
    #[serde(rename = "outfitting")]
    Outfitting,
    #[serde(rename = "powerplay")]
    Powerplay,
    #[serde(rename = "rearm")]
    Rearm,
    #[serde(rename = "refuel")]
    Refuel,
    #[serde(rename = "repair")]
    Repair,
    #[serde(rename = "searchrescue")]
    SearchRescue,
    #[serde(rename = "shipyard")]
    Shipyard,
    #[serde(rename = "shop")]
    Shop,
    #[serde(rename = "stationMenu")]
    StationMenu,
    #[serde(rename = "stationoperations")]
    StationOperations,
    #[serde(rename = "techBroker")]
    TechBroker,
    #[serde(rename = "tuning")]
    Tuning,
    #[serde(rename = "voucherredemption")]
    VoucherRedemption,
    #[serde(rename = "livery")]
    Livery,
    #[serde(rename = "socialspace")]
    SocialSpace,
    #[serde(rename = "bartender")]
    Bartender,
    #[serde(rename = "vistagenomics")]
    VistaGenomics,
    #[serde(rename = "pioneersupplies")]
    PioneerSupplies,
    #[serde(rename = "apexinterstellar")]
    ApexInterstellar,
    #[serde(rename = "frontlinesolutions")]
    FrontlineSolutions,
}

#[cfg(feature = "with-sqlx")]
impl PgHasArrayType for Service {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_service")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "with-sqlx", sqlx(no_pg_array))]
#[serde(rename_all = "PascalCase")]
pub struct EconomyShare {
    pub name: Economy,
    pub proportion: f64,
}

impl Eq for EconomyShare {}

#[cfg(feature = "with-sqlx")]
impl PgHasArrayType for EconomyShare {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_economyshare")
    }
}
