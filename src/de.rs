use serde::{Deserialize, Deserializer};

/// Used to describe types which can consider themselves "null"
///
/// This is useful when deciding when to save a value. See [`null_is_none`].
pub trait Nullable {
    fn is_null(&self) -> bool;
}

/// Map [`Nullable`] types to `Option`
///
/// ### Example
///
/// ```rust
/// use serde::Deserialize;
/// use elite_journal::de::null_is_none;
/// use elite_journal::system::Security;
///
/// #[derive(Deserialize)]
/// struct Foo {
///     #[serde(deserialize_with = "null_is_none")]
///     pub security: Option<Security>,
/// }
///
/// let foo: Foo = serde_json::from_str(r#"{ "security": "Anarchy" }"#).unwrap();
/// assert_eq!(None, foo.security);
/// ```
pub fn null_is_none<'d, D, T: Deserialize<'d> + Nullable>(
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

/// Try to parse a string, returning `None` if it is ""
///
/// ### Example
///
/// ```rust
/// use serde::Deserialize;
/// use elite_journal::de::empty_str_is_none;
///
/// #[derive(Deserialize)]
/// struct Foo {
///     #[serde(deserialize_with = "empty_str_is_none")]
///     pub string: Option<String>,
/// }
///
/// let foo: Foo = serde_json::from_str(r#"{ "string": "" }"#).unwrap();
/// assert_eq!(None, foo.string);
/// ```
pub fn empty_str_is_none<'d, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
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

/// Try to parse a number, returning `None` if it is 0
///
/// ### Example
///
/// ```rust
/// use serde::Deserialize;
/// use elite_journal::de::zero_is_none;
///
/// #[derive(Deserialize)]
/// struct Foo {
///     #[serde(deserialize_with = "zero_is_none")]
///     pub number: Option<u64>,
/// }
///
/// let foo: Foo = serde_json::from_str(r#"{ "number": 0 }"#).unwrap();
/// assert_eq!(None, foo.number);
/// ```
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
