use crate::blob_limits::BlobLimits;
use crate::failure::Failure;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StorageMember {
    #[serde(default)]
    blobs: Option<BlobLimits>,
}

impl StorageMember {
    pub fn blob_limits(self) -> Result<BlobLimits, Failure> {
        self.blobs.unwrap_or_default().checked()
    }
}
