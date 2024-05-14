use serde::{Deserialize, Deserializer};

pub trait Nullable {
    fn is_null(&self) -> bool;
}

// TODO: tests
pub fn enum_is_null<'d, D, T: Deserialize<'d> + Nullable>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'d>,
{
    let variant = Option::<T>::deserialize(deserializer)?;
    if let Some(v) = variant {
        if v.is_null() {
            Ok(None)
        } else {
            Ok(Some(v))
        }
    } else {
        Ok(None)
    }
}

pub fn empty_str_is_none<'d, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'d>,
{
    let string = String::deserialize(deserializer)?;
    if string == "" {
        Ok(None)
    } else {
        Ok(Some(string))
    }
}

///     #[serde(deserialize_with = "de::zero_is_none")]
///     pub population: Option<u64>,

// TODO: tests
pub fn zero_is_none<'d, D, T: Deserialize<'d> + PartialEq<u64>>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'d>,
{
    let number = Option::<T>::deserialize(deserializer)?;
    if let Some(n) = number {
        if n != 0 {
            return Ok(Some(n));
        }
    }
    Ok(None)
}
