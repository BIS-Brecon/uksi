-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.nameserver (
    input_taxon_version_key char(
        16
    ) PRIMARY KEY REFERENCES uksi.taxon_version (
        taxon_version_key
    ) ON DELETE CASCADE,
    taxon_version_form char(1) NULL,
    taxon_version_status char(1) NULL,
    taxon_type char(1) NULL,
    recommended_taxon_version_key char(
        16
    ) NOT NULL REFERENCES uksi.taxon_version (
        taxon_version_key
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
