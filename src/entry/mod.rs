use serde::Deserialize;
use chrono::prelude::*;

/// A single timestamped entry, containing an [`incremental::Event`], [`Route`], etc.
///
/// - Parse [`incremental::Event`]s from `Journal.<timestamp>.<part>.log` files with [`incremental::parse_file`]
/// - TODO `Status.json`
/// or [`incremental::parse_dir`].
/// - Parse [`Route`]s from `NavRoute.json` files with [`route::parse_file`].
/// - TODO `Market.json`
/// - TODO `Shipyard.json`
/// - TODO `Outfitting.json`
/// - TODO `ModulesInfo.json`
#[derive(Deserialize, Debug, PartialEq)]
pub struct Entry<E> {
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub event: E,
}

#[test]
fn entry() {
    use crate::Government;

    #[derive(Deserialize)]
    enum Dumb { Foo }
    assert!(serde_json::from_str::<Entry<Dumb>>(r#"
        {
            "timestamp": "1970-01-01T00:00:00Z",
            "Foo": null
        }
    "#).is_ok());
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    #[allow(unused)]
    struct Dumber {
        key: (),
    }
    assert!(serde_json::from_str::<Entry<Dumber>>(r#"
        {
            "timestamp": "1970-01-01T00:00:00Z",
            "Key": null
        }
    "#).is_ok());
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    #[allow(unused)]
    struct Dumbest {
        key: Government,
    }
    assert!(serde_json::from_str::<Entry<Dumbest>>(r#"
        {
            "timestamp": "1970-01-01T00:00:00Z",
            "Key": "Anarchy"
        }
    "#).is_ok());
}


/// `Journal.<timestamp>.<part>.log`
pub mod incremental;
// TODO: It's time to just merge these all.
pub use self::incremental::Event;

/// `Status.json` TODO
pub mod status {}

/// `NavRoute.json`
pub mod route;
pub use self::route::Route;

/// `Cargo.json` TODO
pub mod cargo {}

/// `Market.json` TODO
pub mod market {}

/// `Shipyard.json` TODO
pub mod shipyard {}

/// `Outfitting.json` TODO
pub mod outfitting {}

/// `ModulesInfo.json` TODO
pub mod modules_info {}
