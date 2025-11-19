-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon_designation_type_kind (
    taxon_designation_type_kind_key char(16) PRIMARY KEY,
    kind text NULL,
    item_name text NULL,
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
