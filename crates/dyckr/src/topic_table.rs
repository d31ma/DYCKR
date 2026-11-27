use crate::topics::Topics;
use crate::route_access::RouteAccess;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TopicTable {
    rules: BTreeMap<String, RouteAccess>,
}

impl TopicTable {
    pub fn insert(&mut self, name: &str, access: RouteAccess) {
        self.rules
            .extend(Topics::slug(name).map(|slug| (slug, access)));
    }

    pub fn access(&self, slug: &str) -> Option<&RouteAccess> {
        self.rules.get(slug)
    }
}
