use crate::Failure;
use crate::model_sources::ModelSources;
use crate::yon_cpp::CppYon;
use dyckr::failure::{diagnostic, source_span};
use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::io::Write as _;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

pub struct DeclaredHandler {
    project_root: PathBuf,
    relative_path: String,
    absolute_path: PathBuf,
    relative_source: PathBuf,
    streaming_methods: BTreeSet<String>,
    routes: Arc<[dyckr::plan::Route]>,
    models: Arc<crate::route_types::ModelTable>,
    language: HandlerLanguage,
    source_bytes: Arc<[u8]>,
    source_root: Arc<OwnedSourceRoot>,
    topics: Arc<crate::events::TopicUse>,
    program: String,
    lowered: Option<String>,
    annotation_entry: bool,
    server_models: Arc<ModelSources>,
}

impl fmt::Debug for DeclaredHandler {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeclaredHandler")
            .field("project_root", &self.project_root)
            .field("relative_path", &self.relative_path)
            .field("absolute_path", &self.absolute_path)
            .field("streaming_methods", &self.streaming_methods)
            .field("routes", &self.routes)
            .field("models", &self.models)
            .field("language", &self.language)
            .field("source_bytes", &"<owned>")
            .field("source_root", &"<owned>")
            .field("topics", &self.topics)
            .finish_non_exhaustive()
    }
}

impl PartialEq for DeclaredHandler {
    fn eq(&self, other: &Self) -> bool {
        self.project_root == other.project_root
            && self.relative_path == other.relative_path
            && self.absolute_path == other.absolute_path
            && self.language == other.language
            && self.source_bytes == other.source_bytes
    }
}

impl Eq for DeclaredHandler {}

impl DeclaredHandler {
    pub const MAX_SOURCE_BYTES: u64 = 1024 * 1024;

    pub fn source_root(&self) -> &Arc<OwnedSourceRoot> {
        &self.source_root
    }

    pub fn relative_source(&self) -> &Path {
        &self.relative_source
    }

    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    pub fn program(&self) -> &str {
        &self.program
    }

    pub fn lowered(&self) -> Option<&str> {
        self.lowered.as_deref()
    }

    pub fn source_bytes(&self) -> &[u8] {
        &self.source_bytes
    }

    pub const fn annotation_entry(&self) -> bool {
        self.annotation_entry
    }

    pub fn server_models(&self) -> &ModelSources {
        &self.server_models
    }

    pub fn streams(&self, name: &str) -> bool {
        self.streaming_methods.contains(name)
    }

    pub fn topics(&self) -> &crate::events::TopicUse {
        &self.topics
    }

    pub fn gateway_unit(
        project_root: &Path,
        unit: &Path,
        assembled: String,
        source_root: &Arc<OwnedSourceRoot>,
        models: ModelSources,
        language: HandlerLanguage,
    ) -> Self {
        Self {
            project_root: project_root.to_path_buf(),
            relative_path: portable(unit),
            absolute_path: project_root.join(unit),
            relative_source: unit.to_path_buf(),
            streaming_methods: BTreeSet::new(),
            routes: Arc::from(Vec::new()),
            models: Arc::default(),
            language,
            source_bytes: Arc::from(assembled.into_bytes()),
            source_root: Arc::clone(source_root),
            topics: Arc::default(),
            program: String::new(),
            lowered: None,
            annotation_entry: false,
            server_models: Arc::new(models),
        }
    }

    #[must_use]
    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }

    #[must_use]
    pub fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }

    #[must_use]
    pub const fn language(&self) -> HandlerLanguage {
        self.language
    }

    pub fn routes(&self) -> &[dyckr::plan::Route] {
        &self.routes
    }

    pub fn models(&self) -> &crate::route_types::ModelTable {
        &self.models
    }

    pub fn declare(
        project_root: PathBuf,
        relative_source: &Path,
        bytes: Vec<u8>,
        source_root: Arc<OwnedSourceRoot>,
    ) -> Result<Self, Failure> {
        let relative_source = validate_relative_source(relative_source)?;
        let portable = portable(relative_source);
        let contents = validated_source_text(&bytes, &portable)?;
        let middleware = is_middleware(relative_source);
        if middleware {
            crate::annotation_layout::validate(relative_source, contents)?;
        }
        let go = relative_source
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("go"));
        let annotation_entry = !middleware && crate::convention::is_yon_language(relative_source);
        let assembled =
            if annotation_entry && !dyckr::cpp_source::CppSource::is_cpp(relative_source) {
                crate::annotation_modules::assemble(
                    source_root.path(),
                    relative_source,
                    contents,
                    "yon",
                )?
            } else {
                None
            };
        let receives_models = annotation_entry || middleware || go;
        let models = server_models(source_root.path(), relative_source, receives_models)?;
        let metadata = models.metadata();
        let has_modules = assembled.is_some();
        let bytes = assembled.map_or(bytes, String::into_bytes);
        let contents = std::str::from_utf8(&bytes)
            .map_err(|_| invalid_source(&portable, "Handler source must be valid UTF-8."))?;
        let program = if annotation_entry {
            metadata.assembled(relative_source, contents)?
        } else {
            contents.to_owned()
        };
        let (lowered, mut routes, topics) = if annotation_entry {
            lower_annotation_entry(
                source_root.path(),
                relative_source,
                &program,
                &portable,
                has_modules,
            )?
        } else {
            CppYon::middleware(middleware, relative_source, &program, source_root.path())?
        };
        let route_types = route_models(source_root.path(), relative_source, &program, &mut routes)?;
        let lowered = with_route_converters(lowered, source_root.path(), relative_source, &routes)?;
        let lowered = lowered
            .map(|source| metadata.without_declarations(relative_source, &source))
            .transpose()?;
        let codecs = !routes.is_empty()
            || lowered
                .as_deref()
                .is_some_and(|lowered| lowered.contains("__YonEvents"));
        let lowered = with_models(lowered, &program, &models, codecs);
        let streaming_methods = crate::convention::streaming_methods(relative_source, &program);
        supported_extension(relative_source, &portable)?;
        Ok(Self {
            absolute_path: project_root.join(relative_source),
            project_root,
            relative_path: portable,
            relative_source: relative_source.to_path_buf(),
            streaming_methods,
            routes: Arc::from(routes),
            models: Arc::new(route_types),
            language: declared_language(relative_source),
            source_bytes: Arc::from(bytes),
            source_root,
            topics: Arc::new(topics),
            program,
            lowered,
            annotation_entry,
            server_models: Arc::new(models),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandlerLanguage {
    JavaScript,
    Python,
    Ruby,
    TypeScript,
    Direct,
}

impl HandlerLanguage {
    #[must_use]
    pub const fn adapter(self) -> &'static str {
        match self {
            Self::JavaScript => "javascript.v1",
            Self::Python => "python.v1",
            Self::Ruby => "ruby.v1",
            Self::TypeScript | Self::Direct => "direct.v1",
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::JavaScript => "javascript",
            Self::Python => "python",
            Self::Ruby => "ruby",
            Self::TypeScript => "typescript",
            Self::Direct => "direct",
        }
    }
}

pub struct RuntimeWorkspace {
    directory: tempfile::TempDir,
}

impl RuntimeWorkspace {
    pub fn new(portable: &str) -> Result<Arc<Self>, Failure> {
        tempfile::Builder::new()
            .prefix("tachyon-handler-runtime-")
            .tempdir()
            .map(|directory| Arc::new(Self { directory }))
            .map_err(|error| {
                invalid_source(
                    portable,
                    &format!("Cannot create runtime workspace: {error}"),
                )
            })
    }

    pub fn path(&self) -> &Path {
        self.directory.path()
    }
}

impl fmt::Debug for RuntimeWorkspace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RuntimeWorkspace(<owned>)")
    }
}

pub struct OwnedSourceRoot {
    workspace: Arc<RuntimeWorkspace>,
}

impl OwnedSourceRoot {
    pub fn new_project(portable: &str) -> Result<Arc<Self>, Failure> {
        RuntimeWorkspace::new(portable).map(|workspace| Arc::new(Self { workspace }))
    }

    pub fn stage(&self, relative: &Path, bytes: &[u8], portable: &str) -> Result<(), Failure> {
        let destination = self.workspace.path().join(relative);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                invalid_source(portable, &format!("Cannot create source snapshot: {error}"))
            })?;
        }
        write_compiler_file(&destination, bytes, portable)
    }

    pub fn path(&self) -> &Path {
        self.workspace.path()
    }
}

impl fmt::Debug for OwnedSourceRoot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("OwnedSourceRoot(<owned>)")
    }
}

pub type LoweredEntry = (
    Option<String>,
    Vec<dyckr::plan::Route>,
    crate::events::TopicUse,
);

pub fn lower_annotation_entry(
    models_root: &Path,
    relative_source: &Path,
    contents: &str,
    portable: &str,
    has_modules: bool,
) -> Result<LoweredEntry, Failure> {
    reject_shebang(contents, portable)?;
    let worker = crate::yon::WorkerRoot::renamed(relative_source, contents)?;
    let contents = worker.as_deref().unwrap_or(contents);
    crate::convention::check_handler(relative_source, contents)?;
    lower_annotated_source(models_root, relative_source, contents, has_modules)
}

pub fn lower_annotated_source(
    models_root: &Path,
    relative_source: &Path,
    contents: &str,
    has_modules: bool,
) -> Result<LoweredEntry, Failure> {
    let lowered = if dyckr::fsharp_syntax::is_fsharp(relative_source) {
        crate::fsharp_source::FsharpYon::lower(relative_source, contents, Some(models_root))?
    } else if dyckr::lua_syntax::is_lua(relative_source) {
        crate::lua_source::LuaYon::lower(relative_source, contents, Some(models_root))?
    } else if dyckr::cpp_source::CppSource::is_cpp(relative_source) {
        Some(crate::yon_cpp::CppYon::lower(
            relative_source,
            contents,
            models_root,
        )?)
    } else {
        crate::yon::lower_with_routes(relative_source, contents, Some(models_root))?
    };
    if let Some(lowered) = lowered {
        return Ok((Some(lowered.source), lowered.routes, lowered.topics));
    }
    if has_modules {
        return Ok((
            Some(contents.to_owned()),
            Vec::new(),
            crate::events::TopicUse::default(),
        ));
    }
    Ok((None, Vec::new(), crate::events::TopicUse::default()))
}

pub fn reject_shebang(contents: &str, portable: &str) -> Result<(), Failure> {
    if !contents.starts_with("#!") {
        return Ok(());
    }
    let first_line_length = contents.lines().next().map_or(0, str::len);
    Err(Failure::one(diagnostic(
        2003,
        format!("Handler source '{portable}' begins with a shebang."),
        Some(String::from(
            "Yon owns the runtime for every supported language. Delete the first line; \
             executable bits and self-selected interpreters are not handler APIs.",
        )),
        source_span(portable, 0, first_line_length),
    )))
}

pub fn server_models(
    source_root: &Path,
    relative_source: &Path,
    receives_models: bool,
) -> Result<ModelSources, Failure> {
    if !receives_models {
        return Ok(ModelSources::none());
    }
    let extension = lowercase_extension(relative_source);
    ModelSources::read(source_root, "server", &extension)
}

pub fn lowercase_extension(source: &Path) -> String {
    source
        .extension()
        .map(|value| value.to_string_lossy().to_lowercase())
        .unwrap_or_default()
}

pub fn route_models(
    root: &Path,
    relative_source: &Path,
    contents: &str,
    routes: &mut [dyckr::plan::Route],
) -> Result<crate::route_types::ModelTable, Failure> {
    if routes.is_empty() {
        return Ok(crate::route_types::ModelTable::default());
    }
    let extension = lowercase_extension(relative_source);
    crate::route_types::RouteTypes::resolve(&extension, root, contents, routes).map_err(|message| {
        let portable = portable(relative_source);
        Failure::one(diagnostic(
            2015,
            format!("Cannot read the route types in '{portable}': {message}"),
            Some(String::from(
                "Route parameters and return values use text, numbers, true or false, lists, or models from server/models/ (ADR 0048).",
            )),
            source_span(&portable, 0, contents.len()),
        ))
    })
}

pub fn with_route_converters(
    lowered: Option<String>,
    root: &Path,
    relative_source: &Path,
    routes: &[dyckr::plan::Route],
) -> Result<Option<String>, Failure> {
    let Some(program) = lowered else {
        return Ok(None);
    };
    let has_events = program.contains("__YonEvents");
    if routes.is_empty() && !has_events {
        return Ok(Some(program));
    }
    let extension = lowercase_extension(relative_source);
    let converters = crate::route_types::Converters::program_text(&extension, root)
        .map_err(|message| invalid_source(&portable(relative_source), &message))?;
    let imports = crate::route_types::Converters::program_imports(&extension, root)
        .map_err(|message| invalid_source(&portable(relative_source), &message))?;
    Ok(Some(format!("{imports}{program}\n{converters}")))
}

pub fn with_models(
    lowered: Option<String>,
    contents: &str,
    models: &ModelSources,
    codecs: bool,
) -> Option<String> {
    if !models.changes_program_text() {
        return lowered;
    }
    let program = lowered.as_deref().unwrap_or(contents);
    Some(models.add_to_yon_program(program, codecs))
}

pub fn is_middleware(source: &Path) -> bool {
    let at_project_root = source
        .parent()
        .is_none_or(|parent| parent.as_os_str().is_empty());
    if !at_project_root {
        return false;
    }
    source
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with("middleware."))
}

pub fn validated_source_text<'source>(
    bytes: &'source [u8],
    portable: &str,
) -> Result<&'source str, Failure> {
    if bytes.len() as u64 > DeclaredHandler::MAX_SOURCE_BYTES {
        return Err(invalid_source(
            portable,
            "Handler source exceeds the 1 MiB Phase 2 limit.",
        ));
    }
    if bytes.contains(&0) {
        return Err(invalid_source(
            portable,
            "Handler source contains a NUL byte.",
        ));
    }
    std::str::from_utf8(bytes)
        .map_err(|_| invalid_source(portable, "Handler source must be valid UTF-8."))
}

pub fn invalid_source(path: &str, message: &str) -> Failure {
    Failure::one(diagnostic(
        2004,
        format!("{message} Source: '{path}'."),
        Some(String::from(
            "Use a regular, UTF-8, project-contained source no larger than 1 MiB.",
        )),
        source_span(path, 0, path.len()),
    ))
}

pub fn validate_relative_source(source: &Path) -> Result<&Path, Failure> {
    if !is_safe_relative_source(source) {
        return Err(Failure::one(diagnostic(
            2002,
            format!(
                "Handler source '{}' is not a safe project-relative route path.",
                source.display()
            ),
            Some(String::from(
                "Use server/controllers/**, server/workers/**, server/tests/**, or a root middleware source.",
            )),
            None,
        )));
    }
    Ok(source)
}

pub fn is_safe_relative_source(source: &Path) -> bool {
    if source.as_os_str().is_empty() || source.is_absolute() {
        return false;
    }
    for component in source.components() {
        let Component::Normal(value) = component else {
            return false;
        };
        if value.to_str().is_none() {
            return false;
        }
    }
    source.starts_with("server/controllers")
        || source.starts_with("server/workers")
        || source.starts_with("server/tests")
        || is_middleware(source)
}

pub fn portable(path: &Path) -> String {
    let mut parts = Vec::new();
    for component in path.components() {
        if let Component::Normal(value) = component
            && let Some(text) = value.to_str()
        {
            parts.push(text);
        }
    }
    parts.join("/")
}

pub fn supported_extension<'a>(
    source: &'a Path,
    portable: &str,
) -> Result<Option<&'a str>, Failure> {
    let name = source.file_name().and_then(|name| name.to_str());
    if matches!(
        name,
        Some("yon.js" | "middleware.js" | "yon.py" | "middleware.py")
    ) {
        return Ok(None);
    }
    let Some(extension) = name.and_then(source_extension) else {
        return Err(no_adapter(portable, "has no file extension to resolve"));
    };
    let supported = crate::convention::YonConvention::EXTENSIONS
        .iter()
        .any(|supported| supported.eq_ignore_ascii_case(extension));
    if !supported {
        return Err(unsupported_language(portable, extension));
    }
    Ok(Some(extension))
}

pub fn no_adapter(portable: &str, reason: &str) -> Failure {
    Failure::one(diagnostic(
        2003,
        format!("Handler source '{portable}' {reason}."),
        Some(format!(
            "Use one of the framework-owned route names: {known}. Reach a program in any \
             other language with an explicit relay(...) call.",
            known = known_extensions(),
        )),
        source_span(portable, 0, portable.len()),
    ))
}

pub fn unsupported_language(portable: &str, extension: &str) -> Failure {
    Failure::one(diagnostic(
        2003,
        format!("Handler source '{portable}' is written in a language Yon does not run."),
        Some(format!(
            "Yon runs {known}; '.{extension}' is not one of its framework-owned handler languages. \
             Call another program explicitly through relay(...) from a supported Handler.",
            known = known_extensions()
        )),
        None,
    ))
}

pub fn known_extensions() -> String {
    let mut names = Vec::new();
    for extension in crate::convention::YonConvention::EXTENSIONS {
        names.push(format!("yon.{extension}"));
    }
    names.join(", ")
}

pub fn declared_language(source: &Path) -> HandlerLanguage {
    let name = source.file_name().and_then(|name| name.to_str());
    match name {
        Some("yon.js" | "middleware.js") => HandlerLanguage::JavaScript,
        Some("yon.py" | "middleware.py") => HandlerLanguage::Python,
        _ => name
            .and_then(source_extension)
            .and_then(interpreted_extension)
            .unwrap_or(HandlerLanguage::Direct),
    }
}

pub fn interpreted_extension(extension: &str) -> Option<HandlerLanguage> {
    if extension.eq_ignore_ascii_case("js") {
        return Some(HandlerLanguage::JavaScript);
    }
    if extension.eq_ignore_ascii_case("py") {
        return Some(HandlerLanguage::Python);
    }
    if extension.eq_ignore_ascii_case("rb") {
        return Some(HandlerLanguage::Ruby);
    }
    if extension.eq_ignore_ascii_case("ts") {
        return Some(HandlerLanguage::TypeScript);
    }
    None
}

pub fn source_extension(name: &str) -> Option<&str> {
    let (_, extension) = name.rsplit_once('.')?;
    if extension.is_empty() || !extension.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        return None;
    }
    Some(extension)
}

pub fn write_compiler_file(path: &Path, bytes: &[u8], portable: &str) -> Result<(), Failure> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            invalid_source(
                portable,
                &format!("Cannot create owned compiler file: {error}"),
            )
        })?;
    file.write_all(bytes)
        .and_then(|()| file.flush())
        .and_then(|()| file.sync_all())
        .map_err(|error| {
            invalid_source(
                portable,
                &format!("Cannot durably write compiler file: {error}"),
            )
        })?;
    Ok(())
}
