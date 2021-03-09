use serde::Deserialize;
use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct Station {
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: Option<f64>,
    #[serde(rename = "StationName")]
    pub name: String,
    #[serde(rename = "StationType")]
    pub ty: StationType,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    #[serde(rename = "StationFaction")]
    pub faction: Faction,
    #[serde(rename = "StationGovernment")]
    pub government: Government,
    #[serde(rename = "StationAllegiance")]
    pub allegiance: Option<Allegiance>,
    #[serde(rename = "StationServices")]
    pub services: Vec<Services>,
    #[serde(rename = "StationEconomies")]
    pub economies: Vec<EconomyShare>,
    // NOTE: Should really be Some(false) when parsed locally. EDDN filters this field.
    pub wanted: Option<bool>,
}


#[derive(Deserialize, Debug)]
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


#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
pub enum Services {
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
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EconomyShare {
    pub name: Economy,
    pub proportion: f64,
}
