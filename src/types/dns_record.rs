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
    pub record_type: DnsRecordType,
    pub name: String,
    pub value: String,
    pub status: String,
}
