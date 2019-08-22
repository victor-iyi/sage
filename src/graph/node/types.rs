/// `NodeTypes` consists of various kinds of valid nodes that is supported by the `sage` engine.
///
/// **NOTE:** This is not to be misplaced for Data types or a `Node` itself.
///
/// TLDR; `NodeType` represent the different forms which `Node` can exist.
#[derive(Debug)]
pub enum NodeTypes {
    /// `BlankNode` containing node with empty or null data.
    BlankNode,

    /// `SchemaNode` is created from some type of data structure.
    /// Usually but not limited to `struct`s.
    SchemaNode,

    /// `HttpNode` is used to represent data coming from an external/http source.
    /// And example of such [James Cameron](https://www.wikidata.org/wiki/Q42574)
    /// node gotten from [wikidata](https://www.wikidata.org/wiki/Wikidata:Main_Page).
    HttpNode,

    /// `LiteralNode` is used to represent nodes with primitive types
    /// like `Strings`, `Numbers`, `Date`, `Time`, `DateTime` etc.
    /// which contains no extra data associated to this node.
    LiteralNode,
}
