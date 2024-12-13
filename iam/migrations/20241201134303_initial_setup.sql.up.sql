CREATE SEQUENCE tenant_sq START WITH 100 INCREMENT BY 1;

CREATE TABLE tenant
(
    id          INTEGER                  NOT NULL DEFAULT NEXTVAL('tenant_sq'),
    version     INTEGER                  NOT NULL DEFAULT 0,
    uuid        UUID                     NOT NULL,
    name        VARCHAR(35)              NOT NULL,
    description VARCHAR(255)             NOT NULL,
    enabled     BOOL                     NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT tenant_pk PRIMARY KEY (id),
    CONSTRAINT tenant_uuid_uk UNIQUE (uuid),
    CONSTRAINT tenant_name_uk UNIQUE (name)
);

ALTER SEQUENCE tenant_sq OWNED BY tenant.id;

CREATE SEQUENCE invitation_sq START WITH 100 INCREMENT BY 1;

CREATE TABLE invitation
(
    id          INTEGER      NOT NULL DEFAULT NEXTVAL('invitation_sq'),
    tenant_id   INTEGER      NOT NULL,
    identifier  VARCHAR(36)  NOT NULL,
    description VARCHAR(255) NOT NULL,
    valid_from  TIMESTAMP WITH TIME ZONE,
    until       TIMESTAMP WITH TIME ZONE,
    CONSTRAINT invitation_pk PRIMARY KEY (id),
    CONSTRAINT invitation_tenant_fk FOREIGN KEY (tenant_id) REFERENCES tenant (id) ON DELETE CASCADE,
    CONSTRAINT invitation_identifier_uk UNIQUE (identifier)
);

ALTER SEQUENCE invitation_sq OWNED BY invitation.id;

CREATE SEQUENCE user_sq START WITH 100 INCREMENT BY 1;

CREATE TABLE "user"
(
    id         INTEGER                  NOT NULL DEFAULT NEXTVAL('user_sq'),
    version    INTEGER                  NOT NULL DEFAULT 0,
    tenant_id  INTEGER                  NOT NULL,
    username   VARCHAR(255)             NOT NULL,
    password   VARCHAR(255)             NOT NULL,
    enabled    BOOL                     NOT NULL DEFAULT TRUE,
    start_date TIMESTAMP WITH TIME ZONE,
    end_date   TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_pk PRIMARY KEY (id),
    CONSTRAINT user_tenant_fk FOREIGN KEY (tenant_id) REFERENCES tenant (id) ON DELETE CASCADE,
    CONSTRAINT user_tenant_username_uk UNIQUE (tenant_id, username)
);

ALTER SEQUENCE user_sq OWNED BY "user".id;

CREATE TABLE person
(
    id                  INTEGER      NOT NULL,
    first_name          VARCHAR(70)  NOT NULL,
    last_name           VARCHAR(70)  NOT NULL,
    email_address       VARCHAR(255) NOT NULL,
    street_name         VARCHAR(150),
    building_number     VARCHAR(18),
    postal_code         VARCHAR(18),
    city                VARCHAR(35),
    state_province      VARCHAR(10),
    country_code        CHAR(2),
    primary_telephone   VARCHAR(20),
    secondary_telephone VARCHAR(20),
    CONSTRAINT person_pk PRIMARY KEY (id),
    CONSTRAINT person_user_fk FOREIGN KEY (id) REFERENCES "user" (id) ON DELETE CASCADE
);

CREATE INDEX person_first_name_ix ON person (first_name);
CREATE INDEX person_last_name_ix ON person (last_name);

CREATE SEQUENCE group_sq START WITH 100 INCREMENT BY 1;

CREATE TABLE "group"
(
    id          INTEGER                  NOT NULL DEFAULT NEXTVAL('group_sq'),
    version     INTEGER                  NOT NULL DEFAULT 0,
    tenant_id   INTEGER                  NOT NULL,
    name        VARCHAR(70)              NOT NULL,
    description VARCHAR(255),
    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT group_pk PRIMARY KEY (id),
    CONSTRAINT group_tenant_fk FOREIGN KEY (tenant_id) REFERENCES tenant (id) ON DELETE CASCADE,
    CONSTRAINT group_tenant_name_uk UNIQUE (tenant_id, name)
);

ALTER SEQUENCE group_sq OWNED BY "group".id;

CREATE TYPE group_member_type AS ENUM ('USER', 'GROUP');

CREATE TABLE group_member
(
    group_id INTEGER           NOT NULL,
    "type"   group_member_type NOT NULL,
    name     VARCHAR(255)      NOT NULL,
    CONSTRAINT group_member_pk PRIMARY KEY (group_id, "type", name),
    CONSTRAINT group_member_group_fk FOREIGN KEY (group_id) REFERENCES "group" (id) ON DELETE CASCADE
);

CREATE SEQUENCE role_sq START WITH 100 INCREMENT BY 1;

CREATE TABLE "role"
(
    id               INTEGER                  NOT NULL DEFAULT NEXTVAL('role_sq'),
    version          INTEGER                  NOT NULL DEFAULT 0,
    tenant_id        INTEGER                  NOT NULL,
    name             VARCHAR(70)              NOT NULL,
    description      VARCHAR(255),
    supports_nesting BOOL                     NOT NULL DEFAULT TRUE,
    group_id         INTEGER                  NOT NULL,
    created_at       TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at       TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT role_pk PRIMARY KEY (id),
    CONSTRAINT role_tenant_fk FOREIGN KEY (tenant_id) REFERENCES tenant (id) ON DELETE CASCADE,
    CONSTRAINT role_group_fk FOREIGN KEY (group_id) REFERENCES "group" (id) ON DELETE CASCADE,
    CONSTRAINT role_tenant_name_uk UNIQUE (tenant_id, name)
);

ALTER SEQUENCE role_sq OWNED BY "role".id;