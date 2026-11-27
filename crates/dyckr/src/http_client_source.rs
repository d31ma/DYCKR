pub const TAC: &'static str = "tac";
pub const BROWSER_SCRIPTS: [&'static str; 2] = ["js", "ts"];
pub const CLASS: &'static str = "httpclient";
pub const STREAM: &'static str = "streamrequest";
pub const SOCKET: &'static str = "socketrequest";
pub const CACHE_OPTION_REFUSAL: &'static str = "@HTTPClient no longer takes \"cache\": its IndexedDB fetch cache was removed (ADR 0087). Mark the class @CacheSettings and each read method @Cached, like Spring's @Cacheable. For reads that work offline, add a rule for the path to tac.cache in client/manifest.json";
pub const OTHER_ROLES: [&'static str; 9] = [
        "service",
        "store",
        "delegate",
        "converter",
        "setup",
        "entity",
        "client",
        "controller",
        "requestroute",
    ];


pub const ROLE_SUFFIXES: [&'static str; 4] = ["Client", "Service", "Store", "Delegate"];

pub const REQUESTS: [(&'static str, &'static str, &'static str); 5] = [
        ("getrequest", "GET", "GetRequest"),
        ("postrequest", "POST", "PostRequest"),
        ("putrequest", "PUT", "PutRequest"),
        ("patchrequest", "PATCH", "PatchRequest"),
        ("deleterequest", "DELETE", "DeleteRequest"),
    ];
pub const PARAMETERS: [(&'static str, &'static str, &'static str); 4] = [
        ("pathvariable", "pathVariables", "PathVariable"),
        ("requestparam", "query", "RequestParam"),
        ("requestbody", "body", "RequestBody"),
        ("requestheader", "headers", "RequestHeader"),
    ];

pub struct HttpClientSource;
use std::ops::Range;
use std::path::Path;

impl HttpClientSource {

    pub fn extension(path: &Path) -> &str {
        path.extension()
            .and_then(|value| value.to_str())
            .unwrap_or("")
    }

    pub fn code(path: &Path, source: &str) -> String {
        let mask = crate::lexical::code_mask(Self::extension(path), source);
        Self::masked(source, &mask.code)
    }

    pub fn apply(source: &str, mut edits: Vec<(Range<usize>, String)>) -> String {
        edits.sort_by_key(|(range, _)| std::cmp::Reverse(range.start));
        let mut output = source.to_owned();
        for (range, replacement) in edits {
            output.replace_range(range, &replacement);
        }
        output
    }

    pub fn masked(source: &str, visible: &[bool]) -> String {
        let mut bytes = source.as_bytes().to_vec();
        for (byte, shown) in bytes.iter_mut().zip(visible) {
            if !shown && !matches!(*byte, b'\n' | b'\r') {
                *byte = b' ';
            }
        }
        String::from_utf8(bytes).unwrap_or_default()
    }

    pub fn normalized(name: &str) -> String {
        name.rsplit("::")
            .next()
            .unwrap_or(name)
            .replace('_', "")
            .to_ascii_lowercase()
    }
}
impl HttpClientSource {

    pub fn request(name: &str) -> Option<(&'static str, &'static str)> {
        for (marker, verb, written) in crate::http_client_source::REQUESTS {
            if marker == name {
                return Some((verb, written));
            }
        }
        None
    }

    pub fn parameter(name: &str) -> Option<(&'static str, &'static str)> {
        let name = HttpClientSource::normalized(name);
        for (marker, key, written) in crate::http_client_source::PARAMETERS {
            if marker == name {
                return Some((key, written));
            }
        }
        None
    }

    pub fn prefix(extension: &str) -> &'static str {
        match extension {
            "rs" | "php" | "rb" => "#[",
            "cs" => "[",
            _ => "@",
        }
    }

    pub fn marker_text(extension: &str, name: &str, arguments: &str) -> String {
        let written = if extension == "py" {
            crate::renamed::RoutingRename::snake_case(name)
        } else {
            name.to_owned()
        };
        let call = if arguments.is_empty() {
            written
        } else {
            format!("{written}({arguments})")
        };
        match Self::prefix(extension) {
            "@" => format!("@{call}"),
            opening => format!("{opening}{call}]"),
        }
    }

    pub fn adapter_name(class: &str) -> String {
        let mut stem = class;
        for suffix in crate::http_client_source::ROLE_SUFFIXES {
            if let Some(rest) = class.strip_suffix(suffix)
                && !rest.is_empty()
            {
                stem = rest;
                break;
            }
        }
        let mut characters = stem.chars();
        let first = characters.next().unwrap_or_default().to_ascii_lowercase();
        format!("{first}{}", characters.as_str())
    }
}
