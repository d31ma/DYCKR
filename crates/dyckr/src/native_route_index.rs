use crate::failure::{Failure, diagnostic};
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NativeRouteIndexEntry {
    pub route: String,
    pub document: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NativeRouteIndex {
    pub contract_version: u8,
    pub entry_route: String,
    pub entry_document: String,
    pub routes: Vec<NativeRouteIndexEntry>,
}

impl NativeRouteIndex {
    pub fn entry_document(
        entries: &[NativeRouteIndexEntry],
        entry_route: &str,
    ) -> Result<String, Failure> {
        for entry in entries {
            if entry.route == entry_route {
                return Ok(entry.document.clone());
            }
        }
        Err(Failure::one(diagnostic(
            1601,
            format!("Native entry route '{entry_route}' has no document to open."),
            Some(String::from(
                "Point start_url in client/manifest.json at a route with a \
                 tac.html, so the application has something to load.",
            )),
            None,
        )))
    }
}

pub fn swift_routes(index: &NativeRouteIndex) -> String {
    let mut pairs = Vec::new();
    for entry in &index.routes {
        let route = crate::quoted_string_escape::quoted_string_escape(&entry.route);
        let document = crate::quoted_string_escape::quoted_string_escape(&entry.document);
        pairs.push(format!("\"{route}\": \"{document}\""));
    }
    format!("[{}]", pairs.join(", "))
}

pub fn route_documents_json(index: &NativeRouteIndex) -> String {
    let mut documents = std::collections::BTreeMap::new();
    for entry in &index.routes {
        documents.insert(&entry.route, &entry.document);
    }
    serde_json::to_string(&documents).unwrap_or_else(|_| String::from("{}"))
}
