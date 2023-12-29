// @generated automatically by Diesel CLI.

diesel::table! {
    homes (id) {
        id -> Varchar,
        title -> Varchar,
        body -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> Varchar,
        name -> Varchar,
        home_id -> Varchar,
    }
}

diesel::joinable!(rooms -> homes (home_id));

diesel::allow_tables_to_appear_in_same_query!(
    homes,
    rooms,
);
