// @generated automatically by Diesel CLI.

diesel::table! {
    classy_lukes (id) {
        id -> Nullable<Integer>,
        catchphrase -> Text,
        glasses -> Integer,
        hat -> Integer,
        left_earring -> Integer,
        right_earring -> Integer,
        sexiness -> Integer,
        suit -> Integer,
    }
}
