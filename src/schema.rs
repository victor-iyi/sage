table! {
    graph (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

table! {
    vertex (id) {
        id -> Int4,
        label -> Varchar,
        schema -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    graph,
    vertex,
);
