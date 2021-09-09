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

use std::fmt;

use crate::{to_dtype, DType, Error, ErrorCode, Map, Number, Result};
use serde::ser::{Impossible, Serialize};

#[cfg(feature = "arbitrary_precision")]
use serde::serde_if_integer128;

impl Serialize for DType {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match *self {
      DType::Null => serializer.serialize_unit(),
      DType::Boolean(b) => serializer.serialize_bool(b),
      DType::Number(ref n) => n.serialize(serializer),
      DType::String(ref s) => serializer.serialize_str(s),
      DType::Array(ref v) => v.serialize(serializer),
      DType::Object(ref m) => {
        use serde::ser::SerializeMap;
        let mut map = tri!(serializer.serialize_map(Some(m.len())));
        for (k, v) in m {
          tri!(map.serialize_entry(k, v));
        }
        map.end()
      }
      // TODO: Handle `DateTime`.
      DType::DateTime(_) => todo!(),
    }
  }
}

/// Serializer whose output is a `DType`.
///
/// This is the serializer that backs [`sage::to_dtype`][crate::to_dtype].
/// Unlike the main sage serializer which goes from some serializable
/// value of type `T` to JSON text, this one goes from `T` to
/// `sage::DType`.
///
/// The `to_dtype` function is implementable as:
///
/// ```rust,ignore
/// use serde::Serialize;
/// use sage::{Error, DType};
///
/// pub fn to_dtype<T>(input: T) -> Result<DType, Error>
/// where
///     T: Serialize,
/// {
///     input.serialize(sage::json::Serializer)
/// }
/// ```
pub struct Serializer;

impl serde::Serializer for Serializer {
  type Ok = DType;
  type Error = Error;

  type SerializeSeq = SerializeVec;
  type SerializeTuple = SerializeVec;
  type SerializeTupleStruct = SerializeVec;
  type SerializeTupleVariant = SerializeTupleVariant;
  type SerializeMap = SerializeMap;
  type SerializeStruct = SerializeMap;
  type SerializeStructVariant = SerializeStructVariant;

  #[inline]
  fn serialize_bool(self, value: bool) -> Result<DType> {
    Ok(DType::Boolean(value))
  }

  #[inline]
  fn serialize_i8(self, value: i8) -> Result<DType> {
    self.serialize_i64(value as i64)
  }

  #[inline]
  fn serialize_i16(self, value: i16) -> Result<DType> {
    self.serialize_i64(value as i64)
  }

  #[inline]
  fn serialize_i32(self, value: i32) -> Result<DType> {
    self.serialize_i64(value as i64)
  }

  fn serialize_i64(self, value: i64) -> Result<DType> {
    Ok(DType::Number(value.into()))
  }

  #[cfg(feature = "arbitrary_precision")]
  serde_if_integer128! {
      fn serialize_i128(self, value: i128) -> Result<DType> {
          Ok(DType::Number(value.into()))
      }
  }

  #[inline]
  fn serialize_u8(self, value: u8) -> Result<DType> {
    self.serialize_u64(value as u64)
  }

  #[inline]
  fn serialize_u16(self, value: u16) -> Result<DType> {
    self.serialize_u64(value as u64)
  }

  #[inline]
  fn serialize_u32(self, value: u32) -> Result<DType> {
    self.serialize_u64(value as u64)
  }

  #[inline]
  fn serialize_u64(self, value: u64) -> Result<DType> {
    Ok(DType::Number(value.into()))
  }

  #[cfg(feature = "arbitrary_precision")]
  serde_if_integer128! {
      fn serialize_u128(self, value: u128) -> Result<DType> {
          Ok(DType::Number(value.into()))
      }
  }

  #[inline]
  fn serialize_f32(self, value: f32) -> Result<DType> {
    self.serialize_f64(value as f64)
  }

  #[inline]
  fn serialize_f64(self, value: f64) -> Result<DType> {
    Ok(Number::from_f64(value).map_or(DType::Null, DType::Number))
  }

  #[inline]
  fn serialize_char(self, value: char) -> Result<DType> {
    let mut s = String::new();
    s.push(value);
    Ok(DType::String(s))
  }

  #[inline]
  fn serialize_str(self, value: &str) -> Result<DType> {
    Ok(DType::String(value.to_owned()))
  }

  fn serialize_bytes(self, value: &[u8]) -> Result<DType> {
    let vec = value.iter().map(|&b| DType::Number(b.into())).collect();
    Ok(DType::Array(vec))
  }

  #[inline]
  fn serialize_unit(self) -> Result<DType> {
    Ok(DType::Null)
  }

  #[inline]
  fn serialize_unit_struct(self, _name: &'static str) -> Result<DType> {
    self.serialize_unit()
  }

  #[inline]
  fn serialize_unit_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
  ) -> Result<DType> {
    self.serialize_str(variant)
  }

  #[inline]
  fn serialize_newtype_struct<T>(
    self,
    _name: &'static str,
    value: &T,
  ) -> Result<DType>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(self)
  }

  fn serialize_newtype_variant<T>(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    value: &T,
  ) -> Result<DType>
  where
    T: ?Sized + Serialize,
  {
    let mut values = Map::new();
    values.insert(String::from(variant), tri!(to_dtype(&value)));
    Ok(DType::Object(values))
  }

  #[inline]
  fn serialize_none(self) -> Result<DType> {
    self.serialize_unit()
  }

  #[inline]
  fn serialize_some<T>(self, value: &T) -> Result<DType>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(self)
  }

  fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
    Ok(SerializeVec {
      vec: Vec::with_capacity(len.unwrap_or(0)),
    })
  }

  fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
    self.serialize_seq(Some(len))
  }

  fn serialize_tuple_struct(
    self,
    _name: &'static str,
    len: usize,
  ) -> Result<Self::SerializeTupleStruct> {
    self.serialize_seq(Some(len))
  }

  fn serialize_tuple_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    len: usize,
  ) -> Result<Self::SerializeTupleVariant> {
    Ok(SerializeTupleVariant {
      name: String::from(variant),
      vec: Vec::with_capacity(len),
    })
  }

  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
    Ok(SerializeMap::Map {
      map: Map::new(),
      next_key: None,
    })
  }

  fn serialize_struct(
    self,
    name: &'static str,
    len: usize,
  ) -> Result<Self::SerializeStruct> {
    match name {
      #[cfg(feature = "arbitrary_precision")]
      crate::number::TOKEN => Ok(SerializeMap::Number { out_value: None }),
      #[cfg(feature = "raw_value")]
      crate::raw::TOKEN => Ok(SerializeMap::RawDType { out_value: None }),
      _ => self.serialize_map(Some(len)),
    }
  }

  fn serialize_struct_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant> {
    Ok(SerializeStructVariant {
      name: String::from(variant),
      map: Map::new(),
    })
  }

  fn collect_str<T: ?Sized>(self, value: &T) -> Result<DType>
  where
    T: fmt::Display,
  {
    Ok(DType::String(value.to_string()))
  }
}

pub struct SerializeVec {
  vec: Vec<DType>,
}

pub struct SerializeTupleVariant {
  name: String,
  vec: Vec<DType>,
}

pub enum SerializeMap {
  Map {
    map: Map<String, DType>,
    next_key: Option<String>,
  },
  #[cfg(feature = "arbitrary_precision")]
  Number { out_value: Option<DType> },
  #[cfg(feature = "raw_value")]
  RawDType { out_value: Option<DType> },
}

pub struct SerializeStructVariant {
  name: String,
  map: Map<String, DType>,
}

impl serde::ser::SerializeSeq for SerializeVec {
  type Ok = DType;
  type Error = Error;

  fn serialize_element<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    self.vec.push(tri!(to_dtype(&value)));
    Ok(())
  }

  fn end(self) -> Result<DType> {
    Ok(DType::Array(self.vec))
  }
}

impl serde::ser::SerializeTuple for SerializeVec {
  type Ok = DType;
  type Error = Error;

  fn serialize_element<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    serde::ser::SerializeSeq::serialize_element(self, value)
  }

  fn end(self) -> Result<DType> {
    serde::ser::SerializeSeq::end(self)
  }
}

impl serde::ser::SerializeTupleStruct for SerializeVec {
  type Ok = DType;
  type Error = Error;

  fn serialize_field<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    serde::ser::SerializeSeq::serialize_element(self, value)
  }

  fn end(self) -> Result<DType> {
    serde::ser::SerializeSeq::end(self)
  }
}

impl serde::ser::SerializeTupleVariant for SerializeTupleVariant {
  type Ok = DType;
  type Error = Error;

  fn serialize_field<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    self.vec.push(tri!(to_dtype(&value)));
    Ok(())
  }

  fn end(self) -> Result<DType> {
    let mut object = Map::new();

    object.insert(self.name, DType::Array(self.vec));

    Ok(DType::Object(object))
  }
}

impl serde::ser::SerializeMap for SerializeMap {
  type Ok = DType;
  type Error = Error;

  fn serialize_key<T>(&mut self, key: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    match *self {
      SerializeMap::Map {
        ref mut next_key, ..
      } => {
        *next_key = Some(tri!(key.serialize(MapKeySerializer)));
        Ok(())
      }
      #[cfg(feature = "arbitrary_precision")]
      SerializeMap::Number { .. } => unreachable!(),
      #[cfg(feature = "raw_value")]
      SerializeMap::RawDType { .. } => unreachable!(),
    }
  }

  fn serialize_value<T>(&mut self, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    match *self {
      SerializeMap::Map {
        ref mut map,
        ref mut next_key,
      } => {
        let key = next_key.take();
        // Panic because this indicates a bug in the program rather than an
        // expected failure.
        let key = key.expect("serialize_value called before serialize_key");
        map.insert(key, tri!(to_dtype(&value)));
        Ok(())
      }
      #[cfg(feature = "arbitrary_precision")]
      SerializeMap::Number { .. } => unreachable!(),
      #[cfg(feature = "raw_value")]
      SerializeMap::RawDType { .. } => unreachable!(),
    }
  }

  fn end(self) -> Result<DType> {
    match self {
      SerializeMap::Map { map, .. } => Ok(DType::Object(map)),
      #[cfg(feature = "arbitrary_precision")]
      SerializeMap::Number { .. } => unreachable!(),
      #[cfg(feature = "raw_value")]
      SerializeMap::RawDType { .. } => unreachable!(),
    }
  }
}

struct MapKeySerializer;

fn key_must_be_a_string() -> Error {
  Error::syntax(ErrorCode::KeyMustBeAString, 0, 0)
}

impl serde::Serializer for MapKeySerializer {
  type Ok = String;
  type Error = Error;

  type SerializeSeq = Impossible<String, Error>;
  type SerializeTuple = Impossible<String, Error>;
  type SerializeTupleStruct = Impossible<String, Error>;
  type SerializeTupleVariant = Impossible<String, Error>;
  type SerializeMap = Impossible<String, Error>;
  type SerializeStruct = Impossible<String, Error>;
  type SerializeStructVariant = Impossible<String, Error>;

  #[inline]
  fn serialize_unit_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
  ) -> Result<String> {
    Ok(variant.to_owned())
  }

  #[inline]
  fn serialize_newtype_struct<T>(
    self,
    _name: &'static str,
    value: &T,
  ) -> Result<String>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(self)
  }

  fn serialize_bool(self, _value: bool) -> Result<String> {
    Err(key_must_be_a_string())
  }

  fn serialize_i8(self, value: i8) -> Result<String> {
    Ok(value.to_string())
  }

  fn serialize_i16(self, value: i16) -> Result<String> {
    Ok(value.to_string())
  }

  fn serialize_i32(self, value: i32) -> Result<String> {
    Ok(value.to_string())
  }

  fn serialize_i64(self, value: i64) -> Result<String> {
    Ok(value.to_string())
  }

  fn serialize_u8(self, value: u8) -> Result<String> {
    Ok(value.to_string())
  }

  fn serialize_u16(self, value: u16) -> Result<String> {
    Ok(value.to_string())
  }

  fn serialize_u32(self, value: u32) -> Result<String> {
    Ok(value.to_string())
  }

  fn serialize_u64(self, value: u64) -> Result<String> {
    Ok(value.to_string())
  }

  fn serialize_f32(self, _value: f32) -> Result<String> {
    Err(key_must_be_a_string())
  }

  fn serialize_f64(self, _value: f64) -> Result<String> {
    Err(key_must_be_a_string())
  }

  #[inline]
  fn serialize_char(self, value: char) -> Result<String> {
    Ok({
      let mut s = String::new();
      s.push(value);
      s
    })
  }

  #[inline]
  fn serialize_str(self, value: &str) -> Result<String> {
    Ok(value.to_owned())
  }

  fn serialize_bytes(self, _value: &[u8]) -> Result<String> {
    Err(key_must_be_a_string())
  }

  fn serialize_unit(self) -> Result<String> {
    Err(key_must_be_a_string())
  }

  fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {
    Err(key_must_be_a_string())
  }

  fn serialize_newtype_variant<T>(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
    _value: &T,
  ) -> Result<String>
  where
    T: ?Sized + Serialize,
  {
    Err(key_must_be_a_string())
  }

  fn serialize_none(self) -> Result<String> {
    Err(key_must_be_a_string())
  }

  fn serialize_some<T>(self, _value: &T) -> Result<String>
  where
    T: ?Sized + Serialize,
  {
    Err(key_must_be_a_string())
  }

  fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
    Err(key_must_be_a_string())
  }

  fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
    Err(key_must_be_a_string())
  }

  fn serialize_tuple_struct(
    self,
    _name: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleStruct> {
    Err(key_must_be_a_string())
  }

  fn serialize_tuple_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleVariant> {
    Err(key_must_be_a_string())
  }

  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
    Err(key_must_be_a_string())
  }

  fn serialize_struct(
    self,
    _name: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStruct> {
    Err(key_must_be_a_string())
  }

  fn serialize_struct_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant> {
    Err(key_must_be_a_string())
  }

  fn collect_str<T: ?Sized>(self, value: &T) -> Result<String>
  where
    T: fmt::Display,
  {
    Ok(value.to_string())
  }
}

impl serde::ser::SerializeStruct for SerializeMap {
  type Ok = DType;
  type Error = Error;

  fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    match *self {
      SerializeMap::Map { .. } => {
        serde::ser::SerializeMap::serialize_entry(self, key, value)
      }
      #[cfg(feature = "arbitrary_precision")]
      SerializeMap::Number { ref mut out_value } => {
        if key == crate::number::TOKEN {
          *out_value = Some(value.serialize(NumberDTypeEmitter)?);
          Ok(())
        } else {
          Err(invalid_number())
        }
      }
      #[cfg(feature = "raw_value")]
      SerializeMap::RawDType { ref mut out_value } => {
        if key == crate::raw::TOKEN {
          *out_value = Some(value.serialize(RawDTypeEmitter)?);
          Ok(())
        } else {
          Err(invalid_raw_value())
        }
      }
    }
  }

  fn end(self) -> Result<DType> {
    match self {
      SerializeMap::Map { .. } => serde::ser::SerializeMap::end(self),
      #[cfg(feature = "arbitrary_precision")]
      SerializeMap::Number { out_value, .. } => {
        Ok(out_value.expect("number value was not emitted"))
      }
      #[cfg(feature = "raw_value")]
      SerializeMap::RawDType { out_value, .. } => {
        Ok(out_value.expect("raw value was not emitted"))
      }
    }
  }
}

impl serde::ser::SerializeStructVariant for SerializeStructVariant {
  type Ok = DType;
  type Error = Error;

  fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
  where
    T: ?Sized + Serialize,
  {
    self.map.insert(String::from(key), tri!(to_dtype(&value)));
    Ok(())
  }

  fn end(self) -> Result<DType> {
    let mut object = Map::new();

    object.insert(self.name, DType::Object(self.map));

    Ok(DType::Object(object))
  }
}

#[cfg(feature = "arbitrary_precision")]
struct NumberDTypeEmitter;

#[cfg(feature = "arbitrary_precision")]
fn invalid_number() -> Error {
  Error::syntax(ErrorCode::InvalidNumber, 0, 0)
}

#[cfg(feature = "arbitrary_precision")]
impl serde::ser::Serializer for NumberDTypeEmitter {
  type Ok = DType;
  type Error = Error;

  type SerializeSeq = Impossible<DType, Error>;
  type SerializeTuple = Impossible<DType, Error>;
  type SerializeTupleStruct = Impossible<DType, Error>;
  type SerializeTupleVariant = Impossible<DType, Error>;
  type SerializeMap = Impossible<DType, Error>;
  type SerializeStruct = Impossible<DType, Error>;
  type SerializeStructVariant = Impossible<DType, Error>;

  fn serialize_bool(self, _v: bool) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_i8(self, _v: i8) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_i16(self, _v: i16) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_i32(self, _v: i32) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_i64(self, _v: i64) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_u8(self, _v: u8) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_u16(self, _v: u16) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_u32(self, _v: u32) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_u64(self, _v: u64) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_f32(self, _v: f32) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_f64(self, _v: f64) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_char(self, _v: char) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_str(self, value: &str) -> Result<DType> {
    let n = tri!(value.to_owned().parse());
    Ok(DType::Number(n))
  }

  fn serialize_bytes(self, _value: &[u8]) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_none(self) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_some<T>(self, _value: &T) -> Result<DType>
  where
    T: ?Sized + Serialize,
  {
    Err(invalid_number())
  }

  fn serialize_unit(self) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_unit_struct(self, _name: &'static str) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_unit_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
  ) -> Result<DType> {
    Err(invalid_number())
  }

  fn serialize_newtype_struct<T>(
    self,
    _name: &'static str,
    _value: &T,
  ) -> Result<DType>
  where
    T: ?Sized + Serialize,
  {
    Err(invalid_number())
  }

  fn serialize_newtype_variant<T>(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
    _value: &T,
  ) -> Result<DType>
  where
    T: ?Sized + Serialize,
  {
    Err(invalid_number())
  }

  fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
    Err(invalid_number())
  }

  fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
    Err(invalid_number())
  }

  fn serialize_tuple_struct(
    self,
    _name: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleStruct> {
    Err(invalid_number())
  }

  fn serialize_tuple_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleVariant> {
    Err(invalid_number())
  }

  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
    Err(invalid_number())
  }

  fn serialize_struct(
    self,
    _name: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStruct> {
    Err(invalid_number())
  }

  fn serialize_struct_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant> {
    Err(invalid_number())
  }
}

#[cfg(feature = "raw_value")]
struct RawDTypeEmitter;

#[cfg(feature = "raw_value")]
fn invalid_raw_value() -> Error {
  Error::syntax(ErrorCode::ExpectedSomeDType, 0, 0)
}

#[cfg(feature = "raw_value")]
impl serde::ser::Serializer for RawDTypeEmitter {
  type Ok = DType;
  type Error = Error;

  type SerializeSeq = Impossible<DType, Error>;
  type SerializeTuple = Impossible<DType, Error>;
  type SerializeTupleStruct = Impossible<DType, Error>;
  type SerializeTupleVariant = Impossible<DType, Error>;
  type SerializeMap = Impossible<DType, Error>;
  type SerializeStruct = Impossible<DType, Error>;
  type SerializeStructVariant = Impossible<DType, Error>;

  fn serialize_bool(self, _v: bool) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_i8(self, _v: i8) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_i16(self, _v: i16) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_i32(self, _v: i32) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_i64(self, _v: i64) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_u8(self, _v: u8) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_u16(self, _v: u16) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_u32(self, _v: u32) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_u64(self, _v: u64) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_f32(self, _v: f32) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_f64(self, _v: f64) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_char(self, _v: char) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_str(self, value: &str) -> Result<DType> {
    crate::from_str(value)
  }

  fn serialize_bytes(self, _value: &[u8]) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_none(self) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_some<T>(self, _value: &T) -> Result<DType>
  where
    T: ?Sized + Serialize,
  {
    Err(invalid_raw_value())
  }

  fn serialize_unit(self) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_unit_struct(self, _name: &'static str) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_unit_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
  ) -> Result<DType> {
    Err(invalid_raw_value())
  }

  fn serialize_newtype_struct<T>(
    self,
    _name: &'static str,
    _value: &T,
  ) -> Result<DType>
  where
    T: ?Sized + Serialize,
  {
    Err(invalid_raw_value())
  }

  fn serialize_newtype_variant<T>(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
    _value: &T,
  ) -> Result<DType>
  where
    T: ?Sized + Serialize,
  {
    Err(invalid_raw_value())
  }

  fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
    Err(invalid_raw_value())
  }

  fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
    Err(invalid_raw_value())
  }

  fn serialize_tuple_struct(
    self,
    _name: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleStruct> {
    Err(invalid_raw_value())
  }

  fn serialize_tuple_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleVariant> {
    Err(invalid_raw_value())
  }

  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
    Err(invalid_raw_value())
  }

  fn serialize_struct(
    self,
    _name: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStruct> {
    Err(invalid_raw_value())
  }

  fn serialize_struct_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant> {
    Err(invalid_raw_value())
  }
}
