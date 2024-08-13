use chrono::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

/// A single timestamped entry, containing an [`Event`], [`NavRoute`], etc.
#[derive(Deserialize, Debug, PartialEq)]
pub struct Entry<E> {
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub event: E,
    // TODO: Technically these could belong here:
    // https://github.com/EDCD/EDDN/blob/master/schemas/journal-v1.0.json
    //
    // They would just only live here then.
    //
    // pub star_system: String,
    // pub star_pos: Coordinate,
    // pub system_address: i64,
    #[serde(default)]
    pub horizons: bool,
    #[serde(default)]
    pub odyssey: bool,
}

#[test]
fn entry() {
    use crate::Government;

    #[derive(Deserialize)]
    enum Dumb {
        Foo,
    }
    assert!(serde_json::from_str::<Entry<Dumb>>(
        r#"
        {
            "timestamp": "1970-01-01T00:00:00Z",
            "Foo": null
        }
    "#
    )
    .is_ok());
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    #[allow(unused)]
    struct Dumber {
        key: (),
    }
    assert!(serde_json::from_str::<Entry<Dumber>>(
        r#"
        {
            "timestamp": "1970-01-01T00:00:00Z",
            "Key": null
        }
    "#
    )
    .is_ok());
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    #[allow(unused)]
    struct Dumbest {
        key: Government,
    }
    assert!(serde_json::from_str::<Entry<Dumbest>>(
        r#"
        {
            "timestamp": "1970-01-01T00:00:00Z",
            "Key": "Anarchy"
        }
    "#
    )
    .is_ok());
}

/// Parse a single file's worth of journal entries
// TODO: Our own error types.
// TODO: add result inside vec too.
// TODO: This should be an interator, since files are updates as the game runs.
pub fn parse_journal_file<P: AsRef<Path>>(path: P) -> Result<Vec<Entry<Event>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect())
}

/// Parse all journals file's entries in a directory
// TODO: Our own error types.
// TODO: add result inside vec too.
// TODO: is there a way to stream this too? search for current running log files?
pub fn parse_journal_dir<P: AsRef<Path>>(path: P) -> Result<Vec<Entry<Event>>, Box<dyn Error>> {
    let mut entries = Vec::new();
    for entry in read_dir(path)? {
        let entry = entry?;
        if entry.file_type().unwrap().is_file()
            && entry.path().extension().and_then(OsStr::to_str) == Some("log")
        {
            entries.append(&mut parse_journal_file(entry.path())?);
        }
    }
    Ok(entries)
}

/// Parse a status file entry
// TODO: Our own error types.
pub fn parse_status_file<P: AsRef<Path>, E>(path: P) -> Result<Entry<E>, serde_json::Error>
where
    for<'de> E: Deserialize<'de>,
{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
}

/// `Journal.<timestamp>.<part>.log`
///
/// These files store a list of events in JSON lines format. The means that each line is a complete
/// JSON object.
///
/// Each incremental journal file will begin with a [`Fileheader`][incremental::Event::Fileheader]
/// event which in addition to some other metadata, also contains the `part` of the log. This, in
/// addition to the ubiquitous `timestamp` makes parsing the filename unnecessary. For more
/// information on each [`Event`] read their individual documentation.
pub mod incremental;
pub use self::incremental::Event;

/// `Status.json` TODO
pub mod status {}

/// `NavRoute.json`
pub mod route;
pub use self::route::NavRoute;

/// `Cargo.json` TODO
pub mod cargo {}

/// `Market.json` TODO
pub mod market;
pub use self::market::Market;

/// `Shipyard.json` TODO
pub mod shipyard {}

/// `Outfitting.json` TODO
pub mod outfitting {}

/// `ModulesInfo.json` TODO
pub mod modules_info {}
