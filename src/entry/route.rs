use crate::prelude::*;
use serde::Deserialize;

/// A route plotted, and the stops on it where they are given
///
/// The game writes `NavRoute` to its log to say a route was plotted and puts
/// the stops in `NavRoute.json` beside it, so a log line carries none of
/// them. EDDN sends the stops in the message. Both are read as this, and the
/// file is read as an [`Entry`][crate::entry::Entry] of it.
#[derive(Deserialize, Debug, Default)]
pub struct NavRoute {
    #[serde(rename = "Route")]
    #[serde(default)]
    pub destinations: Vec<Destination>,
}

/// One stop on a route: a system, where it is, and what burns in the middle
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Destination {
    pub star_system: String, // TODO: actually a system_address, might change name.
    pub system_address: u64,
    pub star_pos: Coordinate,
    pub star_class: String, // TODO: Enum?
}
