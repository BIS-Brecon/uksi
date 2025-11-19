-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.aggregate (
    aggregate_id_key char(16) PRIMARY KEY,
    aggregate_tvk char(16) NOT NULL REFERENCES uksi.taxon_version (
        taxon_version_key
    ) ON DELETE CASCADE,
    component_tvk char(16) NOT NULL REFERENCES uksi.taxon_version (
        taxon_version_key
    ) ON DELETE CASCADE,
    source text NOT NULL,
    notes text NULL,
    vague_date_start date NULL,
    vague_date_end date NULL,
    vague_date_type char(2) NULL,
    entered_by char(16) NOT NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    entry_date timestamp NOT NULL,
    changed_by char(16) NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    changed_date timestamp NULL
);
