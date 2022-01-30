use std::collections::BTreeMap as Map;
use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Debug)]
pub enum BodyType {
    Star,
    Planet,
    PlanetaryRing,
    Moon, // TODO: Does this actually exist?
    StellarRing,
    Station,
    AsteroidCluster,

    // Special case for a body's parent being a barycentre
    Null
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Composition {
    pub ice: f64,
    pub rock: f64,
    pub metal: f64,
}

pub struct Node {
    pub body_type: BodyType,
    pub body_id: i16,
}

#[derive(Deserialize, Debug)]
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
    pub distance_from_arrival: Option<f64>,

    // if body_type == star
    // ...

    // else if body_type == planet/moon
    pub parents: Vec<Map<String, i16>>,
    pub planet_class: Option<String>, // TODO: e.g. "Rocky body"
    pub tidal_lock: Option<bool>,
    pub landable: Option<bool>,
    pub terraform_state: Option<String>, // TODO
    pub atmosphere: Option<String>, // TODO: (see below)
    pub atmosphere_type: Option<String>, // TODO: (see above)
    pub volcanism: Option<String>, // TODO
    /// Body masses in units of earth masses
    pub mass: Option<f64>,
    pub radius: Option<f64>,
    pub surface_gravity: Option<f64>,
    pub surface_temperature: Option<f64>,
    pub surface_pressure: Option<f64>,
    pub composition: Option<Composition>,
    pub semi_major_axis: Option<f64>,
    pub eccentricity: Option<f64>,
    pub orbital_inclination: Option<f64>,
    pub periapsis: Option<f64>,
    pub orbital_period: Option<f64>,
    pub rotation_period: Option<f64>,
    pub axial_tilt: Option<f64>,

    // TODO: Ring info
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Signal {
    // e.g. Alexandrite
    #[serde(rename = "Type")]
    pub ty: String,
    // #[serde(rename = "Type_Localised")]
    // pub ty_loc: String,
    pub count: usize,
}
