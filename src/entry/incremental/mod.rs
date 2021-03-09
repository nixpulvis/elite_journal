//! # Incremental Player Journal
//!
//! The `Journal.<datestamp>.<part>.log` files store a list of events in JSON lines format.
//! The means that each line is a complete JSON object.
//!
//! Each incremental journal file will begin with a [`Fileheader`][incremental::Event::Fileheader]
//! event which in addition to some other metadata, also contains the `part` of the log. This, in
//! addition to the ubiquitous `timestamp` makes parsing the filename unnecessary. For more
//! information on each [`Event`] read their individual documentation.
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;
use std::ffi::OsStr;
use serde::Deserialize;
use crate::Entry;

// "AfmuRepairs"
// "ApproachBody"
// "ApproachSettlement"
// "Bounty"
// "BuyAmmo"
// "BuyDrones"
// "BuyTradeData"
// "Cargo"
// "CargoDepot"
// "CargoTransfer"
// "CarrierBankTransfer"
// "CarrierCrewServices"
// "CarrierDepositFuel"
// "CarrierDockingPermission"
// "CarrierFinance"
// "CarrierJump"
// "CarrierJumpCancelled"
// "CarrierJumpRequest"
// "CarrierModulePack"
// "CarrierNameChange"
// "CarrierShipPack"
// "CarrierStats"
// "CarrierTradeOrder"
// "ChangeCrewRole"
// "CockpitBreached"
// "CodexEntry"
// "CollectCargo"
// "Commander"
// "CommitCrime"
// "CommunityGoal"
// "CommunityGoalDiscard"
// "CommunityGoalJoin"
// "CommunityGoalReward"
// "CrewLaunchFighter"
// "CrewMemberJoins"
// "CrewMemberQuits"
// "CrewMemberRoleChange"
// "CrimeVictim"
// "DatalinkScan"
// "DatalinkVoucher"
// "DataScanned"
// "Died"
// "DiscoveryScan"
// "Docked"
// "DockFighter"
// "DockingCancelled"
// "DockingDenied"
// "DockingGranted"
// "DockingRequested"
// "DockSRV"
// "EjectCargo"
// "EndCrewSession"
// "EngineerContribution"
// "EngineerCraft"
// "EngineerProgress"
// "EscapeInterdiction"
// "FactionKillBond"
// "FetchRemoteModule"
// "FighterDestroyed"
// "FighterRebuilt"
// "Fileheader"
// "Friends"
// "FSDJump"
// "FSDTarget"
// "FSSAllBodiesFound"
// "FSSDiscoveryScan"
// "FSSSignalDiscovered"
// "FuelScoop"
// "HeatDamage"
// "HeatWarning"
// "HullDamage"
// "Interdicted"
// "JetConeBoost"
// "JoinACrew"
// "LaunchDrone"
// "LaunchFighter"
// "LaunchSRV"
// "LeaveBody"
// "Liftoff"
// "LoadGame"
// "Loadout"
// "Location"
// "Market"
// "MarketBuy"
// "MarketSell"
// "MassModuleStore"
// "MaterialCollected"
// "MaterialDiscovered"
// "Materials"
// "MaterialTrade"
// "MiningRefined"
// "MissionAbandoned"
// "MissionAccepted"
// "MissionCompleted"
// "MissionFailed"
// "MissionRedirected"
// "Missions"
// "ModuleBuy"
// "ModuleInfo"
// "ModuleRetrieve"
// "ModuleSell"
// "ModuleSellRemote"
// "ModuleStore"
// "ModuleSwap"
// "MultiSellExplorationData"
// "Music"
// "NavBeaconScan"
// "NavRoute"
// "Outfitting"
// "Passengers"
// "PayBounties"
// "PayFines"
// "Powerplay"
// "PowerplayCollect"
// "PowerplayDeliver"
// "PowerplayFastTrack"
// "PowerplayJoin"
// "PowerplayLeave"
// "PowerplaySalary"
// "PowerplayVoucher"
// "Progress"
// "Promotion"
// "ProspectedAsteroid"
// "PVPKill"
// "QuitACrew"
// "Rank"
// "RebootRepair"
// "ReceiveText"
// "RedeemVoucher"
// "RefuelAll"
// "RefuelPartial"
// "Repair"
// "RepairAll"
// "RepairDrone"
// "Reputation"
// "ReservoirReplenished"
// "RestockVehicle"
// "Resurrect"
// "SAAScanComplete"
// "SAASignalsFound"
// "Scan"
// "Scanned"
// "ScientificResearch"
// "Screenshot"
// "SellDrones"
// "SellExplorationData"
// "SendText"
// "SetUserShipName"
// "ShieldState"
// "ShipTargeted"
// "Shipyard"
// "ShipyardBuy"
// "ShipyardNew"
// "ShipyardSell"
// "ShipyardSwap"
// "ShipyardTransfer"
// "Shutdown"
// "SquadronStartup"
// "SRVDestroyed"
// "StartJump"
// "Statistics"
// "StoredModules"
// "StoredShips"
// "SupercruiseEntry"
// "SupercruiseExit"
// "Synthesis"
// "TechnologyBroker"
// "Touchdown"
// "UnderAttack"
// "Undocked"
// "USSDrop"
// "VehicleSwitch"
// "WingAdd"
// "WingInvite"
// "WingJoin"
// "WingLeave"


/// Information provided by the player journal
#[derive(Deserialize, Debug)]
#[serde(tag = "event")]
pub enum Event {
    Fileheader(startup::Fileheader),
    Cargo(startup::Manifest),
    NewCommander(startup::NewCommander),
    ClearSavedGame(startup::Commander),
    Commander(startup::Commander),
    LoadGame(startup::LoadGame),
    Materials(startup::Materials),

    Docked(travel::Docked),
    #[serde(rename = "FSDTarget")]
    FsdTarget(travel::FsdTarget),
    #[serde(rename = "FSDJump")]
    FsdJump(travel::FsdJump),
    Location(travel::Location),
    /// Signals an update to the [`NavRoute.json`][crate::entry::route] file
    NavRoute,

    BuyExplorationData(exploration::BuyExplorationData),
    SellExplorationData(exploration::SellExplorationData),

    // TODO: We'll leave this in for sure... but we should test without it, and probably find a way
    // to add a generic JSON value to this.
    // https://github.com/serde-rs/serde/issues/1973
    #[serde(other)]
    Other,
}

// TODO: Our own error types.
// TODO: add result inside vec too.
// TODO: This should be an interator, since files are updates as the game runs.
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Vec<Entry<Event>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().map(|line| {
        serde_json::from_str(&line.unwrap()).unwrap()
    }).collect())
}

// TODO: Our own error types.
// TODO: add result inside vec too.
// TODO: is there a way to stream this too? search for current running log files?
pub fn parse_dir<P: AsRef<Path>>(path: P) -> Result<Vec<Entry<Event>>, Box<dyn Error>> {
    let mut entries = Vec::new();
    for entry in read_dir(path)? {
        let entry = entry?;
        if entry.file_type().unwrap().is_file() &&
           entry.path().extension().and_then(OsStr::to_str) == Some("log")
        {
            entries.append(&mut parse_file(entry.path())?);
        }
    }
    Ok(entries)
}

pub mod startup;
pub mod travel;
pub mod exploration;
