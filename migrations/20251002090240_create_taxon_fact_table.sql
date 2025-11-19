-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon_fact (
    taxon_fact_key char(16) PRIMARY KEY,
    title text NOT NULL,
    type char(1) NOT NULL,
    taxon_version_key char(16) NOT NULL REFERENCES uksi.taxon_version (
        taxon_version_key
    ) ON DELETE CASCADE,
    fact_vague_date_start date NULL,
    fact_vague_date_end date NULL,
    fact_vague_date_type char(2) NULL,
    entered_by char(16) NOT NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    entry_date timestamp NOT NULL,
    changed_by char(16) NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    changed_date timestamp NULL
);
