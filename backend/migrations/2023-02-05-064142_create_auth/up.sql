CREATE TABLE auth_users
(
    id           SERIAL PRIMARY KEY,
    display_name TEXT    NOT NULL,
    email        TEXT    NOT NULL UNIQUE,
    active       BOOLEAN NOT NULL DEFAULT TRUE
);


CREATE TABLE auth_groups
(
    id          SERIAL PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL
);

CREATE TABLE auth_permissions
(
    id          SERIAL PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL
);

CREATE TABLE auth_group_permissions
(
    id            SERIAL PRIMARY KEY,
    group_id      INTEGER NOT NULL,
    permission_id INTEGER NOT NULL,
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
    id       SERIAL PRIMARY KEY,
    user_id  INTEGER NOT NULL,
    group_id INTEGER NOT NULL,
    UNIQUE (user_id, group_id),
    CONSTRAINT fk_user_group_user_id
        FOREIGN KEY (user_id)
            REFERENCES auth_users (id),
    CONSTRAINT fk_user_group_group_id
        FOREIGN KEY (group_id)
            REFERENCES auth_groups (id)
);
