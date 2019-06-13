table! {
    graph (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        vertices -> Nullable<Uuid>,
    }
}

table! {
    payload (id) {
        id -> Int4,
        key -> Varchar,
        value -> Text,
        vertex_id -> Uuid,
    }
}

table! {
    vertex (id) {
        id -> Uuid,
        label -> Varchar,
        schema -> Varchar,
    }
}

joinable!(graph -> vertex (vertices));
joinable!(payload -> vertex (vertex_id));

allow_tables_to_appear_in_same_query!(
    graph,
    payload,
    vertex,
);
