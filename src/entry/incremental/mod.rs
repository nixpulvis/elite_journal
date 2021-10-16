use serde::Deserialize;
use crate::entry::route::NavRoute;

// "AfmuRepairs"
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
// "DockFighter"
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

    Location(travel::Location),

    Docked(travel::Docked),

    DockingGranted(travel::DockingGranted),
    DockingRequested(travel::DockingRequested),
    DockingDenied(travel::DockingDenied),
    DockingCancelled(travel::DockingCancelled),
    DockingTimeout(travel::DockingTimeout),

    Liftoff(travel::Liftoff),
    LeaveBody(travel::LeaveBody),
    ApproachBody(travel::ApproachBody),

    #[serde(rename = "FSDTarget")]
    FsdTarget(travel::FsdTarget),
    #[serde(rename = "FSDJump")]
    FsdJump(travel::FsdJump),

    /// Signals an update to the [`NavRoute.json`][crate::entry::route] file
    NavRoute(NavRoute),

    BuyExplorationData(exploration::BuyExplorationData),
    SellExplorationData(exploration::SellExplorationData),

    // TODO: We'll leave this in for sure... but we should test without it, and probably find a way
    // to add a generic JSON value to this.
    // https://github.com/serde-rs/serde/issues/1973
    #[serde(other)]
    Other,
}


pub mod startup;
pub mod travel;
pub mod exploration;
