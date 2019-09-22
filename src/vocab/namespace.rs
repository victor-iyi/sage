use crate::types::IRI;

use std::collections::HashMap;

/// `URI` expands and contracts a URL given it's context and the property.
pub struct URI {
    /// `context` for example http://schema.org which is the base URI for the node.
    context: IRI,

    /// `short` could for example be `Person`. The URI will combine
    /// both together to form "https://schema.org/Person".
    short: IRI,
}

impl URI {
    /// Creates a new `URI` instance.
    ///
    /// ## Basic Usage
    ///
    /// ```rust
    /// use sage::vocab::URI;
    ///
    /// let val = URI::new("https://schema.org", "Person");
    ///
    /// assert_eq!(val.context(), "https://schema.org");
    /// assert_eq!(val.short(), "Person");
    /// assert_eq!(val.expand(), "https://schema.org/Person");
    /// ```
    ///
    pub fn new(context: &str, short: &str) -> URI {
        URI {
            context: context.to_string(),
            short: short.to_string(),
        }
    }

    pub fn context(&self) -> &IRI {
        &self.context
    }

    pub fn short(&self) -> &IRI {
        &self.short
    }

    pub fn expand(&self) -> IRI {
        format!("{}/{}", &self.context.trim_end_matches('/'), &self.short)
    }
}

/// Namespace is a RDF namespace (vocabulary).
#[derive(Debug, PartialEq, Clone)]
pub struct Namespace {
    prefix: IRI,
    full: IRI,
}

impl Namespace {
    /// Creates a new namespace using `IRI` values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::types::IRI;
    /// use sage::vocab::Namespace;
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
    pub fn new(prefix: &str, full: &str) -> Namespace {
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
    /// use sage::vocab::Namespace;
    ///
    /// // Creates a new namespace using a sing literal.
    /// let ns = Namespace::from("rdf:type", "http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
    ///
    /// assert_eq!(ns.prefix(), "rdf:type");
    /// assert_eq!(ns.full(), "http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
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
    /// use sage::vocab::Namespace;
    ///
    /// // Creates a new namespace using a sing literal.
    /// let ns = Namespace::from("rdf:type", "http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
    ///
    /// assert_eq!(ns.prefix(), "rdf:type");
    /// ```
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Returns a reference to the namespace full IRI.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::types::IRI;
    /// use sage::vocab::Namespace;
    ///
    /// // Creates a new namespace using a sing literal.
    /// let ns = Namespace::from("rdf:type", "http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
    ///
    ///  assert_eq!(ns.full(), "http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
    /// ```
    pub fn full(&self) -> &str {
        &self.full
    }
}

impl Default for Namespace {
    /// `Namespace::default` creates a default namespace.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::types::IRI;
    /// use sage::vocab::Namespace;
    ///
    /// let ns : Namespace = Namespace::default();
    ///
    /// assert_eq!(ns.prefix(), "schema:Thing");
    /// assert_eq!(ns.full(), "https://schema.org/Thing");
    /// ```
    fn default() -> Self {
        Namespace::new("schema:Thing", "https://schema.org/Thing")
    }
}

/// `NamespaceStore` is a set of registered NamespaceStore.
#[derive(Debug, PartialEq, Clone)]
pub struct NamespaceStore {
    /// List of registered namespace prefix & full `IRI` values.
    prefixes: HashMap<IRI, IRI>,
}

impl NamespaceStore {
    /// `NamespaceStore::new` Creates a new blank namespace with no registered values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::vocab::NamespaceStore;
    ///
    /// let ns : NamespaceStore = NamespaceStore::new();
    /// assert_eq!(ns.len(), 0);
    /// ```
    ///
    pub fn new() -> NamespaceStore {
        NamespaceStore {
            prefixes: HashMap::new(),
        }
    }

    /// `NamespaceStore::add` adds a new namespace to the registered list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::types::IRI;
    /// use sage::vocab::{Namespace, NamespaceStore};
    ///
    /// // Create a Namespace store.
    /// let mut ns = NamespaceStore::new();
    ///
    /// // Add a new namespace created from `Namespace::from` API.
    /// ns.add(&Namespace::from("rdf:type", "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"));
    /// assert_eq!(ns.len(), 1);
    ///
    ///// Let's register another namespace.
    /// ns.add(&Namespace::from("schema:Thing", "https://schema.org/Thing"));
    /// assert_eq!(ns.len(), 2);
    /// ```
    /// You could also use `NamespaceStore` alias -> `Namespaces`, in case you say it's too long.
    ///
    /// ```rust
    /// use sage::types::IRI;
    /// use sage::vocab::{Namespace, Namespaces};
    ///
    /// // Create a Namespace store.
    /// let mut ns = Namespaces::new();
    ///
    /// // Add a new namespace created from `Namespace::from` API.
    /// ns.add(&Namespace::from("rdf:type", "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"));
    /// assert_eq!(ns.len(), 1);
    ///
    ///// Let's register another namespace.
    /// ns.add(&Namespace::from("schema:Thing", "https://schema.org/Thing"));
    /// assert_eq!(ns.len(), 2);
    ///
    pub fn add(&mut self, ns: &Namespace) {
        self.prefixes
            .insert(ns.prefix().to_string(), ns.full().to_string());
    }

    /// `NamespaceStore::add_prefix` globally associates a given prefix with a base vocabulary `IRI`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::types::IRI;
    /// use sage::vocab::NamespaceStore;
    ///
    /// // Create a mutable namespace store.
    /// let mut ns = NamespaceStore::new();
    ///
    /// // Using string literal.
    /// ns.add_prefix(
    ///   "rdf:type",
    ///   "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
    /// );
    /// assert_eq!(ns.len(), 1);
    ///
    /// // Using IRI reference.
    /// ns.add_prefix(
    ///   "schema:Thing",
    ///   "https://schema.org/Thing",
    /// );
    /// assert_eq!(ns.len(), 2);
    /// ```
    ///
    pub fn add_prefix(&mut self, prefix: &str, full: &str) {
        self.add(&Namespace {
            prefix: prefix.to_string(),
            full: full.to_string(),
        });
    }

    /// `NamespaceStore::add_multiple` globally adds a given list of Namespace objects.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::vocab::{Namespace, NamespaceStore};
    ///
    /// // Create a new mutable namespace store.
    /// let mut ns: NamespaceStore = NamespaceStore::new();
    ///
    /// // You can use any collection that deref-s into `&[Namespace]`
    /// let ns_list: Vec<Namespace> = vec![
    ///   Namespace::from(
    ///     "rdf:type",
    ///     "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
    ///   ),
    ///   Namespace::from("schema:Thing", "https://schema.org/Thing"),
    /// ];
    ///
    /// // Add a collection of namespace objects.
    /// ns.add_multiple(&ns_list);
    ///
    /// assert_eq!(ns.len(), 2);
    /// ```
    ///
    pub fn add_multiple(&mut self, ns: &[Namespace]) {
        for r_ns in ns.iter() {
            self.add(&(*r_ns).clone());
        }
    }

    /// `NamespaceStore::short_IRI` replaces a base IRI of a known vocabulary with it's prefix.
    ///
    ///	short_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#type") // returns "rdf:type"
    ///
    /// # Example
    ///
    /// ```
    /// use sage::types::IRI;
    /// use sage::vocab::NamespaceStore;
    ///
    /// // Create a mutable namespace store.
    /// let mut ns = NamespaceStore::new();
    ///
    /// // Register a namespace (here: using `NamespaceStore::add_prefix`).
    /// ns.add_prefix(
    ///   "rdf:type",
    ///   "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
    /// );
    ///
    /// // `NamespaceStore::short_iri` returns a owned IRI value.
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

    /// `NamespaceStore::full_IRI` replaces known prefix in IRI with it's full vocabulary `IRI`.
    ///
    ///	full_iri("rdf:type") // returns "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::types::IRI;
    /// use sage::vocab::NamespaceStore;
    ///
    ///   // Create a mutable namespace store.
    /// let mut ns = NamespaceStore::new();
    ///
    /// // Register a namespace (here: using `NamespaceStore::add_prefix`).
    /// ns.add_prefix(
    ///   "rdf:type",
    ///   "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
    /// );
    ///
    /// // `NamespaceStore::full_iri` returns a owned IRI value.
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

    /// `NamespaceStore::len` returns the number of registered namespace.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::vocab::{Namespace, NamespaceStore};
    ///
    ///     // Create a new mutable namespace store.
    /// let mut ns = NamespaceStore::new();
    ///
    /// // Add a new namespace with `NamespaceStore::add_prefix`.
    /// ns.add_prefix("rdf:type", "http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
    /// assert_eq!(ns.len(), 1);
    ///
    /// // Add another namespace.
    /// ns.add_prefix("schema:Thing", "https://schema.org/Thing");
    /// assert_eq!(ns.len(), 2);
    ///
    /// ```
    pub fn len(&self) -> usize {
        self.prefixes.len()
    }

    /// `NamespaceStore::is_empty` returns `true` if there are no item in the `NamespaceStore`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::vocab::{Namespace, NamespaceStore};
    ///
    ///     // Create a new mutable namespace store.
    /// let mut ns = NamespaceStore::new();
    /// assert_eq!(ns.is_empty(), true);
    ///
    /// // Add a new namespace with `NamespaceStore::add_prefix`.
    /// ns.add_prefix("rdf:type", "http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
    /// assert_eq!(ns.is_empty(), false);
    ///
    /// // Add another namespace.
    /// ns.add_prefix("schema:Thing", "https://schema.org/Thing");
    /// assert_eq!(ns.len(), 2);
    ///
    /// ```
    pub fn is_empty(&self) -> bool {
        self.prefixes.len() == 0
    }

    /// `NamespaceStore::list` enumerates all registered namespace pairs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::vocab::{Namespace, NamespaceStore};
    ///
    ///     // Create a new mutable namespace store.
    /// let mut ns = NamespaceStore::new();
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
    /// assert_eq!(ns.len(), 2);
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

impl Default for NamespaceStore {
    /// `NamespaceStore::default` Creates a registry of pre-registered NamespaceStore.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::vocab::NamespaceStore;
    ///
    /// let ns : NamespaceStore = NamespaceStore::default();
    /// assert_eq!(ns.len(), 3);
    /// ```
    fn default() -> Self {
        // Use the default vocabularies.
        use crate::vocab::{RdfVocab, RdfsVocab, SchemaVocab, Vocabulary};

        // Create a new mutable namespace store.
        let mut ns = NamespaceStore::new();

        // Add the default vocabularies.
        let ns_list: Vec<Namespace> = vec![
            Namespace::new(&RdfVocab::prefix(), &RdfVocab::full()),
            Namespace::new(&RdfsVocab::prefix(), &RdfsVocab::full()),
            Namespace::new(&SchemaVocab::prefix(), &SchemaVocab::full()),
        ];

        // Add a collection of namespace objects.
        ns.add_multiple(&ns_list);

        ns
    }
}

/// `Namespaces` Alias for `NamespaceStore` to avoid confusion or misinterpretation.
///
/// `NamespaceStore` or `Namespaces` are a collection of multiple
/// `Namespace`.
pub type Namespaces = NamespaceStore;
