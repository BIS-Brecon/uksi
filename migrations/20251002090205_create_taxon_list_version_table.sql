-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon_list_version (
    taxon_list_version_key char(16) PRIMARY KEY,
    taxon_list_key char(16) NOT NULL REFERENCES uksi.taxon_list (
        taxon_list_key
    ) ON DELETE CASCADE,
    version integer NOT NULL,
    authority text NULL,
    owner_key char(16) NULL REFERENCES uksi.owner (owner_key) ON DELETE CASCADE,
    vague_date_start date NULL,
    vague_date_end date NULL,
    vague_date_type char(2) NULL,
    description text NULL,
    version_is_amendment boolean NOT NULL,
    quality text NULL,
    recommended_list boolean NOT NULL,
    last_updated date NULL,
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
