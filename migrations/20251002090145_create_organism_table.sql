-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.organism (
    organism_key char(16) PRIMARY KEY,
    parent_key char(16) NULL REFERENCES uksi.organism (
        organism_key
    ) ON DELETE CASCADE,
    taxon_version_key char(16) NOT NULL REFERENCES uksi.taxon_version (
        taxon_version_key
    ) ON DELETE CASCADE,
    uk_status_key char(16) NULL,
    marine_flag char(1) NULL,
    terrestrial_freshwater_flag char(1) NULL,
    freshwater char(1) NULL,
    redundant_flag char(1) NULL,
    non_native_flag char(1) NULL,
    only_in_not_fit_for_web char(1) NULL,
    orphan char(1) NULL,
    vernacular char(1) NULL,
    organism_rank_key char(16) NULL REFERENCES uksi.taxon_rank (
        taxon_rank_key
    ) ON DELETE CASCADE,
    lineage text NULL,
    sort_level int NULL,
    weight int NULL,
    sort_order text NULL,
    sort_code text NULL,
    has_children boolean NOT NULL,
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

CREATE INDEX idx_organism_tvk ON uksi.organism (taxon_version_key);
