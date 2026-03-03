use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use toml_edit::{value, DocumentMut, Item, Table};

const MANIFEST_PATH: &str = "manifests/curation.v1.toml";
const AGENTS_TOML: &str = "agents.toml";
const LOCK_PATH: &str = ".groundwork/installed.lock.toml";
const ORIGINALS_ALIAS: &str = "groundwork_originals";
const ORIGINALS_REPO: &str = "pentaxis93/groundwork";
const ORIGINALS_PATH: &str = "skills";

#[derive(Parser)]
#[command(name = "groundwork", version, about = "Groundwork installer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Update,
    List,
    Doctor,
}

#[derive(Debug, Deserialize)]
struct CurationManifest {
    version: String,
    curated_sources: Vec<CuratedSource>,
}

#[derive(Debug, Deserialize)]
struct CuratedSource {
    id: String,
    source: String,
    repo: String,
    path: Option<String>,
    rev: Option<String>,
    tag: Option<String>,
    license: String,
    attribution: String,
    url: String,
    skills: Vec<CuratedSkill>,
}

#[derive(Debug, Deserialize)]
struct CuratedSkill {
    name: String,
    path: String,
    reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InstallLock {
    version: u32,
    installer_version: String,
    installed_at: String,
    sk: LockSk,
    entries: Vec<LockEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockSk {
    mode: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockEntry {
    alias: String,
    origin: String,
    source: String,
    repo: String,
    path: Option<String>,
    skill: Option<String>,
    pinned_ref: Option<String>,
}

#[derive(Clone, Copy, Debug)]
enum SkMode {
    Binary,
    Npx,
}

#[derive(Debug)]
struct SkRunner {
    mode: SkMode,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => run_install(true),
        Commands::Update => run_install(false),
        Commands::List => run_list(),
        Commands::Doctor => run_doctor(),
    }
}

fn run_install(is_init: bool) -> Result<()> {
    let cwd = std::env::current_dir().context("failed to determine current directory")?;
    let manifest = read_manifest(&cwd)?;
    validate_manifest(&manifest)?;

    let agents_path = cwd.join(AGENTS_TOML);
    let mut doc = load_or_init_agents_toml(&agents_path)?;

    ensure_agents_table(&mut doc);
    ensure_dependencies_table(&mut doc);

    upsert_originals_dependency(&mut doc);

    let mut lock_entries = Vec::new();
    lock_entries.push(LockEntry {
        alias: ORIGINALS_ALIAS.to_string(),
        origin: "original".to_string(),
        source: "gh".to_string(),
        repo: ORIGINALS_REPO.to_string(),
        path: Some(ORIGINALS_PATH.to_string()),
        skill: None,
        pinned_ref: None,
    });

    for source in &manifest.curated_sources {
        for skill in &source.skills {
            let alias = managed_alias(&source.id, &skill.name);
            upsert_curated_dependency(&mut doc, &alias, source, skill);
            lock_entries.push(LockEntry {
                alias,
                origin: "curated".to_string(),
                source: source.source.clone(),
                repo: source.repo.clone(),
                path: Some(skill.path.clone()),
                skill: Some(skill.name.clone()),
                pinned_ref: source.rev.clone().or_else(|| source.tag.clone()),
            });
        }
    }

    fs::write(&agents_path, doc.to_string())
        .with_context(|| format!("failed to write {}", agents_path.display()))?;

    let sk_runner = ensure_sk_available()?;
    sk_runner.sync()?;

    let sk_version = sk_runner
        .version()
        .unwrap_or_else(|_| "unknown".to_string());
    write_lock(&cwd, sk_runner.mode, &sk_version, lock_entries)?;

    println!(
        "{} complete. Installed 1 originals package + {} curated skills.",
        if is_init { "init" } else { "update" },
        manifest
            .curated_sources
            .iter()
            .map(|s| s.skills.len())
            .sum::<usize>()
    );
    println!("Installed with {} ({})", sk_runner.mode_name(), sk_version);
    print_manifest_summary(&manifest);

    Ok(())
}

fn run_list() -> Result<()> {
    let cwd = std::env::current_dir().context("failed to determine current directory")?;
    let lock_path = cwd.join(LOCK_PATH);

    if !lock_path.exists() {
        bail!(
            "no install lock found at {} (run `groundwork init` first)",
            lock_path.display()
        );
    }

    let lock_text = fs::read_to_string(&lock_path)
        .with_context(|| format!("failed to read {}", lock_path.display()))?;
    let lock: InstallLock = toml::from_str(&lock_text).context("failed to parse install lock")?;

    println!("Groundwork install lock");
    println!("- installer: {}", lock.installer_version);
    println!("- installed_at: {}", lock.installed_at);
    println!("- sk: {} ({})", lock.sk.mode, lock.sk.version);
    println!("- entries: {}", lock.entries.len());

    for entry in lock.entries {
        let ref_str = entry.pinned_ref.as_deref().map_or("unpinned", |r| r);
        let skill_str = entry.skill.as_deref().unwrap_or("(all originals)");
        println!(
            "  - {}: {} {} path={} skill={} ref={}",
            entry.alias,
            entry.origin,
            entry.repo,
            entry.path.as_deref().unwrap_or("-"),
            skill_str,
            ref_str
        );
    }

    Ok(())
}

fn run_doctor() -> Result<()> {
    let cwd = std::env::current_dir().context("failed to determine current directory")?;
    let manifest = read_manifest(&cwd)?;
    validate_manifest(&manifest)?;

    let agents_path = cwd.join(AGENTS_TOML);
    if agents_path.exists() {
        let _ = load_or_init_agents_toml(&agents_path)?;
        println!("ok: {} is readable", agents_path.display());
    } else {
        println!(
            "warn: {} not found (will be created by init)",
            agents_path.display()
        );
    }

    if command_exists("sk") {
        let ver = run_command_capture(&["sk", "--version"]).unwrap_or_else(|_| "unknown".into());
        println!("ok: sk available ({})", ver.trim());
    } else {
        let node = command_exists("node");
        let npm = command_exists("npm");
        let npx = command_exists("npx");
        println!("warn: sk not found");
        println!("info: node={} npm={} npx={}", node, npm, npx);
        if !node || (!npm && !npx) {
            bail!(
                "sk bootstrap unavailable: install Node.js (with npm/npx) or install sk manually"
            );
        }
        println!("ok: bootstrap prerequisites satisfied");
    }

    println!(
        "ok: manifest v{} with {} curated sources",
        manifest.version,
        manifest.curated_sources.len()
    );

    Ok(())
}

fn read_manifest(cwd: &Path) -> Result<CurationManifest> {
    let manifest_path = cwd.join(MANIFEST_PATH);
    let manifest_text = fs::read_to_string(&manifest_path)
        .with_context(|| format!("failed to read {}", manifest_path.display()))?;
    toml::from_str::<CurationManifest>(&manifest_text)
        .with_context(|| format!("failed to parse {}", manifest_path.display()))
}

fn validate_manifest(manifest: &CurationManifest) -> Result<()> {
    if manifest.version.trim().is_empty() {
        bail!("manifest version is empty");
    }

    for source in &manifest.curated_sources {
        if source.id.trim().is_empty() {
            bail!("curated source has empty id");
        }
        if source.source != "gh" && source.source != "git" {
            bail!(
                "unsupported source type `{}` in `{}`",
                source.source,
                source.id
            );
        }
        if source.rev.is_none() && source.tag.is_none() {
            bail!("source `{}` must pin either `rev` or `tag`", source.id);
        }
        if source.skills.is_empty() {
            bail!("source `{}` has no curated skills", source.id);
        }
        if source.license.trim().is_empty() {
            bail!("source `{}` missing license", source.id);
        }
        if source.attribution.trim().is_empty() {
            bail!("source `{}` missing attribution", source.id);
        }
        if source.url.trim().is_empty() {
            bail!("source `{}` missing url", source.id);
        }
        for skill in &source.skills {
            if skill.name.trim().is_empty() {
                bail!("source `{}` has skill with empty name", source.id);
            }
            if skill.path.trim().is_empty() {
                bail!("source `{}` skill `{}` missing path", source.id, skill.name);
            }
            if skill.reason.trim().is_empty() {
                bail!(
                    "source `{}` skill `{}` missing reason",
                    source.id,
                    skill.name
                );
            }
        }
    }

    Ok(())
}

fn load_or_init_agents_toml(path: &Path) -> Result<DocumentMut> {
    if path.exists() {
        let text = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        return text
            .parse::<DocumentMut>()
            .with_context(|| format!("failed to parse {}", path.display()));
    }

    let base = r#"[agents]
claude-code = true
codex = true
opencode = true

[dependencies]
"#;

    base.parse::<DocumentMut>()
        .context("failed to create default agents.toml document")
}

fn ensure_agents_table(doc: &mut DocumentMut) {
    if !doc["agents"].is_table() {
        doc["agents"] = Item::Table(Table::new());
    }

    let agents = doc["agents"].as_table_mut().expect("agents table exists");
    set_if_missing(agents, "claude-code", true);
    set_if_missing(agents, "codex", true);
    set_if_missing(agents, "opencode", true);
}

fn set_if_missing(table: &mut Table, key: &str, v: bool) {
    if !table.contains_key(key) {
        table[key] = value(v);
    }
}

fn ensure_dependencies_table(doc: &mut DocumentMut) {
    if !doc["dependencies"].is_table() {
        doc["dependencies"] = Item::Table(Table::new());
    }
}

fn upsert_originals_dependency(doc: &mut DocumentMut) {
    let deps = doc["dependencies"]
        .as_table_mut()
        .expect("dependencies table exists");

    let mut inline = toml_edit::InlineTable::new();
    inline.insert("gh", toml_edit::Value::from(ORIGINALS_REPO));
    inline.insert("path", toml_edit::Value::from(ORIGINALS_PATH));
    deps[ORIGINALS_ALIAS] = Item::Value(toml_edit::Value::InlineTable(inline));
}

fn upsert_curated_dependency(
    doc: &mut DocumentMut,
    alias: &str,
    source: &CuratedSource,
    skill: &CuratedSkill,
) {
    let deps = doc["dependencies"]
        .as_table_mut()
        .expect("dependencies table exists");

    let mut inline = toml_edit::InlineTable::new();
    inline.insert(
        source.source.as_str(),
        toml_edit::Value::from(source.repo.as_str()),
    );

    if let Some(root_path) = source.path.as_deref() {
        let merged = format!("{}/{}", root_path.trim_end_matches('/'), skill.path);
        inline.insert("path", toml_edit::Value::from(merged));
    } else {
        inline.insert("path", toml_edit::Value::from(skill.path.as_str()));
    }

    if let Some(rev) = source.rev.as_deref() {
        inline.insert("rev", toml_edit::Value::from(rev));
    }
    if let Some(tag) = source.tag.as_deref() {
        inline.insert("tag", toml_edit::Value::from(tag));
    }

    deps[alias] = Item::Value(toml_edit::Value::InlineTable(inline));
}

fn managed_alias(source_id: &str, skill_name: &str) -> String {
    let src = source_id.replace('-', "_");
    let skill = skill_name.replace('-', "_");
    format!("groundwork_{}_{}", src, skill)
}

fn ensure_sk_available() -> Result<SkRunner> {
    if command_exists("sk") {
        return Ok(SkRunner {
            mode: SkMode::Binary,
        });
    }

    if !command_exists("node") {
        bail!("sk not found and Node.js is unavailable; install Node.js or install sk manually");
    }

    if command_exists("npm") {
        let npm_install = Command::new("npm")
            .args(["install", "-g", "@skills-supply/sk"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        if let Ok(status) = npm_install {
            if status.success() && command_exists("sk") {
                return Ok(SkRunner {
                    mode: SkMode::Binary,
                });
            }
        }
    }

    if command_exists("npx") {
        let probe = Command::new("npx")
            .args(["-y", "@skills-supply/sk", "--version"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .context("failed to execute npx for sk bootstrap")?;

        if probe.success() {
            return Ok(SkRunner { mode: SkMode::Npx });
        }
    }

    bail!(
        "failed to bootstrap sk automatically; install with `npm install -g @skills-supply/sk` and rerun"
    );
}

impl SkRunner {
    fn sync(&self) -> Result<()> {
        let status = match self.mode {
            SkMode::Binary => Command::new("sk").arg("sync").status(),
            SkMode::Npx => Command::new("npx")
                .args(["-y", "@skills-supply/sk", "sync"])
                .status(),
        }
        .context("failed to run sk sync")?;

        if !status.success() {
            bail!("sk sync failed with status {}", status);
        }

        Ok(())
    }

    fn version(&self) -> Result<String> {
        let cmd = match self.mode {
            SkMode::Binary => vec!["sk", "--version"],
            SkMode::Npx => vec!["npx", "-y", "@skills-supply/sk", "--version"],
        };

        run_command_capture(&cmd)
    }

    fn mode_name(&self) -> &'static str {
        match self.mode {
            SkMode::Binary => "sk",
            SkMode::Npx => "npx @skills-supply/sk",
        }
    }
}

fn run_command_capture(args: &[&str]) -> Result<String> {
    let (program, rest) = args
        .split_first()
        .ok_or_else(|| anyhow!("empty command invocation"))?;
    let output = Command::new(program)
        .args(rest)
        .output()
        .with_context(|| format!("failed to run `{}`", args.join(" ")))?;
    if !output.status.success() {
        bail!(
            "command `{}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn command_exists(name: &str) -> bool {
    Command::new("sh")
        .args(["-c", &format!("command -v {} >/dev/null 2>&1", name)])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn write_lock(
    cwd: &Path,
    sk_mode: SkMode,
    sk_version: &str,
    entries: Vec<LockEntry>,
) -> Result<()> {
    let lock = InstallLock {
        version: 1,
        installer_version: env!("CARGO_PKG_VERSION").to_string(),
        installed_at: Utc::now().to_rfc3339(),
        sk: LockSk {
            mode: match sk_mode {
                SkMode::Binary => "binary".to_string(),
                SkMode::Npx => "npx".to_string(),
            },
            version: sk_version.to_string(),
        },
        entries,
    };

    let lock_toml = toml::to_string_pretty(&lock).context("failed to serialize install lock")?;
    let lock_path = cwd.join(LOCK_PATH);
    if let Some(parent) = lock_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(&lock_path, lock_toml)
        .with_context(|| format!("failed to write {}", lock_path.display()))
}

fn print_manifest_summary(manifest: &CurationManifest) {
    println!("Curated sources:");
    for source in &manifest.curated_sources {
        let pin = source
            .rev
            .as_deref()
            .or(source.tag.as_deref())
            .unwrap_or("none");
        println!(
            "- {} ({}, {} @ {})",
            source.id, source.repo, source.source, pin
        );
        for skill in &source.skills {
            println!("    - {} ({})", skill.name, skill.path);
        }
    }
}
