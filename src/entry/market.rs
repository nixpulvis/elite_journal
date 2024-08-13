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
    pub commodities : Vec<Commodity>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(all(unix, feature = "with-sqlx"), derive(sqlx::Type))]
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
