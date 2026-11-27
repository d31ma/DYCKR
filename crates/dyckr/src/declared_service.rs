use std::path::Path;


pub fn declared_service(source_path: &str, source: &str) -> Option<String> {
    let path = source_path.replace('\\', "/");
    let (parent, file) = path.rsplit_once('/')?;
    let role_parent = parent == "client/services"
        || parent.starts_with("client/services/")
        || parent.ends_with("/client/services")
        || parent.contains("/client/services/");
    if !role_parent {
        return None;
    }
    let stem = match file.rsplit_once('.') {
        Some((stem, _)) => stem,
        None => file,
    };
    if ![stem.ends_with("Service"), file.ends_with("_service.rs")].contains(&true) {
        return None;
    }
    Some(
        crate::declaration_names::declaration_names_source(std::path::Path::new(&path), source)
            .ok()?
            .into_iter()
            .find(|name| crate::class_file_name::class_file_matches(std::path::Path::new(&path), name))
        .filter(|name| name.ends_with("Service"))
        .unwrap_or_else(|| stem.to_owned()),
    )
}
