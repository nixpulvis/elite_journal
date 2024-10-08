use crate::prelude::*;
use serde::Deserialize;

// Single variant enum to satify serde for the shape of the data inside an [`Entry`].
#[derive(Deserialize, Debug)]
pub enum NavRoute {
    Route(Vec<Destination>),
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Destination {
    pub star_system: String, // TODO: actually a system_address, might change name.
    pub system_address: u64,
    pub star_pos: Coordinate,
    pub star_class: String, // TODO: Enum?
}
