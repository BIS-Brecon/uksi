-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon_version (
    taxon_version_key char(16) PRIMARY KEY,
    taxon_key char(16) NOT NULL REFERENCES uksi.taxon (
        taxon_key
    ) ON DELETE CASCADE,
    attribute text NULL,
    date_from timestamp NULL,
    date_to timestamp NULL,
    comment text NULL,
    validation_level smallint NULL,
    uk_native boolean NOT NULL,
    quality text NULL,
    output_group_key char(16) NULL REFERENCES uksi.taxon_group_name (
        taxon_group_key
    ) ON DELETE CASCADE,
    taxon_rank_key char(16) NOT NULL REFERENCES uksi.taxon_rank (
        taxon_rank_key
    ) ON DELETE CASCADE,
    gender char(1) NULL,
    plural boolean NOT NULL,
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
