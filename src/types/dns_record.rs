use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DnsRecordType {
    A,
    AAAA,
    CNAME,
    MX,
    TXT,
    NS,
    SOA,
    PTR,
    SRV,
    DS,
    DNSKEY,
    RRSIG,
    CAA,
    SVCB,
    HTTPS,
    ANY,
    OPT,
}

#[derive(Serialize, Deserialize)]
pub struct DnsRecord {
    #[serde(rename = "type")]
    record_type: DnsRecordType,
    name: String,
    value: String,
    status: String,
}
