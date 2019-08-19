#![allow(dead_code)]

use crate::schema::values::IRI;

use std::collections::HashMap;

/// Namespace is a RDF namespace (vocabulary).
#[derive(Debug, PartialEq, Clone)]
pub struct Namespace {
  prefix: IRI,
  full: IRI,
}

impl Namespace {
  pub fn new(prefix: &IRI, full: &IRI) -> Namespace {
    Namespace {
      prefix: prefix.to_string(),
      full: full.to_string(),
    }
  }

  pub fn prefix(&self) -> &str {
    return &self.prefix;
  }

  pub fn full(&self) -> &str {
    return &self.full;
  }
}

/// Namespaces is a set of registered namespaces.
#[derive(Debug, PartialEq, Clone)]
pub struct Namespaces {
  /// List of registered namespace prefix & full IRI values.
  prefixes: HashMap<IRI, IRI>,
}


impl Namespaces {

  /// `Namespaces::new` Creates a new blank namespace with no registered values.
  pub fn new() -> Namespaces {
    Namespaces {
      prefixes: HashMap::new(),
    }
  }

  /// `Namespaces::add` a new namespace to the registered list.
  pub fn add(&mut self, ns: &Namespace) {
    &self
      .prefixes
      .insert(ns.prefix().to_string(), ns.full().to_string());
  }

  /// `Namespaces::add_prefix` globally associates a given prefix with a base vocabulary IRI.
  pub fn add_prefix(&mut self, prefix: &IRI, full: &IRI) {
    &self.add(&Namespace {
      prefix: prefix.to_string(),
      full: full.to_string(),
    });
  }

  /// `Namespaces::short_IRI` replaces a base IRI of a known vocabulary with it's prefix.
  ///
  ///	short_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#type") // returns "rdf:type"
  pub fn short_iri(_iri: &IRI) -> &str {
    unimplemented!()
  }

  /// `Namespaces::full_IRI` replaces known prefix in IRI with it's full vocabulary IRI.
  ///
  ///	full_iri("rdf:type") // returns "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
  pub fn full_iri(_iri: &IRI) -> &str {
    unimplemented!()
  }

  /// `Namespaces::list` enumerates all registered namespace pairs.
  pub fn list(&self) -> Vec<Namespace> {
    let mut ns: Vec<Namespace> = Vec::with_capacity(self.prefixes.len());
    for (prefix, full) in self.prefixes.iter() {
      ns.push(Namespace {
        prefix: prefix.to_string(),
        full: full.to_string(),
      });
    }
    ns
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_namespace() {
    let prefix: IRI = IRI::from("rdf:type");
    let full: IRI = IRI::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");

    let ns = Namespace::new(&prefix, &full);

    assert_eq!(ns.prefix(), &prefix);
    assert_eq!(ns.full(), &full);
  }

  #[test]
  fn new_namespaces() {
    let ns = Namespaces::new();
    assert_eq!(ns.list().len(), 0);
  }


  #[test]
  fn add_namespaces() {
    let rdf = Namespace::new(
      &IRI::from("rdf:type"),
      &IRI::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
    );
    let schema = Namespace::new(
      &IRI::from("schema:Thing"),
      &IRI::from("https://schema.org/Thing"),
    );

    let mut ns = Namespaces::new();

    ns.add(&rdf);
    assert_eq!(ns.list().len(), 1);

    ns.add(&schema);
    assert_eq!(ns.list().len(), 2);
  }

  #[test]
  fn add_prefix_namespaces() {
    let mut ns = Namespaces::new();

    ns.add_prefix(
      &IRI::from("rdf:type"),
      &IRI::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
    );
    assert_eq!(ns.list().len(), 1);

    ns.add_prefix(
      &IRI::from("schema:Thing"),
      &IRI::from("https://schema.org/Thing"),
    );
    assert_eq!(ns.list().len(), 2);

  }

  #[test]
  fn list_namespace() {
    let rdf = Namespace::new(
      &IRI::from("rdf:type"),
      &IRI::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
    );
    let schema = Namespace::new(
      &IRI::from("schema:Thing"),
      &IRI::from("https://schema.org/Thing"),
    );

    let mut ns = Namespaces::new();
    ns.add(&rdf);
    ns.add(&schema);

    assert_eq!(vec![schema, rdf], ns.list());
  }

}