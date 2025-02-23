-- This was initially hand-edited on 2024-05-04 01:09:50
-- Once the system goes in production, this file becomes finalized, as it's hash must stay the same.

CREATE TYPE rgb_color AS (
    r "char",
    g "char",
    b "char"
);

CREATE TABLE IF NOT EXISTS universities (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name_full character varying(100) NOT NULL UNIQUE,
    name_mid character varying(50) NOT NULL UNIQUE,
    name_short character varying(50) NOT NULL UNIQUE,
    email_domain_names character varying(100) [] NOT NULL,
    homepage_url character varying(200) NOT NULL,
    cms_url character varying(200) NOT NULL,
    background_color rgb_color NOT NULL,
    text_color rgb_color NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    first_names character varying(200),
    last_name character varying(200),
    -- REFERENCES emails (id):
    primary_email uuid NOT NULL,
    password_hash character varying(250) NOT NULL,
    totp_secret character varying(50),
    nick character varying(100) UNIQUE,
    user_role smallint NOT NULL DEFAULT 1
);

-- Does NOT work: IF NOT EXISTS
CREATE TYPE email_status AS ENUM ('unverified', 'verified', 'disabled');

CREATE TABLE IF NOT EXISTS emails (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    address character varying(500),
    belongs_to_user uuid NOT NULL REFERENCES users (id),
    of_university uuid REFERENCES universities (id),
    "status" email_status NOT NULL -- TODO: Add Columns with added and modified timestamp? 
);

ALTER TABLE
    users
ADD
    CONSTRAINT "user_primary_email_fkey" FOREIGN KEY (primary_email) REFERENCES emails (id);

CREATE TABLE IF NOT EXISTS email_verification (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
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
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
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
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
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
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    of_user uuid REFERENCES users (id),
    token character(43) NOT NULL,
    initial_user_agent character varying(275),
    latest_user_agent character varying(275),
    initial_ip inet,
    latest_ip inet -- TODO: Shouldn't there also be timestamps so we can expire sessions?
);

CREATE TABLE IF NOT EXISTS files (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
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

-- ---------------------------------------------------------
-- 
-- Audit logs
-- Taken from https://wiki.postgresql.org/wiki/Audit_trigger
--
-- ---------------------------------------------------------
-- create a schema named "audit"

-- create a schema named "audit"
create schema audit;
revoke create on schema audit from public;

create table audit.logged_actions (
    schema_name text not null,
    table_name text not null,
    user_name text,
    action_tstamp timestamp with time zone not null default current_timestamp,
    action TEXT NOT NULL check (action in ('I','D','U')),
    original_data text,
    new_data text,
    query text
) with (fillfactor=100);

revoke all on audit.logged_actions from public;

-- You may wish to use different permissions; this lets anybody
-- see the full audit data. In Pg 9.0 and above you can use column
-- permissions for fine-grained control.
grant select on audit.logged_actions to public;

create index logged_actions_schema_table_idx 
on audit.logged_actions(((schema_name||'.'||table_name)::TEXT));

create index logged_actions_action_tstamp_idx 
on audit.logged_actions(action_tstamp);

create index logged_actions_action_idx 
on audit.logged_actions(action);

--
-- Now, define the actual trigger function:
--

CREATE OR REPLACE FUNCTION audit.if_modified_func() RETURNS trigger AS $body$
DECLARE
    v_old_data TEXT;
    v_new_data TEXT;
BEGIN
    /*  If this actually for real auditing (where you need to log EVERY action),
        then you would need to use something like dblink or plperl that could log outside the transaction,
        regardless of whether the transaction committed or rolled back.
    */

    /* This dance with casting the NEW and OLD values to a ROW is not necessary in pg 9.0+ */

    if (TG_OP = 'UPDATE') then
        v_old_data := ROW(OLD.*);
        v_new_data := ROW(NEW.*);
        insert into audit.logged_actions (schema_name,table_name,user_name,action,original_data,new_data,query) 
        values (TG_TABLE_SCHEMA::TEXT,TG_TABLE_NAME::TEXT,session_user::TEXT,substring(TG_OP,1,1),v_old_data,v_new_data, current_query());
        RETURN NEW;
    elsif (TG_OP = 'DELETE') then
        v_old_data := ROW(OLD.*);
        insert into audit.logged_actions (schema_name,table_name,user_name,action,original_data,query)
        values (TG_TABLE_SCHEMA::TEXT,TG_TABLE_NAME::TEXT,session_user::TEXT,substring(TG_OP,1,1),v_old_data, current_query());
        RETURN OLD;
    elsif (TG_OP = 'INSERT') then
        v_new_data := ROW(NEW.*);
        insert into audit.logged_actions (schema_name,table_name,user_name,action,new_data,query)
        values (TG_TABLE_SCHEMA::TEXT,TG_TABLE_NAME::TEXT,session_user::TEXT,substring(TG_OP,1,1),v_new_data, current_query());
        RETURN NEW;
    else
        RAISE WARNING '[AUDIT.IF_MODIFIED_FUNC] - Other action occurred: %, at %',TG_OP,now();
        RETURN NULL;
    end if;

EXCEPTION
    WHEN data_exception THEN
        RAISE WARNING '[AUDIT.IF_MODIFIED_FUNC] - UDF ERROR [DATA EXCEPTION] - SQLSTATE: %, SQLERRM: %',SQLSTATE,SQLERRM;
        RETURN NULL;
    WHEN unique_violation THEN
        RAISE WARNING '[AUDIT.IF_MODIFIED_FUNC] - UDF ERROR [UNIQUE] - SQLSTATE: %, SQLERRM: %',SQLSTATE,SQLERRM;
        RETURN NULL;
    WHEN others THEN
        RAISE WARNING '[AUDIT.IF_MODIFIED_FUNC] - UDF ERROR [OTHER] - SQLSTATE: %, SQLERRM: %',SQLSTATE,SQLERRM;
        RETURN NULL;
END;
$body$
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, audit;

--
-- To add this trigger to a table, use:
-- CREATE TRIGGER tablename_audit
-- AFTER INSERT OR UPDATE OR DELETE ON tablename
-- FOR EACH ROW EXECUTE PROCEDURE audit.if_modified_func();
--

-- Enable audit for universities
CREATE TRIGGER universities_audit AFTER INSERT OR UPDATE OR DELETE ON universities FOR EACH ROW EXECUTE PROCEDURE audit.if_modified_func();

-- Enable audit for users
CREATE TRIGGER users_audit AFTER INSERT OR UPDATE OR DELETE ON users FOR EACH ROW EXECUTE PROCEDURE audit.if_modified_func();

-- Enable audit for emails
CREATE TRIGGER emails_audit AFTER INSERT OR UPDATE OR DELETE ON emails FOR EACH ROW EXECUTE PROCEDURE audit.if_modified_func();

-- Enable audit for email_verification
CREATE TRIGGER email_verification_audit AFTER INSERT OR UPDATE OR DELETE ON email_verification FOR EACH ROW EXECUTE PROCEDURE audit.if_modified_func();

-- Enable audit for courses
CREATE TRIGGER courses_audit AFTER INSERT OR UPDATE OR DELETE ON courses FOR EACH ROW EXECUTE PROCEDURE audit.if_modified_func();

-- Enable audit for profs
CREATE TRIGGER profs_audit AFTER INSERT OR UPDATE OR DELETE ON profs FOR EACH ROW EXECUTE PROCEDURE audit.if_modified_func();

-- Enable audit for uploads
CREATE TRIGGER uploads_audit AFTER INSERT OR UPDATE OR DELETE ON uploads FOR EACH ROW EXECUTE PROCEDURE audit.if_modified_func();

-- Enable audit for purchases
CREATE TRIGGER purchases_audit AFTER INSERT OR UPDATE OR DELETE ON purchases FOR EACH ROW EXECUTE PROCEDURE audit.if_modified_func();

-- Enable audit for system_ec_transactions
CREATE TRIGGER system_ec_transactions_audit AFTER INSERT OR UPDATE OR DELETE ON system_ec_transactions FOR EACH ROW EXECUTE PROCEDURE audit.if_modified_func();

-- Enable audit for sessions
CREATE TRIGGER sessions_audit AFTER INSERT OR UPDATE OR DELETE ON sessions FOR EACH ROW EXECUTE PROCEDURE audit.if_modified_func();

-- Enable audit for files
CREATE TRIGGER files_audit AFTER INSERT OR UPDATE OR DELETE ON files FOR EACH ROW EXECUTE PROCEDURE audit.if_modified_func();
