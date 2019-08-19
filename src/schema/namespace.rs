use crate::types::IRI;

use std::collections::HashMap;

/// Namespace is a RDF namespace (vocabulary).
#[derive(Debug, PartialEq, Clone)]
pub struct Namespace {
  prefix: IRI,
  full: IRI,
}

impl Namespace {

  /// Creates a new namespace using IRI values.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::types::IRI;
  /// use sage::schema::Namespace;
  ///
  /// let prefix: IRI = IRI::from("rdf:type");
  /// let full: IRI = IRI::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
  ///
  /// let ns = Namespace::new(&prefix, &full);
  ///
  /// assert_eq!(ns.prefix(), &prefix);
  /// assert_eq!(ns.full(), &full);
  /// ```
  ///
  pub fn new(prefix: &IRI, full: &IRI) -> Namespace {
    Namespace {
      prefix: prefix.to_string(),
      full: full.to_string(),
    }
  }

  /// Creates a new namespace from a string slice.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::types::IRI;
  /// use sage::schema::Namespace;
  ///
  /// // Creates a new namespace using a sing literal.
  /// let ns = Namespace::from("rdf:type", "http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
  ///
  /// assert_eq!(ns.prefix(), &IRI::from("rdf:type"));
  /// assert_eq!(ns.full(), &IRI::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"));
  /// ```
  ///
  pub fn from(prefix: &str, full: &str) -> Namespace {
    Namespace {
      prefix: prefix.to_string(),
      full: full.to_string(),
    }
  }

  /// Returns a reference to the namespace prefix.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::types::IRI;
  /// use sage::schema::Namespace;
  ///
  /// // Creates a new namespace using a sing literal.
  /// let ns = Namespace::from("rdf:type", "http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
  ///
  /// assert_eq!(ns.prefix(), &IRI::from("rdf:type"));
  /// ```
  pub fn prefix(&self) -> &str {
    return &self.prefix;
  }

  /// Returns a reference to the namespace full IRI.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::types::IRI;
  /// use sage::schema::Namespace;
  ///
  /// // Creates a new namespace using a sing literal.
  /// let ns = Namespace::from("rdf:type", "http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
  ///
  ///  assert_eq!(ns.full(), &IRI::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"));
  /// ```
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
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::schema::Namespaces;
  ///
  /// let ns : Namespaces = Namespaces::new();
  /// assert_eq!(ns.list().len(), 0);
  /// ```
  ///
  pub fn new() -> Namespaces {
    Namespaces {
      prefixes: HashMap::new(),
    }
  }

  /// `Namespaces::add` adds a new namespace to the registered list.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::types::IRI;
  /// use sage::schema::{Namespace, Namespaces};
  ///
  /// // Create a Namespace store.
  /// let mut ns = Namespaces::new();
  ///
  /// ns.add(&Namespace::from("rdf:type", "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"));
  /// assert_eq!(ns.list().len(), 1);
  ///
  /// // Creating namespace to be registered.
  /// ns.add(&Namespace::from("schema:Thing", "https://schema.org/Thing"));
  /// assert_eq!(ns.list().len(), 2);
  /// ```
  ///
  pub fn add(&mut self, ns: &Namespace) {
    &self
      .prefixes
      .insert(ns.prefix().to_string(), ns.full().to_string());
  }

  /// `Namespaces::add_prefix` globally associates a given prefix with a base vocabulary IRI.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::types::IRI;
  /// use sage::schema::Namespaces;
  ///
  /// // Create a mutable namespace store.
  /// let mut ns = Namespaces::new();
  ///
  /// // Using string literal.
  /// ns.add_prefix(
  ///   "rdf:type",
  ///   "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
  /// );
  /// assert_eq!(ns.list().len(), 1);
  ///
  /// // Using IRI reference.
  /// ns.add_prefix(
  ///   &IRI::from("schema:Thing"),
  ///   &IRI::from("https://schema.org/Thing"),
  /// );
  /// assert_eq!(ns.list().len(), 2);
  /// ```
  ///
  pub fn add_prefix(&mut self, prefix: &str, full: &str) {
    &self.add(&Namespace {
      prefix: prefix.to_string(),
      full: full.to_string(),
    });
  }

  /// `Namespaces::short_IRI` replaces a base IRI of a known vocabulary with it's prefix.
  ///
  ///	short_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#type") // returns "rdf:type"
  ///
  /// # Example
  ///
  /// ```
  /// use sage::types::IRI;
  /// use sage::schema::Namespaces;
  ///
  /// // Create a mutable namespace store.
  /// let mut ns = Namespaces::new();
  ///
  /// // Register a namespace (here: using `Namespaces::add_prefix`).
  /// ns.add_prefix(
  ///   "rdf:type",
  ///   "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
  /// );
  ///
  /// // `Namespaces::short_iri` returns a owned IRI value.
  /// assert_eq!(
  ///   ns.short_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
  ///   IRI::from("rdf:type")
  /// );
  ///
  /// // Unregistered namespace will have an undefined behavior.
  /// assert_eq!(ns.short_iri("unknown"), IRI::from("unknown"));
  /// ```
  ///
  pub fn short_iri(&self, iri: &str) -> IRI {
    for (prefix, full) in self.prefixes.iter() {
      if full == iri {
        return prefix.to_string();
      }
    }
    iri.to_string()
  }

  /// `Namespaces::full_IRI` replaces known prefix in IRI with it's full vocabulary IRI.
  ///
  ///	full_iri("rdf:type") // returns "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::types::IRI;
  /// use sage::schema::Namespaces;
  ///
  ///   // Create a mutable namespace store.
  /// let mut ns = Namespaces::new();
  ///
  /// // Register a namespace (here: using `Namespaces::add_prefix`).
  /// ns.add_prefix(
  ///   "rdf:type",
  ///   "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
  /// );
  ///
  /// // `Namespaces::full_iri` returns a owned IRI value.
  /// assert_eq!(
  ///   ns.full_iri("rdf:type"),
  ///   IRI::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
  /// );
  ///
  /// // Unregistered namespace will have an undefined behavior.
  /// assert_eq!(ns.full_iri("unknown"), IRI::from("unknown"));
  /// ```
  ///
  pub fn full_iri(&self, iri: &str) -> IRI {
    match self.prefixes.get(iri) {
      Some(full) => full.to_string(),
      None => iri.to_string(),
    }
  }

  /// `Namespaces::list` enumerates all registered namespace pairs.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::schema::{Namespace, Namespaces};
  ///
  ///     // Create a new mutable namespace store.
  /// let mut ns = Namespaces::new();
  ///
  /// // Create a few namespace to be registered.
  /// let rdf = Namespace::from(
  ///   "rdf:type",
  ///   "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
  /// );
  /// let schema = Namespace::from("schema:Thing", "https://schema.org/Thing");
  ///
  /// // Add created namespace.
  /// ns.add(&rdf);
  /// ns.add(&schema);
  ///
  /// assert_eq!(ns.list().len(), 2);
  /// assert!(ns.list().contains(&rdf));
  /// assert!(ns.list().contains(&schema));
  /// ```
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
