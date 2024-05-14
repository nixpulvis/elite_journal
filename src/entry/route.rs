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
    star_system: String, // TODO: actually a system_address, might change name.
    system_address: u64,
    star_pos: Coordinate,
    star_class: String, // TODO: Enum?
}
