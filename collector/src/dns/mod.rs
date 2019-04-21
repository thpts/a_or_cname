extern crate publicsuffix;

use publicsuffix::Domain;

/// The `publicsuffix` crate does not provide the means to separate a
/// sub-domain from a root domain, which we need when performing tasks against
/// the root of a domain.
///
/// ```text (FIXME: Should be doctest that can run)
/// use publicsuffix::{Domain, List};
/// use damp::dns::get_sub_domain;
///
/// let domain = List.from_domain("foo.example.co.uk")
/// assert_eq!(get_sub_domain(domain), Some("foo".to_string()));
/// ```
pub fn get_sub_domain(domain: &Domain) -> Option<String> {
    let root_domain = match domain.root() {
        Some(domain) => domain,
        None => return None,
    };

    let sub_len = if domain.to_string().len() > root_domain.len() {
        domain.to_string().len() - root_domain.len() - 1
    } else {
        0
    };
    return Some(domain.to_string().chars().take(sub_len).collect());
}

/// Get the root domain, sans the suffix.
/// Thus from `example.com` return `example`.
pub fn get_root_domain(domain: &Domain) -> Option<String> {
    let root_domain = match domain.root() {
        Some(domain) => domain,
        None => return None,
    };

    let suf_len = match domain.suffix() {
        Some(suf) => suf.len(),
        None => 0,
    };

    let root_len = root_domain.len() - suf_len - 1;
    return Some(root_domain.chars().take(root_len).collect());
}
