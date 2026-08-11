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
    use crate::entry::incremental::exploration::ScanTarget;
    use crate::entry::Entry;
    use crate::system::Economy;

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

    /// A belt cluster is scanned as a body and measured as nothing
    ///
    /// About a quarter of the scans EDDN carries are these, and every field a
    /// star or a planet is described by is missing from them. Read as neither
    /// of those, the whole message went unread and the system it names went
    /// unrecorded with it.
    #[test]
    fn a_belt_cluster_is_scanned_without_being_measured() {
        let Event::Scan(scan) = assert_read(
            r#"{
                "timestamp": "2026-08-11T18:00:00Z",
                "event": "Scan",
                "ScanType": "AutoScan",
                "StarSystem": "Eol Prou JH-C d13-55",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "BodyName": "Eol Prou JH-C d13-55 A Belt Cluster 5",
                "BodyID": 5,
                "Parents": [{ "Ring": 1 }, { "Star": 0 }],
                "DistanceFromArrivalLS": 0.0,
                "WasDiscovered": true,
                "WasMapped": false
            }"#,
        ) else {
            panic!("not a scan")
        };

        let ScanTarget::Cluster(cluster) = scan.target else {
            panic!("not a cluster")
        };

        assert_eq!(cluster.name, "Eol Prou JH-C d13-55 A Belt Cluster 5");
        assert_eq!(cluster.id, 5);
        assert!(cluster.discovery.discovered);
        assert!(!cluster.discovery.mapped);
        // The ring it lies in, then what that ring goes round.
        assert_eq!(cluster.parents.len(), 2);
        assert_eq!(cluster.parents[0].get("Ring"), Some(&1));
    }

    /// The cluster variant stands last, so it takes only what is left
    ///
    /// It asks for the few fields every scan carries, which a star and a planet
    /// both have. Were it first it would swallow both.
    #[test]
    fn a_star_and_a_planet_are_read_as_themselves() {
        let Event::Scan(star) = assert_read(
            r#"{
                "timestamp": "2026-08-11T18:00:00Z",
                "event": "Scan",
                "ScanType": "AutoScan",
                "StarSystem": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "BodyName": "Sol",
                "BodyID": 0,
                "Parents": [],
                "StarType": "G",
                "Subclass": 2,
                "StellarMass": 1.0,
                "Radius": 695700000.0,
                "AbsoluteMagnitude": 4.83,
                "Age_MY": 4600,
                "SurfaceTemperature": 5778.0,
                "Luminosity": "V",
                "DistanceFromArrivalLS": 0.0,
                "RotationPeriod": 2164000.0,
                "AxialTilt": 0.126,
                "WasDiscovered": true,
                "WasMapped": false
            }"#,
        ) else {
            panic!("not a scan")
        };
        assert!(matches!(star.target, ScanTarget::Star(_)));

        let Event::Scan(planet) = assert_read(
            r#"{
                "timestamp": "2026-08-11T18:00:00Z",
                "event": "Scan",
                "ScanType": "Detailed",
                "StarSystem": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "BodyName": "Earth",
                "BodyID": 3,
                "Parents": [{ "Star": 0 }],
                "PlanetClass": "Earthlike body",
                "TidalLock": false,
                "MassEM": 1.0,
                "Radius": 6371000.0,
                "SurfaceGravity": 9.807,
                "SurfaceTemperature": 288.0,
                "SemiMajorAxis": 149000000000.0,
                "Eccentricity": 0.0167,
                "OrbitalInclination": 0.0,
                "Periapsis": 114.2,
                "OrbitalPeriod": 31500000.0,
                "AscendingNode": 0.0,
                "MeanAnomaly": 0.0,
                "RotationPeriod": 86164.0,
                "AxialTilt": 0.409,
                "DistanceFromArrivalLS": 499.0,
                "WasDiscovered": true,
                "WasMapped": true
            }"#,
        ) else {
            panic!("not a scan")
        };
        assert!(matches!(planet.target, ScanTarget::Body(_)));
    }

    /// A star missing a field is reported, not filed as a belt cluster
    ///
    /// The cluster asks for almost nothing, so trying the variants in turn
    /// would hand it every star that fell short of its own. Dispatching on
    /// `StarType` instead means the star is read as a star and the missing
    /// field is named.
    #[test]
    fn a_star_short_a_field_says_which_field() {
        let json = r#"{
            "timestamp": "2026-08-11T18:00:00Z",
            "event": "Scan",
            "ScanType": "AutoScan",
            "StarSystem": "Sol",
            "StarPos": [0.0, 0.0, 0.0],
            "SystemAddress": 10477373803,
            "BodyName": "Sol",
            "BodyID": 0,
            "Parents": [],
            "StarType": "G",
            "Subclass": 2,
            "StellarMass": 1.0,
            "Radius": 695700000.0,
            "AbsoluteMagnitude": 4.83,
            "Age_MY": 4600,
            "SurfaceTemperature": 5778.0,
            "DistanceFromArrivalLS": 0.0,
            "RotationPeriod": 2164000.0,
            "AxialTilt": 0.126,
            "WasDiscovered": true,
            "WasMapped": false
        }"#;

        let err = serde_json::from_str::<Entry<Event>>(json)
            .expect_err("a star without its luminosity should not read");

        assert!(
            err.to_string().contains("Luminosity"),
            "said nothing about the missing field: {}",
            err,
        );
    }

    /// A basic scan reports no surface temperature
    ///
    /// `ScanType` says how close a look was taken, and a basic one leaves out
    /// figures a detailed one carries. Gas giants are where it shows.
    #[test]
    fn a_basic_scan_carries_no_temperature() {
        let Event::Scan(scan) = assert_read(
            r#"{
                "timestamp": "2026-08-11T19:38:40Z",
                "event": "Scan",
                "ScanType": "Basic",
                "BodyName": "Blo Eurl TI-K d8-176 A 1",
                "BodyID": 5,
                "Parents": [{ "Star": 1 }, { "Null": 0 }],
                "StarSystem": "Blo Eurl TI-K d8-176",
                "StarPos": [3403.84375, 106.25, 4807.75],
                "SystemAddress": 6058512763723,
                "DistanceFromArrivalLS": 566.304246,
                "PlanetClass": "Sudarsky class V gas giant",
                "MassEM": 816.8172,
                "Radius": 71228168,
                "SurfaceGravity": 64.169796,
                "SemiMajorAxis": 111697691679.00085,
                "Eccentricity": 0.644541,
                "OrbitalInclination": -162.793724,
                "Periapsis": 241.507528,
                "OrbitalPeriod": 22660554.051399,
                "AscendingNode": -75.687522,
                "MeanAnomaly": 121.948236,
                "RotationPeriod": 287282.874636,
                "AxialTilt": 0.159411,
                "WasDiscovered": true,
                "WasMapped": false
            }"#,
        ) else {
            panic!("not a scan")
        };

        let ScanTarget::Body(body) = scan.target else { panic!("not a body") };

        assert_eq!(body.name, "Blo Eurl TI-K d8-176 A 1");
        assert_eq!(body.temperature, None);
        // Everything a basic scan does carry is still read.
        assert_eq!(body.planet_class, "Sudarsky class V gas giant");
        assert_eq!(body.gravity, 64.169796);
    }

    /// A scan without a scan type reads
    ///
    /// Not every uploader sends `ScanType`, and nothing here reads it. A star,
    /// a planet and a belt cluster all arrive from one such sender.
    #[test]
    fn a_scan_without_a_scan_type_reads() {
        let scan = |json: &str| {
            let Event::Scan(scan) = assert_read(json) else {
                panic!("not a scan")
            };
            assert!(scan.scan_type.is_none());
            scan.target
        };

        assert!(matches!(
            scan(
                r#"{
                "timestamp": "2026-08-11T19:44:56Z",
                "event": "Scan",
                "BodyName": "Synuefe DO-F d12-54 A Belt Cluster 1",
                "BodyID": 2,
                "Parents": [{ "Ring": 1 }, { "Star": 0 }],
                "StarSystem": "Synuefe DO-F d12-54",
                "StarPos": [196.1875, -210.65625, 25.125],
                "SystemAddress": 1865953528171,
                "DistanceFromArrivalLS": 7.693174,
                "WasDiscovered": true,
                "WasMapped": false
            }"#
            ),
            ScanTarget::Cluster(_)
        ));

        assert!(matches!(
            scan(
                r#"{
                "timestamp": "2026-08-11T19:44:56Z",
                "event": "Scan",
                "BodyName": "Synuefe DO-F d12-54",
                "BodyID": 0,
                "StarSystem": "Synuefe DO-F d12-54",
                "StarPos": [196.1875, -210.65625, 25.125],
                "SystemAddress": 1865953528171,
                "DistanceFromArrivalLS": 0,
                "StarType": "F",
                "Subclass": 7,
                "StellarMass": 1.238281,
                "Radius": 738883776,
                "AbsoluteMagnitude": 4.245636,
                "Age_MY": 516,
                "SurfaceTemperature": 6413,
                "Luminosity": "Vb",
                "RotationPeriod": 395502.084419,
                "AxialTilt": 0,
                "WasDiscovered": true,
                "WasMapped": false
            }"#
            ),
            ScanTarget::Star(_)
        ));
    }

    /// A settlement whose faction is an empty object reads
    ///
    /// The game writes `{}` where a thing is absent as readily as it writes
    /// nothing at all, and an ancient ruin in the black belongs to nobody.
    #[test]
    fn a_settlement_with_an_empty_faction_reads() {
        let Event::ApproachSettlement(settlement) = assert_read(
            r#"{
                "timestamp": "2026-08-11T19:39:29Z",
                "event": "ApproachSettlement",
                "StarSystem": "Synuefe NL-N c23-4",
                "StarPos": [860.125, -124.59375, -61.0625],
                "StationFaction": {},
                "SystemAddress": 1184840454858,
                "Name": "$Ancient:#index=3;",
                "BodyID": 18,
                "BodyName": "Synuefe NL-N c23-4 B 3",
                "Latitude": 5.614992,
                "Longitude": -148.089981
            }"#,
        ) else {
            panic!("not a settlement")
        };

        assert_eq!(settlement.name, "$Ancient:#index=3;");
        assert!(settlement.faction.is_none());

        // The same claim spelled longer.
        let Event::ApproachSettlement(named_nothing) = assert_read(
            r#"{
                "timestamp": "2026-08-11T19:39:29Z",
                "event": "ApproachSettlement",
                "StarSystem": "Synuefe NL-N c23-4",
                "StarPos": [860.125, -124.59375, -61.0625],
                "StationFaction": { "Name": null },
                "SystemAddress": 1184840454858,
                "Name": "$Ancient:#index=3;",
                "BodyID": 18,
                "BodyName": "Synuefe NL-N c23-4 B 3",
                "Latitude": 5.614992,
                "Longitude": -148.089981
            }"#,
        ) else {
            panic!("not a settlement")
        };
        assert!(named_nothing.faction.is_none());
    }

    /// A scan answering to none of the four kinds is reported, not stored
    ///
    /// Each kind is asked for by something it has, so a shape carrying none of
    /// those things is not quietly taken for the nearest kind to hand. Nothing
    /// the game sends today looks like this; the point is what happens when
    /// something does.
    #[test]
    fn a_scan_of_no_known_kind_is_reported() {
        let err = serde_json::from_str::<Entry<Event>>(
            r#"{
                "timestamp": "2026-08-11T20:00:00Z",
                "event": "Scan",
                "ScanType": "Detailed",
                "StarSystem": "Sol",
                "StarPos": [0.0, 0.0, 0.0],
                "SystemAddress": 10477373803,
                "BodyName": "Sol 5 Something New",
                "BodyID": 9,
                "Parents": [{ "Planet": 8 }, { "Star": 0 }],
                "DistanceFromArrivalLS": 2.5,
                "WasDiscovered": true,
                "WasMapped": false
            }"#,
        )
        .expect_err("a scan of no known kind should not read");

        let said = err.to_string();
        assert!(
            said.contains("no kind read here"),
            "did not say what was wrong: {}",
            said,
        );
        // And which one it was, since a feed carries thirty a second.
        assert!(
            said.contains("Sol 5 Something New"),
            "did not name it: {}",
            said,
        );
    }

    /// A cluster is taken by the ring it lies in
    ///
    /// The nearest of its parents, which is what the game says of a belt
    /// cluster and of nothing else.
    #[test]
    fn a_cluster_is_taken_by_the_ring_it_lies_in() {
        let Event::Scan(scan) = assert_read(
            r#"{
                "timestamp": "2026-08-11T19:44:56Z",
                "event": "Scan",
                "ScanType": "AutoScan",
                "BodyName": "Synuefe UF-K b55-4 A A Belt Cluster 5",
                "BodyID": 10,
                "Parents": [{ "Ring": 5 }, { "Star": 1 }, { "Null": 0 }],
                "StarSystem": "Synuefe UF-K b55-4",
                "StarPos": [386.75, -220.125, 125.25],
                "SystemAddress": 9472147072473,
                "DistanceFromArrivalLS": 5.433045,
                "WasDiscovered": true,
                "WasMapped": false
            }"#,
        ) else {
            panic!("not a scan")
        };

        let ScanTarget::Cluster(cluster) = scan.target else {
            panic!("a cluster in a ring should be read as one")
        };
        assert_eq!(cluster.id, 10);
        assert_eq!(cluster.parents[0].get("Ring"), Some(&5));
    }

    /// A ring scanned in its own right is read as one
    ///
    /// Taken from the feed. It goes round a planet and carries the orbit to
    /// prove it, which is what tells it from a belt cluster lying in a ring.
    #[test]
    fn a_ring_scanned_in_its_own_right_is_read_as_one() {
        let Event::Scan(scan) = assert_read(
            r#"{
                "timestamp": "2026-08-11T21:16:29Z",
                "event": "Scan",
                "ScanType": "AutoScan",
                "BodyName": "Dryeejeae AA-A d4 D 12 A Ring",
                "BodyID": 65,
                "Parents": [{ "Planet": 64 }, { "Star": 6 }, { "Null": 0 }],
                "StarSystem": "Dryeejeae AA-A d4",
                "StarPos": [-8982.125, 1258.21875, 10460.625],
                "SystemAddress": 146037542275,
                "DistanceFromArrivalLS": 377022.119004,
                "SemiMajorAxis": 36267683.804035,
                "Eccentricity": 0,
                "OrbitalInclination": 0,
                "Periapsis": 0,
                "OrbitalPeriod": 15402.967334,
                "AscendingNode": 0,
                "MeanAnomaly": 166.00566,
                "WasDiscovered": false,
                "WasMapped": false
            }"#,
        ) else {
            panic!("not a scan")
        };

        let ScanTarget::Ring(ring) = scan.target else {
            panic!("a ring should not be read as anything else")
        };
        assert_eq!(ring.name, "Dryeejeae AA-A d4 D 12 A Ring");
        assert_eq!(ring.id, 65);
        // It goes round the planet, not in a ring, which is the whole
        // difference.
        assert_eq!(ring.parents[0].get("Planet"), Some(&64));
        assert_eq!(ring.orbit.orbital_period, 15402.967334);
    }

    /// An orbit short of its last two still reads
    ///
    /// Taken from the feed. The game sends all seven; `Stellar Data Relay`
    /// sends five, and the five are the path. Where the body stood along it is
    /// what the other two would have said.
    #[test]
    fn an_orbit_without_its_last_two_still_reads() {
        let Event::Scan(scan) = assert_read(
            r#"{
                "timestamp": "2026-08-11T22:20:15Z",
                "event": "Scan",
                "ScanType": "AutoScan",
                "StarSystem": "Colonia",
                "StarPos": [-9530.5, -910.28125, 19808.125],
                "SystemAddress": 3238296097059,
                "BodyName": "Colonia 7 c",
                "BodyID": 50,
                "Parents": [{ "Star": 44 }, { "Star": 0 }],
                "PlanetClass": "Rocky body",
                "TidalLock": true,
                "MassEM": 0.029988,
                "Radius": 2152178.5,
                "SurfaceGravity": 2.580479,
                "SurfaceTemperature": 128.0793,
                "SurfacePressure": 0,
                "Landable": true,
                "Atmosphere": "",
                "AtmosphereType": "None",
                "Volcanism": "",
                "TerraformState": "",
                "Composition": { "Ice": 0, "Metal": 0.088844, "Rock": 0.911156 },
                "SemiMajorAxis": 10566104650.497437,
                "Eccentricity": 0.000385,
                "OrbitalInclination": -0.010772,
                "Periapsis": 185.317867,
                "OrbitalPeriod": 5153734.087944,
                "RotationPeriod": 5153831.783096,
                "AxialTilt": 0.032617,
                "WasDiscovered": true,
                "WasMapped": true
            }"#,
        ) else {
            panic!("not a scan")
        };

        let ScanTarget::Body(body) = scan.target else { panic!("not a body") };

        // The path is known.
        assert_eq!(body.orbit.semi_major_axis, 10566104650.497437);
        assert_eq!(body.orbit.orbital_period, 5153734.087944);
        // Where it stood along the path is not.
        assert_eq!(body.orbit.ascending_node, None);
        assert_eq!(body.orbit.mean_anomaly, None);
    }

    /// A rescue ship trades under an economy of its own
    ///
    /// A megaship sent to a system whose station has been attacked. Docking at
    /// one carries `$economy_Rescue;`.
    #[test]
    fn a_rescue_ship_docks_under_its_own_economy() {
        let Event::Docked(docked) = assert_read(
            r#"{
                "timestamp": "2026-08-11T22:26:55Z",
                "event": "Docked",
                "StarSystem": "Luyten's Star",
                "StarPos": [6.5625, 2.34375, -10.25],
                "SystemAddress": 7268024264097,
                "StationName": "Rescue Ship Hutner",
                "StationType": "MegaShip",
                "MarketID": 129020287,
                "StationFaction": { "Name": "Independent Rescue Coalition" },
                "StationGovernment": "$government_Corporate;",
                "StationEconomy": "$economy_Rescue;",
                "StationEconomies": [
                    { "Name": "$economy_Rescue;", "Proportion": 1 }
                ],
                "DistFromStarLS": 297.0377,
                "LandingPads": { "Small": 4, "Medium": 2, "Large": 1 }
            }"#,
        ) else {
            panic!("not a docking")
        };

        let economies =
            docked.station.economies.as_ref().expect("an economy share");
        assert_eq!(economies.len(), 1);
        assert_eq!(economies[0].name, Economy::Rescue);
    }

    /// A planet that arrived without its class is reported, not filed as a ring
    ///
    /// A ring is a path and nothing else. This carries a radius, a mass and a
    /// rotation, so whatever it is it is not a path, and being unable to say
    /// what it is beats saying the wrong thing about it.
    #[test]
    fn a_planet_short_of_its_class_is_not_taken_for_a_ring() {
        let err = serde_json::from_str::<Entry<Event>>(
            r#"{
                "timestamp": "2026-08-11T22:20:15Z",
                "event": "Scan",
                "ScanType": "Detailed",
                "StarSystem": "Colonia",
                "StarPos": [-9530.5, -910.28125, 19808.125],
                "SystemAddress": 3238296097059,
                "BodyName": "Colonia 7 c",
                "BodyID": 50,
                "Parents": [{ "Star": 44 }, { "Star": 0 }],
                "MassEM": 0.029988,
                "Radius": 2152178.5,
                "SurfaceGravity": 2.580479,
                "SurfaceTemperature": 128.0793,
                "SemiMajorAxis": 10566104650.497437,
                "Eccentricity": 0.000385,
                "OrbitalInclination": -0.010772,
                "Periapsis": 185.317867,
                "OrbitalPeriod": 5153734.087944,
                "RotationPeriod": 5153831.783096,
                "AxialTilt": 0.032617,
                "WasDiscovered": true,
                "WasMapped": true
            }"#,
        )
        .expect_err("a planet with no class should not read as anything");

        let said = err.to_string();
        assert!(
            said.contains("no kind read here"),
            "did not report it: {}",
            said,
        );
        assert!(said.contains("Colonia 7 c"), "did not name it: {}", said);
    }
}
