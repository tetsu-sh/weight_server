// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Datetime,
    }
}

diesel::table! {
    weights (id) {
        id -> Integer,
        timestamp -> Datetime,
        created_at -> Datetime,
        weight -> Float,
        device_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    devices,
    weights,
);
