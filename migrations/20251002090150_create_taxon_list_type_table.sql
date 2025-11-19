-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon_list_type (
    taxon_list_type_key char(16) PRIMARY KEY,
    short_name text NOT NULL,
    long_name text NOT NULL,
    description text NULL,
    schedule boolean NOT NULL,
    entered_by char(16) NOT NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    entry_date timestamp NOT NULL,
    system_supplied_data boolean NOT NULL
);
