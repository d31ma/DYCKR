//! yckr — the Vyckr bootstrap dispatcher.
//!
//! `yckr --kw|--k|--b [--<flag>-version <v>] <args…>` resolves a product engine,
//! downloads and SHA-256-verifies it from its GitHub release (caching it), then
//! delegates the remaining arguments to it. It owns no product code.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use sha2::{Digest, Sha256};

const GITHUB: &str = "https://github.com/d31ma";

struct Product {
    flag: &'static str,
    repo: &'static str,
    engine: &'static str,
}

const PRODUCTS: [Product; 3] = [
    Product { flag: "kw", repo: "KWYCKR", engine: "kwyckr" },
    Product { flag: "k", repo: "KYCKR", engine: "kyckr" },
    Product { flag: "b", repo: "BYCKR", engine: "byckr" },
];

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(product) = PRODUCTS.iter().find(|p| args.iter().any(|a| a == &format!("--{}", p.flag))) else {
        eprintln!("usage: yckr --kw|--k|--b [--<flag>-version <version>] <args…>");
        return ExitCode::from(2);
    };

    let version = match requested_version(&args, product) {
        Ok(v) => v,
        Err(e) => { eprintln!("yckr: {e}"); return ExitCode::from(2); }
    };
    let version = match resolve_version(product, &version) {
        Ok(v) => v,
        Err(e) => { eprintln!("yckr: {e}"); return ExitCode::from(1); }
    };
    let bin = match engine_binary(product, &version) {
        Ok(p) => p,
        Err(e) => { eprintln!("yckr: {e}"); return ExitCode::from(1); }
    };

    let rest: Vec<String> = args
        .into_iter()
        .filter(|a| {
            a != &format!("--{}", product.flag)
                && !(a.starts_with(&format!("--{}-version", product.flag))
                    || a == "--version" || a == "-v")
        })
        .collect();

    match Command::new(&bin).args(rest).status() {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
        Err(error) => { eprintln!("yckr: cannot run {}: {error}", bin.display()); ExitCode::from(1) }
    }
}

fn requested_version(args: &[String], product: &Product) -> Result<String, String> {
    let key = format!("--{}-version", product.flag);
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == &key {
            return iter.next().cloned().ok_or_else(|| format!("{key} needs a value"));
        }
    }
    let env_key = format!("YCKR_{}_VERSION", product.flag.to_uppercase());
    Ok(env::var(env_key).unwrap_or_else(|_| "latest".into()))
}

fn resolve_version(product: &Product, requested: &str) -> Result<String, String> {
    if requested == "latest" {
        let url = format!("{GITHUB}/{}/releases/latest", product.repo);
        let response = ureq::get(&url).call().map_err(|e| format!("{url}: {e}"))?;
        let body = response.into_body().read_to_string().map_err(|e| e.to_string())?;
        let value: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;
        value["tag_name"]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| format!("{url}: no tag_name"))
    } else {
        Ok(requested.to_owned())
    }
}

fn engine_binary(product: &Product, version: &str) -> Result<PathBuf, String> {
    let dir = cache_dir(product, version);
    let bin = dir.join(product.engine);
    if bin.is_file() {
        return Ok(bin);
    }
    fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let tag = version.trim_start_matches('v');
    let target = platform_target();
    let asset = format!("{}-{target}", product.engine);
    let base = format!("{GITHUB}/{}/releases/download/{tag}", product.repo);
    let body = download(&format!("{base}/{asset}"))?;
    let expected = String::from_utf8(download(&format!("{base}/{asset}.sha256"))?)
        .map_err(|e| e.to_string())?;
    let expected = expected.trim();
    let actual = hex(&Sha256::digest(&body));
    if actual != expected {
        return Err(format!("{asset}: SHA-256 mismatch (expected {expected}, got {actual})"));
    }
    fs::write(&bin, &body).map_err(|e| format!("{}: {e}", bin.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&bin, fs::Permissions::from_mode(0o755));
    }
    Ok(bin)
}

fn download(url: &str) -> Result<Vec<u8>, String> {
    let response = ureq::get(url).call().map_err(|e| format!("{url}: {e}"))?;
    response.into_body().read_to_vec().map_err(|e| format!("{url}: {e}"))
}

fn cache_dir(product: &Product, version: &str) -> PathBuf {
    let root = env::var("YCKR_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home_cache());
    root.join(product.flag).join(version)
}

fn home_cache() -> PathBuf {
    if let Ok(dir) = env::var("XDG_CACHE_HOME") {
        return PathBuf::from(dir).join("yckr");
    }
    if let Ok(home) = env::var("HOME") {
        return Path::new(&home).join("Library/Caches/yckr");
    }
    PathBuf::from(".yckr")
}

fn platform_target() -> String {
    let os = match env::consts::OS {
        "macos" => "macos",
        "linux" => "linux",
        "windows" => "windows",
        other => other,
    };
    let arch = match env::consts::ARCH {
        "aarch64" => "arm64",
        "x86_64" => "x86_64",
        other => other,
    };
    format!("{os}-{arch}")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
