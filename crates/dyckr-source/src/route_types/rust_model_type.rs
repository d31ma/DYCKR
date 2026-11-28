pub use dyckr::rust_model_type::RustModelType;

pub trait RustModelTypeExt {
    fn owner_types(
        source: &str,
        path: &std::path::Path,
        excluded: &[String],
    ) -> Result<Vec<String>, crate::Failure> {
        Self::selected_types(source, &path.to_string_lossy(), excluded)
    }

    fn selected_types(
        source: &str,
        path: &str,
        excluded: &[String],
    ) -> Result<Vec<String>, crate::Failure> {
        let wanted = dyckr::declared_service::declared_service(path, source);
        let (name, body) =
            dyckr::companion_syntax::block_named(source, "struct", wanted.as_deref()).ok_or_else(
                || dyckr::companion_failures::unnamed_service(path, wanted.as_deref(), "struct"),
            )?;
        let collaborators =
            dyckr::rust_collaborators::RustCollaborators::declared_beside(&name, source);
        let fields = dyckr::companion_member::rust_struct_fields(body)
            .into_iter()
            .filter(|member| !collaborators.holds(member.declared_type.as_deref()))
            .filter(|member| !excluded.contains(&member.name))
            .filter_map(|member| member.declared_type);
        let methods = dyckr::companion_syntax::blocks(source, "impl")
            .into_iter()
            .filter(|(header, _)| header.trim() == name)
            .flat_map(|(_, body)| dyckr::companion_member::rust_methods(body))
            .filter(|(member, _)| !excluded.contains(&member.name))
            .flat_map(|(member, parameters)| parameters.into_iter().chain(member.declared_type));
        Ok(fields
            .chain(methods)
            .filter(|kind| RustModelType::is_model(kind))
            .collect())
    }
}

impl RustModelTypeExt for RustModelType {}
