// @generated automatically by Diesel CLI.

diesel::table! {
    session (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        token -> Text,
        token_valid -> Bool,
    }
}

diesel::table! {
    user (id) {
        id -> Nullable<Integer>,
        name -> Text,
        lastname -> Text,
        email -> Text,
        address -> Text,
        phone_number -> Text,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    session,
    user,
);
