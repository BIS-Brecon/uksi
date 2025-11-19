-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon (
    taxon_key char(16) PRIMARY KEY,
    item_name text NOT NULL,
    authority text NULL,
    introduced_vague_date_start date NULL,
    introduced_vague_date_end date NULL,
    introduced_vague_date_type char(2) NULL,
    language text NOT NULL,
    taxon_name_type_key char(16) NOT NULL REFERENCES uksi.taxon_name_type (
        taxon_name_type_key
    ) ON DELETE CASCADE,
    abbreviation char(5) NULL,
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
