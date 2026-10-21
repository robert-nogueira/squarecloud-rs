use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Options for a blob upload request.
#[derive(Debug, Clone, Default)]
pub struct UploadOptions {
    /// Optional prefix to group the object. Must match `[a-zA-Z0-9_]`, 3-32 characters.
    pub prefix: Option<String>,
    /// Number of days until the object expires (1-365).
    pub expire: Option<u16>,
    /// When `true`, the URL includes a signed hash that prevents direct access
    /// without the signature.
    pub security_hash: Option<bool>,
    /// When `true`, browsers receive `Content-Disposition: attachment` and
    /// download the file instead of previewing it.
    pub auto_download: Option<bool>,
}

/// A blob object returned immediately after a successful upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobObject {
    /// Unique object identifier, starting with `pub/` or `prv/`. Treat it
    /// as opaque: it changes when the object changes visibility or expiry.
    pub id: String,
    /// The name sent in the request.
    pub name: String,
    /// The prefix, when one was sent.
    #[serde(default)]
    pub prefix: Option<String>,
    /// Size of the uploaded object in bytes.
    pub size: u64,
    /// Whether the object is private.
    pub private: bool,
    /// Public URL of the object, or `None` for a private object.
    pub url: Option<String>,
    /// When the object will be deleted, if it expires.
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
    /// SHA-256 of the stored object, in hex.
    pub sha256: String,
    /// `true` when the upload replaced an object with the same name.
    pub replaced: bool,
}

/// A single entry in the blob object listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobListItem {
    /// Unique object identifier.
    pub id: String,
    /// Size in bytes.
    pub size: u64,
    /// Timestamp when the object was created.
    pub created_at: DateTime<Utc>,
    /// Timestamp when the object will expire, if applicable.
    pub expires_at: Option<DateTime<Utc>>,
    /// Whether the object is private.
    pub private: bool,
    /// Public URL of the object, or `None` for a private object.
    #[serde(default)]
    pub url: Option<String>,
    /// The object's ETag; it changes when the content changes.
    pub etag: String,
}

/// Paginated list of blob objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobObjectList {
    /// The objects returned for this page.
    pub objects: Vec<BlobListItem>,
    /// Opaque token to pass as `continuation_token` to retrieve the next
    /// page; `None` on the last page.
    #[serde(default, rename = "continuationToken")]
    pub continuation_token: Option<String>,
}

/// Current usage statistics for the blob plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobUsage {
    /// Number of objects stored.
    pub objects: u64,
    /// Total storage used in bytes.
    pub storage: u64,
}

/// Included storage limit from the subscribed plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobPlan {
    /// Bytes of storage included in the plan at no extra cost.
    pub included: u64,
}

/// Estimated billing breakdown for the current billing period.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobBilling {
    /// Extra storage in bytes beyond the included quota.
    pub extra_storage: u64,
    /// Cost of the extra storage (currency determined by the account).
    pub storage_price: f64,
    /// Cost based on the number of objects stored.
    pub objects_price: f64,
    /// Combined cost estimate for the billing period.
    pub total_estimate: f64,
}

/// Storage usage averaged over the current month.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobMonth {
    /// Days of the current month (UTC) that have a usage sample.
    pub days: u32,
    /// Average of those daily samples, in bytes.
    pub average_storage: f64,
}

/// Full statistics for the blob storage account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobStats {
    /// Current usage figures.
    pub usage: BlobUsage,
    /// Plan limits.
    pub plan: BlobPlan,
    /// Billing estimates for the current period.
    pub billing: BlobBilling,
    /// This month's average usage; `None` until the month has a sample.
    #[serde(default)]
    pub month: Option<BlobMonth>,
}
