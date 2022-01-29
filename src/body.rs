use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum BodyType {
    Star,
    Planet,
    Moon,

    // Special case for a body's parent being a berry-center
    Null
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Composition {
    ice: f64,
    rock: f64,
    metal: f64,
}

pub struct Node {
    body_type: BodyType,
    body_id: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Body {
    #[serde(rename = "BodyID")]
    pub id: u16,
    #[serde(rename = "BodyName")]
    pub name: String,
    /// Distance from primary star in light seconds
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival: f64,


    // if body_type == star
    #[serde(rename = "StarSystem")]
    pub system_name: String, // double check this iss aways here
    // TODO: More star info

    // else if body_type == planet/moon
    // pub parents: Vec<(BodyType, u16)>,
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
