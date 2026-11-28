use super::super::{Prepared, module_failure, prepare};
use super::JavaRuntimeSections;
use crate::Failure;
use crate::behavior::HttpClientSourceExt;
use dyckr::http_client_source::HttpClientSource;
use std::path::Path;

#[derive(Clone)]
pub struct JavaPageRuntime {
    module_id: String,
    sections: JavaRuntimeSections,
}

impl JavaPageRuntime {
    pub fn new(first_service: &Path, sections: JavaRuntimeSections) -> Self {
        Self {
            module_id: dyckr::module_id::module_id(first_service),
            sections,
        }
    }

    pub fn configured(&self) -> impl Iterator<Item = &str> {
        self.sections.configured()
    }

    pub fn lowered(
        &self,
        path: &Path,
        source: &str,
        environment: &str,
    ) -> Result<Option<String>, Failure> {
        let rewritten = HttpClientSource::lowering_source(path, source, environment)?;
        let text = rewritten.as_deref().unwrap_or(source);
        let lowered = prepare(path, text, environment)?
            .map(|prepared| {
                self.prepared(prepared)
                    .map_err(|message| module_failure(path, text, message))
            })
            .transpose()?;
        Ok(lowered.or(rewritten))
    }

    pub(super) fn fill_runtime(&self, environment: &str) -> String {
        self.sections.fill_runtime(&self.module_id, environment)
    }

    fn prepared(&self, prepared: Prepared) -> Result<String, String> {
        match prepared {
            dyckr::prepared::Prepared::Erased(erased) => Ok(erased),
            dyckr::prepared::Prepared::Annotated(module) => super::lower(&module, Some(self)),
        }
    }
}
