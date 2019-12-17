//! `sage::types` contains all/most types used by the `sage` engine. Many types wrap
//!  native Rust types. Although it's highly advised to use these types rather than
//! native rust because they include extended functionalities and can also be dereferenced
//!  back and forth into native Rust types  like [Strings](https://doc.rust-lang.org/stable/alloc/string/struct.String.html) and sage types.
//!

/// `IRI` stands for International Resource Identifer. (ex: <name>).
pub type IRI = String;

/// `URI` is used to represent any URL-like `IRI`.
pub type URI = String;

#[derive(Debug, Clone, PartialEq)]
pub enum DTypes {
  Boolean(bool),
  Text(String),
  Number,
  Time,
  DateTime,
}

#[derive(Debug)]
enum Number {
  Float(f64),
  Integer(isize),
}

/*
enum Number {
  Float(f64),
  Integer(isize),
}

enum Boolean {
  True,
  False,
}

enum Class {

}

enum Date {
  Date,
  Time,
  DateTime,
}

enum Property {

}

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
