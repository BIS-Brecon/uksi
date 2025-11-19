-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.individual (
    name_key char(16) PRIMARY KEY,
    title text NULL,
    forename text NULL,
    initials text NULL,
    honorifics text NULL,
    surname text NULL,
    comment text NULL,
    born_vague_date_start date NULL,
    born_vague_date_end date NULL,
    born_vague_date_type varchar(2) NULL,
    died_vague_date_start date NULL,
    died_vague_date_end date NULL,
    died_vague_date_type varchar(2) NULL,
    person_floreat text NULL,
    entered_by char(16) NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    entry_date timestamp NULL,
    changed_by char(16) NULL REFERENCES uksi.individual (
        name_key
    ) ON DELETE CASCADE,
    changed_date timestamp NULL,
    system_supplied_data boolean NOT NULL
);
