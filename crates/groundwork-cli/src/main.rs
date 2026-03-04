use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use toml_edit::{value, DocumentMut, InlineTable, Item, Table, Value};

const AGENTS_TOML: &str = "agents.toml";
const LOCK_PATH: &str = ".groundwork/installed.lock.toml";
const ORIGINALS_REPO: &str = "pentaxis93/groundwork";
const CURATION_MANIFEST_TOML: &str = include_str!("../../../manifests/curation.v1.toml");
const ORIGINAL_SKILLS: [(&str, &str); 7] = [
    ("ground", "skills/foundation/ground"),
    ("research", "skills/foundation/research"),
    ("bdd", "skills/specification/bdd"),
    ("planning", "skills/decomposition/planning"),
    ("issue-craft", "skills/decomposition/issue-craft"),
    ("land", "skills/completion/land"),
    ("using-groundwork", "skills/using-groundwork"),
];

#[derive(Parser)]
#[command(name = "groundwork", version, about = "Groundwork installer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init(InstallArgs),
    Update(InstallArgs),
    List,
    Doctor,
}

#[derive(Args, Debug, Clone, Default)]
struct InstallArgs {
    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug, Clone, Default)]
struct InstallOptions {
    dry_run: bool,
}

#[derive(Debug, Clone, Default)]
struct InstallResult {
    upserted: usize,
    pruned: usize,
    total_managed: usize,
}

#[derive(Debug)]
struct ReconcileResult {
    upserted: usize,
    pruned: usize,
    total_managed: usize,
    lock_entries: Vec<LockEntry>,
}

#[derive(Debug, Clone)]
struct ManagedDependencySpec {
    alias: String,
    origin: String,
    source_key: String,
    repo: String,
    dependency_path: String,
    skill: Option<String>,
    pin: Option<PinRef>,
}

#[derive(Debug, Clone)]
struct PinRef {
    key: String,
    value: String,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
        Commands::Init(args) => run_install(true, &InstallOptions::from(args)),
        Commands::Update(args) => run_install(false, &InstallOptions::from(args)),
        Commands::List => run_list(),
        Commands::Doctor => run_doctor(),
    }
}

impl From<InstallArgs> for InstallOptions {
    fn from(value: InstallArgs) -> Self {
        Self {
            dry_run: value.dry_run,
        }
    }
}

fn run_install(is_init: bool, options: &InstallOptions) -> Result<()> {
    let cwd = std::env::current_dir().context("failed to determine current directory")?;
    run_install_in_directory(&cwd, is_init, options).map(|_| ())
}

fn run_install_in_directory(
    base_path: &Path,
    is_init: bool,
    options: &InstallOptions,
) -> Result<InstallResult> {
    let manifest = read_manifest()?;
    validate_manifest(&manifest)?;

    let managed_specs = build_managed_specs(&manifest);

    let agents_path = base_path.join(AGENTS_TOML);
    let mut doc = load_or_init_agents_toml(&agents_path)?;

    ensure_agents_table(&mut doc);
    ensure_dependencies_table(&mut doc);

    let previously_managed = read_previous_managed_aliases(base_path);
    let reconcile = reconcile_manifest_to_doc(&mut doc, &managed_specs, &previously_managed)?;

    let result = InstallResult {
        upserted: reconcile.upserted,
        pruned: reconcile.pruned,
        total_managed: reconcile.total_managed,
    };

    if options.dry_run {
        println!(
            "dry-run {} complete. planned upserts={}, planned prunes={}, managed aliases={}",
            if is_init { "init" } else { "update" },
            result.upserted,
            result.pruned,
            result.total_managed
        );
        print_manifest_summary(&manifest);
        return Ok(result);
    }

    fs::write(&agents_path, doc.to_string())
        .with_context(|| format!("failed to write {}", agents_path.display()))?;

    let sk_runner = ensure_sk_available()?;
    sk_runner.sync()?;

    bootstrap_issue_sync(base_path);

    let sk_version = sk_runner
        .version()
        .unwrap_or_else(|_| "unknown".to_string());
    write_lock(
        base_path,
        sk_runner.mode,
        &sk_version,
        reconcile.lock_entries,
    )?;

    println!(
        "{} complete. managed upserts={}, managed prunes={}, managed aliases={}",
        if is_init { "init" } else { "update" },
        result.upserted,
        result.pruned,
        result.total_managed
    );
    println!("Installed with {} ({})", sk_runner.mode_name(), sk_version);
    print_manifest_summary(&manifest);

    Ok(result)
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
        let skill_str = entry.skill.as_deref().unwrap_or("(unspecified)");
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
    let manifest = read_manifest()?;
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

    if command_exists("gh-issue-sync") {
        let ver = run_command_capture(&["gh-issue-sync", "--version"])
            .unwrap_or_else(|_| "unknown".into());
        println!("ok: gh-issue-sync available ({})", ver.trim());
    } else {
        println!(
            "warn: gh-issue-sync not found (install from https://github.com/mitsuhiko/gh-issue-sync)"
        );
    }

    println!(
        "ok: manifest v{} with {} curated sources",
        manifest.version,
        manifest.curated_sources.len()
    );

    Ok(())
}

fn read_manifest() -> Result<CurationManifest> {
    toml::from_str::<CurationManifest>(CURATION_MANIFEST_TOML)
        .context("failed to parse embedded curation manifest")
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

fn build_managed_specs(manifest: &CurationManifest) -> Vec<ManagedDependencySpec> {
    let mut specs = Vec::new();

    for (skill_name, dependency_path) in ORIGINAL_SKILLS {
        specs.push(ManagedDependencySpec {
            alias: managed_alias(skill_name),
            origin: "original".to_string(),
            source_key: "gh".to_string(),
            repo: ORIGINALS_REPO.to_string(),
            dependency_path: dependency_path.to_string(),
            skill: Some(skill_name.to_string()),
            pin: None,
        });
    }

    for source in &manifest.curated_sources {
        for skill in &source.skills {
            let alias = managed_alias(&skill.name);
            let dependency_path = resolve_curated_path(source.path.as_deref(), &skill.path);
            let pin = source
                .rev
                .as_ref()
                .map(|v| PinRef {
                    key: "rev".to_string(),
                    value: v.clone(),
                })
                .or_else(|| {
                    source.tag.as_ref().map(|v| PinRef {
                        key: "tag".to_string(),
                        value: v.clone(),
                    })
                });

            specs.push(ManagedDependencySpec {
                alias,
                origin: "curated".to_string(),
                source_key: source.source.clone(),
                repo: source.repo.clone(),
                dependency_path,
                skill: Some(skill.name.clone()),
                pin,
            });
        }
    }

    specs
}

fn resolve_curated_path(source_root: Option<&str>, skill_path: &str) -> String {
    match source_root {
        Some(root) => format!("{}/{}", root.trim_end_matches('/'), skill_path),
        None => skill_path.to_string(),
    }
}

fn reconcile_manifest_to_doc(
    doc: &mut DocumentMut,
    specs: &[ManagedDependencySpec],
    previously_managed: &HashSet<String>,
) -> Result<ReconcileResult> {
    ensure_dependencies_table(doc);

    let desired_aliases: HashSet<String> = specs.iter().map(|s| s.alias.clone()).collect();
    let deps = doc["dependencies"]
        .as_table_mut()
        .ok_or_else(|| anyhow!("dependencies table missing after ensure"))?;

    let pruned = prune_stale_managed_dependencies(deps, &desired_aliases, previously_managed);

    let mut upserted = 0;
    for spec in specs {
        if upsert_dependency(deps, spec) {
            upserted += 1;
        }
    }

    let lock_entries = specs
        .iter()
        .map(|spec| LockEntry {
            alias: spec.alias.clone(),
            origin: spec.origin.clone(),
            source: spec.source_key.clone(),
            repo: spec.repo.clone(),
            path: Some(spec.dependency_path.clone()),
            skill: spec.skill.clone(),
            pinned_ref: spec.pin.as_ref().map(|p| p.value.clone()),
        })
        .collect();

    Ok(ReconcileResult {
        upserted,
        pruned,
        total_managed: specs.len(),
        lock_entries,
    })
}

fn read_previous_managed_aliases(base_path: &Path) -> HashSet<String> {
    let lock_path = base_path.join(LOCK_PATH);
    let Ok(text) = fs::read_to_string(&lock_path) else {
        return HashSet::new();
    };
    let Ok(lock) = toml::from_str::<InstallLock>(&text) else {
        return HashSet::new();
    };
    lock.entries.iter().map(|e| e.alias.clone()).collect()
}

fn prune_stale_managed_dependencies(
    deps: &mut Table,
    desired_aliases: &HashSet<String>,
    previously_managed: &HashSet<String>,
) -> usize {
    let to_remove: Vec<String> = deps
        .iter()
        .filter_map(|(k, _)| {
            if previously_managed.contains(k) && !desired_aliases.contains(k) {
                Some(k.to_string())
            } else {
                None
            }
        })
        .collect();

    for k in &to_remove {
        deps.remove(k);
    }

    to_remove.len()
}

fn upsert_dependency(deps: &mut Table, spec: &ManagedDependencySpec) -> bool {
    let mut inline = InlineTable::new();
    inline.insert(spec.source_key.as_str(), Value::from(spec.repo.as_str()));
    inline.insert("path", Value::from(spec.dependency_path.as_str()));
    if let Some(pin) = &spec.pin {
        inline.insert(pin.key.as_str(), Value::from(pin.value.as_str()));
    }

    let new_item = Item::Value(Value::InlineTable(inline));
    let changed = deps
        .get(spec.alias.as_str())
        .map(|existing| existing.to_string() != new_item.to_string())
        .unwrap_or(true);

    if changed {
        deps[spec.alias.as_str()] = new_item;
    }

    changed
}

fn managed_alias(skill_name: &str) -> String {
    skill_name.replace('-', "_")
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

fn bootstrap_issue_sync(base_path: &Path) {
    if !command_exists("gh-issue-sync") {
        println!(
            "note: install gh-issue-sync for local issue mirroring (https://github.com/mitsuhiko/gh-issue-sync)"
        );
        return;
    }

    let issues_dir = base_path.join(".issues");
    if issues_dir.exists() {
        return;
    }

    let init_status = Command::new("gh-issue-sync")
        .arg("init")
        .current_dir(base_path)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .status();

    if !init_status.map(|s| s.success()).unwrap_or(false) {
        println!("warn: gh-issue-sync init failed");
        return;
    }

    let pull_output = Command::new("gh-issue-sync")
        .arg("pull")
        .current_dir(base_path)
        .output();

    match pull_output {
        Ok(output) if output.status.success() => {
            let count = issues_dir
                .read_dir()
                .map(|entries| entries.filter_map(|e| e.ok()).count())
                .unwrap_or(0);
            println!("ok: synced {} issues to .issues/", count);
        }
        _ => {
            println!("warn: gh-issue-sync pull failed");
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn parse_doc(input: &str) -> DocumentMut {
        input.parse::<DocumentMut>().expect("valid toml")
    }

    fn test_manifest() -> CurationManifest {
        CurationManifest {
            version: "1".to_string(),
            curated_sources: vec![CuratedSource {
                id: "superpowers".to_string(),
                source: "gh".to_string(),
                repo: "obra/superpowers".to_string(),
                path: Some("bundle".to_string()),
                rev: Some("abc123".to_string()),
                tag: None,
                license: "MIT".to_string(),
                attribution: "x".to_string(),
                url: "https://example.com".to_string(),
                skills: vec![CuratedSkill {
                    name: "test-driven-development".to_string(),
                    path: "skills/test-driven-development".to_string(),
                    reason: "reason".to_string(),
                }],
            }],
        }
    }

    #[test]
    fn resolve_curated_path_merges_root_and_skill_path() {
        assert_eq!(
            resolve_curated_path(Some("bundle"), "skills/test"),
            "bundle/skills/test"
        );
        assert_eq!(
            resolve_curated_path(Some("bundle/"), "skills/test"),
            "bundle/skills/test"
        );
        assert_eq!(resolve_curated_path(None, "skills/test"), "skills/test");
    }

    #[test]
    fn update_prunes_stale_aliases_and_preserves_external() {
        let mut doc = parse_doc(
            r#"[agents]
claude-code = true

[dependencies]
old_bdd = { gh = "pentaxis93/groundwork", path = "skills/specification/bdd" }
old_ground = { gh = "pentaxis93/groundwork", path = "skills/foundation/ground" }
external_dep = { gh = "org/repo", path = "y" }
"#,
        );

        let previously_managed: HashSet<String> =
            ["old_bdd", "old_ground"].iter().map(|s| s.to_string()).collect();

        let specs = build_managed_specs(&test_manifest());
        let result =
            reconcile_manifest_to_doc(&mut doc, &specs, &previously_managed).expect("reconcile ok");

        let deps = doc["dependencies"].as_table().expect("deps table");
        assert!(!deps.contains_key("old_bdd"));
        assert!(!deps.contains_key("old_ground"));
        assert!(deps.contains_key("external_dep"));
        assert!(deps.contains_key("ground"));
        assert!(deps.contains_key("research"));
        assert!(deps.contains_key("bdd"));
        assert!(deps.contains_key("planning"));
        assert!(deps.contains_key("issue_craft"));
        assert!(deps.contains_key("land"));
        assert!(deps.contains_key("using_groundwork"));
        assert!(deps.contains_key("test_driven_development"));
        assert_eq!(result.pruned, 2);
    }

    #[test]
    fn lock_entry_path_matches_written_dependency_path_with_source_root() {
        let mut doc = parse_doc(
            r#"[agents]
claude-code = true

[dependencies]
"#,
        );

        let specs = build_managed_specs(&test_manifest());
        let previously_managed = HashSet::new();
        let result =
            reconcile_manifest_to_doc(&mut doc, &specs, &previously_managed).expect("reconcile ok");

        let alias = "test_driven_development";
        let deps_str = doc.to_string();
        assert!(deps_str.contains("path = \"bundle/skills/test-driven-development\""));

        let lock_entry = result
            .lock_entries
            .iter()
            .find(|e| e.alias == alias)
            .expect("lock entry exists");

        assert_eq!(
            lock_entry.path.as_deref(),
            Some("bundle/skills/test-driven-development")
        );
    }

    #[test]
    fn second_reconcile_is_idempotent() {
        let mut doc = parse_doc(
            r#"[agents]
claude-code = true

[dependencies]
"#,
        );

        let specs = build_managed_specs(&test_manifest());
        let empty: HashSet<String> = HashSet::new();

        let first =
            reconcile_manifest_to_doc(&mut doc, &specs, &empty).expect("first reconcile ok");
        assert!(first.upserted > 0);

        let after_first: HashSet<String> =
            first.lock_entries.iter().map(|e| e.alias.clone()).collect();
        let second =
            reconcile_manifest_to_doc(&mut doc, &specs, &after_first).expect("second reconcile ok");
        assert_eq!(second.upserted, 0);
        assert_eq!(second.pruned, 0);
    }

    #[test]
    fn dry_run_makes_no_filesystem_changes() {
        let temp = TempDir::new().expect("tempdir");
        let base = temp.path();

        let agents_path = base.join(AGENTS_TOML);
        fs::write(
            &agents_path,
            "[agents]\nclaude-code=true\n\n[dependencies]\n",
        )
        .expect("write agents");

        let before = fs::read_to_string(&agents_path).expect("read before");

        let result = run_install_in_directory(base, false, &InstallOptions { dry_run: true })
            .expect("dry run succeeds");

        let after = fs::read_to_string(&agents_path).expect("read after");
        assert_eq!(before, after);
        assert!(result.upserted > 0);
        assert!(!base.join(LOCK_PATH).exists());
    }

    #[test]
    fn prune_only_removes_previously_managed_aliases() {
        let mut doc = parse_doc(
            r#"[agents]
claude-code = true

[dependencies]
old_skill = { gh = "foo/bar", path = "x" }
unrelated = { gh = "org/repo", path = "y" }
"#,
        );

        let previously_managed: HashSet<String> =
            ["old_skill"].iter().map(|s| s.to_string()).collect();
        let desired: HashSet<String> = ["bdd"].iter().map(|s| s.to_string()).collect();

        let deps = doc["dependencies"].as_table_mut().expect("deps table");
        let pruned = prune_stale_managed_dependencies(deps, &desired, &previously_managed);

        assert_eq!(pruned, 1);
        assert!(!deps.contains_key("old_skill"));
        assert!(deps.contains_key("unrelated"));
    }
}
