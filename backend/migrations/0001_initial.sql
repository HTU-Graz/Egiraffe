-- This was initially hand-edited on 2024-05-04 01:09:50
-- Once the system goes in production, this file becomes finalized, as it's hash must stay the same.
CREATE TABLE IF NOT EXISTS university (
    id uuid PRIMARY KEY,
    name_full character varying(100) NOT NULL,
    name_mid character varying(50) NOT NULL,
    name_short character varying(50) NOT NULL,
    domain_names character varying(100) [] NOT NULL
);

CREATE TABLE IF NOT EXISTS "user" (
    id uuid PRIMARY KEY,
    first_names character varying(200),
    last_name character varying(200),
    -- REFERENCES email (id):
    primary_email uuid NOT NULL,
    password_hash character varying(250) NOT NULL,
    totp_secret character varying(50),
    nick character varying(100),
    user_role smallint NOT NULL DEFAULT 1
);

CREATE TYPE IF NOT EXISTS email_status AS ENUM ("unverified", "verified", "disabled");

CREATE TABLE IF NOT EXISTS email (
    id uuid PRIMARY KEY,
    address character varying(500),
    belongs_to_user uuid NOT NULL REFERENCES "user" (id),
    of_university uuid REFERENCES university (id),
    "status" email_status NOT NULL,
    -- TODO: Add Columns with added and modified timestamp? 
);

ALTER TABLE
    "user"
ADD
    CONSTRAINT "user_primary_email_fkey" FOREIGN KEY (primary_email) REFERENCES email (id);

CREATE TABLE IF NOT EXISTS email_verification (
    id uuid PRIMARY KEY,
    token character(43) NOT NULL,
    belongs_to_email uuid NOT NULL REFERENCES "email" (id),
    expires_at timestamp without time zone NOT NULL,
);

CREATE TABLE IF NOT EXISTS course (
    id uuid NOT NULL PRIMARY KEY,
    held_at uuid NOT NULL REFERENCES university (id),
    course_name character varying(200) NOT NULL
);

CREATE TABLE IF NOT EXISTS prof (
    id uuid PRIMARY KEY,
    prof_name character varying(500) NOT NULL
);

CREATE TABLE IF NOT EXISTS upload (
    id uuid PRIMARY KEY,
    upload_name character varying(200) NOT NULL,
    description text NOT NULL,
    price smallint NOT NULL,
    uploader uuid NOT NULL REFERENCES "user" (id),
    upload_date timestamp without time zone NOT NULL,
    last_modified_date timestamp without time zone NOT NULL,
    belongs_to uuid NOT NULL REFERENCES course (id),
    held_by uuid REFERENCES prof (id)
);

CREATE TABLE IF NOT EXISTS purchase (
    -- This table uses a composite primary key
    user_id uuid REFERENCES "user" (id),
    upload_id uuid REFERENCES upload (id),
    ecs_spent smallint NOT NULL,
    purchase_date timestamp without time zone NOT NULL,
    rating smallint,
    PRIMARY KEY (user_id, upload_id)
);

CREATE TABLE IF NOT EXISTS system_ec_transaction (
    -- This table uses a composite primary key
    affected_user uuid REFERENCES "user" (id),
    transaction_date timestamp without time zone NOT NULL,
    delta_ec bigint NOT NULL,
    reason character varying(1000),
    PRIMARY KEY (affected_user, transaction_date)
);

CREATE TABLE IF NOT EXISTS "session" (
    id uuid PRIMARY KEY,
    of_user uuid REFERENCES "user" (id),
    token character(43) NOT NULL,
    initial_user_agent character varying(275),
    latest_user_agent character varying(275),
    initial_ip inet,
    latest_ip inet
    -- TODO: Shouldn't there also be timestamps so we can expire sessions?
);

CREATE TABLE IF NOT EXISTS "file" (
    id uuid PRIMARY KEY,
    name character varying(255) NOT NULL,
    mime_type character varying(200) NOT NULL,
    size bigint NOT NULL,
    revision_at timestamp without time zone NOT NULL,
    upload_id uuid NOT NULL REFERENCES upload (id),
    approval_uploader boolean NOT NULL,
    approval_mod boolean NOT NULL
);
