// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        #[max_length = 50]
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
