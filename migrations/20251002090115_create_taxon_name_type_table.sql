-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon_name_type (
    taxon_name_type_key char(16) PRIMARY KEY,
    short_name text NOT NULL,
    long_name text NULL,
    description text NULL,
    authority text NULL,
    entered_by char(16) NOT NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    entry_date timestamp NOT NULL,
    system_supplied_data boolean NOT NULL
);
