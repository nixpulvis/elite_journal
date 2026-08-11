use crate::entry::route::NavRoute;
use serde::Deserialize;

// "AfmuRepairs"
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
// "CarrierJumpCancelled"
// "CarrierJumpRequest"
// "CarrierModulePack"
// "CarrierNameChange"
// "CarrierShipPack"
// "CarrierStats"
// "CarrierTradeOrder"
// "ChangeCrewRole"
// "CockpitBreached"
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

    // #[serde(rename = "Docked")]
    Docked(travel::Docked),

    DockingGranted(travel::DockingGranted),
    DockingRequested(travel::DockingRequested),
    DockingDenied(travel::DockingDenied),
    DockingCancelled(travel::DockingCancelled),
    DockingTimeout(travel::DockingTimeout),
    Undocked(travel::Undocked),

    Liftoff(travel::Liftoff),
    LeaveBody(travel::LeaveBody),
    ApproachBody(travel::ApproachBody),
    ApproachSettlement(travel::ApproachSettlement),

    #[serde(rename = "FSDTarget")]
    FsdTarget(travel::FsdTarget),
    #[serde(rename = "FSDJump")]
    FsdJump(travel::FsdJump),
    CarrierJump(travel::CarrierJump),

    /// Signals an update to the [`NavRoute.json`][crate::entry::route] file
    NavRoute(NavRoute),

    BuyExplorationData(exploration::BuyExplorationData),
    SellExplorationData(exploration::SellExplorationData),
    SAASignalsFound(exploration::SAASignalsFound),
    Scan(exploration::Scan),
    ScanBaryCentre(exploration::ScanBaryCentre),

    /// How much there is in a system, as against what has been found in it
    #[serde(rename = "FSSDiscoveryScan")]
    FssDiscoveryScan(exploration::FssDiscoveryScan),
    #[serde(rename = "FSSAllBodiesFound")]
    FssAllBodiesFound(exploration::FssAllBodiesFound),
    NavBeaconScan(exploration::NavBeaconScan),

    /// What is in a system besides its bodies, and what is written on them
    #[serde(rename = "FSSBodySignals")]
    FssBodySignals(exploration::FssBodySignals),
    #[serde(rename = "FSSSignalDiscovered")]
    FssSignalDiscovered(exploration::FssSignalDiscovered),
    CodexEntry(exploration::CodexEntry),

    // TODO: We'll leave this in for sure... but we should test without it, and probably find a way
    // to add a generic JSON value to this.
    // https://github.com/serde-rs/serde/issues/1973
    #[serde(other)]
    Other,
}

pub mod exploration;
pub mod startup;
pub mod travel;

/// Every event read here, read from the shape EDDN actually sends
///
/// Written against the schemas rather than against the structs. A struct that
/// disagrees with the game about a field name parses nothing, and says so
/// nowhere: the event simply never matches and is filed as
/// [`Event::Other`] forever. That is not a failure any amount of running the
/// thing will show you, which is what these are for.
///
/// <https://github.com/EDCD/EDDN/tree/master/schemas>
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::Entry;

    /// Read a whole message the way the consumer does, tag and all
    fn event(json: &str) -> Event {
        serde_json::from_str::<Entry<Event>>(json)
            .expect("message should parse")
            .event
    }

    /// Anything landing on `Other` was not read, whatever else was true of it
    fn assert_read(json: &str) -> Event {
        let event = event(json);
        assert!(
            !matches!(event, Event::Other),
            "fell through to Other: {}",
            json,
        );
        event
    }

    /// `journal/1`, and the five events on it that were already read
    #[test]
    fn the_journal_schema() {
        assert!(matches!(
            assert_read(
                r#"{
                    "timestamp": "2026-08-08T12:00:00Z",
                    "event": "FSDJump",
                    "StarSystem": "Sol",
                    "StarPos": [0.0, 0.0, 0.0],
                    "SystemAddress": 10477373803
                }"#
            ),
            Event::FsdJump(_)
        ));

        assert!(matches!(
            assert_read(
                r#"{
                    "timestamp": "2026-08-08T12:00:00Z",
                    "event": "Location",
                    "StarSystem": "Sol",
                    "StarPos": [0.0, 0.0, 0.0],
                    "SystemAddress": 10477373803,
                    "Docked": false
                }"#
            ),
            Event::Location(_)
        ));
    }

    /// A carrier jump, which says as much about a system as arriving does
    ///
    /// Was a TODO on the enum and went unread, taking a full picture of a
    /// system with it every time a carrier moved.
    #[test]
    fn a_carrier_jump_is_a_visit() {
        let Event::CarrierJump(jump) = assert_read(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "event": "CarrierJump",
                "StarSystem": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "SystemAllegiance": "Federation",
                "SystemEconomy": "$economy_Refinery;",
                "Population": 22780919531,
                "Docked": true,
                "StationName": "K7Q-BQL",
                "StationType": "FleetCarrier",
                "MarketID": 3700571136
            }"#,
        ) else {
            panic!("not a carrier jump")
        };

        assert_eq!(jump.system.name, "Sol");
        assert_eq!(jump.system.population, Some(22780919531));
        assert_eq!(jump.docked, Some(true));
        assert_eq!(
            jump.station.as_ref().map(|s| s.name.as_str()),
            Some("K7Q-BQL"),
        );
    }

    /// `Docked` arrived partway through the event's life, so it is optional
    #[test]
    fn a_carrier_jump_without_docked() {
        let Event::CarrierJump(jump) = assert_read(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "event": "CarrierJump",
                "StarSystem": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803
            }"#,
        ) else {
            panic!("not a carrier jump")
        };

        assert_eq!(jump.docked, None);
    }

    /// The surface scan, which asked for a `Body` the game has never sent
    ///
    /// Every one of these failed on the missing field and was dropped. The
    /// event names the body and numbers it, and says nothing else about it.
    #[test]
    fn a_surface_scan_names_its_body_rather_than_describing_it() {
        let Event::SAASignalsFound(found) = assert_read(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "event": "SAASignalsFound",
                "StarSystem": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "BodyName": "Sol 4",
                "BodyID": 12,
                "Signals": [
                    { "Type": "$SAA_SignalType_Geological;", "Count": 3 },
                    { "Type": "$SAA_SignalType_Biological;", "Count": 1 }
                ]
            }"#,
        ) else {
            panic!("not a surface scan")
        };

        assert_eq!(found.body_name, "Sol 4");
        assert_eq!(found.body_id, 12);
        assert_eq!(found.signals.len(), 2);
        assert_eq!(found.signals[0].count, 3);
    }

    /// The honk, which names its system `SystemName` and not `StarSystem`
    #[test]
    fn the_honk_counts_bodies_and_the_rest() {
        let Event::FssDiscoveryScan(honk) = assert_read(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "event": "FSSDiscoveryScan",
                "SystemName": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "BodyCount": 40,
                "NonBodyCount": 10,
                "horizons": true,
                "odyssey": true
            }"#,
        ) else {
            panic!("not a honk")
        };

        assert_eq!(honk.system_name, "Sol");
        assert_eq!(honk.body_count, 40);
        assert_eq!(honk.non_body_count, 10);
    }

    /// The all-found tally, which also says `SystemName`
    #[test]
    fn every_body_found_is_the_same_count() {
        let Event::FssAllBodiesFound(found) = assert_read(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "event": "FSSAllBodiesFound",
                "SystemName": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "Count": 40
            }"#,
        ) else {
            panic!("not an all-bodies-found")
        };

        assert_eq!(found.count, 40);
    }

    /// A beacon reports the same number, and calls its system `StarSystem`
    ///
    /// The naming really does differ between these three. Reading them all
    /// the same way is what does not work.
    #[test]
    fn a_beacon_names_its_system_the_usual_way() {
        let Event::NavBeaconScan(scan) = assert_read(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "event": "NavBeaconScan",
                "StarSystem": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "NumBodies": 40
            }"#,
        ) else {
            panic!("not a beacon scan")
        };

        assert_eq!(scan.star_system, "Sol");
        assert_eq!(scan.num_bodies, 40);
    }

    /// Body signals from orbit, whose body name is optional
    #[test]
    fn the_honk_finds_signals_on_bodies() {
        let Event::FssBodySignals(found) = assert_read(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "event": "FSSBodySignals",
                "StarSystem": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "BodyID": 12,
                "Signals": [
                    { "Type": "$SAA_SignalType_Geological;", "Count": 3 }
                ]
            }"#,
        ) else {
            panic!("not body signals")
        };

        assert_eq!(found.body_id, 12);
        assert_eq!(found.body_name, None);
        assert_eq!(found.signals.len(), 1);
    }

    /// System signals, which arrive a batch at a time
    ///
    /// The outer timestamp belongs to the first signal only. Each carries its
    /// own, spelled lowercase where every other field of the event is not.
    #[test]
    fn system_signals_arrive_in_batches() {
        let Event::FssSignalDiscovered(found) = assert_read(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "event": "FSSSignalDiscovered",
                "StarSystem": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "signals": [
                    {
                        "timestamp": "2026-08-08T12:00:00Z",
                        "SignalName": "Abraham Lincoln",
                        "IsStation": true
                    },
                    {
                        "timestamp": "2026-08-08T12:05:00Z",
                        "SignalName": "$USS_HighGradeEmissions;",
                        "SignalType": "USS",
                        "USSType": "$USS_Type_VeryValuableSalvage;",
                        "SpawningState": "$FactionState_None;",
                        "SpawningFaction": "Mother Gaia",
                        "ThreatLevel": 0
                    }
                ]
            }"#,
        ) else {
            panic!("not signals discovered")
        };

        assert_eq!(found.signals.len(), 2);
        assert_eq!(found.signals[0].signal_name, "Abraham Lincoln");
        assert_eq!(found.signals[0].is_station, Some(true));
        assert_eq!(
            found.signals[1].uss_type.as_deref(),
            Some("$USS_Type_VeryValuableSalvage;"),
        );
        assert_eq!(found.signals[1].threat_level, Some(0));

        // The two signals were seen five minutes apart, and writing both
        // under the message's stamp would lose that.
        assert_ne!(found.signals[0].timestamp, found.signals[1].timestamp);
    }

    /// A codex sighting, which names its system `System` and nothing else does
    #[test]
    fn a_codex_sighting_names_its_system_its_own_way() {
        let Event::CodexEntry(entry) = assert_read(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "event": "CodexEntry",
                "System": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "EntryID": 2100701,
                "Name": "$Codex_Ent_Sulphur_Name;",
                "Category": "$Codex_Category_Biology;",
                "SubCategory": "$Codex_SubCategory_Organic_Structures;",
                "Region": "$Codex_RegionName_18;",
                "BodyID": 12,
                "BodyName": "Sol 4"
            }"#,
        ) else {
            panic!("not a codex entry")
        };

        assert_eq!(entry.system_name, "Sol");
        assert_eq!(entry.entry_id, 2100701);
        assert_eq!(entry.body_id, Some(12));
    }

    /// A settlement, named `Name` where a station is named `StationName`
    #[test]
    fn a_settlement_is_a_station_somewhere_on_a_body() {
        let Event::ApproachSettlement(settlement) = assert_read(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "event": "ApproachSettlement",
                "Name": "Bloomfield Vision Installation",
                "MarketID": 3510085376,
                "StarSystem": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "BodyID": 12,
                "BodyName": "Sol 4",
                "Latitude": 12.5,
                "Longitude": -47.25,
                "StationAllegiance": "Federation",
                "StationGovernment": "$government_Corporate;",
                "StationServices": ["dock", "refuel"]
            }"#,
        ) else {
            panic!("not a settlement")
        };

        assert_eq!(settlement.name, "Bloomfield Vision Installation");
        assert_eq!(settlement.body_name, "Sol 4");
        assert_eq!(settlement.latitude, Some(12.5));
        assert_eq!(settlement.longitude, Some(-47.25));
        assert_eq!(settlement.market_id, Some(3510085376));
    }

    /// An event nothing here reads is filed as `Other` rather than refused
    ///
    /// Which is right, and is also why a struct that quietly disagrees with
    /// the game is invisible: it lands in exactly the same place.
    #[test]
    fn an_unread_event_is_other() {
        assert!(matches!(
            event(
                r#"{
                    "timestamp": "2026-08-08T12:00:00Z",
                    "event": "ProspectedAsteroid",
                    "Content": "$AsteroidMaterialContent_High;",
                    "Remaining": 100.0
                }"#
            ),
            Event::Other
        ));
    }

    /// The docking events that never reach EDDN, read as journal lines
    ///
    /// EDDN has schemas for two of the six -- granted and denied -- so the
    /// other four arrive only in a commander's own `Journal.<stamp>.log`,
    /// which is what `parse_journal_file` reads. That is the whole of the
    /// difference: they are the same events written by the same game, and
    /// there is nothing about them that cannot be read here.
    ///
    /// All four were failing on `MarketId`, a field the game does not send.
    #[test]
    fn the_docking_events_that_only_reach_a_journal_file() {
        let requested = assert_read(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "event": "DockingRequested",
                "MarketID": 3226577920,
                "StationName": "Ray Gateway",
                "StationType": "Coriolis",
                "LandingPads": { "Small": 19, "Medium": 21, "Large": 9 }
            }"#,
        );

        let Event::DockingRequested(requested) = requested else {
            panic!("not a docking request")
        };
        assert_eq!(requested.market_id, 3226577920);

        // A count per size, which is what the game sends. Asking for one
        // size instead read none of these at all.
        let pads = requested.landing_pads.expect("pads should read");
        assert_eq!(pads.small, 19);
        assert_eq!(pads.medium, 21);
        assert_eq!(pads.large, 9);

        assert!(matches!(
            assert_read(
                r#"{
                    "timestamp": "2026-08-08T12:00:00Z",
                    "event": "DockingCancelled",
                    "MarketID": 3226577920,
                    "StationName": "Ray Gateway",
                    "StationType": "Coriolis"
                }"#
            ),
            Event::DockingCancelled(_)
        ));

        assert!(matches!(
            assert_read(
                r#"{
                    "timestamp": "2026-08-08T12:00:00Z",
                    "event": "DockingTimeout",
                    "MarketID": 3226577920,
                    "StationName": "Ray Gateway",
                    "StationType": "Coriolis"
                }"#
            ),
            Event::DockingTimeout(_)
        ));
    }

    /// Undocking, which had a struct and no way to reach it
    ///
    /// `Undocked` was written and never added to this enum, so every one of
    /// them fell through to `Other` no matter what the struct said.
    #[test]
    fn undocking_is_read() {
        let Event::Undocked(undocked) = assert_read(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "event": "Undocked",
                "StationName": "Ray Gateway",
                "MarketID": 3226577920,
                "Taxi": false,
                "Multicrew": false
            }"#,
        ) else {
            panic!("not an undocking")
        };

        assert_eq!(undocked.station_name, "Ray Gateway");
        assert_eq!(undocked.market_id, 3226577920);
    }
}
