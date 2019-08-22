// TODO(victor): Use attribute macros to automate this task.
/// `Vocabulary` is the base trait for all namespace IRIs' that implements a prefix & suffix.
///
/// # Example
///
/// ```rust
/// use sage::types::IRI;
/// use sage::voc::Vocabulary;
///
/// // My custom example vocabulary namespace.
/// struct ExampleVoc;
///
/// // Implement the traits.
/// impl Vocabulary for ExampleVoc {
///   type Prefix = IRI;
///   type Full = IRI;
///
///  fn prefix() -> Self::Prefix {
///    IRI::from("ex:")
///  }
///
///   fn full() -> Self::Full {
///     IRI::from("https://example.com/")
///   }
/// }
/// ```
pub trait Vocabulary {
    // TODO(victor): Impose a trailing colon `:` if it's not present.
    /// `Namespace::prefix` returns a short `IRI` vocabulary e.g `IRI::from("ex:")`
    ///
    type Prefix;

    // TODO: Impose a trailing forward slash `/` or `#` if it's not present.
    /// `Namespace::full` returns an expanded `IRI` vocabulary e.g `IRI::from("https://exmaple.com/")`.
    ///
    type Full;

    /// `Vocabulary::prefix` contains the short `IRI` for the target vocabulary to be used.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sage::types::IRI;
    /// use sage::voc::Vocabulary;
    ///
    /// // My custom example vocabulary namespace.
    /// struct ExampleVoc;
    ///
    /// // Implement the traits.
    /// impl Vocabulary for ExampleVoc {
    ///   type Prefix = IRI;
    ///   type Full = IRI;
    ///
    ///  fn prefix() -> Self::Prefix {
    ///    IRI::from("ex:")
    ///  }
    ///
    ///   fn full() -> Self::Full {
    ///     IRI::from("https://example.com/")
    ///   }
    /// }
    ///
    /// assert_eq!(ExampleVoc::prefix(), IRI::from("ex:"));
    /// ```
    fn prefix() -> Self::Prefix;

    /// `Vocabulary::full` contains the short IRI for the target vocabulary to be used.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use sage::types::IRI;
    /// use sage::voc::Vocabulary;
    ///
    /// // My custom example vocabulary namespace.
    /// struct ExampleVoc;
    ///
    /// // Implement the traits.
    /// impl Vocabulary for ExampleVoc {
    ///   type Prefix = IRI;
    ///   type Full = IRI;
    ///
    ///  fn prefix() -> Self::Prefix {
    ///    IRI::from("ex:")
    ///  }
    ///
    ///   fn full() -> Self::Full {
    ///     IRI::from("https://example.com/")
    ///   }
    /// }
    ///
    /// assert_eq!(ExampleVoc::full(), IRI::from("https://example.com/"));
    /// ```
    fn full() -> Self::Full;
}
