// @generated automatically by Diesel CLI.

diesel::table! {
    livros (id) {
        id -> Int4,
        title -> Varchar,
        author -> Text,
        published -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    livros,
    posts,
);
