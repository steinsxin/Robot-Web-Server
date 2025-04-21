// @generated automatically by Diesel CLI.

diesel::table! {
    robot_manager (id) {
        id -> Int4,
        robot_id -> Varchar,
        device_id -> Varchar,
        electricity -> Int4,
        activate -> Bool,
        updated_at -> Timestamp,
    }
}
