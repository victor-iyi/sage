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

//! Module `rdfs` contains constants of the RDF Schema vocabulary (RDFS)

use crate::types::IRI;
use crate::vocab::Vocabulary;

/// `RdfsVocab` contains constants of the RDFs vocabulary (RDFS).
///
/// `RdfsVocab` implements the `Vocabulary` trait which provides
/// `RdfsVocab::prefix()` and `RdfsVocab::full()` by default.
///
/// ## Basic Usage
///
/// ```rust
/// use sage::types::IRI;
/// use sage::vocab::{Vocabulary, RdfsVocab};
///
/// assert_eq!(RdfsVocab::prefix(), IRI::from("rdfs:"));
/// assert_eq!(RdfsVocab::full(), IRI::from("http://www.w3.org/2000/01/rdf-schema#"));
/// ```
pub struct RdfsVocab;

impl Vocabulary for RdfsVocab {
  type Prefix = IRI;
  type Full = IRI;

  fn prefix() -> Self::Prefix {
    IRI::from("rdfs:")
  }

  fn full() -> Self::Full {
    IRI::from("http://www.w3.org/2000/01/rdf-schema#")
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | Classes.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
/*
/// The class resource, everything.
pub const Resource: IRI = prefix + "Resource";
/// The class of classes.
pub const Class: IRI = prefix + "Class";
/// The class of literal values, eg. textual strings and integers.
pub const Literal: IRI = prefix + "Literal";
/// The class of RDF containers.
pub const Container: IRI = prefix + "Container";
/// The class of RDF data types.
pub const DataType: IRI = prefix + "DataType";
/// The class of container membership properties, rdf:_1, rdf:_2, ...,
/// all of which are sub-properties of 'member'.
pub const ContainerMembershipProperty: IRI = prefix + "ContainerMembershipProperty";
*/
/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | Properties.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
/*
/// The subject is a subclass of a class.
pub const SubClassOf: IRI = prefix + "subClassOf";
/// The subject is a sub-property of a property.
pub const SubPropertyOf: IRI = prefix + "subPropertyOf";
/// A description of the subject resource.
pub const Comment: IRI = prefix + "comment";
/// A human-readable name for the subject.
pub const Label: IRI = prefix + "label";
/// A domain of the subject property.
pub const Domain: IRI = prefix + "domain";
/// A range of the subject property.
pub const Range: IRI = prefix + "range";
/// Further information about the subject resource.
pub const SeeAlso: IRI = prefix + "seeAlso";
/// The definition of the subject resource.
pub const IsDefinedBy: IRI = prefix + "isDefinedBy";
/// A member of the subject resource.
pub const Member: IRI = prefix + "member";
*/
