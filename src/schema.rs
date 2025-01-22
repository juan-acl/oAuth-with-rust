// @generated automatically by Diesel CLI.

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
