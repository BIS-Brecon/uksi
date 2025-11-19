-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon_list_item (
    taxon_list_item_key char(16) PRIMARY KEY,
    taxon_version_key char(16) NOT NULL REFERENCES uksi.taxon_version (
        taxon_version_key
    ) ON DELETE CASCADE,
    taxon_list_version_key char(
        16
    ) NOT NULL REFERENCES uksi.taxon_list_version (
        taxon_list_version_key
    ) ON DELETE CASCADE,
    taxon_list_version_to char(16) NULL REFERENCES uksi.taxon_list_version (
        taxon_list_version_key
    ) ON DELETE CASCADE,
    preferred_name char(16) NOT NULL REFERENCES uksi.taxon_list_item (
        taxon_list_item_key
    ) ON DELETE CASCADE,
    sort_code integer NULL,
    lst_itm_code text NULL,
    parent char(16) NULL REFERENCES uksi.taxon_list_item (
        taxon_list_item_key
    ) ON DELETE CASCADE,
    taxon_rank_key char(16) NOT NULL REFERENCES uksi.taxon_rank (
        taxon_rank_key
    ) ON DELETE CASCADE,
    code_source text NULL,
    note text NULL,
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
