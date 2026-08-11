use crate::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "with-sqlx")]
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use std::fmt;

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
    #[serde(default)]
    #[serde(deserialize_with = "crate::de::empty_map_is_none")]
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
        write!(
            f,
            "Large: {}, Medium: {}, Small: {}",
            self.large, self.medium, self.small
        )
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
    /// A station standing on a planet, which is not the same as a crater port
    SurfaceStation,
    /// An Odyssey settlement, walked around rather than landed in
    OnFootSettlement,
    /// The eight sided orbital, which the game spells in full nowhere
    Dodec,
    // Where a colonisation project is built up, in orbit and on the ground.
    SpaceConstructionDepot,
    PlanetaryConstructionDepot,
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum DockingDeniedReason {
    NoSpace,
    TooLarge,
    Hostile,
    Offences,
    Distance,
    ActiveFighter,
    NoReason,
    RestrictedAccess,
    /// Spelled `DockingUnavliable` by the game, which is what is matched on
    #[serde(rename = "DockingUnavliable")]
    DockingUnavailable,
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
    /// Where a system is claimed for colonisation
    #[serde(rename = "registeringcolonisation")]
    RegisteringColonisation,
    /// Where materials are handed over to a colonisation project
    #[serde(rename = "colonisationcontribution")]
    ColonisationContribution,
    /// A mission offered on docking rather than from the mission board
    #[serde(rename = "ondockmission")]
    OnDockMission,
    #[serde(rename = "squadronBank")]
    SquadronBank,
    #[serde(rename = "refinery")]
    Refinery,
    /// The carrier's own shop, as against the services it runs for others
    #[serde(rename = "carriervendor")]
    CarrierVendor,
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

#[test]
fn service() {
    let read = |json: &str| {
        serde_json::from_str::<Service>(json)
            .unwrap_or_else(|e| panic!("{} should read: {}", json, e))
    };

    assert_eq!(Service::Dock, read(r#""dock""#));
    // Lowercase and unbroken, as every service is, however many words the
    // name is made of.
    assert_eq!(
        Service::RegisteringColonisation,
        read(r#""registeringcolonisation""#)
    );
    assert_eq!(
        Service::ColonisationContribution,
        read(r#""colonisationcontribution""#)
    );
    assert_eq!(Service::OnDockMission, read(r#""ondockmission""#));
    // Camel cased, as a handful of them are and most are not.
    assert_eq!(Service::SquadronBank, read(r#""squadronBank""#));
    assert_eq!(Service::Refinery, read(r#""refinery""#));
    assert_eq!(Service::CarrierVendor, read(r#""carriervendor""#));
}

/// The kinds of station the game docks at and lands on
///
/// Every one of these was sent by the live feed while going unread, and a
/// station whose kind would not read took its whole message with it.
#[test]
fn station_type() {
    let read = |json: &str| {
        serde_json::from_str::<StationType>(json)
            .unwrap_or_else(|e| panic!("{} should read: {}", json, e))
    };

    assert_eq!(StationType::Coriolis, read(r#""Coriolis""#));
    assert_eq!(StationType::SurfaceStation, read(r#""SurfaceStation""#));
    assert_eq!(StationType::OnFootSettlement, read(r#""OnFootSettlement""#));
    assert_eq!(StationType::Dodec, read(r#""Dodec""#));
    assert_eq!(
        StationType::SpaceConstructionDepot,
        read(r#""SpaceConstructionDepot""#)
    );
    assert_eq!(
        StationType::PlanetaryConstructionDepot,
        read(r#""PlanetaryConstructionDepot""#)
    );
}

/// Why docking was refused, as the game spells each reason
#[test]
fn docking_denied_reason() {
    let read = |json: &str| {
        serde_json::from_str::<DockingDeniedReason>(json)
            .unwrap_or_else(|e| panic!("{} should read: {}", json, e))
    };

    assert_eq!(DockingDeniedReason::NoSpace, read(r#""NoSpace""#));
    assert_eq!(
        DockingDeniedReason::RestrictedAccess,
        read(r#""RestrictedAccess""#)
    );
    // The game's own spelling, which is not ours to correct on the wire.
    assert_eq!(
        DockingDeniedReason::DockingUnavailable,
        read(r#""DockingUnavliable""#)
    );
}
