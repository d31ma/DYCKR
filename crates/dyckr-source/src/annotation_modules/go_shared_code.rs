use super::staged_files::StagedFiles;
use crate::Failure;
use dyckr::failure::{diagnostic, source_span};
use std::path::{Path, PathBuf};

pub struct GoSharedCode;

impl GoSharedCode {
    const FOLDER: &str = "server/shared";
    const HELP: &str = "server/shared/ holds plain Go code of package main. Put an annotated type in its role folder, for example server/services/.";

    pub fn read(source_root: &Path) -> Result<Vec<(PathBuf, String)>, Failure> {
        let mut paths = StagedFiles::beneath(source_root, Path::new(Self::FOLDER));
        paths.retain(|path| Self::is_go(path));
        let mut files = Vec::new();
        for (index, relative) in paths.iter().enumerate() {
            let text = Self::staged_text(source_root, relative)?;
            files.push((PathBuf::from(format!("shared_{index}.go")), text));
        }
        Ok(files)
    }

    fn staged_text(source_root: &Path, relative: &Path) -> Result<String, Failure> {
        let portable = relative.to_string_lossy().replace('\\', "/");
        let bytes = std::fs::read(source_root.join(relative)).map_err(|error| {
            Self::failure(&portable, format!("Cannot read '{portable}': {error}"))
        })?;
        let Ok(source) = String::from_utf8(bytes) else {
            return Err(Self::failure(
                &portable,
                format!("'{portable}' must be UTF-8."),
            ));
        };
        let source = dyckr::without_bom::without_bom(&source);
        if crate::annotation_layout::has_framework_annotation(relative, source)? {
            return Err(Self::failure(
                &portable,
                format!("'{portable}' is plain shared code and cannot carry a Tachyon annotation."),
            ));
        }
        let code = dyckr::source_mask::masked_source(relative, source)?;
        let mut words = code.split_whitespace();
        let position = format!("//line {portable}:1\n");
        if words.next() != Some("package") {
            return Ok(format!("{position}package main; {source}"));
        }
        if words.next() != Some("main") {
            return Err(Self::failure(
                &portable,
                format!("'{portable}' is compiled as package main; found another package clause."),
            ));
        }
        Ok(format!("{position}{source}"))
    }

    fn is_go(source: &Path) -> bool {
        source
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("go"))
    }

    fn failure(portable: &str, message: String) -> Failure {
        Failure::one(diagnostic(
            1310,
            message,
            Some(String::from(Self::HELP)),
            source_span(portable, 0, 0),
        ))
    }
}
