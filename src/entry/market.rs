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
