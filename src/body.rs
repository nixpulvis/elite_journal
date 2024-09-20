use crate::de;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum BodyType {
    Star,
    Planet,
    PlanetaryRing,
    Moon, // TODO: Does this actually exist?
    StellarRing,
    Station,
    AsteroidCluster,

    // Special case for a body's parent being a barycentre
    Null,
}

// TODO: enum AtmosphereType {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Composition {
    pub ice: f32,
    pub rock: f32,
    pub metal: f32,
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub atmosphere_type: String, // TODO: use AtmosphereType enum
    #[serde(deserialize_with = "de::empty_str_is_none")]
    pub volcanism: Option<String>,
    pub materials: Option<Vec<Material>>,
    /// Body masses in units of earth masses
    #[serde(rename = "MassEM")]
    pub mass: f32,
    pub radius: f32,
    pub surface_gravity: f32,
    pub surface_temperature: f32,
    pub surface_pressure: f32,
    pub composition: Composition,
    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub orbital_inclination: f32,
    pub periapsis: f32,
    pub orbital_period: f32,
    pub rotation_period: f32,
    pub axial_tilt: f32,
    pub ascending_node: f32,
    pub mean_anomaly: f32,

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
    pub parents: Vec<Map<String, i16>>,

    pub absolute_magnitude: f32,
    #[serde(rename = "Age_MY")]
    pub age_my: i32,
    pub distance_from_arrival_lS: f32,
    // TODO: enum?
    pub luminosity: String,
    pub star_class: String,
    pub stellar_mass: f32,
    pub subclass: i16,

    pub ascending_node: f32,
    pub axial_tilt: f32,
    pub eccentricity: f32,
    pub mean_anomaly: f32,
    pub orbital_inclination: f32,
    pub orbital_period: f32,
    pub periapsis: f32,
    pub radius: f32,
    pub rotation_period: f32,
    pub semi_major_axis: f32,
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
