use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::Deserialize;
use crate::{Entry, Coordinate};

// Single variant enum to satify serde for the shape of the data inside an [`Entry`].
#[derive(Deserialize, Debug, PartialEq)]
pub enum Route {
    Route(Vec<Destination>)
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Destination {
    star_system: String,  // TODO: actually a system_address, might change name.
    system_address: u64,
    star_pos: Coordinate,
    star_class: String,  // TODO: Enum?
}

// TODO: Our own error types.
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Entry<Route>, serde_json::Error> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
}
