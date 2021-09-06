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
//!  native Rust types. Although it's highly advised to use these types rather than
//! native rust because they include extended functionalities and can also be dereferenced
//!  back and forth into native Rust types  like [Strings](https://doc.rust-lang.org/stable/alloc/string/struct.String.html) and sage types.
//!

use std::fmt;

mod datetime;
mod number;
mod object;

use {self::datetime::DateTime, number::Number, object::Map};

/// `IRI` stands for International Resource Identifer. (ex: <name>).
pub type IRI = String;

/// `URI` is used to represent any URL-like `IRI`.
pub type URI = String;

/// `DTypes` represents the various types which data in the Sage Knowledge
/// Graph can be represented as.
#[derive(Clone, Eq, PartialEq)]
pub enum DTypes {
  /// Represents a JSON null value.
  Null,

  /// Represents a boolean (true or false) value.
  Boolean(bool),

  /// Represents a numeric value,
  /// either integer or floating point values.
  Number(Number),

  /// Represents a String or string-like value.
  String(String),

  /// Represents a collection of values.
  Array(Vec<DTypes>),

  /// Represents an object type with Key & values.
  ///
  /// By default it uses `BTreeMap` which does not preserve
  /// the entries' order.
  Object(Map<String, DTypes>),

  /// Represents date, time or datetime.
  DateTime(DateTime),
}

impl fmt::Debug for DTypes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      DTypes::Null => f.debug_tuple("Null").finish(),
      DTypes::Boolean(b) => f.debug_tuple("Boolean").field(&b).finish(),
      DTypes::Number(ref n) => fmt::Debug::fmt(&n, f),
      DTypes::String(ref s) => f.debug_tuple("Sting").field(s).finish(),
      DTypes::Array(ref a) => {
        f.write_str("Array(")?;
        fmt::Debug::fmt(a, f)?;
        f.write_str(")")
      }
      DTypes::Object(ref o) => {
        f.write_str("Object(")?;
        fmt::Debug::fmt(o, f)?;
        f.write_str(")")
      }
      DTypes::DateTime(ref d) => fmt::Debug::fmt(&d, f),
    }
  }
}

impl fmt::Display for DTypes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      DTypes::Boolean(b) => write!(f, "{}", b),
      DTypes::String(t) => write!(f, "{}", t),
      // For every other variant, use the Debug trait.
      _ => fmt::Debug::fmt(self, f),
    }
  }
}

/*
enum SchemaTypes {
  // DateTypes.
  Boolean(bool),
  Text(IRI),
  URL(URI),
  Number,
  Date,

  Class,
  Property,
}


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
