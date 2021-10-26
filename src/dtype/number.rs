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

#![allow(dead_code)]

use std::fmt;

use serde::{
  de::{self, Unexpected, Visitor},
  forward_to_deserialize_any, serde_if_integer128, Deserialize, Deserializer,
  Serialize,
};

#[cfg(feature = "arbitrary_precision")]
use crate::error::ErrorCode;
use crate::{Error, Result};
#[cfg(feature = "arbitrary_precision")]
use serde::de::{IntoDeserializer, MapAccess};
#[cfg(feature = "arbitrary_precision")]
pub(crate) const TOKEN: &str = "$sage::dtype::Number";

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | Number
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

/// Represents a number, whether integer or floating point.
#[derive(Clone, Eq, PartialEq)]
pub struct Number {
  /// Number enum implementation.
  pub(crate) n: NumImpl,
}

/// Number implementation without arbitrary precision.
#[cfg(not(feature = "arbitrary_precision"))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NumImpl {
  /// Floating point number (always finite).
  Float(f64),
  /// Greater or equal to zero.
  PositiveInt(u64),
  /// Always less than zero.
  NegativeInt(i64),
}

// Implementing Eq is fine since any float values are always finite.
#[cfg(not(feature = "arbitrary_precision"))]
impl Eq for NumImpl {}

/// Number representation with arbitrary precision.
#[cfg(feature = "arbitrary_precision")]
pub type NumImpl = String;

impl Number {
  /// Returns true if the `Number` is an integer between `i64::MIN` & `i64::MAX`.
  ///
  /// For any `Number` on which `is_i64` returns true, `as_i64` is guaranteed to
  /// return the integer value.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let big = i64::MAX as u64 + 10;
  /// let obj = json!({ "a": 65, "b": big, "c": 256.0 });
  ///
  /// assert!(obj["a"].is_i64());
  ///
  /// // Greater than i64::MAX.
  /// assert!(!obj["b"].is_i64());
  ///
  /// // Numbers with a decimal point are not considered integers.
  /// assert!(!obj["c"].is_i64());
  /// ```
  #[inline]
  pub fn is_i64(&self) -> bool {
    #[cfg(not(feature = "arbitrary_precision"))]
    match self.n {
      NumImpl::PositiveInt(n) => n <= i64::MAX as u64,
      NumImpl::NegativeInt(_) => true,
      NumImpl::Float(_) => false,
    }

    #[cfg(feature = "arbitrary_precision")]
    self.as_i64().is_some()
  }

  /// Returns true if the `Number` is an integer between zero and `u64::MAX`.
  ///
  /// For any `Number` on which `is_u64` returns true, `as_u64` is guaranteed to
  /// return the integer value.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({ "a": 65, "b": -65, "c": 256.0 });
  /// assert!(obj["a"].is_u64());
  ///
  /// // Negative integer.
  /// assert!(!obj["b"].is_u64());
  ///
  /// // Numbers with a decimal point are not considered integers.
  /// assert!(!obj["c"].is_u64());
  /// ```
  #[inline]
  pub fn is_u64(&self) -> bool {
    #[cfg(not(feature = "arbitrary_precision"))]
    return matches!(self.n, NumImpl::PositiveInt(_));
    #[cfg(feature = "arbitrary_precision")]
    self.as_u64().is_some()
  }

  /// Returns true if the `Number` can be represented by `f64`.
  ///
  /// For any `Number` on which `is_f64` returns true, `as_f64` is guaranteed to
  /// return the floating point value.
  ///
  /// Currently this function returns true if and only if both `is_i64` and
  /// `is_u64` return false but this is not a guarantee in the future.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({ "a": 256.0, "b": 65, "c": -65 });
  /// assert!(obj["a"].is_f64());
  ///
  /// // Integers.
  /// assert!(!obj["b"].is_f64());
  /// assert!(!obj["c"].is_f64());
  /// ```
  #[inline]
  pub fn is_f64(&self) -> bool {
    #[cfg(not(feature = "arbitrary_precision"))]
    return matches!(self.n, NumImpl::Float(_));

    #[cfg(feature = "arbitrary_precision")]
    {
      for c in self.n.chars() {
        if c == '.' || c == 'e' || c == 'E' {
          return self.n.parse::<f64>().ok().map_or(false, |f| f.is_finite());
        }
      }
      false
    }
  }

  /// If the `Number` is an integer, represent it as `i64` if possible. Returns
  /// `None` otherwise.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let big = i64::MAX as u64 + 10;
  /// let obj = json!({ "a": 65, "b": big, "c": 256.0 });
  ///
  /// assert_eq!(obj["a"].as_i64(), Some(65));
  /// assert_eq!(obj["b"].as_i64(), None);
  /// assert_eq!(obj["c"].as_i64(), None);
  /// ```
  #[inline]
  pub fn as_i64(&self) -> Option<i64> {
    #[cfg(not(feature = "arbitrary_precision"))]
    match self.n {
      NumImpl::PositiveInt(n) => {
        if n <= i64::MAX as u64 {
          Some(n as i64)
        } else {
          None
        }
      }
      NumImpl::NegativeInt(n) => Some(n),
      NumImpl::Float(_) => None,
    }

    #[cfg(feature = "arbitrary_precision")]
    self.n.parse().ok()
  }

  /// If the `Number` is an integer, represent it as `u64` if possible. Returns
  /// `None` otherwise.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({ "a": 65, "b": -65, "c": 256.0 });
  ///
  /// assert_eq!(obj["a"].as_u64(), Some(65));
  /// assert_eq!(obj["b"].as_u64(), None);
  /// assert_eq!(obj["c"].as_u64(), None);
  /// ```
  #[inline]
  pub fn as_u64(&self) -> Option<u64> {
    #[cfg(not(feature = "arbitrary_precision"))]
    match self.n {
      NumImpl::PositiveInt(n) => Some(n),
      NumImpl::NegativeInt(_) | NumImpl::Float(_) => None,
    }

    #[cfg(feature = "arbitrary_precision")]
    self.n.parse().ok()
  }

  /// Represents the number as `f64` if possible. Returns `None` otherwise.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({ "a": 256.0, "b": 65, "c": -65 });
  ///
  /// assert_eq!(obj["a"].as_f64(), Some(256.0));
  /// assert_eq!(obj["b"].as_f64(), Some(65.0));
  /// assert_eq!(obj["c"].as_f64(), Some(-65.0));
  /// ```
  #[inline]
  pub fn as_f64(&self) -> Option<f64> {
    #[cfg(not(feature = "arbitrary_precision"))]
    match self.n {
      NumImpl::PositiveInt(u) => Some(u as f64),
      NumImpl::NegativeInt(i) => Some(i as f64),
      NumImpl::Float(f) => Some(f),
    }

    #[cfg(feature = "arbitrary_precision")]
    self.n.parse::<f64>().ok().filter(|f| f.is_finite())
  }

  /// Converts a finite `f64` to a `Number`. Infinite or `NaN` values are not
  /// represented.
  ///
  /// ```rust
  /// # use std::f64;
  /// # use sage::Number;
  /// #
  /// assert!(Number::from_f64(256.0).is_some());
  ///
  /// assert!(Number::from_f64(f64::NAN).is_none());
  /// ```
  #[inline]
  pub fn from_f64(f: f64) -> Option<Number> {
    if f.is_finite() {
      let n = {
        #[cfg(not(feature = "arbitrary_precision"))]
        {
          NumImpl::Float(f)
        }
        #[cfg(feature = "arbitrary_precision")]
        ryu::Buffer::new().format_finite(f).to_owned()
      };
      Some(Number { n })
    } else {
      None
    }
  }

  #[cfg(feature = "arbitrary_precision")]
  /// Not public API. Only test use this.
  #[doc(hidden)]
  #[inline]
  pub fn from_string_unchecked(n: String) -> Self {
    Number { n }
  }
}

impl fmt::Display for Number {
  #[cfg(not(feature = "arbitrary_precision"))]
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.n {
      NumImpl::Float(f) => fmt::Display::fmt(&f, formatter),
      NumImpl::PositiveInt(u) => fmt::Display::fmt(&u, formatter),
      NumImpl::NegativeInt(i) => fmt::Display::fmt(&i, formatter),
    }
  }

  #[cfg(feature = "arbitrary_precision")]
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&self.n, formatter)
  }
}

impl fmt::Debug for Number {
  #[cfg(not(feature = "arbitrary_precision"))]
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut debug = formatter.debug_tuple("Number");
    match self.n {
      NumImpl::PositiveInt(u) => debug.field(&u),
      NumImpl::NegativeInt(i) => debug.field(&i),
      NumImpl::Float(f) => debug.field(&f),
    };
    debug.finish()
  }

  #[cfg(feature = "arbitrary_precision")]
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter
      .debug_tuple("Number")
      .field(&format_args!("{}", self.n))
      .finish()
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `Serialize` for `Number`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

impl Serialize for Number {
  #[cfg(not(feature = "arbitrary_precision"))]
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match self.n {
      NumImpl::PositiveInt(u) => serializer.serialize_u64(u),
      NumImpl::NegativeInt(i) => serializer.serialize_i64(i),
      NumImpl::Float(f) => serializer.serialize_f64(f),
    }
  }

  #[cfg(feature = "arbitrary_precision")]
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    use serde::ser::SerializeStruct;

    let mut s = serializer.serialize_struct(TOKEN, 1)?;
    s.serialize_field(TOKEN, &self.n)?;
    s.end()
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `Deserialize` for `Number`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

impl<'de> Deserialize<'de> for Number {
  #[inline]
  fn deserialize<D>(deserilizer: D) -> Result<Number, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct NumberVisitor;

    impl<'de> Visitor<'de> for NumberVisitor {
      type Value = Number;

      fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a JSON number")
      }

      #[inline]
      fn visit_i64<E>(self, value: i64) -> Result<Number, E> {
        Ok(value.into())
      }

      #[inline]
      fn visit_u64<E>(self, value: u64) -> Result<Number, E> {
        Ok(value.into())
      }

      #[inline]
      fn visit_f64<E>(self, value: f64) -> Result<Number, E>
      where
        E: de::Error,
      {
        Number::from_f64(value)
          .ok_or_else(|| de::Error::custom("not a JSON number"))
      }

      #[cfg(feature = "arbitrary_precision")]
      #[inline]
      fn visit_map<V>(self, mut visitor: V) -> Result<Number, V::Error>
      where
        V: de::MapAccess<'de>,
      {
        let value = visitor.next_key::<NumberKey>()?;
        if value.is_none() {
          return Err(de::Error::invalid_type(Unexpected::Map, &self));
        }
        let v: NumberFromString = visitor.next_value()?;
        Ok(v.value)
      }
    }

    deserilizer.deserialize_any(NumberVisitor)
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `NumberKey` - feature = "arbitrary_precision".
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

#[cfg(feature = "arbitrary_precision")]
struct NumberKey;

#[cfg(feature = "arbitrary_precision")]
impl<'de> de::Deserialize<'de> for NumberKey {
  fn deserialize<D>(deserializer: D) -> Result<NumberKey, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    struct FieldVisitor;

    impl<'de> de::Visitor<'de> for FieldVisitor {
      type Value = ();

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid number field")
      }

      fn visit_str<E>(self, s: &str) -> Result<(), E>
      where
        E: de::Error,
      {
        if s == TOKEN {
          Ok(())
        } else {
          Err(de::Error::custom("expected field with custom name"))
        }
      }
    }

    deserializer.deserialize_identifier(FieldVisitor)?;
    Ok(NumberKey)
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `NumberFromString` - feature = "arbitrary_precision".
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

#[cfg(feature = "arbitrary_precision")]
pub struct NumberFromString {
  pub value: Number,
}

#[cfg(feature = "arbitrary_precision")]
impl<'de> de::Deserialize<'de> for NumberFromString {
  fn deserialize<D>(deserializer: D) -> Result<NumberFromString, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
      type Value = NumberFromString;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string containing a number")
      }

      fn visit_str<E>(self, s: &str) -> Result<NumberFromString, E>
      where
        E: de::Error,
      {
        let n = tri!(s.parse().map_err(de::Error::custom));
        Ok(NumberFromString { value: n })
      }
    }

    deserializer.deserialize_str(Visitor)
  }
}

#[cfg(feature = "arbitrary_precision")]
fn invalid_number() -> Error {
  Error::syntax(ErrorCode::InvalidNumber, 0, 0)
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `deserialize_any`, `deserialize_number` macro.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

macro_rules! deserialize_any {
  (@expand [$($num_string:tt)*]) => {
    #[cfg(not(feature = "arbitrary_precision"))]
    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
      V: Visitor<'de>,
    {
      match self.n {
        NumImpl::PositiveInt(u) => visitor.visit_u64(u),
        NumImpl::NegativeInt(i) => visitor.visit_i64(i),
        NumImpl::Float(f) => visitor.visit_f64(f),
      }
    }

    #[cfg(feature = "arbitrary_precision")]
    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
      V: Visitor<'de>,
    {
      if let Some(u) = self.as_u64() {
        return visitor.visit_u64(u);
      } else if let Some(i) = self.as_i64() {
        return visitor.visit_i64(i);
      } else if let Some(f) = self.as_f64() {
        if ryu::Buffer::new().format_finite(f) == self.n || f.to_string() == self.n {
          return visitor.visit_f64(f);
        }
      }

      visitor.visit_map(NumberDeserializer {
        number: Some(self.$($num_string)*),
      })
    }
  };

  (owned) => {
    deserialize_any!(@expand [n]);
  };

  (ref) => {
    deserialize_any!(@expand [n.clone()]);
  };
}

macro_rules! deserialize_number {
  ($deserialize:ident => $visit:ident) => {
    #[cfg(not(feature = "arbitrary_precision"))]
    fn $deserialize<V>(self, visitor: V) -> Result<V::Value, Error>
    where
      V: Visitor<'de>,
    {
      self.deserialize_any(visitor)
    }

    #[cfg(feature = "arbitrary_precision")]
    fn $deserialize<V>(self, visitor: V) -> Result<V::Value, Error>
    where
      V: de::Visitor<'de>,
    {
      visitor.$visit(self.n.parse().map_err(|_| invalid_number())?)
    }
  };
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `Deserializer` for `Number`. - owned
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

impl<'de> Deserializer<'de> for Number {
  type Error = Error;

  deserialize_any!(owned);

  deserialize_number!(deserialize_i8 => visit_i8);
  deserialize_number!(deserialize_i16 => visit_i16);
  deserialize_number!(deserialize_i32 => visit_i32);
  deserialize_number!(deserialize_i64 => visit_i64);
  deserialize_number!(deserialize_u8 => visit_u8);
  deserialize_number!(deserialize_u16 => visit_u16);
  deserialize_number!(deserialize_u32 => visit_u16);
  deserialize_number!(deserialize_u64 => visit_u64);
  deserialize_number!(deserialize_f32 => visit_f32);
  deserialize_number!(deserialize_f64 => visit_f64);

  serde_if_integer128! {
    deserialize_number!(deserialize_i128 => visit_i128);
    deserialize_number!(deserialize_u128 => visit_u128);
  }

  forward_to_deserialize_any! {
      bool char str string bytes byte_buf option unit unit_struct
      newtype_struct seq tuple tuple_struct map struct enum identifier
      ignored_any
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `Deserializer` for `&Number` - ref.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

impl<'de, 'a> Deserializer<'de> for &'a Number {
  type Error = Error;

  deserialize_any!(ref);

  deserialize_number!(deserialize_i8 => visit_i8);
  deserialize_number!(deserialize_i16 => visit_i16);
  deserialize_number!(deserialize_i32 => visit_i32);
  deserialize_number!(deserialize_i64 => visit_i64);
  deserialize_number!(deserialize_u8 => visit_u8);
  deserialize_number!(deserialize_u16 => visit_u16);
  deserialize_number!(deserialize_u32 => visit_u16);
  deserialize_number!(deserialize_u64 => visit_u64);
  deserialize_number!(deserialize_f32 => visit_f32);
  deserialize_number!(deserialize_f64 => visit_f64);

  serde_if_integer128! {
    deserialize_number!(deserialize_i128 => visit_i128);
    deserialize_number!(deserialize_u128 => visit_u128);
  }

  forward_to_deserialize_any! {
      bool char str string bytes byte_buf option unit unit_struct
      newtype_struct seq tuple tuple_struct map struct enum identifier
      ignored_any
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `NumberDeserializer` - feature = "arbitrary_precision".
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

#[cfg(feature = "arbitrary_precision")]
pub(crate) struct NumberDeserializer {
  pub number: Option<String>,
}

#[cfg(feature = "arbitrary_precision")]
impl<'de> MapAccess<'de> for NumberDeserializer {
  type Error = Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
  where
    K: de::DeserializeSeed<'de>,
  {
    if self.number.is_none() {
      return Ok(None);
    }
    seed.deserialize(NumberFieldDeserializer).map(Some)
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
  where
    V: de::DeserializeSeed<'de>,
  {
    seed.deserialize(self.number.take().unwrap().into_deserializer())
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `NumberFieldDeserializer` - feature = "arbitrary_precision".
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

#[cfg(feature = "arbitrary_precision")]
struct NumberFieldDeserializer;

#[cfg(feature = "arbitrary_precision")]
impl<'de> Deserializer<'de> for NumberFieldDeserializer {
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
 * | | `impl_from_unsigned`, `impl_fromsigned` macros.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

//
// Implement From trait for Rust integer unsigned types.
//
macro_rules! impl_from_unsigned {
  (
    $($ty:ty),*
  ) => {
    $(
      impl From<$ty> for Number {
        #[inline]
        fn from(u: $ty) -> Self {
          let n = {
            #[cfg(not(feature = "arbitrary_precision"))]
            { NumImpl::PositiveInt(u as u64) }
            #[cfg(feature = "arbitrary_precision")]
            { itoa::Buffer::new().format(u).to_owned() }
          };

          Number { n }
        }
      }
    )*
  };
}

//
// Implement From trait for Rust integer signed types.
//
macro_rules! impl_from_signed {
  (
    $($ty:ty),*
  ) => {
    $(
      impl From<$ty> for Number {
        #[inline]
        fn from(i: $ty) -> Self {
          let n = {
            #[cfg(not(feature = "arbitrary_precision"))]
            {
              if i < 0 {
                NumImpl::NegativeInt(i as i64)
              } else {
                NumImpl::PositiveInt(i as u64)
              }
            }
            #[cfg(feature = "arbitrary_precision")]
            { itoa::Buffer::new().format(i).to_owned()}
          };
          Number { n }
        }
      }
    )*
  };
}

impl_from_unsigned!(u8, u16, u32, u64, usize);
impl_from_signed!(i8, i16, i32, i64, isize);

#[cfg(feature = "arbitrary_precision")]
serde_if_integer128! {
  impl From<i128> for Number {
    fn from(i: i128) -> Self {
      Number { n: i.to_string() }
    }
  }

  impl From<u128> for Number {
    fn from(u: u128) -> Self {
      Number { n: u.to_string() }
    }
  }
}

impl Number {
  #[cfg(not(feature = "arbitrary_precision"))]
  #[cold]
  pub(crate) fn unexpected(&self) -> Unexpected {
    match self.n {
      NumImpl::PositiveInt(u) => Unexpected::Unsigned(u),
      NumImpl::NegativeInt(i) => Unexpected::Signed(i),
      NumImpl::Float(f) => Unexpected::Float(f),
    }
  }

  #[cfg(feature = "arbitrary_precision")]
  #[cold]
  pub(crate) fn unexpected(&self) -> Unexpected {
    Unexpected::Other("number")
  }
}
