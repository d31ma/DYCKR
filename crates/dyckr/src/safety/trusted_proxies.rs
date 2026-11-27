use crate::safety::AddressRanges;
use std::net::IpAddr;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TrustedProxies {
    ranges: AddressRanges,
}

impl TrustedProxies {
    pub fn parse(entries: &[&str]) -> Option<Self> {
        let ranges = AddressRanges::parse(entries)?;
        Some(Self { ranges })
    }

    pub fn contains(&self, address: IpAddr) -> bool {
        self.ranges.contains(address)
    }
}
