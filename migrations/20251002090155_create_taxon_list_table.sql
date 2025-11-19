-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon_list (
    taxon_list_key char(16) PRIMARY KEY,
    item_name text NOT NULL,
    description_long text NULL,
    description_short text NULL,
    authority text NULL,
    owner_key char(16) NULL REFERENCES uksi.owner (owner_key) ON DELETE CASCADE,
    taxon_list_type_key char(16) NOT NULL REFERENCES uksi.taxon_list_type (
        taxon_list_type_key
    ) ON DELETE CASCADE,
    update_mechanism text NULL,
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
