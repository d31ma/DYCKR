use crate::Failure;
use crate::behavior::HttpClientSourceExt;
use crate::behavior::ServiceRolesExt;
use crate::route_types::ModelTable;
use std::path::Path;

pub struct CsharpTaskModels;

impl CsharpTaskModels {
    pub fn lower(
        path: &Path,
        source: &str,
        models: &ModelTable,
    ) -> Result<Option<String>, Failure> {
        Self::prepared(path, source, models)
    }

    pub fn codec() -> &'static str {
        include_str!("models/TacModels.cs")
    }

    fn prepared(path: &Path, source: &str, models: &ModelTable) -> Result<Option<String>, Failure> {
        let rewritten = dyckr::http_client_source::HttpClientSource::rewrite(path, source)?;
        let written = rewritten.as_deref().unwrap_or(source);
        let roles = crate::behavior::ServiceRoles::read(path, written)?.source;
        let lowered = super::prepare(path, &roles, "tac")?
            .map(|prepared| {
                dyckr::csharp_task_models::lower_prepared(prepared, models, &|m| {
                    super::csharp::lower(m)
                })
                .map_err(|message| dyckr::module_failure::module_failure(path, source, message))
            })
            .transpose()?;
        lowered.map_or_else(
            || Self::unheld_streams(path, source, rewritten.is_some()),
            |text| Ok(Some(text)),
        )
    }

    fn unheld_streams(path: &Path, source: &str, clients: bool) -> Result<Option<String>, Failure> {
        if !clients {
            return Ok(None);
        }
        dyckr::http_client_source::HttpClientSource::without_clients(path, source)
    }
}
