-- This was initially hand-edited on 2024-05-04 01:09:50
-- Once the system goes in production, this file becomes finalized, as it's hash must stay the same.
CREATE TABLE IF NOT EXISTS universities (
    id uuid PRIMARY KEY,
    name_full character varying(100) NOT NULL,
    name_mid character varying(50) NOT NULL,
    name_short character varying(50) NOT NULL,
    domain_names character varying(100) [] NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id uuid PRIMARY KEY,
    first_names character varying(200),
    last_name character varying(200),
    -- REFERENCES emails (id):
    primary_email uuid NOT NULL,
    password_hash character varying(250) NOT NULL,
    totp_secret character varying(50),
    nick character varying(100),
    user_role smallint NOT NULL DEFAULT 1
);

-- Does NOT work: IF NOT EXISTS
CREATE TYPE email_status AS ENUM ('unverified', 'verified', 'disabled');

CREATE TABLE IF NOT EXISTS emails (
    id uuid PRIMARY KEY,
    address character varying(500),
    belongs_to_user uuid NOT NULL REFERENCES users (id),
    of_university uuid REFERENCES universities (id),
    "status" email_status NOT NULL
    -- TODO: Add Columns with added and modified timestamp? 
);

ALTER TABLE
    users
ADD
    CONSTRAINT "user_primary_email_fkey" FOREIGN KEY (primary_email) REFERENCES emails (id);

CREATE TABLE IF NOT EXISTS email_verification (
    id uuid PRIMARY KEY,
    token character(43) NOT NULL,
    belongs_to_email uuid NOT NULL REFERENCES emails (id),
    expires_at timestamp without time zone NOT NULL
);

CREATE TABLE IF NOT EXISTS courses (
    id uuid NOT NULL PRIMARY KEY,
    held_at uuid NOT NULL REFERENCES universities (id),
    course_name character varying(200) NOT NULL
);

CREATE TABLE IF NOT EXISTS profs (
    id uuid PRIMARY KEY,
    prof_name character varying(500) NOT NULL
);

CREATE TYPE upload_type_enum AS enum (
    'exam',
    'exam_prep',
    'course_summary',
    'homework',
    'lecture_notes',
    'question_collection',
    'protocol',
    'other',
    'script',
    'presentation',
    'unknown'
);

CREATE TABLE IF NOT EXISTS uploads (
    id uuid PRIMARY KEY,
    upload_name character varying(200) NOT NULL,
    description text NOT NULL,
    price smallint NOT NULL,
    uploader uuid NOT NULL REFERENCES users (id),
    -- The date when the upload was uploaded,
    upload_date timestamp without time zone NOT NULL,
    last_modified_date timestamp without time zone NOT NULL,
    -- The date associated with the upload, e.g. the date of the exam (nullable)
    associated_date timestamp without time zone,
    upload_type upload_type_enum NOT NULL,
    belongs_to uuid NOT NULL REFERENCES courses (id),
    held_by uuid REFERENCES profs (id)
);

CREATE TABLE IF NOT EXISTS purchases (
    -- This table uses a composite primary key
    user_id uuid REFERENCES users (id),
    upload_id uuid REFERENCES uploads (id),
    ecs_spent smallint NOT NULL,
    purchase_date timestamp without time zone NOT NULL,
    rating smallint,
    PRIMARY KEY (user_id, upload_id)
);

CREATE TABLE IF NOT EXISTS system_ec_transactions (
    -- This table uses a composite primary key
    affected_user uuid REFERENCES users (id),
    transaction_date timestamp without time zone NOT NULL,
    delta_ec bigint NOT NULL,
    reason character varying(1000),
    PRIMARY KEY (affected_user, transaction_date)
);

CREATE TABLE IF NOT EXISTS sessions (
    id uuid PRIMARY KEY,
    of_user uuid REFERENCES users (id),
    token character(43) NOT NULL,
    initial_user_agent character varying(275),
    latest_user_agent character varying(275),
    initial_ip inet,
    latest_ip inet
    -- TODO: Shouldn't there also be timestamps so we can expire sessions?
);

CREATE TABLE IF NOT EXISTS files (
    id uuid PRIMARY KEY,
    name character varying(255) NOT NULL,
    mime_type character varying(200) NOT NULL,
    size bigint NOT NULL,
    sha3_256 character varying(64) NOT NULL,
    revision_at timestamp without time zone NOT NULL,
    upload_id uuid NOT NULL REFERENCES uploads (id),
    approval_uploader boolean NOT NULL,
    approval_mod boolean NOT NULL
);

CREATE INDEX idx_upload_uploader ON uploads(uploader);

CREATE INDEX idx_purchase_user_id ON purchases(user_id);

CREATE INDEX idx_purchase_user_id_rating ON purchases(user_id, rating);

CREATE INDEX idx_purchase_upload_id ON purchases(upload_id);

CREATE INDEX idx_system_ec_transaction_affected_user ON system_ec_transactions(affected_user);
