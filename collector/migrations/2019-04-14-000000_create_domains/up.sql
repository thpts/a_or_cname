CREATE TABLE domain (
    rank INTEGER,
    fqdn TEXT,
    sub TEXT NULL,
    root TEXT NULL,
    suffix TEXT NULL
);

CREATE TABLE record (
    domain INTEGER,
    parent INTEGER NULL,
    is_www BOOLEAN NOT NULL CHECK (is_www IN (0,1)),
    response_code INTEGER,
    record_type TEXT NULL,
    ttl INTEGER NULL,
    address TEXT NULL,
    asn INTEGER NULL,
    query_time INTEGER
);
