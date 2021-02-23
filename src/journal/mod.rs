use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;
use std::ffi::OsStr;
use serde::Deserialize;
use chrono::prelude::*;


/// A single log entry, containing an [`Event`]
#[derive(Deserialize, Debug)]
pub struct Entry {
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub event: Event,
}

/// Information provided by the player journal
#[derive(Deserialize, Debug)]
#[serde(tag = "event")]
pub enum Event {
    Fileheader(startup::Fileheader),
    Cargo(startup::Manifest),
    NewCommander(startup::NewCommander),
    ClearSavedGame(startup::Commander),
    Commander(startup::Commander),
    LoadGame(startup::LoadGame),
    Materials(startup::Materials),

    #[serde(rename = "FSDJump")]
    FsdJump(travel::FsdJump),
    Location(travel::Location),

    BuyExplorationData(exploration::BuyExplorationData),
    SellExplorationData(exploration::SellExplorationData),

    // TODO: We'll leave this in for sure... but we should test without it, and probably find a way
    // to add a generic JSON value to this.
    // https://github.com/serde-rs/serde/issues/1973
    #[serde(other)]
    Other,
}

// TODO: add result inside vec too.
// TODO: This should be an interator, since files are updates as the game runs.
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Vec<Entry>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().map(|line| {
        serde_json::from_str(&line.unwrap()).unwrap()
    }).collect())
}


// TODO: add result inside vec too.
// TODO: is there a way to stream this too? search for current running log files?
pub fn parse_dir<P: AsRef<Path>>(path: P) -> Result<Vec<Entry>, Box<dyn Error>> {
    let mut entries = Vec::new();
    for entry in read_dir(path)? {
        let entry = entry?;
        if entry.file_type().unwrap().is_file() &&
           entry.path().extension().and_then(OsStr::to_str) == Some("log")
        {
            entries.append(&mut parse_file(entry.path())?);
        }
    }
    Ok(entries)
}

pub mod startup;
pub mod travel;
pub mod exploration;
