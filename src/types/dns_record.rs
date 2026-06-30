use serde::{Deserialize, Serialize};

/// The type of a DNS resource record.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DnsRecordType {
    /// IPv4 address record.
    A,
    /// IPv6 address record.
    AAAA,
    /// Canonical name (alias) record.
    CNAME,
    /// Mail exchange record.
    MX,
    /// Text record (used for SPF, DKIM, DMARC, etc.).
    TXT,
    /// Name server record.
    NS,
    /// Start of authority record.
    SOA,
    /// Pointer record (reverse DNS).
    PTR,
    /// Service record.
    SRV,
    /// DNSSEC delegation signer record.
    DS,
    /// DNSSEC key record.
    DNSKEY,
    /// DNSSEC signature record.
    RRSIG,
    /// Certification authority authorization record.
    CAA,
    /// Service binding record.
    SVCB,
    /// HTTPS service binding record.
    HTTPS,
    /// Wildcard query matching any record type.
    ANY,
    /// DNSSEC option record.
    OPT,
}

/// A DNS record associated with an application's domain.
///
/// Returned by
/// [`AppResource::dns_record`](crate::resources::AppResource::dns_record).
/// Describes what DNS entry the application's domain must resolve to for
/// custom-domain verification to succeed.
#[derive(Debug, Serialize, Deserialize)]
pub struct DnsRecord {
    /// The record type.
    #[serde(rename = "type")]
    pub record_type: DnsRecordType,
    /// The record name (typically the domain or subdomain).
    pub name: String,
    /// The record value (e.g. an IP address or CNAME target).
    pub value: String,
    /// Verification status as reported by the platform.
    pub status: String,
}
