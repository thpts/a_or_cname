table! {
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
