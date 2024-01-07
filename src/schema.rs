// @generated automatically by Diesel CLI.

diesel::table! {
    homes (id) {
        id -> Varchar,
        title -> Varchar,
        body -> Text,
    }
}

diesel::table! {
    items (id) {
        id -> Varchar,
        room_id -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
        category -> Varchar,
        purchase_date -> Varchar,
        expiry_date -> Nullable<Varchar>,
        value -> Float8,
    }
}

diesel::table! {
    rooms (id) {
        id -> Varchar,
        name -> Varchar,
        home_id -> Varchar,
    }
}

diesel::joinable!(items -> rooms (room_id));
diesel::joinable!(rooms -> homes (home_id));

diesel::allow_tables_to_appear_in_same_query!(
    homes,
    items,
    rooms,
);
