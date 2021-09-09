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

//! `sage::dtype` contains all/most types used by the `sage` engine. Many types wrap
//! native Rust types. Although it's highly advised to use these types rather than
//! native rust because they include extended functionalities and can also be dereferenced
//! back and forth into native Rust types like [Strings] and sage types.
//!
//! [Strings]: https://doc.rust-lang.org/stable/alloc/string/struct.String.html

use std::fmt;

use serde::{de::DeserializeOwned, ser::Serialize};

use crate::Result;

pub mod datetime;
pub mod map;
pub mod number;
mod ops;

// Re-export public members.
pub use {datetime::DateTime, map::Map, number::Number};

/// `IRI` stands for International Resource Identifer. (ex: <name>).
pub type IRI = String;

/// `URI` is used to represent any URL-like `IRI`.
pub type URI = String;

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `DType`
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

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

/// [`DType`] represents the various types which data in the Sage Knowledge
/// Graph can be represented as.
///
///
/// Common source of error is forgetting capitalizing "T" in `DType`.
/// This is an alias to represent [`DType`] as `Dtype`.
///
/// [`DType`]: struct.DType.html
pub type Dtype = DType;

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
  /// Index into a JSON array or map. A string index can be used to access a
  /// value in a map, and a usize index can be used to access an element of an
  /// array.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let object = json!({"A": 65, "B": 66, "C": 67});
  /// assert_eq!(*object.get("A").unwrap(), json!(65));
  ///
  /// let array = json!(["A", "B", "C"]);
  /// assert_eq!(*array.get(2).unwrap(), json!("C"));
  ///
  /// assert_eq!(array.get("A"), None);
  /// ```
  ///
  /// Square brackets can also be used to index into a value in a more concise
  /// way. This returns `DType::Null` in cases where `get` would have returned
  /// `None`.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let object = json!({
  ///     "A": ["a", "à", "â"],
  ///     "B": ["b"],
  ///     "C": ["C", "Ç", "Ć"]
  /// });
  /// assert_eq!(object["B"][0], json!("b"));
  ///
  /// assert_eq!(object["D"], json!(null));
  /// assert_eq!(object[0]["x"]["y"]["z"], json!(null));
  /// ```
  ///
  pub fn get<I: ops::index::Index>(&self, index: I) -> Option<&DType> {
    index.index_into(self)
  }

  /// Mutably index into a JSON array or map. A string index can be used to
  /// access a value in a map, and a usize index can be used to access an
  /// element of an array.
  ///
  /// Returns `None` if the type of `self` does not match the type of the
  /// index, for example if the index is a string and `self` is an array or a
  /// number. Also returns `None` if the given key does not exist in the map
  /// or the given index is not within the bounds of the array.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let mut object = json!({ "A": 65, "B": 66, "C": 67});
  /// *object.get_mut("A").unwrap() = json!(69);
  ///
  /// let mut array = json!(["A", "B", "C"]);
  /// *array.get_mut(2).unwrap() = json!("D");
  /// ```
  pub fn get_mut<I: ops::index::Index>(
    &mut self,
    index: I,
  ) -> Option<&mut DType> {
    index.index_into_mut(self)
  }

  /// Returns true if the `DType` is an Object. Returns false otherwise.
  ///
  /// For any value on which `is_object` returns true, `as_object` and
  /// `as_object_mut` are guarnteed to return the map representation of the
  /// object.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({
  ///   "a": { "nested": true },
  ///   "b": ["an", "array"]
  /// });
  ///
  /// assert!(obj.is_object());
  /// assert!(obj["a"].is_object());
  ///
  /// // array, not an object.
  /// assert!(!obj["b"].is_object());
  /// ```
  pub fn is_object(&self) -> bool {
    self.as_object().is_some()
  }

  /// If the `DType` is an Object, returns the associated `Map`. Returns `None`
  /// otherwise.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({
  ///   "a": { "nested": true },
  ///   "b": ["an", "array"]
  /// });
  ///
  /// // The length of `{"nested": true} is 1 entry.
  /// assert_eq!(obj["a"].as_object().unwrap().len(), 1);
  ///
  /// // The array `["an", "array"] is not an object.
  /// assert_eq!(obj["b"].as_object(), None);
  /// ```
  pub fn as_object(&self) -> Option<&Map<String, DType>> {
    match *self {
      DType::Object(ref m) => Some(m),
      _ => None,
    }
  }

  /// If the `DType` is an Object, returns the associated mutable `Map` (`&mut Map`).
  /// Returns `None` otherwise.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let mut obj = json!({"a": { "nested": true }});
  ///
  /// obj["a"].as_object_mut().unwrap().clear();
  /// assert_eq!(obj, json!({ "a": {} }));
  /// ```
  pub fn as_object_mut(&mut self) -> Option<&mut Map<String, DType>> {
    match *self {
      DType::Object(ref mut m) => Some(m),
      _ => None,
    }
  }

  /// Returns true if the `DType` is an Array. Returns false otherwise.
  ///
  /// For any `DType` on which `is_array` returns true, `as_array` and
  /// `as_array_mut` are guaranteed to return the vector representing the
  /// array.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({
  ///   "a": ["an", "array"],
  ///   "b": { "an": "object" }
  /// });
  /// assert!(obj["a"].is_array());
  ///
  /// // an object, not an array.
  /// assert!(!obj["b"].is_array());
  /// ```
  pub fn is_array(&self) -> bool {
    self.as_array().is_some()
  }

  /// If the `DType` is an Array, returns the associated vector. Returns `None`
  /// otherwise.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({
  ///   "a": ["an", "array"],
  ///   "b": {"an": "object"},
  /// });
  ///
  /// // The length of `["an", "array"]` is 2 elements.
  /// assert_eq!(obj["a"].as_array().unwrap().len(), 2);
  ///
  /// // The object `{"an": "object"}` is not an array.
  /// assert_eq!(obj["b"].as_array(), None);
  /// ```
  pub fn as_array(&self) -> Option<&Vec<DType>> {
    match *self {
      DType::Array(ref a) => Some(&*a),
      _ => None,
    }
  }

  /// If the `DType` is an Array, returns the associated mutable vector (`&mut Vec<DType>`).
  /// Returns `None` otherwise.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let mut obj = json!({ "a": ["an", "array"] });
  ///
  /// obj["a"].as_array_mut().unwrap().clear();
  /// assert_eq!(obj, json!({ "a": [] }));
  /// ```
  pub fn as_array_mut(&mut self) -> Option<&mut Vec<DType>> {
    match *self {
      DType::Array(ref mut v) => Some(v),
      _ => None,
    }
  }

  /// Returns true if the `DType` is a String. Returns false otherwise.
  ///
  /// For any `DType` on which `is_string` returns true, `as_str` is guaranteed to
  /// return the string slice.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({ "a": "some string", "b": false });
  /// assert!(obj["a"].is_string());
  ///
  /// // The boolean `false` is not a string
  /// assert!(!obj["b"].is_string());
  /// ```
  pub fn is_string(&self) -> bool {
    self.as_str().is_some()
  }

  /// If the `DType` is a String, returns the associated str. Returns None
  /// otherwise
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({ "a": "some string", "b": false });
  /// assert!(obj["a"].is_string());
  ///
  /// // The boolean `false` is not a string
  /// assert!(!obj["b"].is_string());
  ///
  /// // JSON values are printed in JSON representation, so strings are in quotes.
  /// //
  /// // The value is "some string"
  /// println!("The value is: {}", obj["a"]);
  ///
  /// // Rust strings are printed without quotes.
  /// //
  /// //  The value is: some string
  /// println!("The value is: {}", obj["a"].as_str().unwrap());
  /// ```
  pub fn as_str(&self) -> Option<&str> {
    match *self {
      DType::String(ref s) => Some(s),
      _ => None,
    }
  }

  /// Returns true if the `DType` is a number. Returns false otherwise.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({ "a": 1, "b": "2" });
  /// assert!(obj["a"].is_number());
  ///
  /// // The string "2" is a string, not a number.
  /// assert!(!obj["b"].is_number());
  /// ```
  pub fn is_number(&self) -> bool {
    matches!(*self, DType::Number(_))
  }

  /// Returns true if the `DType` is an integer between `i64::MIN` and
  /// `i64::MAX`.
  ///
  /// For any `DType` on which `is_i64` returns true, `as_i64` is guaranteed to
  /// return the integer value.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let big = i64::MAX as u64 + 10;
  /// let obj = json!({ "a": 65, "b": big, "c": 256.0 });
  /// assert!(obj["a"].is_i64());
  ///
  /// // Greater than i64::MAX
  /// assert!(!obj["b"].is_i64());
  ///
  /// // Numbers with a decimal point are not considered integers.
  /// assert!(!obj["c"].is_i64());
  /// ```
  pub fn is_i64(&self) -> bool {
    matches!(*self, DType::Number(ref n) if n.is_i64())
  }

  /// Returns true if the `DType` is an integer between zero and `u64::MAX`.
  ///
  /// For any `DType` on which `is_u64` returns true, `as_u64` is guaranteed to
  /// return the integer value.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({ "a": 65, "b": -65, "c": 256.0 });
  /// assert!(obj["a"].is_u64());
  ///
  /// // Negative ingter.
  /// assert!(!obj["b"].is_u64());
  ///
  /// // Numbers with a decimal point are not considered integers.
  /// assert!(!obj["c"].is_u64());
  /// ```
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
  pub fn is_f64(&self) -> bool {
    matches!(*self, DType::Number(ref n) if n.is_f64())
  }

  /// If the `DType` is an integer, represent it as `i64` if possible. Returns
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
  pub fn as_i64(&self) -> Option<i64> {
    match *self {
      DType::Number(ref n) => n.as_i64(),
      _ => None,
    }
  }

  /// If the `DType` is an integer, represent it as `u64` if possible. Returns
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
  pub fn as_u64(&self) -> Option<u64> {
    match *self {
      DType::Number(ref n) => n.as_u64(),
      _ => None,
    }
  }

  /// If the `DType` is a number, represent it as `f64` if possible. Returns
  /// `None` otherwise.
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
  /// # use sage::json;
  /// #
  /// let obj = json!({ "a": false, "b": "false" });
  /// assert!(obj["a"].is_bool());
  ///
  /// // The string "false" is a string, not a boolean.
  /// assert!(!obj["b"].is_bool());
  /// ```
  pub fn is_bool(&self) -> bool {
    self.as_bool().is_some()
  }

  /// If the `DType` is a `Boolean`, returns the associated bool.
  /// Returns `None` otherwise.
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let obj = json!({ "a": false, "b": "false" });
  /// assert_eq!(obj["a"].as_bool(), Some(false));
  ///
  /// // The string "false" is a string, not a boolean.
  /// assert_eq!(obj["b"].as_bool(), None);
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
  /// # use sage::json;
  /// #
  /// let obj = json!({ "a": null, "b": false });
  /// assert!(obj["a"].is_null());
  ///
  /// // The boolean `false` is not null.
  /// assert!(!obj["b"].is_null());
  /// ```
  pub fn is_null(&self) -> bool {
    self.as_null().is_some()
  }

  /// If the `DType` is a `Null`, return `()` *(unit type)*. Returns `None` otherwise.
  ///
  /// ```
  /// # use sage::json;
  /// #
  /// let obj = json!({ "a": null, "b": false });
  /// assert_eq!(obj["a"].as_null(), Some(()));
  ///
  /// // The boolean `false` is not null.
  /// assert_eq!(obj.as_null(), None);
  /// ```
  pub fn as_null(&self) -> Option<()> {
    match *self {
      DType::Null => Some(()),
      _ => None,
    }
  }

  /// Looks up a value by a JSON Pointer.
  ///
  /// JSON Pointer defines a string syntax for identifying a specific value
  /// within a JavaScript Object Notation (JSON) document.
  ///
  /// A Pointer is a Unicode string with the reference tokens separated by `/`.
  /// Inside tokens `/` is replaced by `~1` and `~` is replaced by `~0`. The
  /// addressed value is returned and if there is no such value `None` is
  /// returned.
  ///
  /// For more information read [RFC6901].
  ///
  /// # Example
  ///
  /// ```rust
  /// # use sage::json;
  /// #
  /// let data = json!({
  ///     "x": {
  ///         "y": ["z", "zz"]
  ///     }
  /// });
  ///
  /// assert_eq!(data.pointer("/x/y/1").unwrap(), &json!("zz"));
  /// assert_eq!(data.pointer("/a/b/c"), None);
  /// ```
  ///
  /// [RFC6901]: https://tools.ietf.org/html/rfc6901
  pub fn pointer(&self, pointer: &str) -> Option<&DType> {
    if pointer.is_empty() {
      return Some(self);
    }
    if !pointer.starts_with('/') {
      return None;
    }
    pointer
      .split('/')
      .skip(1)
      .map(|x| x.replace("~1", "/").replace("~0", "~"))
      .try_fold(self, |target, token| match target {
        DType::Object(map) => map.get(&token),
        DType::Array(list) => {
          Self::parse_index(&token).and_then(|x| list.get(x))
        }
        _ => None,
      })
  }

  /// Looks up a value by a JSON Pointer and returns a mutable reference to
  /// that value.
  ///
  /// JSON Pointer defines a string syntax for identifying a specific value
  /// within a JavaScript Object Notation (JSON) document.
  ///
  /// A Pointer is a Unicode string with the reference tokens separated by `/`.
  /// Inside tokens `/` is replaced by `~1` and `~` is replaced by `~0`. The
  /// addressed value is returned and if there is no such value `None` is
  /// returned.
  ///
  /// For more information read [RFC6901](https://tools.ietf.org/html/rfc6901).
  ///
  /// # Example
  ///
  /// ```rust,ignore
  /// use sage::DType;
  ///
  /// fn main() {
  ///
  ///     let s = r#"{"x": 1.0, "y": 2.0}"#;
  ///     let mut obj: DType = sage::json::from_str(s).unwrap();
  ///
  ///     // Check value using read-only pointer
  ///     assert_eq!(obj.pointer("/x"), Some(&1.0.into()));
  ///     // Change value with direct assignment
  ///     *obj.pointer_mut("/x").unwrap() = 1.5.into();
  ///     // Check that new value was written
  ///     assert_eq!(obj.pointer("/x"), Some(&1.5.into()));
  ///     // Or change the value only if it exists
  ///     obj.pointer_mut("/x").map(|o| *o = 1.5.into());
  ///
  ///     // "Steal" ownership of a value. Can replace with any valid DType.
  ///     let old_x = obj.pointer_mut("/x").map(DType::take).unwrap();
  ///     assert_eq!(old_x, 1.5);
  ///     assert_eq!(obj.pointer("/x").unwrap(), &DType::Null);
  /// }
  /// ```
  pub fn pointer_mut(&mut self, pointer: &str) -> Option<&mut DType> {
    if pointer.is_empty() {
      return Some(self);
    }
    if !pointer.starts_with('/') {
      return None;
    }
    pointer
      .split('/')
      .skip(1)
      .map(|x| x.replace("~1", "/").replace("~0", "~"))
      .try_fold(self, |target, token| match target {
        DType::Object(map) => map.get_mut(&token),
        DType::Array(list) => {
          Self::parse_index(&token).and_then(move |x| list.get_mut(x))
        }
        _ => None,
      })
  }

  /// Takes the value of the `DType`, leaving a `Null` in its place.
  ///
  /// # Example
  ///
  /// ```rust
  /// # use sage::json;
  ///
  /// let mut obj = json!({ "x": "y" });
  /// assert_eq!(obj["x"].take(), json!("y"));
  ///
  /// assert_eq!(obj, json!({ "x": null }));
  /// ```
  pub fn take(&mut self) -> DType {
    std::mem::replace(self, DType::Null)
  }

  #[cold]
  fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
      return None;
    }
    s.parse().ok()
  }
}

/// The default value is `DType::Null`.
///
/// This is useful for handling omitted `DType` fields when deserializing.
///
/// # Examples
///
/// ```rust,ignore
/// # use serde::Deserialize;
/// # use sage::Result;
/// use sage::json;
///
/// #[derive(Deserialize)]
/// struct Settings {
///     level: i32,
///     #[serde(default)]
///     extras: DType,
/// }
///
/// # fn try_main() -> Result<()> {
/// let data = r#" { "level": 42 } "#;
/// let s: Settings = sage::from_str(data)?;
///
/// assert_eq!(s.level, 42);
/// assert_eq!(s.extras, DType::Null);
/// #
/// #     Ok(())
/// # }
/// #
/// # try_main().unwrap()
/// ```
impl Default for DType {
  fn default() -> DType {
    DType::Null
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | to_dtype, from_dtype - Converting type `T` from & to `DType`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

pub fn to_dtype<T>(value: T) -> Result<DType>
where
  T: Serialize,
{
  value.serialize(ops::ser::Serializer)
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
