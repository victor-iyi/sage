// Copyright 2021 Victor I. Afolabi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::error::Error;

use serde::{
  de::{
    self, value::BorrowedStrDeserializer, Deserialize, DeserializeSeed,
    Deserializer, IntoDeserializer, MapAccess, Unexpected, Visitor,
  },
  forward_to_deserialize_any,
  ser::{Serialize, SerializeStruct, Serializer},
};

use std::{fmt, mem};

pub const TOKEN: &str = "$sage::raw::RawDType";

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `RawDType`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

/// Reference to a range of bytes encompassing a single valid JSON value in the
/// input data.
///
/// A `RawDType` can be used to defer parsing parts of a payload until later,
/// or to avoid parsing it at all in the case that part of the payload just
/// needs to be transferred verbatim into a different output object.
///
/// When serializing, a value of this type will retain its original formatting
/// and will not be minified or pretty-printed.
///
/// # Note
///
/// `RawDType` is only available if sage is built with the `"raw_dtype"`
/// feature.
///
/// ```toml
/// [dependencies]
/// sage = { version = "*", features = ["raw_dtype"] }
/// ```
///
/// # Example
///
/// ```rust,ignore
/// use serde::{Deserialize, Serialize};
/// use sage::{Result, json::RawDType};
///
/// #[derive(Deserialize)]
/// struct Input<'a> {
///     code: u32,
///     #[serde(borrow)]
///     payload: &'a RawDType,
/// }
///
/// #[derive(Serialize)]
/// struct Output<'a> {
///     info: (u32, &'a RawDType),
/// }
///
/// // Efficiently rearrange JSON input containing separate "code" and "payload"
/// // keys into a single "info" key holding an array of code and payload.
/// //
/// // This could be done equivalently using `sage::DType` as the type for
/// // payload, but &RawDType will perform better because it does not require
/// // memory allocation. The correct range of bytes is borrowed from the input
/// // data and pasted verbatim into the output.
/// fn rearrange(input: &str) -> Result<String> {
///     let input: Input = sage::json::from_str(input)?;
///
///     let output = Output {
///         info: (input.code, input.payload),
///     };
///
///     sage::to_string(&output)
/// }
///
/// fn main() -> Result<()> {
///     let out = rearrange(r#" {"code": 200, "payload": {}} "#)?;
///
///     assert_eq!(out, r#"{"info":[200,{}]}"#);
///
///     Ok(())
/// }
/// ```
///
/// # Ownership
///
/// The typical usage of `RawDType` will be in the borrowed form:
///
/// ```rust,ignore
/// # use serde::Deserialize;
/// # use sage::json::RawDType;
/// #
/// #[derive(Deserialize)]
/// struct SomeStruct<'a> {
///     #[serde(borrow)]
///     raw_dtype: &'a RawDType,
/// }
/// ```
///
/// The borrowed form is suitable when deserializing through
/// [`sage::json::from_str`] and [`sage::json::from_slice`] which support
/// borrowing from the input data without memory allocation.
///
/// When deserializing through [`sage::from_reader`] you will need to use
/// the boxed form of `RawDType` instead. This is almost as efficient but
/// involves buffering the raw value from the I/O stream into memory.
///
/// [`sage::json::from_str`]: ../fn.from_str.html
/// [`sage::json::from_slice`]: ../fn.from_slice.html
/// [`sage::json::from_reader`]: ../fn.from_reader.html
///
/// ```rust,ignore
/// # use serde::Deserialize;
/// # use sage::json::RawDType;
/// #
/// #[derive(Deserialize)]
/// struct SomeStruct {
///     raw_dtype: Box<RawDType>,
/// }
/// ```
#[repr(C)]
#[cfg_attr(docsrs, doc(cfg(feature = "raw_dtype")))]
pub struct RawDType {
  json: str,
}

impl RawDType {
  fn from_borrowed(json: &str) -> &Self {
    unsafe { mem::transmute::<&str, &RawDType>(json) }
  }

  fn from_owned(json: Box<str>) -> Box<Self> {
    unsafe { mem::transmute::<Box<str>, Box<RawDType>>(json) }
  }
}

impl Clone for Box<RawDType> {
  fn clone(&self) -> Self {
    (**self).to_owned()
  }
}

impl ToOwned for RawDType {
  type Owned = Box<RawDType>;

  fn to_owned(&self) -> Self::Owned {
    RawDType::from_owned(self.json.to_owned().into_boxed_str())
  }
}

impl Default for Box<RawDType> {
  fn default() -> Self {
    RawDType::from_borrowed("null").to_owned()
  }
}

impl fmt::Debug for RawDType {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter
      .debug_tuple("RawDType")
      .field(&format_args!("{}", &self.json))
      .finish()
  }
}

impl fmt::Display for RawDType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.json)
  }
}

impl RawDType {
  /// Convert an owned `String` of JSON data to an owned `RawDType`.
  ///
  /// This function is equivalent to `sage::json::from_str::<Box<RawDType>>`
  /// except that we avoid an allocation and memcpy if both of the following
  /// are true:
  ///
  /// - the input has no leading or trailing whitespace, and
  /// - the input has capacity equal to its length.
  pub fn from_string(json: String) -> Result<Box<Self>, Error> {
    {
      let borrowed = crate::json::from_str::<&Self>(&json)?;
      if borrowed.json.len() < json.len() {
        return Ok(borrowed.to_owned());
      }
    }
    Ok(Self::from_owned(json.into_boxed_str()))
  }

  /// Access the JSON text underlying a raw value.
  ///
  /// # Example
  ///
  /// ```rust,ignore
  /// use serde::Deserialize;
  /// use sage::{Result, json::RawDType};
  ///
  /// #[derive(Deserialize)]
  /// struct Response<'a> {
  ///     code: u32,
  ///     #[serde(borrow)]
  ///     payload: &'a RawDType,
  /// }
  ///
  /// fn process(input: &str) -> Result<()> {
  ///     let response: Response = sage::json::from_str(input)?;
  ///
  ///     let payload = response.payload.get();
  ///     if payload.starts_with('{') {
  ///         // handle a payload which is a JSON map
  ///     } else {
  ///         // handle any other type
  ///     }
  ///
  ///     Ok(())
  /// }
  ///
  /// fn main() -> Result<()> {
  ///     process(r#" {"code": 200, "payload": {}} "#)?;
  ///     Ok(())
  /// }
  /// ```
  pub fn get(&self) -> &str {
    &self.json
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `to_raw_dtype`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

/// Convert a `T` into a boxed `RawDType`.
///
/// # Example
///
/// ```rust,ignore
/// // Upstream crate
/// # #[derive(Serialize)]
/// pub struct Thing {
///     foo: String,
///     bar: Option<String>,
///     extra_data: Box<RawDType>,
/// }
///
/// // Local crate
/// use serde::Serialize;
/// use sage::json::{to_raw_dtype, RawDType};
///
/// #[derive(Serialize)]
/// struct MyExtraData {
///     a: u32,
///     b: u32,
/// }
///
/// let my_thing = Thing {
///     foo: "FooVal".into(),
///     bar: None,
///     extra_data: to_raw_dtype(&MyExtraData { a: 1, b: 2 }).unwrap(),
/// };
/// # assert_eq!(
/// #   sage::to_dtype(my_thing).unwrap(),
/// #   sage::json!({
/// #     "foo": "FooVal",
/// #     "bar": null,
/// #     "extra_data": { "a": 1, "b": 2 }
/// #   })
/// # );
/// ```
///
/// # Errors
///
/// This conversion can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
///
/// ```rust
/// use std::collections::BTreeMap;
///
/// // The keys in this map are vectors, not strings.
/// let mut map = BTreeMap::new();
/// map.insert(vec![32, 64], "x86");
///
/// println!("{}", sage::json::to_raw_dtype(&map).unwrap_err());
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "raw_dtype")))]
pub fn to_raw_dtype<T>(value: &T) -> Result<Box<RawDType>, Error>
where
  T: Serialize,
{
  let json_string = crate::json::to_string(value)?;
  Ok(RawDType::from_owned(json_string.into_boxed_str()))
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `serde::ser::Serialize` for `RawDType`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

impl Serialize for RawDType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct(TOKEN, 1)?;
    s.serialize_field(TOKEN, &self.json)?;
    s.end()
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `serde::de::Deserialize` for `RawDType` & `Box<RawDType>`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/
impl<'de: 'a, 'a> Deserialize<'de> for &'a RawDType {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ReferenceVisitor;

    impl<'de> Visitor<'de> for ReferenceVisitor {
      type Value = &'de RawDType;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "any valid JSON value")
      }

      fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
      where
        V: MapAccess<'de>,
      {
        let value = visitor.next_key::<RawKey>()?;
        if value.is_none() {
          return Err(de::Error::invalid_type(Unexpected::Map, &self));
        }
        visitor.next_value_seed(ReferenceFromString)
      }
    }

    deserializer.deserialize_newtype_struct(TOKEN, ReferenceVisitor)
  }
}

impl<'de> Deserialize<'de> for Box<RawDType> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct BoxedVisitor;

    impl<'de> Visitor<'de> for BoxedVisitor {
      type Value = Box<RawDType>;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "any valid JSON value")
      }

      fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
      where
        V: MapAccess<'de>,
      {
        let value = visitor.next_key::<RawKey>()?;
        if value.is_none() {
          return Err(de::Error::invalid_type(Unexpected::Map, &self));
        }
        visitor.next_value_seed(BoxedFromString)
      }
    }

    deserializer.deserialize_newtype_struct(TOKEN, BoxedVisitor)
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `RawKey`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

struct RawKey;

impl<'de> Deserialize<'de> for RawKey {
  fn deserialize<D>(deserializer: D) -> Result<RawKey, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct FieldVisitor;

    impl<'de> Visitor<'de> for FieldVisitor {
      type Value = ();

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("raw value")
      }

      fn visit_str<E>(self, s: &str) -> Result<(), E>
      where
        E: de::Error,
      {
        if s == TOKEN {
          Ok(())
        } else {
          Err(de::Error::custom("unexpected raw value"))
        }
      }
    }

    deserializer.deserialize_identifier(FieldVisitor)?;
    Ok(RawKey)
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `ReferenceFromString`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

pub struct ReferenceFromString;

impl<'de> DeserializeSeed<'de> for ReferenceFromString {
  type Value = &'de RawDType;

  fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(self)
  }
}

impl<'de> Visitor<'de> for ReferenceFromString {
  type Value = &'de RawDType;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("raw value")
  }

  fn visit_borrowed_str<E>(self, s: &'de str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    Ok(RawDType::from_borrowed(s))
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `BoxedFromString`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

pub struct BoxedFromString;

impl<'de> DeserializeSeed<'de> for BoxedFromString {
  type Value = Box<RawDType>;

  fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(self)
  }
}

impl<'de> Visitor<'de> for BoxedFromString {
  type Value = Box<RawDType>;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("raw value")
  }

  fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    self.visit_string(s.to_owned())
  }

  fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    Ok(RawDType::from_owned(s.into_boxed_str()))
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `RawKeyDeserializer`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

struct RawKeyDeserializer;

impl<'de> Deserializer<'de> for RawKeyDeserializer {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
  where
    V: de::Visitor<'de>,
  {
    visitor.visit_borrowed_str(TOKEN)
  }

  forward_to_deserialize_any! {
      bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 char str string seq
      bytes byte_buf map struct option unit newtype_struct ignored_any
      unit_struct tuple_struct tuple enum identifier
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `OwnedRawDeserializer`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

pub struct OwnedRawDeserializer {
  pub raw_dtype: Option<String>,
}

impl<'de> MapAccess<'de> for OwnedRawDeserializer {
  type Error = Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
  where
    K: de::DeserializeSeed<'de>,
  {
    if self.raw_dtype.is_none() {
      return Ok(None);
    }
    seed.deserialize(RawKeyDeserializer).map(Some)
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
  where
    V: de::DeserializeSeed<'de>,
  {
    seed.deserialize(self.raw_dtype.take().unwrap().into_deserializer())
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `BorrowRawDeserializer`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

pub struct BorrowedRawDeserializer<'de> {
  pub raw_dtype: Option<&'de str>,
}

impl<'de> MapAccess<'de> for BorrowedRawDeserializer<'de> {
  type Error = Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
  where
    K: de::DeserializeSeed<'de>,
  {
    if self.raw_dtype.is_none() {
      return Ok(None);
    }
    seed.deserialize(RawKeyDeserializer).map(Some)
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
  where
    V: de::DeserializeSeed<'de>,
  {
    seed
      .deserialize(BorrowedStrDeserializer::new(self.raw_dtype.take().unwrap()))
  }
}
