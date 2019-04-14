table! {
    /// # Domain
    /// This table represents the domains to be queried and their ranking.
    /// For the sake of further analysis, we persist not only the fully qualified
    /// domain name (FQDN), but also its constituent components, however we have
    /// decided not to separate any secondary top-level domains from their parents.
    ///
    /// Using Public Suffix List nomenclature, a domain comprises of a few elements:
    ///
    /// ```text
    ///              foo.example.co.uk
    ///             |-----------------|  - Fully Qualified Domain Name (FQDN)
    ///             |---|                - Sub-domain
    ///                 |-------------|  - Root, comprised of domain with any TLD
    ///                         |-----|  - Suffix, including all tiers of TLD
    /// ```
    domain (rank) { // FIXME rank should *not* be primary key, rowid is sufficient

        /// Ranking of the domain as provided by the list source
        rank -> Integer,

        /// Fully Qualified Domain Name, e.g. "foo.example.co.uk"
        fqdn -> Text,

        /// Sub-domain, e.g "foo"
        sub -> Text,

        /// Root domain, e.g "example.co.uk"
        root -> Text,

        /// Suffix, e.g "co.uk"
        suffix -> Text,
    }
}

table! {
    /// # Record
    /// This table represents the answers performed for the DNS queries made. Each row represents a
    /// record returned from any number of DNS queries performed - one query may result in `n`, for
    /// example a query for `IN A example.com` may return `CNAME example.net` and `A 127.0.0.1`.
    record (domain) {

        /// row-id of the domain from which the DNS query was derived from.
        domain -> Integer,

        /// If the record is a child, (e.g. the A of a CNAME), this integer refers to the row-id
        /// of the parent.
        parent -> Integer,

        /// RCODE value from the Answer (see
        /// [RFC 1035 &sect; 4.1.1](https://tools.ietf.org/html/rfc1035#section-4.1.1)) - this value
        /// is persisted to distinguish failures such as receiving `NXDOMAIN`.
        response_code -> Integer,

        /// Record Type, e.g "CNAME", "A" etc.
        record_type -> Text,

        /// The record's Time To Live value, which may have applicability in future understanding
        /// record freshness across the dataset.
        ttl -> Integer,

        /// Address provided in the record, this may be a FQDN or IP address.
        address -> Text,

        /// The Autonomous System Number assigned for any IP addresses found in the address. This is
        /// not provided in the DNS response but by matching an address value that equals an IP
        /// address against a database of CIDR ranges to match against it.
        asn -> Integer,
    }
}