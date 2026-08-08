use serde::{Deserialize, Serialize};
#[cfg(feature = "with-sqlx")]
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

#[derive(Serialize, Deserialize, Debug)]
pub struct Market {
    #[serde(rename = "systemName")]
    pub system_name: String,
    #[serde(rename = "stationName")]
    pub station_name: String,
    #[serde(rename = "marketId")]
    pub market_id: i64,
    pub commodities: Vec<Commodity>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "with-sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "with-sqlx", sqlx(no_pg_array))]
#[serde(rename_all = "camelCase")]
pub struct Commodity {
    pub name: String,
    pub mean_price: i32,
    pub buy_price: i32,
    pub sell_price: i32,
    pub demand: i32,
    pub demand_bracket: i32,
    pub stock: i32,
    pub stock_bracket: i32,
}

#[cfg(feature = "with-sqlx")]
impl PgHasArrayType for Commodity {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_commodity")
    }
}

/// What a station sells in its outfitting bay
#[derive(Serialize, Deserialize, Debug)]
pub struct Outfitting {
    #[serde(rename = "systemName")]
    pub system_name: String,
    #[serde(rename = "stationName")]
    pub station_name: String,
    #[serde(rename = "marketId")]
    pub market_id: i64,
    pub modules: Vec<Module>,
}

/// One module on sale, however much the sender saw fit to say about it
///
/// The two live outfitting schemas disagree about this and only this. Version
/// 2 sends a bare symbolic name; version 3 sends the name with what it costs.
/// Both are still sent, so both are read, and which one arrived is a question
/// about the sender rather than about the module.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Module {
    /// `outfitting/3`: the name and the prices beside it
    Priced(PricedModule),
    /// `outfitting/2`: the symbolic name alone
    Named(String),
}

impl Module {
    /// The symbolic name, e.g. `Int_Engine_Size3_Class5_Fast`
    pub fn name(&self) -> &str {
        match self {
            Module::Priced(module) => &module.name,
            Module::Named(name) => name,
        }
    }

    /// What it sells for, where that was sent
    pub fn buy_price(&self) -> Option<i64> {
        match self {
            Module::Priced(module) => Some(module.buy_price),
            Module::Named(_) => None,
        }
    }

    /// What it sells for in merits, where that was sent
    pub fn merc_coins_price(&self) -> Option<i64> {
        match self {
            Module::Priced(module) => Some(module.buy_merc_coins_price),
            Module::Named(_) => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PricedModule {
    pub id: i64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "BuyPrice")]
    pub buy_price: i64,
    #[serde(rename = "BuyMercCoinsPrice")]
    pub buy_merc_coins_price: i64,
}

/// What a station sells in its shipyard
#[derive(Serialize, Deserialize, Debug)]
pub struct Shipyard {
    #[serde(rename = "systemName")]
    pub system_name: String,
    #[serde(rename = "stationName")]
    pub station_name: String,
    #[serde(rename = "marketId")]
    pub market_id: i64,
    /// Symbolic ship names, e.g. `Federation_Corvette`
    pub ships: Vec<String>,
    /// Whether the sender could buy a Cobra MkIV, which most cannot
    ///
    /// A property of the commander rather than of the shipyard, and the
    /// reason a Cobra MkIV missing from `ships` says nothing.
    #[serde(rename = "allowCobraMkIV")]
    pub allow_cobra_mk_iv: Option<bool>,
}

/// One commodity as a station's black market takes it
///
/// A single sale rather than a list: the game reports the black market one
/// commodity at a time, which is why this says nothing about what else is
/// traded there and cannot be read as the whole of it.
#[derive(Serialize, Deserialize, Debug)]
pub struct BlackMarket {
    #[serde(rename = "systemName")]
    pub system_name: String,
    #[serde(rename = "stationName")]
    pub station_name: String,
    /// Not required by the schema, though a sale cannot be placed without it
    #[serde(rename = "marketId")]
    pub market_id: Option<i64>,

    pub name: String,
    #[serde(rename = "sellPrice")]
    pub sell_price: i32,
    pub prohibited: bool,
}

/// The market schemas, read from the shape EDDN sends
///
/// None of these carries an `event`, so nothing about the payload says what it
/// is; the `$schemaRef` above it is the only thing that does. They are written
/// out here so that a rename that stops one being read fails here rather than
/// silently going quiet on the wire.
///
/// <https://github.com/EDCD/EDDN/tree/master/schemas>
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::Entry;

    /// `outfitting/3`, which prices each module
    #[test]
    fn outfitting_with_prices() {
        let entry: Entry<Outfitting> = serde_json::from_str(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "systemName": "Sol",
                "stationName": "Abraham Lincoln",
                "marketId": 128016384,
                "modules": [
                    {
                        "id": 128064258,
                        "Name": "Int_Engine_Size3_Class5_Fast",
                        "BuyPrice": 5103953,
                        "BuyMercCoinsPrice": 0
                    }
                ]
            }"#,
        )
        .expect("outfitting/3 should parse");

        let module = &entry.event.modules[0];
        assert_eq!(module.name(), "Int_Engine_Size3_Class5_Fast");
        assert_eq!(module.buy_price(), Some(5103953));
        assert_eq!(module.merc_coins_price(), Some(0));
    }

    /// `outfitting/2`, which sends the name alone
    ///
    /// Still a live schema, still sent, and the only difference between the
    /// two versions. A module read this way says a station sells it and
    /// nothing about what it costs.
    #[test]
    fn outfitting_without_prices() {
        let entry: Entry<Outfitting> = serde_json::from_str(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "systemName": "Sol",
                "stationName": "Abraham Lincoln",
                "marketId": 128016384,
                "modules": [
                    "Int_Engine_Size3_Class5_Fast",
                    "Hpt_ChaffLauncher_Tiny"
                ]
            }"#,
        )
        .expect("outfitting/2 should parse");

        assert_eq!(entry.event.modules.len(), 2);
        assert_eq!(
            entry.event.modules[0].name(),
            "Int_Engine_Size3_Class5_Fast"
        );
        assert_eq!(entry.event.modules[0].buy_price(), None);
        assert_eq!(entry.event.modules[0].merc_coins_price(), None);
    }

    #[test]
    fn a_shipyard_sends_names_only() {
        let entry: Entry<Shipyard> = serde_json::from_str(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "systemName": "Sol",
                "stationName": "Abraham Lincoln",
                "marketId": 128016384,
                "ships": ["SideWinder", "Federation_Corvette"],
                "allowCobraMkIV": false
            }"#,
        )
        .expect("shipyard/2 should parse");

        assert_eq!(entry.event.ships, ["SideWinder", "Federation_Corvette"]);
        assert_eq!(entry.event.allow_cobra_mk_iv, Some(false));
    }

    /// One commodity at a time, and the market id the schema does not demand
    #[test]
    fn a_black_market_sale() {
        let entry: Entry<BlackMarket> = serde_json::from_str(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "systemName": "Sol",
                "stationName": "Abraham Lincoln",
                "marketId": 128016384,
                "name": "Gold",
                "sellPrice": 9432,
                "prohibited": false
            }"#,
        )
        .expect("blackmarket/1 should parse");

        assert_eq!(entry.event.name, "Gold");
        assert_eq!(entry.event.sell_price, 9432);
        assert_eq!(entry.event.market_id, Some(128016384));
    }

    /// A sale with no market id still reads, and cannot be placed
    #[test]
    fn a_black_market_sale_without_a_market() {
        let entry: Entry<BlackMarket> = serde_json::from_str(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "systemName": "Sol",
                "stationName": "Abraham Lincoln",
                "name": "Gold",
                "sellPrice": 9432,
                "prohibited": true
            }"#,
        )
        .expect("blackmarket/1 without a market id should parse");

        assert_eq!(entry.event.market_id, None);
        assert!(entry.event.prohibited);
    }

    /// The commodity schema, which was read before any of the others
    #[test]
    fn a_commodity_market() {
        let entry: Entry<Market> = serde_json::from_str(
            r#"{
                "timestamp": "2026-08-08T12:00:00Z",
                "systemName": "Sol",
                "stationName": "Abraham Lincoln",
                "marketId": 128016384,
                "commodities": [
                    {
                        "name": "gold",
                        "meanPrice": 9411,
                        "buyPrice": 0,
                        "sellPrice": 9432,
                        "demand": 1148,
                        "demandBracket": 2,
                        "stock": 0,
                        "stockBracket": 0
                    }
                ]
            }"#,
        )
        .expect("commodity/3 should parse");

        assert_eq!(entry.event.commodities[0].name, "gold");
        assert_eq!(entry.event.commodities[0].sell_price, 9432);
    }
}
