CREATE TABLE domain (
    rank INTEGER,
    fqdn TEXT,
    sub TEXT,
    root TEXT,
    suffix TEXT
);

CREATE TABLE record (
    domain INTEGER,
    parent INTEGER,
    response_code INTEGER,
    record_type TEXT,
    ttl INTEGER,
    address TEXT,
    asn INTEGER
);