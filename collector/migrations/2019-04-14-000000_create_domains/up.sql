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
    response_code INTEGER,
    record_type TEXT NULL,
    ttl INTEGER NULL,
    address TEXT NULL,
    asn INTEGER NULL,
    query_time INTEGER
);
