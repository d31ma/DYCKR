use crate::failure::Failure;
use crate::lua_shadow::LuaShadow;
use std::path::{Path, PathBuf};
use tachyon_diagnostics::{Diagnostic, SourceSpan};

pub struct LuaView {
    pub shadow_path: PathBuf,
    pub shadow_name: String,
    pub source_name: String,
    pub shadow: LuaShadow,
    pub line_starts: Vec<usize>,
    pub source_length: usize,
}
impl LuaView {
    pub const ANALYSIS_EXTENSION: &'static str = "rb";
    pub const YON_REALM: &'static str = "server";
    pub const SHADOW_FILE: (&'static str, &'static str) = (".rb'", ".lua'");
    pub const WORDS: [(&'static str, &'static str); 11] = [
        ("`#[", "`-- @"),
        ("]`", "`"),
        ("# @param", "---@param"),
        ("# @return", "---@return"),
        ("attr_reader :", "field "),
        ("attr_reader", "a field"),
        ("`def ", "`function Name:"),
        ("YARD", "LuaCATS"),
        ("Ruby annotations", "Lua annotations"),
        ("Ruby class", "Lua class table"),
        ("a Ruby ", "a Lua "),
    ];

    pub fn of(path: &Path, source: &str) -> Option<Self> {
        crate::lua_syntax::is_lua(path).then(|| LuaView::joined(path, source))
    }

    pub fn joined(path: &Path, source: &str) -> Self {
        let shadows: [fn(&str) -> crate::lua_shadow::LuaShadow; 2] = [
            crate::lua_shadow::LuaShadow::joined,
            crate::lua_shadow::LuaShadow::yon,
        ];
        let shadow = shadows[usize::from(path.starts_with(LuaView::YON_REALM))](source);
        LuaView::with_shadow(path, source, shadow)
    }

    pub fn yon(path: &Path, source: &str) -> Self {
        LuaView::with_shadow(path, source, crate::lua_shadow::LuaShadow::yon(source))
    }

    pub fn with_shadow(path: &Path, source: &str, shadow: crate::lua_shadow::LuaShadow) -> Self {
        let shadow_path = path.with_extension(LuaView::ANALYSIS_EXTENSION);
        Self {
            shadow_name: LuaView::portable(&shadow_path),
            source_name: LuaView::portable(path),
            shadow_path,
            shadow,
            line_starts: LuaView::line_starts(source),
            source_length: source.len(),
        }
    }

    pub fn analysis(path: &Path, source: &str) -> (PathBuf, String) {
        LuaView::of(path, source).map_or_else(
            || (path.to_path_buf(), source.to_owned()),
            |view| (view.shadow_path.clone(), view.text().to_owned()),
        )
    }

    pub fn either<T>(
        path: &Path,
        source: &str,
        check: impl Fn(&Path, &str) -> Result<T, Failure>,
    ) -> Result<T, Failure> {
        LuaView::of(path, source).map_or_else(
            || crate::fsharp_view::FsharpView::either(path, source, &check),
            |view| view.checked(&check),
        )
    }

    pub fn either_shadow<T>(
        path: &Path,
        source: &str,
        check: impl Fn(&Path, &str) -> Result<T, Failure>,
        shadow_check: impl Fn(&Path, &str) -> Result<T, Failure>,
    ) -> Result<T, Failure> {
        LuaView::of(path, source).map_or_else(
            || crate::fsharp_view::FsharpView::either(path, source, &check),
            |view| view.checked(&shadow_check),
        )
    }

    pub fn path(&self) -> &Path {
        &self.shadow_path
    }

    pub fn text(&self) -> &str {
        self.shadow.text()
    }

    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    pub fn line_start(&self, source_line: usize) -> usize {
        self.line_starts
            .get(source_line)
            .copied()
            .unwrap_or_default()
    }

    pub fn failure(&self, failure: &Failure) -> Failure {
        Failure::new(self.diagnostics(failure.diagnostics()))
    }

    pub fn checked<T>(
        &self,
        check: &impl Fn(&Path, &str) -> Result<T, Failure>,
    ) -> Result<T, Failure> {
        check(self.path(), self.text()).map_err(|failure| self.failure(&failure))
    }

    pub fn line_starts(source: &str) -> Vec<usize> {
        std::iter::once(0)
            .chain(source.match_indices('\n').map(|(at, _)| at + 1))
            .collect()
    }

    pub fn portable(path: &Path) -> String {
        path.to_string_lossy().replace('\\', "/")
    }

    pub fn diagnostics(&self, diagnostics: &[Diagnostic]) -> Vec<Diagnostic> {
        diagnostics
            .iter()
            .map(|diagnostic| self.in_lua(diagnostic))
            .collect()
    }

    pub fn in_lua(&self, diagnostic: &Diagnostic) -> Diagnostic {
        let mut mapped = diagnostic.clone();
        mapped.message = self.renamed(&diagnostic.message);
        mapped.help = self.renamed_help(diagnostic.help.as_deref());
        mapped.spans = self.spans(&diagnostic.spans);
        mapped
    }

    pub fn renamed_help(&self, help: Option<&str>) -> Option<String> {
        help.map(|help| self.renamed(help))
    }

    pub fn spans(&self, spans: &[SourceSpan]) -> Vec<SourceSpan> {
        spans.iter().map(|span| self.span(span)).collect()
    }

    pub fn renamed(&self, text: &str) -> String {
        let (shadow_file, source_file) = LuaView::SHADOW_FILE;
        LuaView::WORDS.iter().fold(
            text.replace(&self.shadow_name, &self.source_name)
                .replace(shadow_file, source_file),
            |renamed, (ruby, lua)| renamed.replace(ruby, lua),
        )
    }

    pub fn span(&self, span: &SourceSpan) -> SourceSpan {
        if span.file != self.shadow_name {
            return span.clone();
        }
        let start = self.mapped(span.start);
        let last = span.end.saturating_sub(1).max(span.start);
        SourceSpan {
            file: self.source_name.clone(),
            start,
            end: self.mapped_line_end(last).max(start),
        }
    }

    pub fn mapped(&self, offset: u64) -> u64 {
        let offset = usize::try_from(offset).unwrap_or(usize::MAX);
        u64::try_from(self.source_offset(offset)).unwrap_or(u64::MAX)
    }

    pub fn mapped_line_end(&self, offset: u64) -> u64 {
        let offset = usize::try_from(offset).unwrap_or(usize::MAX);
        let next_line = self.source_line(offset) + 1;
        let end = self
            .line_starts
            .get(next_line)
            .map_or(self.source_length, |next| next - 1);
        u64::try_from(end).unwrap_or(u64::MAX)
    }

    pub fn source_offset(&self, shadow_offset: usize) -> usize {
        self.line_start(self.source_line(shadow_offset))
    }

    pub fn source_line(&self, shadow_offset: usize) -> usize {
        let shadow_offset = shadow_offset.min(self.text().len());
        let shadow_line = self.text()[..shadow_offset].matches('\n').count();
        self.shadow.source_line(shadow_line).unwrap_or_default()
    }
}
