table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
