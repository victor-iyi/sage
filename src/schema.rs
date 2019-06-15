table! {
    edge (id) {
        id -> Int4,
        predicate -> Varchar,
        dest_vertex_id -> Varchar,
        src_vertex_id -> Varchar,
    }
}

table! {
    graph (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Text,
    }
}

table! {
    vertex (id) {
        id -> Varchar,
        label -> Varchar,
        schema -> Varchar,
        graph_id -> Varchar,
    }
}

joinable!(edge -> vertex (src_vertex_id));
joinable!(vertex -> graph (graph_id));

allow_tables_to_appear_in_same_query!(edge, graph, vertex,);
