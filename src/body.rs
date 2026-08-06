use crate::de;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;
use std::fmt;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub enum BodyType {
    Star,
    Planet,
    PlanetaryRing,
    Moon, // TODO: Does this actually exist?
    StellarRing,
    Station,
    AsteroidCluster,

    // Special case for a body's parent being a barycenter
    Null,

    #[serde(untagged)]
    Unknown(String),
}

impl From<&str> for BodyType {
    fn from(name: &str) -> Self {
        serde_json::from_value(serde_json::Value::String(name.to_owned()))
            .unwrap_or_else(|_| Self::Unknown(name.to_owned()))
    }
}

impl fmt::Display for BodyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown(name) => write!(f, "{}", name),
            named => write!(f, "{:?}", named),
        }
    }
}

/// What a body's atmosphere is mostly made of
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AtmosphereType {
    Ammonia,
    AmmoniaOxygen,
    AmmoniaRich,
    Argon,
    ArgonRich,
    CarbonDioxide,
    CarbonDioxideRich,
    EarthLike,
    Helium,
    MetallicVapour,
    Methane,
    MethaneRich,
    Neon,
    NeonRich,
    Nitrogen,
    Oxygen,
    SilicateVapour,
    SulphurDioxide,
    Water,
    WaterRich,
    None,

    #[serde(untagged)]
    Unknown(String),
}

impl From<&str> for AtmosphereType {
    fn from(name: &str) -> Self {
        serde_json::from_value(serde_json::Value::String(name.to_owned()))
            .unwrap_or_else(|_| Self::Unknown(name.to_owned()))
    }
}

impl fmt::Display for AtmosphereType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown(name) => write!(f, "{}", name),
            named => write!(f, "{:?}", named),
        }
    }
}

/// What a body with a surface has, and a gas giant has none of
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Surface {
    pub atmosphere_type: AtmosphereType,
    #[serde(rename = "SurfacePressure")]
    pub pressure: f32,
    pub composition: Composition,
}

/// The path a thing takes around whatever it goes round
///
/// The seven arrive together or not at all: a scan of something that orbits
/// nothing carries none of them.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Orbit {
    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub orbital_inclination: f32,
    pub periapsis: f32,
    pub orbital_period: f32,
    pub ascending_node: f32,
    pub mean_anomaly: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Composition {
    pub ice: f32,
    pub rock: f32,
    pub metal: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Material {
    pub name: String,
    pub percent: f64,
}

pub struct Node {
    pub body_type: BodyType,
    pub body_id: i16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Body {
    #[serde(rename = "BodyID")]
    pub id: i16,
    #[serde(rename = "BodyName")]
    #[serde(alias = "Body")]
    pub name: String,
    #[serde(rename = "BodyType")]
    pub ty: Option<BodyType>,
    /// Distance from primary star in light seconds
    #[serde(rename = "DistanceFromArrivalLS")]
    #[serde(alias = "DistFromStarLS")]
    pub distance_from_arrival: Option<f32>,
    pub parents: Vec<Map<String, i16>>,

    pub planet_class: String, // TODO: e.g. "Rocky body"
    pub tidal_lock: bool,
    pub landable: bool,
    #[serde(deserialize_with = "de::empty_str_is_none")]
    pub terraform_state: Option<String>,
    #[serde(deserialize_with = "de::empty_str_is_none")]
    pub atmosphere: Option<String>,
    #[serde(deserialize_with = "de::empty_str_is_none")]
    pub volcanism: Option<String>,
    pub materials: Option<Vec<Material>>,
    /// Body masses in units of earth masses
    #[serde(rename = "MassEM")]
    pub mass: f32,
    pub radius: f32,
    pub surface_gravity: f32,
    pub surface_temperature: f32,
    /// [`None`] for a body with no surface, which is to say a gas giant
    #[serde(flatten)]
    pub surface: Option<Surface>,
    #[serde(flatten)]
    pub orbit: Orbit,
    pub rotation_period: f32,
    pub axial_tilt: f32,

    // TODO: Ring info
    // pub reserve_level: Option<String>,
    pub was_mapped: bool,
    pub was_discovered: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Signal {
    // e.g. Alexandrite
    #[serde(rename = "Type")]
    pub ty: String,
    // #[serde(rename = "Type_Localised")]
    // pub ty_loc: String,
    pub count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Star {
    #[serde(rename = "BodyName")]
    pub name: String,
    #[serde(rename = "BodyID")]
    pub id: i16,
    /// Empty for the primary, which is what everything else is measured from
    #[serde(default)]
    pub parents: Vec<Map<String, i16>>,

    pub absolute_magnitude: f32,
    #[serde(rename = "Age_MY")]
    pub age_my: i32,
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f32,
    // TODO: enum?
    pub luminosity: String,
    /// What class of star this is, which the game writes as `StarType`
    #[serde(rename = "StarType")]
    pub star_class: String,
    pub stellar_mass: f32,
    pub subclass: i16,

    /// [`None`] for the primary, which goes round nothing
    #[serde(flatten)]
    pub orbit: Option<Orbit>,
    pub axial_tilt: f32,
    pub radius: f32,
    pub rotation_period: f32,
    pub surface_temperature: f32,

    pub was_discovered: bool,
    pub was_mapped: bool,
    // "Rings": Array [
    //     Object {
    //         "InnerRad": Number(1168900000.0),
    //         "MassMT": Number(123920000000000.0),
    //         "Name": String("Spase IA-Y c17-73 A A Belt"),
    //         "OuterRad": Number(2304600000.0),
    //         "RingClass": String("eRingClass_Rocky"),
    //     },
    //     Object {
    //         "InnerRad": Number(4919500000.0),
    //         "MassMT": Number(7495800000000000.0),
    //         "Name": String("Spase IA-Y c17-73 A B Belt"),
    //         "OuterRad": Number(314850000000.0),
    //         "RingClass": String("eRingClass_MetalRich"),
    //     },
    // ],
}

#[test]
fn a_primary_star_goes_round_nothing_and_names_no_ancestor() {
    let star = serde_json::from_str::<Star>(
        r#"
        {
            "timestamp": "2026-08-06T12:00:00Z",
            "event": "Scan",
            "ScanType": "AutoScan",
            "BodyName": "LHS 1788",
            "BodyID": 0,
            "StarSystem": "LHS 1788",
            "SystemAddress": 670417692049,
            "DistanceFromArrivalLS": 0.0,
            "StarType": "M",
            "Subclass": 5,
            "StellarMass": 0.320312,
            "Radius": 331889440.0,
            "AbsoluteMagnitude": 9.716263,
            "Age_MY": 8130,
            "SurfaceTemperature": 3130.0,
            "Luminosity": "Va",
            "RotationPeriod": 168903.53,
            "AxialTilt": 0.0,
            "WasDiscovered": true,
            "WasMapped": false
        }
    "#,
    )
    .unwrap();
    assert_eq!("M", star.star_class);
    assert_eq!(0, star.id);
    assert!(star.parents.is_empty());
    assert_eq!(None, star.orbit);
}

#[test]
fn a_second_star_carries_an_orbit_about_the_first() {
    let star = serde_json::from_str::<Star>(
        r#"
        {
            "timestamp": "2026-08-06T12:00:00Z",
            "event": "Scan",
            "ScanType": "Detailed",
            "BodyName": "LHS 1788 B",
            "BodyID": 1,
            "StarSystem": "LHS 1788",
            "SystemAddress": 670417692049,
            "Parents": [ { "Null": 0 } ],
            "DistanceFromArrivalLS": 1256.7,
            "StarType": "M",
            "Subclass": 7,
            "StellarMass": 0.144531,
            "Radius": 232084704.0,
            "AbsoluteMagnitude": 11.06012,
            "Age_MY": 8130,
            "SurfaceTemperature": 2492.0,
            "Luminosity": "V",
            "SemiMajorAxis": 376903772354.12,
            "Eccentricity": 0.077108,
            "OrbitalInclination": 1.919367,
            "Periapsis": 173.351376,
            "OrbitalPeriod": 566327810.28,
            "AscendingNode": -47.633911,
            "MeanAnomaly": 46.744045,
            "RotationPeriod": 141936.492,
            "AxialTilt": 0.0,
            "WasDiscovered": true,
            "WasMapped": false
        }
    "#,
    )
    .unwrap();
    assert_eq!(1, star.id);
    assert_eq!(1256.7, star.distance_from_arrival_ls);
    let orbit = star.orbit.unwrap();
    assert_eq!(376903770000., orbit.semi_major_axis);
    assert_eq!(1, star.parents.len());
}
