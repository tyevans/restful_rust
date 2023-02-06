CREATE TABLE auth_users
(
    id           uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    display_name TEXT    NOT NULL,
    email        TEXT    NOT NULL UNIQUE,
    active       BOOLEAN NOT NULL DEFAULT TRUE
);


CREATE TABLE auth_groups
(
    id          uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL
);

CREATE TABLE auth_permissions
(
    id          uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL
);

CREATE TABLE auth_group_permissions
(
    id            uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id      uuid NOT NULL,
    permission_id uuid NOT NULL,
    UNIQUE (group_id, permission_id),
    CONSTRAINT fk_group_permissions_group_id
        FOREIGN KEY (group_id)
            REFERENCES auth_groups (id),
    CONSTRAINT fk_group_permissions_permission_id
        FOREIGN KEY (permission_id)
            REFERENCES auth_permissions (id)
);


CREATE TABLE auth_user_groups
(
    id       uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id  uuid NOT NULL,
    group_id uuid NOT NULL,
    UNIQUE (user_id, group_id),
    CONSTRAINT fk_user_group_user_id
        FOREIGN KEY (user_id)
            REFERENCES auth_users (id) ON DELETE CASCADE,
    CONSTRAINT fk_user_group_group_id
        FOREIGN KEY (group_id)
            REFERENCES auth_groups (id) ON DELETE CASCADE
);
