-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon_designation (
    taxon_designation_key char(16) PRIMARY KEY,
    date_from date NOT NULL,
    date_to date NULL,
    status_geographic_area text NULL,
    status_constraint text NULL,
    detail text NULL,
    taxon_designation_type_key char(
        16
    ) NOT NULL REFERENCES uksi.taxon_designation_type (
        taxon_designation_type_key
    ) ON DELETE CASCADE,
    taxon_list_item_key char(16) NOT NULL REFERENCES uksi.taxon_list_item (
        taxon_list_item_key
    ) ON DELETE CASCADE,
    status_exclusion boolean NOT NULL,
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
