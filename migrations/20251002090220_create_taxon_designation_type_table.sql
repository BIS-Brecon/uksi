-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon_designation_type (
    taxon_designation_type_key char(16) PRIMARY KEY,
    short_name text NOT NULL,
    long_name text NOT NULL,
    description text NULL,
    kind char(16) NOT NULL REFERENCES uksi.taxon_designation_type_kind (
        taxon_designation_type_kind_key
    ) ON DELETE CASCADE,
    entered_by char(16) NOT NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    entry_date timestamp NOT NULL,
    changed_by char(16) NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    changed_date timestamp NULL,
    system_supplied_data boolean NOT NULL
);
