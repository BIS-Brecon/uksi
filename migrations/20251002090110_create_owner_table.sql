-- Add migration script here
CREATE TABLE IF NOT EXISTS uksi.owner (
    owner_key char(16) PRIMARY KEY,
    owner_name text NOT NULL,
    owner_shortname text NOT NULL,
    home_url text NULL
);
