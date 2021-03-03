use serde::{Deserialize, Deserializer};
use crate::Nullable;

pub fn enum_is_null<'d, D, T: Deserialize<'d> + Nullable>(deserializer: D) -> Result<Option<T>, D::Error>
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
