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

//! `sage::types` contains all/most types used by the `sage` engine. Many types wrap
//! native Rust types. Although it's highly advised to use these types rather than
//! native rust because they include extended functionalities and can also be dereferenced
//! back and forth into native Rust types like [Strings] and sage types.
//!
//! [Strings]: https://doc.rust-lang.org/stable/alloc/string/struct.String.html

use std::fmt;

use serde::{de::DeserializeOwned, ser::Serialize};

use crate::Result;

pub mod datetime;
pub mod index;
pub mod map;
pub mod number;

pub use {self::datetime::DateTime, map::Map, number::Number, ser::Serializer};

/// `IRI` stands for International Resource Identifer. (ex: <name>).
pub type IRI = String;

/// `URI` is used to represent any URL-like `IRI`.
pub type URI = String;

/// `DType` represents the various types which data in the Sage Knowledge
/// Graph can be represented as.
#[derive(Clone, Eq, PartialEq)]
pub enum DType {
  /// Represents a collection of values.
  Array(Vec<DType>),

  /// Represents a boolean (true or false) value.
  Boolean(bool),

  /// Represents date, time or datetime.
  DateTime(DateTime),

  /// Represents a JSON null value.
  Null,

  /// Represents a numeric value,
  /// either integer or floating point values.
  Number(Number),

  /// Represents an object type with Key & values.
  ///
  /// By default it uses `BTreeMap` which does not preserve
  /// the entries' order.
  Object(Map<String, DType>),

  /// Represents a String or string-like value.
  String(String),
}

impl fmt::Debug for DType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      DType::Null => f.debug_tuple("Null").finish(),
      DType::Boolean(b) => f.debug_tuple("Boolean").field(&b).finish(),
      DType::Number(ref n) => fmt::Debug::fmt(&n, f),
      DType::String(ref s) => f.debug_tuple("Sting").field(s).finish(),
      DType::Array(ref a) => {
        f.write_str("Array(")?;
        fmt::Debug::fmt(a, f)?;
        f.write_str(")")
      }
      DType::Object(ref o) => {
        f.write_str("Object(")?;
        fmt::Debug::fmt(o, f)?;
        f.write_str(")")
      }
      DType::DateTime(ref d) => fmt::Debug::fmt(&d, f),
    }
  }
}

impl fmt::Display for DType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      DType::Null => f.write_str("null"),
      DType::Boolean(b) => write!(f, "{}", b),
      DType::String(s) => f.write_str(s),
      // For every other variant, use the Debug trait.
      _ => fmt::Debug::fmt(self, f),
    }
  }
}

impl DType {
  pub fn get<I: index::Index>(&self, index: I) -> Option<&DType> {
    index.index_into(self)
  }

  pub fn get_mut<I: index::Index>(&mut self, index: I) -> Option<&mut DType> {
    index.index_into_mut(self)
  }

  pub fn is_object(&self) -> bool {
    self.as_object().is_some()
  }

  pub fn as_object(&self) -> Option<&Map<String, DType>> {
    match *self {
      DType::Object(ref m) => Some(m),
      _ => None,
    }
  }

  pub fn as_object_mut(&mut self) -> Option<&mut Map<String, DType>> {
    match *self {
      DType::Object(ref mut m) => Some(m),
      _ => None,
    }
  }

  pub fn is_array(&self) -> bool {
    self.as_array().is_some()
  }

  pub fn as_array(&self) -> Option<&Vec<DType>> {
    match *self {
      DType::Array(ref a) => Some(&*a),
      _ => None,
    }
  }

  pub fn as_array_mut(&mut self) -> Option<&mut Vec<DType>> {
    match *self {
      DType::Array(ref mut v) => Some(v),
      _ => None,
    }
  }

  pub fn is_string(&self) -> bool {
    self.as_str().is_some()
  }
  /// If the `DType` is a String, returns the associated str. Returns None
  /// otherwise
  ///
  /// ```rust
  /// # use sage::DType;
  /// #
  /// let foo = DType::String("foo".to_string());
  /// assert_eq!(foo.as_str(), Some("foo"));
  ///
  /// // The value is: "foo"
  /// println!("The value of foo: {}", foo);
  /// ```
  pub fn as_str(&self) -> Option<&str> {
    match *self {
      DType::String(ref s) => Some(s),
      _ => None,
    }
  }

  /// Returns true if the `DType` is a number. Returns false otherwise.
  ///
  pub fn is_number(&self) -> bool {
    matches!(*self, DType::Number(_))
  }

  /// Returns true if the `DType` is an integer between `i64::MIN` and
  /// `i64::MAX`.
  ///
  /// For any `DType` on which `is_i64` returns true, `as_i64` is guaranteed to
  /// return the integer value.
  ///
  pub fn is_i64(&self) -> bool {
    matches!(*self, DType::Number(ref n) if n.is_i64())
  }

  /// Returns true if the `DType` is an integer between zero and `u64::MAX`.
  ///
  /// For any `DType` on which `is_u64` returns true, `as_u64` is guaranteed to
  /// return the integer value.
  ///
  pub fn is_u64(&self) -> bool {
    matches!(*self, DType::Number(ref n) if n.is_u64())
  }

  /// Returns true if `DType` is a number that can be represented by `f64`.
  ///
  /// For any `DType` on which `is_f64` returns true, `as_f64` is guaranteed to
  /// return the floating point value.
  ///
  /// Currently this function returns true if and only if both `is_i64` and
  /// `is_u64` return false but this is not a guarantee in the future.
  ///
  pub fn is_f64(&self) -> bool {
    // match *self {
    //   DType::Number(ref n) => n.is_f64(),
    //   _ => false,
    // }
    matches!(*self, DType::Number(ref n) if n.is_f64())
  }

  /// If the `DType` is an integer, represent it as `i64` if possible. Returns
  /// `None` otherwise.
  ///
  pub fn as_i64(&self) -> Option<i64> {
    match *self {
      DType::Number(ref n) => n.as_i64(),
      _ => None,
    }
  }

  /// If the `DType` is an integer, represent it as `u64` if possible. Returns
  /// `None` otherwise.
  ///
  pub fn as_u64(&self) -> Option<u64> {
    match *self {
      DType::Number(ref n) => n.as_u64(),
      _ => None,
    }
  }

  /// If the `DType` is a number, represent it as `f64` if possible. Returns
  /// `None` otherwise.
  ///
  pub fn as_f64(&self) -> Option<f64> {
    match *self {
      DType::Number(ref n) => n.as_f64(),
      _ => None,
    }
  }

  /// Returns true if `DType` is a `Boolean`. Returns false otherwise.
  ///
  /// For any `DType` on which `is_bool` returns true, `as_bool` is
  /// guaranteed to return the boolean value.
  ///
  /// ```rust
  /// # use sage::DType;
  /// #
  /// let value = DType::Boolean(false);
  /// assert!(value.is_bool());
  ///
  /// // The string "false" is a string, not a boolean.
  /// let not_bool = DType::String("false".to_string());
  /// assert!(!not_bool.is_bool());
  /// ```
  pub fn is_bool(&self) -> bool {
    self.as_bool().is_some()
  }

  /// If the `DType` is a `Boolean`, returns the associated bool.
  /// Returns `None` otherwise.
  ///
  /// ```rust
  /// # use sage::DType;
  /// #
  /// let value = DType::Boolean(false);
  /// assert_eq!(value.as_bool(), Some(false));
  ///
  /// // The string "false" is a string, not a boolean.
  /// let not_bool = DType::String("false".to_string());
  /// assert_eq!(not_bool.as_bool(), None)
  /// ```
  pub fn as_bool(&self) -> Option<bool> {
    match *self {
      DType::Boolean(b) => Some(b),
      _ => None,
    }
  }

  /// Returns true if the `DType` is a `Null`. Returns false otherwise.
  ///
  /// For any `DType` on which `is_null` returns true, `as_null` is guaranteed
  /// to return `Some(())`.
  ///
  /// ```rust
  /// # use sage::DType;
  /// #
  /// let value = DType::Null;
  /// assert!(value.is_null());
  ///
  /// // The boolean `false` is not null.
  /// let not_null =  DType::Boolean(false);
  /// assert!(!not_null.is_null());
  /// ```
  pub fn is_null(&self) -> bool {
    self.as_null().is_some()
  }

  /// If the `DType` is a `Null`, return `()` *(unit type)*. Returns `None` otherwise.
  ///
  /// ```
  /// # use sage::DType;
  /// #
  /// let value = DType::Null;
  /// assert_eq!(value.as_null(), Some(()));
  ///
  /// // The boolean `false` is not null.
  /// let not_null = DType::Boolean(false);
  /// assert_eq!(not_null.as_null(), None);
  /// ```
  pub fn as_null(&self) -> Option<()> {
    match *self {
      DType::Null => Some(()),
      _ => None,
    }
  }

  /// Takes the value of the `DType`, leaving a `Null` in its place.
  ///
  /// # Example
  ///
  /// ```rust
  /// # use sage::DType;
  ///
  /// let mut value = DType::String("Foo".to_string());
  /// assert_eq!(value.take(), DType::String("Foo".to_string()));
  ///
  /// assert_eq!(value, DType::Null);
  /// ```
  pub fn take(&mut self) -> DType {
    std::mem::replace(self, DType::Null)
  }
}

impl Default for DType {
  fn default() -> DType {
    DType::Null
  }
}

// impl Serialize for DType {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: serde::Serializer,
//   {
//     match *self {
//       DType::Null => serializer.serialize_unit(),
//       DType::Boolean(b) => serializer.serialize_bool(b),
//       DType::Number(ref n) => n.serialize(serializer),
//       DType::String(ref s) => serializer.serialize_str(s),
//       DType::Array(ref v) => v.serialize(serializer),
//       // #[cfg(any(feature = "std", feature = "alloc"))]
//       DType::Object(ref m) => {
//         use serde::ser::SerializeMap;
//         let mut map = tri!(serializer.serialize_map(Some(m.len())));
//         for (k, v) in m {
//           tri!(map.serialize_entry(k, v));
//         }
//         map.end()
//       }
//       // TODO: Handle `DateTime`.
//       DType::DateTime(_) => todo!(),
//     }
//   }
// }

mod de;
mod from;
mod partial_eq;
mod ser;

pub fn to_dtype<T>(value: T) -> Result<DType>
where
  T: Serialize,
{
  value.serialize(ser::Serializer)
}

pub fn from_dtype<T>(value: DType) -> Result<T>
where
  T: DeserializeOwned,
{
  T::deserialize(value)
}

/*
/// The basic data types such as Integers, Strings, etc.
const DataType: IRI = prefix + "DataType";
/// Boolean: True or False.
const Boolean: IRI = prefix + "Boolean";
/// The boolean value true.
const True: IRI = prefix + "True";
/// The boolean value false.
const False: IRI = prefix + "False";
/// Data type: Text.
const Text: IRI = prefix + "Text";
/// Data type: URL.
const URL: IRI = prefix + "URL";
/// Data type: Number.
const Number: IRI = prefix + "Number";
/// Data type: Integer.
const Integer: IRI = prefix + "Integer";
/// Data type: Floating number.
const Float: IRI = prefix + "Float";
/// A date value in ISO 8601 date format.
const Date: IRI = prefix + "Date";
/// A point in time recurring on multiple days in the
/// form hh:mm:ss[Z|(+|-)hh:mm].
const Time: IRI = prefix + "Time";
/// A combination of date and time of day in the form
/// [-]CCYY-MM-DDThh:mm:ss[Z|(+|-)hh:mm] (see Chapter 5.4 of ISO 8601).
const DateTime: IRI = prefix + "DateTime";


/// A class, also often called a 'Type'; equivalent to rdfs:Class.
const Class: IRI = prefix + "Class";
/// A property, used to indicate attributes and relationships
/// of some Thing; equivalent to rdf:Property.
const Property: IRI = prefix + "Property";

/// The name of the item.
const Name: IRI = prefix + "name";
/// The URL of the item property.
const UrlProp: IRI = prefix + "url";
*/
