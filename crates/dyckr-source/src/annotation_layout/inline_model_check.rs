use crate::Failure;
use dyckr::failure::{diagnostic, source_span};
use dyckr::model_blocks::ModelBlocks;
use dyckr::model_blocks::model_block::ModelBlock;
use std::ffi::OsStr;
use std::path::{Component, Path};

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests;

pub struct InlineModelCheck;

impl InlineModelCheck {
    pub const ENFORCED: bool = true;
    const CODE: u16 = 1310;
    const REALMS: [&'static str; 2] = ["client", "server"];
    const MODELS: &'static str = "models";
    const LANGUAGE: &'static str = "js";
    const RULE: &'static str = "A JavaScript model is one @typedef in its own file in the models folder, like a model in every other language (ADR 0067).";

    pub fn check(path: &Path, source: &str) -> Result<(), Failure> {
        Self::refuse_when_enforced(path, source)
    }

    pub fn refuse_inline_models(path: &Path, source: &str) -> Result<(), Failure> {
        let blocks = Self::inline_blocks(path, source)?;
        Self::require_none(path, &blocks)
    }

    pub fn applies(path: &Path) -> bool {
        let checks = [
            Self::extension(path) == Self::LANGUAGE,
            Self::REALMS.contains(&Self::part(path, 0)),
            Self::part(path, 1) != Self::MODELS,
        ];
        !checks.contains(&false)
    }

    pub fn realm(path: &Path) -> &str {
        Self::part(path, 0)
    }

    fn refuse_when_enforced(path: &Path, source: &str) -> Result<(), Failure> {
        if !Self::ENFORCED {
            return Ok(());
        }
        Self::refuse_inline_models(path, source)
    }

    fn inline_blocks(path: &Path, source: &str) -> Result<Vec<ModelBlock>, Failure> {
        if !Self::applies(path) {
            return Ok(Vec::new());
        }
        ModelBlocks::read(path, source)
    }

    fn require_none(path: &Path, blocks: &[ModelBlock]) -> Result<(), Failure> {
        let Some(first) = blocks.first() else {
            return Ok(());
        };
        Err(Self::failure(path, first))
    }

    fn failure(path: &Path, block: &ModelBlock) -> Failure {
        let file = path.to_string_lossy().replace('\\', "/");
        let realm = Self::realm(path);
        let name = &block.name;
        let message = format!(
            "'{file}' declares the model '{name}' with a @typedef; a JavaScript model lives in its own file in '{realm}/models/'"
        );
        let help = format!(
            "Move the @typedef of {name} into '{realm}/models/{name}.js', followed by `export {{}}`. Every JavaScript file of the realm then uses {name} without an import. {} `ty migrate fix --write` moves it for you, and uses a model that already exists in any language instead of a copy.",
            Self::RULE
        );
        Failure::one(diagnostic(
            Self::CODE,
            message,
            Some(help),
            source_span(&file, block.start, block.start + 1),
        ))
    }

    fn part(path: &Path, index: usize) -> &str {
        let part = path.components().nth(index);
        let text = part.and_then(|component| Self::normal(component));
        text.unwrap_or_default()
    }

    fn normal(component: Component<'_>) -> Option<&str> {
        let Component::Normal(text) = component else {
            return None;
        };
        text.to_str()
    }

    fn extension(path: &Path) -> &str {
        path.extension().and_then(OsStr::to_str).unwrap_or_default()
    }
}
