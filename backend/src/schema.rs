// @generated automatically by Diesel CLI.

diesel::table! {
    auth_group_permissions (id) {
        id -> Int4,
        group_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    auth_groups (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    auth_permissions (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    auth_user_groups (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    auth_users (id) {
        id -> Int4,
        display_name -> Text,
        email -> Text,
        active -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::joinable!(auth_group_permissions -> auth_groups (group_id));
diesel::joinable!(auth_group_permissions -> auth_permissions (permission_id));
diesel::joinable!(auth_user_groups -> auth_groups (group_id));
diesel::joinable!(auth_user_groups -> auth_users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_group_permissions,
    auth_groups,
    auth_permissions,
    auth_user_groups,
    auth_users,
    posts,
);
