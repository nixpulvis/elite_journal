use crate::{de::*, prelude::*};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

#[cfg(feature = "with-postgis-sqlx")]
use geozero::{
    wkb::{FromWkb, WkbDialect},
    CoordDimensions, GeomProcessor, GeozeroGeometry,
};
#[cfg(feature = "with-postgis-sqlx")]
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct System {
    #[serde(rename = "SystemAddress")]
    pub address: i64,
    #[serde(rename = "StarPos")]
    pub pos: Option<Coordinate>,
    #[serde(rename = "StarSystem")]
    pub name: String,

    #[serde(default)]
    #[serde(deserialize_with = "zero_is_none")]
    pub population: Option<u64>,
    #[serde(rename = "SystemSecurity")]
    #[serde(default)]
    #[serde(deserialize_with = "null_is_none")]
    pub security: Option<Security>,

    #[serde(rename = "SystemGovernment")]
    #[serde(default)]
    #[serde(deserialize_with = "null_is_none")]
    pub government: Option<Government>,
    #[serde(rename = "SystemAllegiance")]
    #[serde(default)]
    #[serde(deserialize_with = "null_is_none")]
    pub allegiance: Option<Allegiance>,
    #[serde(rename = "SystemEconomy")]
    #[serde(default)]
    #[serde(deserialize_with = "null_is_none")]
    pub economy: Option<Economy>,
    #[serde(rename = "SystemSecondEconomy")]
    #[serde(default)]
    #[serde(deserialize_with = "null_is_none")]
    pub second_economy: Option<Economy>,

    #[serde(rename = "SystemFaction")]
    #[serde(default)]
    #[serde(deserialize_with = "crate::de::empty_map_is_none")]
    pub controlling_faction: Option<Faction>,
    #[serde(default)]
    pub factions: Vec<FactionInfo>,
    #[serde(default)]
    pub conflicts: Vec<FactionConflict>,

    pub powers: Option<Vec<Power>>,
    pub powerplay_state: Option<PowerplayState>,
}

impl System {
    pub fn new(address: i64, name: &str) -> Self {
        System {
            address,
            name: name.to_string(),

            pos: None,
            population: None,
            security: None,
            government: None,
            allegiance: None,
            economy: None,
            second_economy: None,
            controlling_faction: None,
            factions: vec![],
            conflicts: vec![],
            powers: None,
            powerplay_state: None,
        }
    }
}

#[test]
fn system() {
    let system = serde_json::from_str::<System>(
        r#"
        {
            "StarPos": [123.321, 1337.42, 0.0],
            "StarSystem": "Somewhere",
            "SystemAddress": 1928374650,
            "Population": 0,
            "SystemSecurity": "",
            "SystemGovernment": "",
            "SystemAllegiance": "",
            "SystemEconomy": "",
            "SystemSecondEconomy": ""
        }
    "#,
    )
    .unwrap();
    assert_eq!(Some(0.), system.pos.map(|p| p.z));
    assert_eq!(None, system.population);
    assert_eq!(None, system.security);
    assert_eq!(None, system.government);
    assert_eq!(None, system.allegiance);
    assert_eq!(None, system.economy);
    assert_eq!(None, system.second_economy);
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[serde(rename_all = "PascalCase")]
pub enum Security {
    #[serde(alias = "$SYSTEM_SECURITY_high;")]
    #[serde(alias = "$GAlAXY_MAP_INFO_state_high;")]
    High,
    #[serde(alias = "$SYSTEM_SECURITY_medium;")]
    #[serde(alias = "$GAlAXY_MAP_INFO_state_medium;")]
    Medium,
    #[serde(alias = "$SYSTEM_SECURITY_low;")]
    #[serde(alias = "$GAlAXY_MAP_INFO_state_low;")]
    Low,
    #[serde(alias = "$SYSTEM_SECURITY_anarchy;")]
    #[serde(alias = "$GAlAXY_MAP_INFO_state_anarchy;")]
    Anarchy,
    #[serde(alias = "")]
    None,
}

impl fmt::Display for Security {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Nullable for Security {
    fn is_null(&self) -> bool {
        match self {
            Security::None => true,
            Security::Anarchy => true,
            _ => false,
        }
    }
}

impl PartialEq for Security {
    fn eq(&self, other: &Self) -> bool {
        match (*other, *self) {
            (Security::None, Security::None) => false,
            (l, r) => l as u8 == r as u8,
        }
    }
}

impl PartialOrd for Security {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*other, *self) {
            (_, Security::None) | (Security::None, _) => None,
            (l, r) => (l as u8).partial_cmp(&(r as u8)),
        }
    }
}

#[test]
fn security() {
    let high = serde_json::from_str(
        r#"
        "$SYSTEM_SECURITY_high;"
    "#,
    )
    .unwrap();
    assert_eq!(Security::High, high);
    let low = serde_json::from_str(
        r#"
        "$GAlAXY_MAP_INFO_state_low;"
    "#,
    )
    .unwrap();
    assert_eq!(Security::Low, low);
    let anarchy = serde_json::from_str(
        r#"
        "Anarchy"
    "#,
    )
    .unwrap();
    assert_eq!(Security::Anarchy, anarchy);
    assert!(anarchy.is_null());
    let none = serde_json::from_str(r#""""#).unwrap();
    assert!(anarchy != none);
    assert!(Security::None != none);
    assert!(none.is_null());
    assert!(high > low);
    assert!(low > anarchy);
    assert!(!(anarchy > none));
    assert!(!(anarchy < none));
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
pub enum PowerplayState {
    InPrepareRadius,
    Prepared,
    Exploited,
    Contested,
    Controlled,
    Turmoil,
    HomeSystem,
    // The standings the second Powerplay added. A system is acquired and then
    // built up, so it stands at one of these rather than merely exploited.
    Unoccupied,
    Fortified,
    Stronghold,
}

#[test]
fn powerplay_state() {
    let read = |json: &str| {
        serde_json::from_str::<PowerplayState>(json)
            .unwrap_or_else(|e| panic!("{} should read: {}", json, e))
    };

    assert_eq!(PowerplayState::Exploited, read(r#""Exploited""#));
    // The second Powerplay's three, which a live feed sends more of than it
    // sends the first Powerplay's.
    assert_eq!(PowerplayState::Unoccupied, read(r#""Unoccupied""#));
    assert_eq!(PowerplayState::Fortified, read(r#""Fortified""#));
    assert_eq!(PowerplayState::Stronghold, read(r#""Stronghold""#));
}

/// A Powerplay power holding or contesting a system
///
/// Spelled as the journal's `Powers` array spells them. Arissa Lavigny-Duval
/// is `A. Lavigny-Duval` in the journal and `Arissa Lavigny-Duval` everywhere
/// the API and the dumps repeat her; both read to the one power. The second
/// Powerplay added Nakato Kaine and Jerome Archer and retired no one from the
/// historical record, so Zachary Hudson stays.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
pub enum Power {
    #[serde(rename = "Aisling Duval")]
    AislingDuval,
    #[serde(rename = "Archon Delaine")]
    ArchonDelaine,
    #[serde(rename = "A. Lavigny-Duval", alias = "Arissa Lavigny-Duval")]
    ArissaLavignyDuval,
    #[serde(rename = "Denton Patreus")]
    DentonPatreus,
    #[serde(rename = "Edmund Mahon")]
    EdmundMahon,
    #[serde(rename = "Felicia Winters")]
    FeliciaWinters,
    #[serde(rename = "Jerome Archer")]
    JeromeArcher,
    #[serde(rename = "Li Yong-Rui")]
    LiYongRui,
    #[serde(rename = "Nakato Kaine")]
    NakatoKaine,
    #[serde(rename = "Pranav Antal")]
    PranavAntal,
    #[serde(rename = "Yuri Grom")]
    YuriGrom,
    #[serde(rename = "Zachary Hudson")]
    ZacharyHudson,
    #[serde(rename = "Zemina Torval")]
    ZeminaTorval,
}

#[test]
fn power() {
    let read = |json: &str| {
        serde_json::from_str::<Power>(json)
            .unwrap_or_else(|e| panic!("{} should read: {}", json, e))
    };

    assert_eq!(Power::EdmundMahon, read(r#""Edmund Mahon""#));
    // The journal abbreviates her; the API and the dumps do not.
    assert_eq!(Power::ArissaLavignyDuval, read(r#""A. Lavigny-Duval""#));
    assert_eq!(Power::ArissaLavignyDuval, read(r#""Arissa Lavigny-Duval""#));
    // The second Powerplay's additions.
    assert_eq!(Power::NakatoKaine, read(r#""Nakato Kaine""#));
    assert_eq!(Power::JeromeArcher, read(r#""Jerome Archer""#));
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[serde(rename_all = "PascalCase")]
pub enum Economy {
    #[serde(alias = "$economy_Agri;")]
    Agriculture,
    #[serde(alias = "$economy_Colony;")]
    Colony,
    #[serde(alias = "$economy_Extraction;")]
    Extraction,
    #[serde(alias = "$economy_HighTech;")]
    #[serde(alias = "High Tech")]
    HighTech,
    #[serde(alias = "$economy_Industrial;")]
    Industrial,
    #[serde(alias = "$economy_Military;")]
    Military,
    #[serde(alias = "$economy_Refinery;")]
    Refinery,
    #[serde(alias = "$economy_Service;")]
    Service,
    #[serde(alias = "$economy_Terraforming;")]
    Terraforming,
    #[serde(alias = "$economy_Tourism;")]
    Tourism,
    #[serde(alias = "$economy_Carrier;")]
    Carrier,
    #[serde(alias = "$economy_Prison;")]
    Prison,
    #[serde(alias = "$economy_Rescue;")]
    Rescue,
    #[serde(alias = "$economy_Undefined;")]
    Undefined,
    #[serde(alias = "")]
    #[serde(alias = "$economy_None;")]
    None,
}

impl fmt::Display for Economy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Nullable for Economy {
    fn is_null(&self) -> bool {
        match self {
            Economy::None => true,
            _ => false,
        }
    }
}

impl PartialEq for Economy {
    fn eq(&self, other: &Self) -> bool {
        match (*other, *self) {
            (Economy::None, Economy::None) => false,
            (l, r) => l as u8 == r as u8,
        }
    }
}

#[test]
fn economy() {
    let high_tech = serde_json::from_str(
        r#"
        "High Tech"
    "#,
    )
    .unwrap();
    assert_eq!(Economy::HighTech, high_tech);
    let extraction = serde_json::from_str(
        r#"
        "$economy_Extraction;"
    "#,
    )
    .unwrap();
    assert_eq!(Economy::Extraction, extraction);
    assert!(Economy::None != Economy::None);
    assert!(serde_json::from_str::<Economy>(r#""$economy_None;""#)
        .unwrap()
        .is_null());
    assert!(serde_json::from_str::<Economy>(r#""None""#).unwrap().is_null());
    assert!(serde_json::from_str::<Economy>(r#""""#).unwrap().is_null());
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[cfg(feature = "with-postgis-sqlx")]
impl GeomProcessor for Coordinate {
    fn dimensions(&self) -> CoordDimensions {
        CoordDimensions::xyz()
    }

    fn coordinate(
        &mut self,
        x: f64,
        y: f64,
        z: Option<f64>,
        _m: Option<f64>,
        _t: Option<f64>,
        _tm: Option<u64>,
        _idx: usize,
    ) -> geozero::error::Result<()> {
        self.x = x;
        self.y = y;
        self.z = z.unwrap_or(0.0);
        Ok(())
    }
}

#[cfg(feature = "with-postgis-sqlx")]
impl GeozeroGeometry for Coordinate {
    fn process_geom<P: GeomProcessor>(
        &self,
        processor: &mut P,
    ) -> std::result::Result<(), geozero::error::GeozeroError> {
        processor.point_begin(0)?;
        processor.coordinate(
            self.x,
            self.y,
            Some(self.z),
            None,
            None,
            None,
            0,
        )?;
        processor.point_end(0)
    }

    fn dims(&self) -> CoordDimensions {
        CoordDimensions::xyz()
    }
}

#[cfg(feature = "with-postgis-sqlx")]
impl FromWkb for Coordinate {
    fn from_wkb<R: Read>(
        rdr: &mut R,
        dialect: WkbDialect,
    ) -> geozero::error::Result<Self> {
        let mut pt = Coordinate { x: 0., y: 0., z: 0. };
        geozero::wkb::process_wkb_type_geom(rdr, &mut pt, dialect)?;
        Ok(pt)
    }
}
