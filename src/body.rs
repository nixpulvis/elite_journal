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
///
/// The first three are what a gas giant is scanned without, and so are what
/// decides whether there is a surface here at all. The rest are optional in
/// their own right: a body can have a surface and no volcanism. They are held
/// here rather than on [`Body`] to say where they can and cannot be expected,
/// since none of them means anything without somewhere to stand.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Surface {
    pub atmosphere_type: AtmosphereType,
    #[serde(rename = "SurfacePressure")]
    pub pressure: f32,
    pub composition: Composition,

    /// Whether a ship can be set down on it
    #[serde(default)]
    pub landable: bool,
    /// What the atmosphere is called, where [`Surface::atmosphere_type`] is
    /// what it is made of
    #[serde(default, deserialize_with = "de::empty_str_is_none")]
    pub atmosphere: Option<String>,
    #[serde(default, deserialize_with = "de::empty_str_is_none")]
    pub volcanism: Option<String>,
    #[serde(default, deserialize_with = "de::empty_str_is_none")]
    pub terraform_state: Option<String>,
    /// What can be picked up off it, which is nothing unless it is landable
    #[serde(default)]
    pub materials: Vec<Material>,
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

/// How a thing turns on its own axis
///
/// Nothing to do with an orbit, and the two are independent in both
/// directions: a system's primary star turns and goes round nothing, and a
/// barycenter goes round something and does not turn at all.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Spin {
    #[serde(rename = "RotationPeriod")]
    pub period: f32,
    #[serde(rename = "AxialTilt")]
    pub tilt: f32,
}

/// What is known about a thing rather than about the thing itself
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Discovery {
    #[serde(rename = "WasDiscovered")]
    pub discovered: bool,
    #[serde(rename = "WasMapped")]
    pub mapped: bool,
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
    /// [`None`] where the scan does not report it, which a basic one does not
    pub tidal_lock: Option<bool>,
    /// Body masses in units of earth masses
    #[serde(rename = "MassEM")]
    pub mass: f32,
    pub radius: f32,
    /// Measured at the cloud tops where there is no surface, which is why
    /// these two are not part of [`Surface`] though the game names them for
    /// one
    #[serde(rename = "SurfaceGravity")]
    pub gravity: f32,
    /// [`None`] where the scan was a basic one, which does not report it
    #[serde(rename = "SurfaceTemperature")]
    pub temperature: Option<f32>,
    /// [`None`] for a body with no surface, which is to say a gas giant
    #[serde(flatten)]
    pub surface: Option<Surface>,
    #[serde(flatten)]
    pub orbit: Orbit,
    #[serde(flatten)]
    pub spin: Spin,

    // TODO: Ring info
    // pub reserve_level: Option<String>,
    #[serde(flatten)]
    pub discovery: Discovery,
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
    #[serde(flatten)]
    pub spin: Spin,
    pub radius: f32,
    /// A star has no surface either, whatever the game calls this
    #[serde(rename = "SurfaceTemperature")]
    pub temperature: f32,

    #[serde(flatten)]
    pub discovery: Discovery,
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
