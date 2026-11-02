use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
struct Baseline {
    version: u32,
    root: String,
    files: BTreeMap<String, String>,
}

fn hash_file(path: &Path) -> io::Result<String> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn collect(root: &Path, baseline_path: &Path) -> Result<BTreeMap<String, String>, String> {
    let mut files = BTreeMap::new();
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().is_file() { continue; }
        let path = entry.path();
        // Do not hash the manifest itself when it lives inside the scanned tree.
        if path == baseline_path { continue; }
        let relative = path.strip_prefix(root).map_err(|e| e.to_string())?;
        let name = relative.to_string_lossy().replace('\\', "/");
        let hash = hash_file(path).map_err(|e| format!("{}: {e}", path.display()))?;
        files.insert(name, hash);
    }
    Ok(files)
}

fn usage() {
    eprintln!("Usage: lorshed-sentinel <init|check> <directory> <baseline.json>");
    eprintln!("Example: lorshed-sentinel init /etc ./etc-baseline.json");
}

fn run() -> Result<i32, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 || !matches!(args[1].as_str(), "init" | "check") {
        usage();
        return Ok(2);
    }
    let root = fs::canonicalize(&args[2]).map_err(|e| format!("directory: {e}"))?;
    if !root.is_dir() { return Err("target must be a directory".into()); }
    let manifest_arg = PathBuf::from(&args[3]);
    let manifest_parent = manifest_arg.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    let parent = fs::canonicalize(manifest_parent).map_err(|e| format!("manifest parent: {e}"))?;
    let filename = manifest_arg.file_name().ok_or("invalid manifest path")?;
    let manifest = parent.join(filename);
    if manifest.is_symlink() { return Err("manifest must not be a symlink".into()); }

    if args[1] == "init" {
        if manifest.exists() { return Err("baseline already exists; move it before reinitializing".into()); }
        let files = collect(&root, &manifest)?;
        let baseline = Baseline { version: 1, root: root.to_string_lossy().into_owned(), files };
        let serialized = serde_json::to_vec_pretty(&baseline).map_err(|e| e.to_string())?;
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        let mut output = options.open(&manifest).map_err(|e| format!("create baseline: {e}"))?;
        output.write_all(&serialized).map_err(|e| e.to_string())?;
        println!("Baseline created: {} files", baseline.files.len());
        return Ok(0);
    }

    let baseline: Baseline = serde_json::from_slice(&fs::read(&manifest).map_err(|e| format!("read baseline: {e}"))?)
        .map_err(|e| format!("parse baseline: {e}"))?;
    if baseline.version != 1 || baseline.root != root.to_string_lossy() {
        return Err("baseline version or root does not match".into());
    }
    let current = collect(&root, &manifest)?;
    let old_names: BTreeSet<_> = baseline.files.keys().collect();
    let new_names: BTreeSet<_> = current.keys().collect();
    let mut changes = 0;
    for name in old_names.difference(&new_names) { println!("REMOVED  {name}"); changes += 1; }
    for name in new_names.difference(&old_names) { println!("ADDED    {name}"); changes += 1; }
    for name in old_names.intersection(&new_names) {
        if baseline.files[*name] != current[*name] { println!("MODIFIED {name}"); changes += 1; }
    }
    if changes == 0 { println!("OK: {} files unchanged", current.len()); Ok(0) }
    else { println!("ALERT: {changes} change(s) detected"); Ok(1) }
}

fn main() {
    match run() {
        Ok(code) => std::process::exit(code),
        Err(error) => { eprintln!("ERROR: {error}"); std::process::exit(2); }
    }
}
