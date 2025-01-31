// @generated automatically by Diesel CLI.

diesel::table! {
    session (id) {
        id -> Integer,
        user_id -> Text,
        token -> Text,
        token_valid -> Bool,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        name -> Text,
        lastname -> Text,
        email -> Text,
        address -> Text,
        phone_number -> Text,
        password -> Text,
        active -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    session,
    user,
);
