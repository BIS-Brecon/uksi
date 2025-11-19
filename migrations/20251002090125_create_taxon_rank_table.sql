-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.taxon_rank (
    taxon_rank_key char(16) PRIMARY KEY,
    sequence smallint NOT NULL,
    short_name text NULL,
    long_name text NOT NULL,
    abbreviation text NULL,
    description text NULL,
    list_font_italic boolean NOT NULL,
    display_in_details boolean NOT NULL,
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
