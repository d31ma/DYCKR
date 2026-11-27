use crate::vendor_provenance::VendorProvenance;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum StyleSource {
    Foundation,
    Vendor(VendorProvenance),
}
