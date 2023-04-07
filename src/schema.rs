// @generated automatically by Diesel CLI.

diesel::table! {
    weights (id) {
        id -> Integer,
        timestamp -> Datetime,
        created_at -> Datetime,
        weight -> Float,
    }
}
