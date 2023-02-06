// @generated automatically by Diesel CLI.

diesel::table! {
    auth_group_permissions (id) {
        id -> Uuid,
        group_id -> Uuid,
        permission_id -> Uuid,
    }
}

diesel::table! {
    auth_groups (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    auth_permissions (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    auth_user_groups (id) {
        id -> Uuid,
        user_id -> Uuid,
        group_id -> Uuid,
    }
}

diesel::table! {
    auth_users (id) {
        id -> Uuid,
        display_name -> Text,
        email -> Text,
        active -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
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
