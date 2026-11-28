    pub fn with_fsharp_attributes(mut text: String, rules: &[&'static str]) -> String {
        for name in rules {
            let csharp = format!("[{}]", name);
            text = text.replace(&csharp, &format!("[<{}>]", name));
        }
        text
    }

use std::path::{Path, PathBuf};
use crate::failure::Failure;
use crate::fsharp_shadow::FsharpShadow;
use tachyon_diagnostics::{Diagnostic, SourceSpan};

pub struct FsharpView {
    pub shadow_path: PathBuf,
    pub shadow_name: String,
    pub source_name: String,
    pub shadow: FsharpShadow,
}

impl FsharpView {
    pub const ANALYSIS_EXTENSION: &'static str = "cs";
    pub const WORDS: [(&'static str, &'static str); 5] = [
        (".cs'", ".fs'"),
        ("`switch`", "`match`"),
        ("`foreach`", "`for`"),
        ("in C# Tac code", "in F# Tac code"),
        ("in C# Yon code", "in F# Yon code"),
    ];
    pub const USINGS: &'static str = "using System;\nusing System.Collections.Generic;\nusing System.Net.Http;\nusing Microsoft.FSharp.Core;\nusing Microsoft.FSharp.Collections;\n";

    pub fn of(path: &Path, source: &str) -> Option<Self> {
        crate::fsharp_syntax::is_fsharp(path).then(|| FsharpView::new(path, source))
    }

    pub fn new(path: &Path, source: &str) -> Self {
        let shadow_path = path.with_extension(FsharpView::ANALYSIS_EXTENSION);
        Self {
            shadow_name: FsharpView::portable(&shadow_path),
            source_name: FsharpView::portable(path),
            shadow_path,
            shadow: crate::fsharp_shadow::FsharpShadow::of(source),
        }
    }

    pub fn tac(path: &Path, source: &str) -> Self {
        let view = FsharpView::new(path, source);
        Self {
            shadow: view.shadow.without_constructor_needs(),
            ..view
        }
    }

    pub fn unmoved(path: &Path, source: &str) -> Self {
        Self {
            shadow: crate::fsharp_shadow::FsharpShadow::unmoved(source),
            ..FsharpView::new(path, source)
        }
    }

    pub fn either<T>(
        path: &Path,
        source: &str,
        check: impl Fn(&Path, &str) -> Result<T, Failure>,
    ) -> Result<T, Failure> {
        FsharpView::run(path, source, &check).unwrap_or_else(|| check(path, source))
    }

    pub fn run<T>(
        path: &Path,
        source: &str,
        check: impl FnOnce(&Path, &str) -> Result<T, Failure>,
    ) -> Option<Result<T, Failure>> {
        let view = FsharpView::of(path, source)?;
        Some(check(view.path(), view.text()).map_err(|failure| view.failure(&failure)))
    }

    pub fn portable(path: &Path) -> String {
        path.to_string_lossy().replace('\\', "/")
    }

    pub fn path(&self) -> &Path {
        &self.shadow_path
    }

    pub fn text(&self) -> &str {
        self.shadow.text()
    }

    pub fn source_line(&self, shadow_line: usize) -> Option<usize> {
        self.shadow.source_line(shadow_line)
    }

    pub fn usings(&self) -> String {
        self.using_lines()
    }

    pub fn authored_usings(&self) -> String {
        FsharpView::using_lines_in(self.text())
    }

    pub fn using_lines_in(text: &str) -> String {
        let mut usings = String::new();
        for line in text.lines() {
            FsharpView::add_using(&mut usings, line.trim());
        }
        usings
    }

    pub fn using_lines(&self) -> String {
        format!("{}{}", FsharpView::USINGS, self.authored_usings())
    }

    pub fn add_using(usings: &mut String, line: &str) {
        if line.starts_with("using ") {
            usings.push_str(line);
            usings.push('\n');
        }
    }

    pub fn failure(&self, failure: &Failure) -> Failure {
        let diagnostics = failure
            .diagnostics()
            .iter()
            .map(|diagnostic| self.diagnostic(diagnostic))
            .collect();
        Failure::new(diagnostics)
    }

    pub fn diagnostic(&self, diagnostic: &Diagnostic) -> Diagnostic {
        Diagnostic {
            message: self.renamed(&diagnostic.message),
            help: diagnostic.help.as_deref().map(|help| self.renamed(help)),
            spans: diagnostic
                .spans
                .iter()
                .map(|span| self.span(span))
                .collect(),
            ..diagnostic.clone()
        }
    }

    pub fn renamed(&self, text: &str) -> String {
        let mut renamed = text.replace(&self.shadow_name, &self.source_name);
        for (csharp, fsharp) in FsharpView::WORDS {
            renamed = renamed.replace(csharp, fsharp);
        }
        FsharpView::with_fsharp_attributes(renamed)
    }

    pub fn with_fsharp_attributes(text: String) -> String {
        let rules: Vec<&'static str> = crate::annotation_rule::AnnotationRules::rules().map(|rule| rule.name).collect();
        crate::fsharp_view::with_fsharp_attributes(text, &rules)
    }

    pub fn span(&self, span: &SourceSpan) -> SourceSpan {
        let ours = span.file == self.shadow_name;
        let mapped = ours.then(|| self.mapped_span(span));
        mapped.unwrap_or_else(|| span.clone())
    }

    pub fn mapped_span(&self, span: &SourceSpan) -> SourceSpan {
        let start = self.source_offset(span.start);
        let end = self.source_offset(span.end).max(start);
        SourceSpan {
            file: self.source_name.clone(),
            start,
            end,
        }
    }

    pub fn source_at(&self, shadow_offset: usize) -> usize {
        self.shadow
            .source_offset(shadow_offset.min(self.text().len()))
    }

    pub fn source_offset(&self, offset: u64) -> u64 {
        let offset = usize::try_from(offset).unwrap_or(usize::MAX);
        u64::try_from(self.source_at(offset)).unwrap_or(u64::MAX)
    }
}

