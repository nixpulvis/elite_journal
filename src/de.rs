use serde::{Deserialize, Deserializer};
use crate::Nullable;

// TODO: tests
pub fn enum_is_null<'d, D, T: Deserialize<'d> + Nullable>(deserializer: D)
    -> Result<Option<T>, D::Error>
    where D: Deserializer<'d>,
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

// TODO: tests
pub fn zero_is_none<'d, D, T: Deserialize<'d> + PartialEq<u64>>(deserializer: D)
    -> Result<Option<T>, D::Error>
    where D: Deserializer<'d>,
{
    let number = T::deserialize(deserializer)?;
    if number == 0 {
        Ok(None)
    } else {
        Ok(Some(number))
    }
}
