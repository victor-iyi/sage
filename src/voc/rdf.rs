//! Module `rdf` contains constants of the RDF Concepts Vocabulary (RDF)

use crate::types::IRI;
use crate::voc::Vocabulary;

/// RdfVoc contains constants of the RDF Schema vocabulary (RDFS).
pub struct RdfVoc;

impl Vocabulary for RdfVoc {
  type Prefix = IRI;
  type Full = IRI;

  fn prefix() -> Self::Prefix {
    IRI::from("rdf:")
  }

  fn full() -> Self::Full {
    IRI::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#")
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | Types.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
/*
/// The datatype of RDF literals storing fragments of HTML content
const HTML: IRI = prefix + "HTML";
/// The datatype of language-tagged string values
const LangString: IRI = prefix + "LangString";
/// The class of plain (i.e. untyped) literal values, as used in RIF and OWL 2
const PlainLiteral: IRI = prefix + "PlainLiteral";
/// The class of RDF properties.
const Property: IRI = prefix + "Property";
/// The class of RDF statements.
const Statement: IRI = prefix + "Statement";
*/
/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | Properties.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
/*
/// The subject is an instance of a class.
const Type: IRI = prefix + "type";
/// Idiomatic property used for structured values.
const Value: IRI = prefix + "value";
/// The subject of the subject RDF statement.
const Subject: IRI = prefix + "subject";
/// The predicate of the subject RDF statement.
const Predicate: IRI = prefix + "predicate";
/// The object of the subject RDF statement.
const Object: IRI = prefix + "object";
*/

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | Containers.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
/*
/// The class of unordered containers.
const Bag: IRI = prefix + "Bag";
/// The class of ordered containers.
const Seq: IRI = prefix + "Seq";
/// The class of containers of alternatives.
const Alt: IRI = prefix + "Alt";
/// The class of RDF Lists.
const List: IRI = prefix + "List";
/// The empty list, with no items in it.
/// If the rest of a list is nil then the list has no more items in it.
const Nil: IRI = prefix + "nil";
/// The first item in the subject RDF list.
const First: IRI = prefix + "first";
/// The rest of the subject RDF list after the first item.
const Rest: IRI = prefix + "rest";
/// The datatype of XML literal values.
const XMLLiteral: IRI = prefix + "XMLLiteral";
/// The datatype of URL property values.
const UrlProp: IRI = prefix + "url";
*/