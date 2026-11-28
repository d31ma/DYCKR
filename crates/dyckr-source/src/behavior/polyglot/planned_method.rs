use crate::behavior::ServiceRolesExt;
pub use dyckr::planned_method::PlannedMethod;
use serde_json::Value;

pub trait PlannedMethodExt {
    fn rust_mapping_types(
        path: &std::path::Path,
        source: &str,
    ) -> Result<Vec<String>, crate::Failure> {
        Self::mapping_types(path, source)
    }

    fn mapping_types(path: &std::path::Path, source: &str) -> Result<Vec<String>, crate::Failure> {
        let source = crate::behavior::ServiceRoles::without_roles(path, source)?;
        let prepared = super::prepare(path, &source, "tac")?;
        Ok(prepared
            .into_iter()
            .flat_map(Self::prepared_mapping_types)
            .collect())
    }

    fn prepared_mapping_types(prepared: dyckr::prepared::Prepared) -> Vec<String> {
        match prepared {
            dyckr::prepared::Prepared::Erased(_) => Vec::new(),
            dyckr::prepared::Prepared::Annotated(module) => module
                .types
                .into_iter()
                .flat_map(|kind| kind.methods)
                .flat_map(|method| Self::method_mapping_types(&method))
                .collect(),
        }
    }

    fn method_mapping_types(method: &dyckr::behavior_module::Method) -> Vec<String> {
        method
            .metadata
            .get("mapping")
            .and_then(|policy| policy.get("result"))
            .and_then(Value::as_str)
            .map(|result| {
                method
                    .parameters
                    .iter()
                    .map(|parameter| parameter.type_source.clone())
                    .chain(std::iter::once(result.to_owned()))
                    .filter(|kind| dyckr::rust_model_type::RustModelType::is_model(kind))
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl PlannedMethodExt for PlannedMethod {}
