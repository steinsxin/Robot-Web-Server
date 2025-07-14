// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        context -> Text,
        published -> Bool,
    }
}

diesel::table! {
    robot_manager (id) {
        id -> Int4,
        robot_id -> Varchar,
        electricity -> Int4,
        activate -> Bool,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    robot_manager,
);
