-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon_group_name (
    taxon_group_key char(16) PRIMARY KEY,
    taxon_group_name text NOT NULL,
    kingdom text NULL,
    input_level_0_flag boolean NOT NULL,
    input_level1_flag boolean NOT NULL,
    input_level1_sort_code smallint NULL,
    input_level2_flag boolean NOT NULL,
    input_level2_sort_code smallint NULL,
    output_flag boolean NOT NULL,
    input_level1_descriptor text NULL,
    input_level2_descriptor text NULL,
    parent text NULL,
    entered_by char(16) NOT NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    entry_date timestamp NOT NULL,
    changed_by char(16) NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    changed_date timestamp NULL
);
