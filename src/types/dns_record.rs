use serde::{Deserialize, Serialize};

/// The type of a DNS resource record required for custom-domain
/// verification.
///
/// The endpoint that returns [`DnsRecord`] only ever produces these two
/// kinds: `txt` for ownership and SSL validation, `cname` for the
/// routing record pointing at `cname.squareweb.app`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DnsRecordType {
    /// Text record, used for ownership and SSL validation challenges.
    Txt,
    /// Canonical name record, used for routing traffic to SquareCloud.
    Cname,
}

/// A DNS record the caller must configure at their domain registrar.
///
/// Returned by
/// [`AppResource::dns_records`](crate::resources::AppResource::dns_records).
/// One entry among the (typically two or three) records required for
/// custom-domain verification to succeed.
#[derive(Debug, Serialize, Deserialize)]
pub struct DnsRecord {
    /// The record type.
    #[serde(rename = "type")]
    pub record_type: DnsRecordType,
    /// The record name to set at the DNS provider.
    pub name: String,
    /// The record value. For `cname` records, this is always
    /// `"cname.squareweb.app"`.
    pub value: String,
    /// The edge provider's validation state (e.g. `"pending"`,
    /// `"pending_validation"`, `"active"`).
    pub status: String,
}
