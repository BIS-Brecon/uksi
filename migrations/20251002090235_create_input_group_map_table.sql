-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.input_group_map (
    input_group_map_key char(16) PRIMARY KEY,
    taxon_group_key char(16) NOT NULL REFERENCES uksi.taxon_group_name (
        taxon_group_key
    ) ON DELETE CASCADE,
    taxon_version_key char(16) NOT NULL REFERENCES uksi.taxon_version (
        taxon_version_key
    ) ON DELETE CASCADE,
    taxon_rank char(16) NOT NULL REFERENCES uksi.taxon_rank (
        taxon_rank_key
    ) ON DELETE CASCADE,
    entered_by char(16) NOT NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    entry_date timestamp NOT NULL,
    changed_by char(16) NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    changed_date timestamp NULL
);
