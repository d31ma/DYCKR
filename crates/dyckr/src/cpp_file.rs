use crate::cpp_class::CppClass;
use crate::cpp_failure::CppFailure;
use crate::cpp_file_reader::CppFileReader;
use crate::cpp_source::CppSource;
use crate::cpp_tokens::CppTokens;
use crate::failure::Failure;
use std::ops::Range;
use std::path::Path;

pub struct CppFile {
    pub class: CppClass,
    pub directives: Vec<Range<usize>>,
}

impl CppFile {
    pub fn read(path: &Path, source: &str) -> Result<Self, Failure> {
        let read =
            CppSource::read(source).map_err(|message| CppFailure::unreadable(path, 0, message))?;
        let tokens = CppTokens::new(read.tokens);
        let class = CppFileReader::only_class(path, source, &tokens)?;
        Ok(Self {
            class,
            directives: read.directives,
        })
    }
}
