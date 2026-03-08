use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use toml_edit::{value, DocumentMut, InlineTable, Item, Table, Value};

const AGENTS_TOML: &str = "agents.toml";
const SCHEMA_DIR: &str = ".groundwork/schemas";
const ARTIFACTS_DIR: &str = ".groundwork/artifacts";
const LOCK_PATH: &str = ".groundwork/installed.lock.toml";
const GROUNDWORK_REPO: &str = "pentaxis93/groundwork";
const SKILLS_SUPPLY_FORK_URL: &str = "https://github.com/pentaxis93/skills-supply";
const SKILLS_SUPPLY_FORK_REF: &str = "groundwork-v1";
const SKILLS_SUPPLY_FORK_COMMIT: &str = "ec257c4057c32230478e5c1b86347177134e1469";
const GH_ISSUE_SYNC_REPO: &str = "mitsuhiko/gh-issue-sync";
const GH_ISSUE_SYNC_RELEASE_TAG: &str = "v0.3.0";
const SHIPPED_SKILLS_TOML: &str = include_str!("../../../skills/skills.toml");
const EMBEDDED_SCHEMAS: [EmbeddedSchema; 8] = [
    EmbeddedSchema {
        filename: "artifact-frontmatter.schema.json",
        content: include_str!("../../../schemas/artifact-frontmatter.schema.json"),
    },
    EmbeddedSchema {
        filename: "behavior-contract.schema.json",
        content: include_str!("../../../schemas/behavior-contract.schema.json"),
    },
    EmbeddedSchema {
        filename: "completion-evidence.schema.json",
        content: include_str!("../../../schemas/completion-evidence.schema.json"),
    },
    EmbeddedSchema {
        filename: "completion-record.schema.json",
        content: include_str!("../../../schemas/completion-record.schema.json"),
    },
    EmbeddedSchema {
        filename: "groundwork-frontmatter.schema.json",
        content: include_str!("../../../schemas/groundwork-frontmatter.schema.json"),
    },
    EmbeddedSchema {
        filename: "implementation-plan.schema.json",
        content: include_str!("../../../schemas/implementation-plan.schema.json"),
    },
    EmbeddedSchema {
        filename: "research-record.schema.json",
        content: include_str!("../../../schemas/research-record.schema.json"),
    },
    EmbeddedSchema {
        filename: "test-evidence.schema.json",
        content: include_str!("../../../schemas/test-evidence.schema.json"),
    },
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
    schema_created: usize,
    schema_updated: usize,
    schema_unchanged: usize,
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

#[derive(Debug, Clone, Copy)]
struct EmbeddedSchema {
    filename: &'static str,
    content: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SchemaFileAction {
    Create,
    Update,
    Unchanged,
}

#[derive(Debug, Clone, Copy)]
struct SchemaFilePlan {
    filename: &'static str,
    action: SchemaFileAction,
}

#[derive(Debug, Clone, Default)]
struct SchemaSyncPlan {
    create_schema_dir: bool,
    create_artifacts_dir: bool,
    files: Vec<SchemaFilePlan>,
}

#[derive(Debug, Clone, Default)]
struct SchemaDoctorReport {
    schema_dir_exists: bool,
    schema_dir_is_directory: bool,
    missing_files: Vec<&'static str>,
    mismatched_files: Vec<&'static str>,
    unreadable_files: Vec<&'static str>,
    extra_files: Vec<String>,
}

impl SchemaSyncPlan {
    fn created_count(&self) -> usize {
        self.files
            .iter()
            .filter(|f| f.action == SchemaFileAction::Create)
            .count()
    }

    fn updated_count(&self) -> usize {
        self.files
            .iter()
            .filter(|f| f.action == SchemaFileAction::Update)
            .count()
    }

    fn unchanged_count(&self) -> usize {
        self.files
            .iter()
            .filter(|f| f.action == SchemaFileAction::Unchanged)
            .count()
    }
}

#[derive(Debug, Deserialize)]
struct SkillsManifest {
    version: String,
    skills: Vec<ShippedSkill>,
}

#[derive(Debug, Deserialize)]
struct ShippedSkill {
    name: String,
    path: String,
    provider: String,
    repo: String,
    rev: Option<String>,
    tag: Option<String>,
    use_when: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InstallLock {
    version: u32,
    installer_version: String,
    installed_at: String,
    sk: LockSk,
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_sync: Option<LockIssueSync>,
    entries: Vec<LockEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockSk {
    mode: SkMode,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockIssueSync {
    mode: IssueSyncMode,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum SkMode {
    Binary,
    /// Retained for lock file backward compatibility; no longer used at runtime.
    Npx,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum IssueSyncMode {
    Binary,
    GoInstall,
}

impl std::fmt::Display for SkMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkMode::Binary => write!(f, "binary"),
            SkMode::Npx => write!(f, "npx"),
        }
    }
}

impl std::fmt::Display for IssueSyncMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueSyncMode::Binary => write!(f, "binary"),
            IssueSyncMode::GoInstall => write!(f, "go-install"),
        }
    }
}

impl SkMode {
    fn mode_name(&self) -> &'static str {
        match self {
            SkMode::Binary => "binary",
            SkMode::Npx => "npx",
        }
    }
}

impl IssueSyncMode {
    fn mode_name(&self) -> &'static str {
        match self {
            IssueSyncMode::Binary => "binary",
            IssueSyncMode::GoInstall => "go install",
        }
    }
}

#[derive(Debug)]
struct SkRunner {
    mode: SkMode,
    binary_path: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct IssueSyncReleaseAsset {
    name: &'static str,
    sha256: &'static str,
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
    let schema_plan = plan_schema_sync(base_path)?;

    let result = InstallResult {
        upserted: reconcile.upserted,
        pruned: reconcile.pruned,
        total_managed: reconcile.total_managed,
        schema_created: schema_plan.created_count(),
        schema_updated: schema_plan.updated_count(),
        schema_unchanged: schema_plan.unchanged_count(),
    };

    if options.dry_run {
        println!(
            "dry-run {} complete. planned upserts={}, planned prunes={}, managed aliases={}",
            if is_init { "init" } else { "update" },
            result.upserted,
            result.pruned,
            result.total_managed
        );
        println!(
            "dry-run: planned schema sync creates={}, updates={}, unchanged={}",
            result.schema_created, result.schema_updated, result.schema_unchanged
        );
        print_schema_sync_plan_summary(&schema_plan, true);
        print_manifest_summary(&manifest);
        return Ok(result);
    }

    fs::write(&agents_path, doc.to_string())
        .with_context(|| format!("failed to write {}", agents_path.display()))?;

    let sk_runner = ensure_sk_available()?;
    sk_runner.sync()?;

    let issue_sync_info = bootstrap_issue_sync(base_path, is_init)?;

    apply_schema_sync(base_path, &schema_plan)?;

    let sk_version = capture_required_version("sk", sk_runner.version())?;
    write_lock(
        base_path,
        sk_runner.mode,
        &sk_version,
        issue_sync_info
            .as_ref()
            .map(|(mode, ver)| (*mode, ver.as_str())),
        reconcile.lock_entries,
    )?;

    println!(
        "{} complete. managed upserts={}, managed prunes={}, managed aliases={}",
        if is_init { "init" } else { "update" },
        result.upserted,
        result.pruned,
        result.total_managed
    );
    println!(
        "schema sync complete. creates={}, updates={}, unchanged={}",
        result.schema_created, result.schema_updated, result.schema_unchanged
    );
    print_schema_sync_plan_summary(&schema_plan, false);
    println!("Installed with {} ({})", sk_runner.mode_name(), sk_version);
    if let Some((mode, ref ver)) = issue_sync_info {
        println!("Installed gh-issue-sync via {} ({})", mode.mode_name(), ver);
    }
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
    let manifest = read_manifest()?;
    validate_manifest(&manifest)?;
    let order = manifest_position_map(&manifest);

    println!("Groundwork install lock");
    println!("- installer: {}", lock.installer_version);
    println!("- installed_at: {}", lock.installed_at);
    println!("- sk: {} ({})", lock.sk.mode.mode_name(), lock.sk.version);
    if let Some(ref is) = lock.issue_sync {
        println!("- gh-issue-sync: {} ({})", is.mode.mode_name(), is.version);
    } else {
        println!("- gh-issue-sync: not installed");
    }
    println!("- entries: {}", lock.entries.len());

    let mut entries = lock.entries;
    entries.sort_by_key(|entry| order.get(&entry.alias).copied().unwrap_or(usize::MAX));

    for entry in entries {
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
        let ver = first_line_version(run_command_capture(&["sk", "--version"]));
        println!("ok: sk available ({})", ver);
        let help_output = run_command_capture_ignore_status(&["sk", "sync", "--help"]);
        if !supports_skill_target_option(&help_output) {
            println!(
                "error: sk is missing required `--skill-target` sync option; install forked sk from {}",
                SKILLS_SUPPLY_FORK_URL
            );
        }
    } else {
        let node = command_exists("node");
        let npm = command_exists("npm");
        let git = command_exists("git");
        println!("warn: sk not found");
        println!("info: node={} npm={} git={}", node, npm, git);
        if !node || !npm || !git {
            bail!(
                "sk bootstrap unavailable: install Node.js, npm, and git (or install sk manually)"
            );
        }
        println!("ok: bootstrap prerequisites satisfied");
    }

    let gh_available = if command_exists("gh") {
        let ver = first_line_version(run_command_capture(&["gh", "--version"]));
        println!("ok: gh CLI available ({})", ver);
        true
    } else {
        println!("warn: gh CLI not found (prerequisite for gh-issue-sync)");
        false
    };

    let issue_sync_available = if command_exists("gh-issue-sync") {
        let ver = first_line_version(run_command_capture(&["gh-issue-sync", "--version"]));
        println!("ok: gh-issue-sync available ({})", ver);
        true
    } else {
        let (has_gh, supported_platform) = issue_sync_install_methods_available();
        println!("warn: gh-issue-sync not found");
        println!(
            "info: auto-install capability: gh={} verified-platform={}",
            has_gh, supported_platform
        );
        if !has_gh || !supported_platform {
            println!(
                "info: install manually from https://github.com/{}/releases/tag/{}",
                GH_ISSUE_SYNC_REPO, GH_ISSUE_SYNC_RELEASE_TAG
            );
        } else {
            println!("info: will auto-install on next init/update");
        }
        false
    };

    if gh_available && issue_sync_available {
        print_issue_sync_doctor_status(&cwd);
    }

    println!(
        "ok: manifest v{} with {} shipped skills",
        manifest.version,
        manifest.skills.len()
    );
    print_schema_doctor_report(&cwd, inspect_schema_directory(&cwd)?);

    Ok(())
}

fn plan_schema_sync(base_path: &Path) -> Result<SchemaSyncPlan> {
    let schema_dir = base_path.join(SCHEMA_DIR);
    let artifacts_dir = base_path.join(ARTIFACTS_DIR);

    if schema_dir.exists() && !schema_dir.is_dir() {
        bail!("error: .groundwork/schemas exists but is not a directory");
    }
    if artifacts_dir.exists() && !artifacts_dir.is_dir() {
        bail!("error: .groundwork/artifacts exists but is not a directory");
    }

    let create_schema_dir = !schema_dir.exists();
    let create_artifacts_dir = !artifacts_dir.exists();
    let mut files = Vec::with_capacity(EMBEDDED_SCHEMAS.len());

    for schema in EMBEDDED_SCHEMAS {
        let target = schema_dir.join(schema.filename);
        let action = if !target.exists() {
            SchemaFileAction::Create
        } else {
            let existing = fs::read(&target)
                .with_context(|| format!("failed to read {}", target.display()))?;
            if existing == schema.content.as_bytes() {
                SchemaFileAction::Unchanged
            } else {
                SchemaFileAction::Update
            }
        };
        files.push(SchemaFilePlan {
            filename: schema.filename,
            action,
        });
    }

    Ok(SchemaSyncPlan {
        create_schema_dir,
        create_artifacts_dir,
        files,
    })
}

fn apply_schema_sync(base_path: &Path, plan: &SchemaSyncPlan) -> Result<()> {
    let schema_dir = base_path.join(SCHEMA_DIR);
    let artifacts_dir = base_path.join(ARTIFACTS_DIR);

    if plan.create_schema_dir {
        fs::create_dir_all(&schema_dir)
            .with_context(|| format!("failed to create {}", schema_dir.display()))?;
    }
    if plan.create_artifacts_dir {
        fs::create_dir_all(&artifacts_dir)
            .with_context(|| format!("failed to create {}", artifacts_dir.display()))?;
    }

    for file in &plan.files {
        match file.action {
            SchemaFileAction::Create | SchemaFileAction::Update => {
                let schema = embedded_schema(file.filename)
                    .ok_or_else(|| anyhow!("embedded schema `{}` not found", file.filename))?;
                let target = schema_dir.join(file.filename);
                fs::write(&target, schema.content)
                    .with_context(|| format!("failed to write {}", target.display()))?;
            }
            SchemaFileAction::Unchanged => {}
        }
    }

    Ok(())
}

fn embedded_schema(filename: &str) -> Option<&'static EmbeddedSchema> {
    EMBEDDED_SCHEMAS.iter().find(|s| s.filename == filename)
}

fn inspect_schema_directory(base_path: &Path) -> Result<SchemaDoctorReport> {
    let schema_dir = base_path.join(SCHEMA_DIR);
    if !schema_dir.exists() {
        return Ok(SchemaDoctorReport {
            schema_dir_exists: false,
            schema_dir_is_directory: false,
            missing_files: EMBEDDED_SCHEMAS.iter().map(|s| s.filename).collect(),
            mismatched_files: vec![],
            unreadable_files: vec![],
            extra_files: vec![],
        });
    }
    if !schema_dir.is_dir() {
        return Ok(SchemaDoctorReport {
            schema_dir_exists: true,
            schema_dir_is_directory: false,
            missing_files: EMBEDDED_SCHEMAS.iter().map(|s| s.filename).collect(),
            mismatched_files: vec![],
            unreadable_files: vec![],
            extra_files: vec![],
        });
    }

    let mut installed = BTreeSet::new();
    for entry in fs::read_dir(&schema_dir)
        .with_context(|| format!("failed to read {}", schema_dir.display()))?
    {
        let entry = entry.with_context(|| format!("failed to read {}", schema_dir.display()))?;
        let path = entry.path();
        if path.is_file() {
            installed.insert(entry.file_name().to_string_lossy().to_string());
        }
    }

    let expected: BTreeSet<&'static str> = EMBEDDED_SCHEMAS.iter().map(|s| s.filename).collect();
    let mut missing_files = Vec::new();
    let mut mismatched_files = Vec::new();
    let mut unreadable_files = Vec::new();

    for schema in EMBEDDED_SCHEMAS {
        let target = schema_dir.join(schema.filename);
        if !target.exists() {
            missing_files.push(schema.filename);
            continue;
        }
        match fs::read(&target) {
            Ok(installed_bytes) => {
                if std::str::from_utf8(&installed_bytes).is_err() {
                    unreadable_files.push(schema.filename);
                } else if installed_bytes != schema.content.as_bytes() {
                    mismatched_files.push(schema.filename);
                }
            }
            Err(_) => unreadable_files.push(schema.filename),
        }
    }

    let extra_files = installed
        .into_iter()
        .filter(|name| !expected.contains(name.as_str()))
        .collect();

    Ok(SchemaDoctorReport {
        schema_dir_exists: true,
        schema_dir_is_directory: true,
        missing_files,
        mismatched_files,
        unreadable_files,
        extra_files,
    })
}

fn print_schema_sync_plan_summary(plan: &SchemaSyncPlan, dry_run: bool) {
    if dry_run {
        if plan.create_schema_dir {
            println!("dry-run: would create {SCHEMA_DIR}");
        }
        if plan.create_artifacts_dir {
            println!("dry-run: would create {ARTIFACTS_DIR}");
        }
    } else {
        if plan.create_schema_dir {
            println!("created {SCHEMA_DIR}");
        }
        if plan.create_artifacts_dir {
            println!("created {ARTIFACTS_DIR}");
        }
    }
}

fn print_schema_doctor_report(base_path: &Path, report: SchemaDoctorReport) {
    let schema_dir = base_path.join(SCHEMA_DIR);
    if !report.schema_dir_exists {
        println!(
            "warn: {} not found (will be created by init)",
            schema_dir.display()
        );
        return;
    }
    if !report.schema_dir_is_directory {
        println!(
            "warn: {} exists but is not a directory",
            schema_dir.display()
        );
        return;
    }

    println!("ok: {} exists", schema_dir.display());
    if report.missing_files.is_empty()
        && report.mismatched_files.is_empty()
        && report.unreadable_files.is_empty()
    {
        println!(
            "ok: managed schemas present and up to date ({})",
            EMBEDDED_SCHEMAS.len()
        );
    } else {
        if !report.missing_files.is_empty() {
            println!(
                "warn: .groundwork/schemas/ missing {} files: {}",
                report.missing_files.len(),
                report.missing_files.join(", ")
            );
        }
        if !report.mismatched_files.is_empty() {
            println!(
                "warn: .groundwork/schemas/ {} files differ from embedded versions: {}",
                report.mismatched_files.len(),
                report.mismatched_files.join(", ")
            );
        }
        if !report.unreadable_files.is_empty() {
            println!(
                "warn: .groundwork/schemas/ {} files are unreadable (schema drifted): {}",
                report.unreadable_files.len(),
                report.unreadable_files.join(", ")
            );
        }
    }

    if report.extra_files.is_empty() {
        println!("ok: no extra schema files detected");
    } else {
        println!(
            "info: .groundwork/schemas/ has {} extra files (left unchanged): {}",
            report.extra_files.len(),
            report.extra_files.join(", ")
        );
    }
}

fn read_manifest() -> Result<SkillsManifest> {
    toml::from_str::<SkillsManifest>(SHIPPED_SKILLS_TOML)
        .context("failed to parse embedded shipped skills manifest")
}

fn validate_manifest(manifest: &SkillsManifest) -> Result<()> {
    if manifest.version.trim().is_empty() {
        bail!("manifest version is empty");
    }

    if manifest.skills.is_empty() {
        bail!("manifest has no shipped skills");
    }

    let mut names = HashSet::new();
    let mut paths = HashSet::new();
    for skill in &manifest.skills {
        if skill.name.trim().is_empty() {
            bail!("manifest has skill with empty name");
        }
        if !names.insert(skill.name.clone()) {
            bail!("duplicate skill name in manifest: {}", skill.name);
        }
        if skill.path.trim().is_empty() {
            bail!("skill `{}` missing path", skill.name);
        }
        if !paths.insert(skill.path.clone()) {
            bail!("duplicate skill path in manifest: {}", skill.path);
        }
        if skill.provider != "gh" && skill.provider != "git" {
            bail!(
                "unsupported source type `{}` in `{}`",
                skill.provider,
                skill.name
            );
        }
        if skill.repo.trim().is_empty() {
            bail!("skill `{}` missing repo", skill.name);
        }
        if skill.rev.is_some() && skill.tag.is_some() {
            bail!("skill `{}` cannot pin both `rev` and `tag`", skill.name);
        }
        if skill.repo != GROUNDWORK_REPO && skill.rev.is_none() && skill.tag.is_none() {
            bail!("skill `{}` must pin either `rev` or `tag`", skill.name);
        }
        if skill.use_when.trim().is_empty() {
            bail!("skill `{}` missing use_when", skill.name);
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

fn build_managed_specs(manifest: &SkillsManifest) -> Vec<ManagedDependencySpec> {
    let mut specs = Vec::new();

    for skill in &manifest.skills {
        let pin = skill
            .rev
            .as_ref()
            .map(|v| PinRef {
                key: "rev".to_string(),
                value: v.clone(),
            })
            .or_else(|| {
                skill.tag.as_ref().map(|v| PinRef {
                    key: "tag".to_string(),
                    value: v.clone(),
                })
            });

        specs.push(ManagedDependencySpec {
            alias: managed_alias(&skill.name),
            origin: if skill.repo == GROUNDWORK_REPO {
                "local".to_string()
            } else {
                "upstream".to_string()
            },
            source_key: skill.provider.clone(),
            repo: skill.repo.clone(),
            dependency_path: skill.path.clone(),
            skill: Some(skill.name.clone()),
            pin,
        });
    }

    specs
}

fn manifest_position_map(manifest: &SkillsManifest) -> std::collections::HashMap<String, usize> {
    manifest
        .skills
        .iter()
        .enumerate()
        .map(|(index, skill)| (managed_alias(&skill.name), index))
        .collect()
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
        let mut runner = SkRunner {
            mode: SkMode::Binary,
            binary_path: None,
        };

        if !runner.supports_skill_target_option() {
            println!(
                "warn: existing sk lacks --skill-target support; installing forked sk from {}",
                SKILLS_SUPPLY_FORK_URL
            );
            let installed_binary = install_sk_from_fork()?;
            runner = SkRunner {
                mode: SkMode::Binary,
                binary_path: Some(installed_binary),
            };
            if !runner.supports_skill_target_option() {
                bail!(
                    "installed sk is missing required `--skill-target` sync option; install forked sk from {} and rerun",
                    SKILLS_SUPPLY_FORK_URL
                );
            }
        }
        return Ok(runner);
    }

    if !command_exists("node") {
        bail!("sk not found and Node.js is unavailable; install Node.js or install sk manually");
    }

    if !command_exists("npm") {
        bail!("sk not found and npm is unavailable; install npm or install sk manually");
    }

    if !command_exists("git") {
        bail!("sk not found and git is unavailable; install git or install sk manually");
    }

    let installed_binary = install_sk_from_fork()?;
    let runner = SkRunner {
        mode: SkMode::Binary,
        binary_path: Some(installed_binary),
    };
    if !runner.supports_skill_target_option() {
        bail!(
            "installed sk is missing required `--skill-target` sync option; install forked sk from {} and rerun",
            SKILLS_SUPPLY_FORK_URL
        );
    }
    Ok(runner)
}

fn install_sk_from_fork() -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir().join(format!(
        "groundwork-sk-bootstrap-{}-{}",
        std::process::id(),
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ));

    let clone = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "--branch",
            SKILLS_SUPPLY_FORK_REF,
            SKILLS_SUPPLY_FORK_URL,
        ])
        .arg(&temp_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("failed to clone sk fork repository")?;

    if !clone.success() {
        let _ = fs::remove_dir_all(&temp_dir);
        bail!("failed to clone sk fork from {}", SKILLS_SUPPLY_FORK_URL);
    }

    let head = run_command_capture_in_dir(&["git", "rev-parse", "HEAD"], &temp_dir)
        .context("failed to read cloned sk fork commit")?;
    if let Err(err) = check_trusted_commit(head.trim(), SKILLS_SUPPLY_FORK_COMMIT) {
        let _ = fs::remove_dir_all(&temp_dir);
        return Err(err).context(format!(
            "refusing to bootstrap sk from {} {}",
            SKILLS_SUPPLY_FORK_URL, SKILLS_SUPPLY_FORK_REF
        ));
    }

    let deps = Command::new("npm")
        .args([
            "install",
            "--workspace",
            "packages/core",
            "--workspace",
            "packages/sk",
        ])
        .current_dir(&temp_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("failed to install workspace dependencies for sk fork build")?;
    if !deps.success() {
        let _ = fs::remove_dir_all(&temp_dir);
        bail!(
            "failed to install dependencies while bootstrapping sk from {}",
            SKILLS_SUPPLY_FORK_URL
        );
    }

    let build_core = Command::new("npm")
        .args(["--workspace", "packages/core", "run", "build"])
        .current_dir(&temp_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("failed to build @skills-supply/core while bootstrapping sk")?;
    if !build_core.success() {
        let _ = fs::remove_dir_all(&temp_dir);
        bail!(
            "failed to build @skills-supply/core while bootstrapping sk from {}",
            SKILLS_SUPPLY_FORK_URL
        );
    }

    let build_sk = Command::new("npm")
        .args(["--workspace", "packages/sk", "run", "build:node"])
        .current_dir(&temp_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("failed to build sk CLI while bootstrapping")?;
    if !build_sk.success() {
        let _ = fs::remove_dir_all(&temp_dir);
        bail!(
            "failed to build sk CLI while bootstrapping from {}",
            SKILLS_SUPPLY_FORK_URL
        );
    }

    let core_package_path = temp_dir.join("packages/core");
    let core_pack = Command::new("npm")
        .arg("pack")
        .current_dir(&core_package_path)
        .output()
        .context("failed to pack core npm tarball")?;
    if !core_pack.status.success() {
        let _ = fs::remove_dir_all(&temp_dir);
        bail!(
            "failed to package core tarball while bootstrapping from {}: {}",
            SKILLS_SUPPLY_FORK_URL,
            String::from_utf8_lossy(&core_pack.stderr)
        );
    }
    let core_packed = String::from_utf8_lossy(&core_pack.stdout);
    let core_tar_name = core_packed
        .lines()
        .rev()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .ok_or_else(|| anyhow!("npm pack returned no core tarball filename"))?;
    let core_tarball_path = core_package_path.join(core_tar_name);

    let sk_package_path = temp_dir.join("packages/sk");
    let sk_pack = Command::new("npm")
        .arg("pack")
        .current_dir(&sk_package_path)
        .output()
        .context("failed to pack sk npm tarball")?;
    if !sk_pack.status.success() {
        let _ = fs::remove_dir_all(&temp_dir);
        bail!(
            "failed to package sk tarball while bootstrapping from {}: {}",
            SKILLS_SUPPLY_FORK_URL,
            String::from_utf8_lossy(&sk_pack.stderr)
        );
    }
    let sk_packed = String::from_utf8_lossy(&sk_pack.stdout);
    let sk_tar_name = sk_packed
        .lines()
        .rev()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .ok_or_else(|| anyhow!("npm pack returned no sk tarball filename"))?;
    let sk_tarball_path = sk_package_path.join(sk_tar_name);

    let install = Command::new("npm")
        .args(["install", "-g"])
        .arg(&core_tarball_path)
        .arg(&sk_tarball_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| {
            format!(
                "failed to install sk/core from tarballs {} and {}",
                core_tarball_path.display(),
                sk_tarball_path.display()
            )
        })?;

    let _ = fs::remove_dir_all(&temp_dir);
    if !install.success() {
        bail!("failed to install sk from {}", SKILLS_SUPPLY_FORK_URL);
    }

    let npm_prefix = run_command_capture(&["npm", "prefix", "-g"])
        .context("failed to resolve npm global prefix")?;
    let installed_sk = PathBuf::from(npm_prefix).join("bin").join("sk");
    if !installed_sk.exists() {
        bail!(
            "sk installed from fork but binary not found at {}",
            installed_sk.display()
        );
    }

    Ok(installed_sk)
}

impl SkRunner {
    fn sk_bin(&self) -> Result<String> {
        match self.mode {
            SkMode::Binary => {
                let bin = self
                    .binary_path
                    .as_deref()
                    .unwrap_or_else(|| Path::new("sk"));
                Ok(bin.to_string_lossy().to_string())
            }
            SkMode::Npx => bail!(
                "npx mode is no longer supported; install sk binary from {}",
                SKILLS_SUPPLY_FORK_URL
            ),
        }
    }

    fn sync(&self) -> Result<()> {
        let sk = self.sk_bin()?;
        let status = Command::new(&sk)
            .args(["sync", "--skill-target", "name"])
            .status()
            .context("failed to run sk sync")?;

        if !status.success() {
            bail!("sk sync failed with status {}", status);
        }

        Ok(())
    }

    fn version(&self) -> Result<String> {
        let sk = self.sk_bin()?;
        run_command_capture(&[sk.as_str(), "--version"])
    }

    fn mode_name(&self) -> &'static str {
        match self.mode {
            SkMode::Binary => "sk",
            SkMode::Npx => "npx @skills-supply/sk",
        }
    }

    fn supports_skill_target_option(&self) -> bool {
        let sk = match self.sk_bin() {
            Ok(s) => s,
            Err(_) => return false,
        };
        let help = run_command_capture_ignore_status(&[sk.as_str(), "sync", "--help"]);
        supports_skill_target_option(&help)
    }
}

fn supports_skill_target_option(help: &str) -> bool {
    help.lines()
        .any(|line| line.trim_start().starts_with("--skill-target"))
}

fn issue_sync_install_methods_available() -> (bool, bool) {
    (
        command_exists("gh"),
        issue_sync_release_asset_for(std::env::consts::OS, std::env::consts::ARCH).is_some(),
    )
}

fn ensure_issue_sync_available() -> Option<IssueSyncMode> {
    if command_exists("gh-issue-sync") {
        return Some(IssueSyncMode::Binary);
    }

    if !command_exists("gh") {
        println!("warn: gh CLI not found (prerequisite for gh-issue-sync)");
        return None;
    }

    if issue_sync_release_asset_for(std::env::consts::OS, std::env::consts::ARCH).is_none() {
        println!(
            "note: no verified gh-issue-sync asset is configured for platform {}-{}; install manually from https://github.com/{}/releases/tag/{}",
            std::env::consts::OS,
            std::env::consts::ARCH,
            GH_ISSUE_SYNC_REPO,
            GH_ISSUE_SYNC_RELEASE_TAG
        );
        return None;
    }

    println!(
        "info: installing gh-issue-sync from verified release asset ({} {})...",
        GH_ISSUE_SYNC_REPO, GH_ISSUE_SYNC_RELEASE_TAG
    );
    match install_issue_sync_verified_release() {
        Ok(path) => {
            println!("ok: installed gh-issue-sync to {}", path.display());
            Some(IssueSyncMode::Binary)
        }
        Err(err) => {
            println!("warn: verified gh-issue-sync auto-install failed: {}", err);
            println!(
                "note: install manually from https://github.com/{}/releases/tag/{}",
                GH_ISSUE_SYNC_REPO, GH_ISSUE_SYNC_RELEASE_TAG
            );
            None
        }
    }
}

fn issue_sync_release_asset_for(
    target_os: &str,
    target_arch: &str,
) -> Option<IssueSyncReleaseAsset> {
    match (target_os, target_arch) {
        ("linux", "x86_64") => Some(IssueSyncReleaseAsset {
            name: "gh-issue-sync-linux-amd64",
            sha256: "095749599f9d6c81d91765bd16b14d5c76215fd1d3ef6bccf5d005efb2d9b84e",
        }),
        ("linux", "aarch64") => Some(IssueSyncReleaseAsset {
            name: "gh-issue-sync-linux-arm64",
            sha256: "ca86c002640426b2282cc947a5b34da3e51c91e62423c3996085587110704785",
        }),
        ("macos", "aarch64") => Some(IssueSyncReleaseAsset {
            name: "gh-issue-sync-darwin-arm64",
            sha256: "928d8a9ca9d9a8588b72ef38f22f9e067f9619d12be6cf28a1bef41ff04713e7",
        }),
        ("windows", "x86_64") => Some(IssueSyncReleaseAsset {
            name: "gh-issue-sync-windows-amd64.exe",
            sha256: "f8b1f1a0a1bb53c8c378848ac1423f7ff7660c141134ce2ee4fcd62b008840ac",
        }),
        ("windows", "aarch64") => Some(IssueSyncReleaseAsset {
            name: "gh-issue-sync-windows-arm64.exe",
            sha256: "e6a4cb05c91f6e283ab2743cb3961c0ccd62d2fc85008019f8ca8c9674e92ba5",
        }),
        _ => None,
    }
}

fn install_issue_sync_verified_release() -> Result<PathBuf> {
    let asset = issue_sync_release_asset_for(std::env::consts::OS, std::env::consts::ARCH)
        .ok_or_else(|| {
            anyhow!(
                "unsupported platform {}-{}",
                std::env::consts::OS,
                std::env::consts::ARCH
            )
        })?;

    let temp_path = std::env::temp_dir().join(format!(
        "{}-{}-{}",
        asset.name,
        std::process::id(),
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ));

    let download = Command::new("gh")
        .args([
            "release",
            "download",
            GH_ISSUE_SYNC_RELEASE_TAG,
            "--repo",
            GH_ISSUE_SYNC_REPO,
            "--pattern",
            asset.name,
            "--output",
        ])
        .arg(&temp_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("failed to download verified gh-issue-sync release asset")?;
    if !download.success() {
        let _ = fs::remove_file(&temp_path);
        bail!("`gh release download` did not succeed");
    }

    let digest = sha256_file_hex(&temp_path)?;
    if !digest.eq_ignore_ascii_case(asset.sha256) {
        let _ = fs::remove_file(&temp_path);
        bail!(
            "checksum mismatch for {}: expected {}, got {}",
            asset.name,
            asset.sha256,
            digest
        );
    }

    let install_dir = issue_sync_install_dir()?;
    fs::create_dir_all(&install_dir)
        .with_context(|| format!("failed to create install dir {}", install_dir.display()))?;
    let binary_name = if cfg!(windows) {
        "gh-issue-sync.exe"
    } else {
        "gh-issue-sync"
    };
    let target = install_dir.join(binary_name);
    fs::copy(&temp_path, &target)
        .with_context(|| format!("failed to install gh-issue-sync to {}", target.display()))?;
    let _ = fs::remove_file(&temp_path);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&target)
            .with_context(|| format!("failed to read {}", target.display()))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&target, perms)
            .with_context(|| format!("failed to set executable bit on {}", target.display()))?;
    }

    prepend_to_path(&install_dir);
    Ok(target)
}

fn issue_sync_install_dir() -> Result<PathBuf> {
    if cfg!(windows) {
        let local =
            std::env::var_os("LOCALAPPDATA").ok_or_else(|| anyhow!("LOCALAPPDATA is not set"))?;
        return Ok(PathBuf::from(local).join("groundwork").join("bin"));
    }

    let home = std::env::var_os("HOME").ok_or_else(|| anyhow!("HOME is not set"))?;
    Ok(PathBuf::from(home).join(".local").join("bin"))
}

fn prepend_to_path(dir: &Path) {
    let mut paths = vec![dir.to_path_buf()];
    if let Some(existing) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&existing));
    }
    if let Ok(joined) = std::env::join_paths(paths) {
        std::env::set_var("PATH", joined);
    }
}

fn sha256_file_hex(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    let mut file = fs::File::open(path)
        .with_context(|| format!("failed to open {} for checksum", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file
            .read(&mut buf)
            .with_context(|| format!("failed to read {} for checksum", path.display()))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn check_trusted_commit(actual: &str, expected: &str) -> Result<()> {
    if actual == expected {
        return Ok(());
    }
    bail!(
        "trusted commit mismatch: expected {}, got {}",
        expected,
        actual
    );
}

fn bootstrap_issue_sync(
    base_path: &Path,
    is_init: bool,
) -> Result<Option<(IssueSyncMode, String)>> {
    let mode = match ensure_issue_sync_available() {
        Some(mode) => mode,
        None if is_init => {
            bail!(
                "gh-issue-sync is required for `groundwork init` to complete operational sync; install it and rerun init"
            )
        }
        None => return Ok(None),
    };

    let version = capture_required_version(
        "gh-issue-sync",
        run_command_capture(&["gh-issue-sync", "--version"]),
    )?;

    let issues_dir = base_path.join(".issues");
    let mut should_pull = false;
    if !issues_dir.exists() {
        let init_status = Command::new("gh-issue-sync")
            .arg("init")
            .current_dir(base_path)
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .status();

        if !init_status.map(|s| s.success()).unwrap_or(false) {
            if is_init {
                bail!("gh-issue-sync init failed during `groundwork init`");
            }
            println!("warn: gh-issue-sync init failed");
            return Ok(Some((mode, version)));
        }
        should_pull = true;
    }

    if !should_pull
        && issue_sync_status_is_never_pulled(&run_command_capture_ignore_status_in_dir(
            &["gh-issue-sync", "status"],
            base_path,
        ))
    {
        should_pull = true;
    }

    if should_pull {
        let pull_output = Command::new("gh-issue-sync")
            .arg("pull")
            .current_dir(base_path)
            .output();

        match pull_output {
            Ok(output) if output.status.success() => {
                let count = count_issue_markdown_files(&issues_dir);
                println!("ok: synced {} issues to .issues/", count);
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                if is_init {
                    let mut detail = String::new();
                    if !stderr.is_empty() {
                        detail.push_str(first_line(&stderr));
                    } else {
                        detail.push_str("unknown error");
                    }
                    if let Some(hint) = issue_sync_pull_remediation_hint(&stderr) {
                        detail.push_str(&format!("; {}", hint));
                    }
                    detail.push_str("; run `gh-issue-sync status` after remediation");
                    bail!(
                        "gh-issue-sync pull failed during `groundwork init`: {}",
                        detail
                    );
                }
                println!("warn: gh-issue-sync pull failed");
                if !stderr.is_empty() {
                    println!("info: {}", first_line(&stderr));
                }
                if let Some(hint) = issue_sync_pull_remediation_hint(&stderr) {
                    println!("info: {}", hint);
                }
                println!("info: run `gh-issue-sync status` to verify mirror state");
            }
            Err(err) => {
                if is_init {
                    bail!(
                        "gh-issue-sync pull failed during `groundwork init`: {}; run `gh-issue-sync status` after remediation",
                        err
                    );
                }
                println!("warn: gh-issue-sync pull failed");
                println!("info: {}", err);
                println!("info: run `gh-issue-sync status` to verify mirror state");
            }
        }
    }

    if is_init {
        let output = Command::new("gh-issue-sync")
            .arg("status")
            .current_dir(base_path)
            .output()
            .context("failed to run `gh-issue-sync status` after pull")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let detail = if stderr.is_empty() {
                "unknown error".to_string()
            } else {
                first_line(&stderr).to_string()
            };
            bail!(
                "gh-issue-sync status failed during `groundwork init`: {}; run `gh auth refresh -h github.com -s read:project`, then `gh-issue-sync pull` and `gh-issue-sync status`",
                detail
            );
        }

        let status = String::from_utf8_lossy(&output.stdout);
        if !issue_sync_status_has_completed_pull(&status) {
            bail!(
                "issue sync is not operational: `Last full pull` is missing or `never`; run `gh auth refresh -h github.com -s read:project`, then `gh-issue-sync pull` and `gh-issue-sync status`"
            );
        }
    }

    Ok(Some((mode, version)))
}

fn print_issue_sync_doctor_status(base_path: &Path) {
    let output = Command::new("gh-issue-sync")
        .arg("status")
        .current_dir(base_path)
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if issue_sync_status_is_never_pulled(&stdout) {
                println!("warn: local issue mirror has never completed a full pull");
                println!("info: run `gh auth refresh -h github.com -s read:project`");
                println!("info: then run `gh-issue-sync pull` and `gh-issue-sync status`");
            }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            println!("warn: unable to read issue mirror status");
            if !stderr.trim().is_empty() {
                println!("info: {}", first_line(stderr.trim()));
            }
            println!("info: run `gh-issue-sync status` manually for details");
        }
        Err(err) => {
            println!("warn: unable to run `gh-issue-sync status`: {}", err);
        }
    }
}

fn run_command_capture_ignore_status_in_dir(args: &[&str], cwd: &Path) -> String {
    let (program, rest) = match args.split_first() {
        Some(pair) => pair,
        None => return String::new(),
    };
    Command::new(program)
        .args(rest)
        .current_dir(cwd)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

fn issue_sync_status_is_never_pulled(status: &str) -> bool {
    status.lines().any(|line| {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("Last full pull:") {
            return rest.trim().eq_ignore_ascii_case("never");
        }
        false
    })
}

fn issue_sync_status_has_completed_pull(status: &str) -> bool {
    status.lines().any(|line| {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("Last full pull:") {
            let value = rest.trim();
            return !value.is_empty() && !value.eq_ignore_ascii_case("never");
        }
        false
    })
}

fn issue_sync_pull_remediation_hint(stderr: &str) -> Option<&'static str> {
    let lower = stderr.to_ascii_lowercase();
    if lower.contains("required scopes")
        && (lower.contains("read:project") || lower.contains("project"))
    {
        return Some("refresh GH auth scopes: `gh auth refresh -h github.com -s read:project`");
    }
    if lower.contains("token") && lower.contains("invalid") {
        return Some("re-authenticate GH CLI: `gh auth login -h github.com`");
    }
    if lower.contains("error connecting") || lower.contains("api.github.com") {
        return Some("check connectivity and GitHub status, then retry `gh-issue-sync pull`");
    }
    None
}

fn count_issue_markdown_files(issues_dir: &Path) -> usize {
    ["open", "closed"]
        .iter()
        .filter_map(|dir| issues_dir.join(dir).read_dir().ok())
        .flat_map(|entries| entries.filter_map(|e| e.ok()))
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|ext| ext == "md")
                .unwrap_or(false)
        })
        .count()
}

fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or(s)
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

fn run_command_capture_in_dir(args: &[&str], cwd: &Path) -> Result<String> {
    let (program, rest) = args
        .split_first()
        .ok_or_else(|| anyhow!("empty command invocation"))?;
    let output = Command::new(program)
        .args(rest)
        .current_dir(cwd)
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

/// Capture stdout regardless of exit code (useful for `--help` which may return non-zero).
fn run_command_capture_ignore_status(args: &[&str]) -> String {
    let (program, rest) = match args.split_first() {
        Some(pair) => pair,
        None => return String::new(),
    };
    Command::new(program)
        .args(rest)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

fn first_line_version(result: Result<String>) -> String {
    result
        .map(|v| v.lines().next().unwrap_or("unknown").to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

fn parse_version_line(raw: &str) -> Option<String> {
    raw.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToOwned::to_owned)
}

fn capture_required_version(tool: &str, result: Result<String>) -> Result<String> {
    let raw = result.with_context(|| format!("failed to capture {} version", tool))?;
    parse_version_line(&raw).ok_or_else(|| {
        anyhow!(
            "{} version output is empty; refusing to record unverified provenance",
            tool
        )
    })
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
    issue_sync_info: Option<(IssueSyncMode, &str)>,
    entries: Vec<LockEntry>,
) -> Result<()> {
    let lock = InstallLock {
        version: 1,
        installer_version: env!("CARGO_PKG_VERSION").to_string(),
        installed_at: Utc::now().to_rfc3339(),
        sk: LockSk {
            mode: sk_mode,
            version: sk_version.to_string(),
        },
        issue_sync: issue_sync_info.map(|(mode, ver)| LockIssueSync {
            mode,
            version: ver.to_string(),
        }),
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

fn print_manifest_summary(manifest: &SkillsManifest) {
    println!("Shipped skills:");
    for skill in &manifest.skills {
        let pin = skill
            .rev
            .as_deref()
            .or(skill.tag.as_deref())
            .unwrap_or("unpinned");
        println!(
            "- {} ({}, {} path={}, ref={})",
            skill.name, skill.repo, skill.provider, skill.path, pin
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn parse_doc(input: &str) -> DocumentMut {
        input.parse::<DocumentMut>().expect("valid toml")
    }

    fn sample_manifest() -> SkillsManifest {
        SkillsManifest {
            version: "1".to_string(),
            skills: vec![
                ShippedSkill {
                    name: "ground".to_string(),
                    path: "skills/ground".to_string(),
                    provider: "gh".to_string(),
                    repo: GROUNDWORK_REPO.to_string(),
                    rev: None,
                    tag: None,
                    use_when: "before creating designs".to_string(),
                },
                ShippedSkill {
                    name: "test-first".to_string(),
                    path: "skills/test-first".to_string(),
                    provider: "gh".to_string(),
                    repo: GROUNDWORK_REPO.to_string(),
                    rev: None,
                    tag: None,
                    use_when: "when implementing".to_string(),
                },
            ],
        }
    }

    #[test]
    fn manifest_validation_rejects_duplicate_skill_names() {
        let manifest = SkillsManifest {
            version: "1".to_string(),
            skills: vec![
                ShippedSkill {
                    name: "ground".to_string(),
                    path: "skills/ground".to_string(),
                    provider: "gh".to_string(),
                    repo: GROUNDWORK_REPO.to_string(),
                    rev: None,
                    tag: None,
                    use_when: "before designing".to_string(),
                },
                ShippedSkill {
                    name: "ground".to_string(),
                    path: "skills/ground-2".to_string(),
                    provider: "gh".to_string(),
                    repo: GROUNDWORK_REPO.to_string(),
                    rev: None,
                    tag: None,
                    use_when: "duplicate".to_string(),
                },
            ],
        };

        let err = validate_manifest(&manifest).expect_err("expected duplicate-name error");
        assert_eq!(err.to_string(), "duplicate skill name in manifest: ground");
    }

    #[test]
    fn managed_specs_follow_shipped_skill_order_and_flat_paths() {
        let manifest = read_manifest().expect("embedded shipped-skill manifest parses");
        let specs = build_managed_specs(&manifest);

        let aliases: Vec<&str> = specs.iter().map(|spec| spec.alias.as_str()).collect();
        assert_eq!(
            aliases,
            vec![
                "using_groundwork",
                "ground",
                "research",
                "bdd",
                "plan",
                "issue_craft",
                "begin",
                "test_first",
                "subagent_driven_development",
                "systematic_debugging",
                "requesting_code_review",
                "receiving_code_review",
                "third_force",
                "documentation",
                "verification_before_completion",
                "propose",
                "land",
            ]
        );

        let local_paths: Vec<&str> = specs
            .iter()
            .filter(|spec| spec.repo == GROUNDWORK_REPO)
            .map(|spec| spec.dependency_path.as_str())
            .collect();
        assert_eq!(
            local_paths,
            vec![
                "skills/using-groundwork",
                "skills/ground",
                "skills/research",
                "skills/bdd",
                "skills/plan",
                "skills/issue-craft",
                "skills/begin",
                "skills/test-first",
                "skills/third-force",
                "skills/documentation",
                "skills/propose",
                "skills/land",
            ]
        );
    }

    #[test]
    fn update_prunes_stale_aliases_and_preserves_external() {
        let mut doc = parse_doc(
            r#"[agents]
claude-code = true

[dependencies]
old_bdd = { gh = "pentaxis93/groundwork", path = "skills/bdd" }
old_ground = { gh = "pentaxis93/groundwork", path = "skills/ground" }
external_dep = { gh = "org/repo", path = "y" }
"#,
        );

        let previously_managed: HashSet<String> = ["old_bdd", "old_ground"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let specs = build_managed_specs(&read_manifest().expect("embedded manifest"));
        let result =
            reconcile_manifest_to_doc(&mut doc, &specs, &previously_managed).expect("reconcile ok");

        let deps = doc["dependencies"].as_table().expect("deps table");
        assert!(!deps.contains_key("old_bdd"));
        assert!(!deps.contains_key("old_ground"));
        assert!(deps.contains_key("external_dep"));
        assert!(deps.contains_key("ground"));
        assert!(deps.contains_key("research"));
        assert!(deps.contains_key("bdd"));
        assert!(deps.contains_key("begin"));
        assert!(deps.contains_key("plan"));
        assert!(deps.contains_key("issue_craft"));
        assert!(deps.contains_key("land"));
        assert!(deps.contains_key("using_groundwork"));
        assert!(deps.contains_key("test_first"));
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

        let specs = build_managed_specs(&sample_manifest());
        let previously_managed = HashSet::new();
        let result =
            reconcile_manifest_to_doc(&mut doc, &specs, &previously_managed).expect("reconcile ok");

        let alias = "test_first";
        let deps_str = doc.to_string();
        assert!(deps_str.contains("path = \"skills/test-first\""));

        let lock_entry = result
            .lock_entries
            .iter()
            .find(|e| e.alias == alias)
            .expect("lock entry exists");

        assert_eq!(
            lock_entry.path.as_deref(),
            Some("skills/test-first")
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

        let specs = build_managed_specs(&sample_manifest());
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
        assert_eq!(result.schema_created, EMBEDDED_SCHEMAS.len());
        assert_eq!(result.schema_updated, 0);
        assert_eq!(result.schema_unchanged, 0);
        assert!(!base.join(LOCK_PATH).exists());
        assert!(!base.join(".groundwork").exists());
    }

    #[test]
    fn schema_sync_creates_expected_layout_and_schema_files() {
        let temp = TempDir::new().expect("tempdir");
        let base = temp.path();

        let plan = plan_schema_sync(base).expect("schema sync plan");
        assert!(plan.create_schema_dir);
        assert!(plan.create_artifacts_dir);
        assert_eq!(plan.created_count(), EMBEDDED_SCHEMAS.len());
        assert_eq!(plan.updated_count(), 0);
        assert_eq!(plan.unchanged_count(), 0);

        apply_schema_sync(base, &plan).expect("schema sync apply");

        assert!(base.join(SCHEMA_DIR).is_dir());
        assert!(base.join(ARTIFACTS_DIR).is_dir());
        for schema in EMBEDDED_SCHEMAS {
            let written = fs::read_to_string(base.join(SCHEMA_DIR).join(schema.filename))
                .expect("read written schema");
            assert_eq!(written, schema.content);
        }
    }

    #[test]
    fn schema_sync_preserves_artifacts_and_extra_schema_files() {
        let temp = TempDir::new().expect("tempdir");
        let base = temp.path();
        let schema_dir = base.join(SCHEMA_DIR);
        let artifacts_dir = base.join(ARTIFACTS_DIR);
        fs::create_dir_all(&schema_dir).expect("create schema dir");
        fs::create_dir_all(&artifacts_dir).expect("create artifacts dir");

        let artifact_file = artifacts_dir.join("custom-note.md");
        fs::write(&artifact_file, "keep me").expect("write artifact");

        let first = EMBEDDED_SCHEMAS[0];
        let second = EMBEDDED_SCHEMAS[1];
        fs::write(schema_dir.join(first.filename), first.content).expect("write matching schema");
        fs::write(schema_dir.join(second.filename), "{}\n").expect("write stale schema");
        let extra_schema = schema_dir.join("custom.schema.json");
        fs::write(&extra_schema, "{\"type\":\"object\"}\n").expect("write extra schema");

        let plan = plan_schema_sync(base).expect("schema sync plan");
        assert!(!plan.create_schema_dir);
        assert!(!plan.create_artifacts_dir);
        assert!(plan.updated_count() >= 1);
        assert!(plan.unchanged_count() >= 1);

        apply_schema_sync(base, &plan).expect("schema sync apply");
        assert_eq!(
            fs::read_to_string(&artifact_file).expect("artifact still readable"),
            "keep me"
        );
        assert!(extra_schema.exists());

        let refreshed = fs::read_to_string(schema_dir.join(second.filename))
            .expect("refreshed schema readable");
        assert_eq!(refreshed, second.content);
    }

    #[test]
    fn schema_directory_inspection_reports_missing_mismatched_and_extra_files() {
        let temp = TempDir::new().expect("tempdir");
        let base = temp.path();
        let schema_dir = base.join(SCHEMA_DIR);
        fs::create_dir_all(&schema_dir).expect("create schema dir");

        let matching = EMBEDDED_SCHEMAS[0];
        let mismatched = EMBEDDED_SCHEMAS[1];
        fs::write(schema_dir.join(matching.filename), matching.content).expect("write matching");
        fs::write(schema_dir.join(mismatched.filename), "{\"broken\":true}\n")
            .expect("write mismatched");
        fs::write(schema_dir.join("custom-extra.schema.json"), "{}\n").expect("write extra");

        let report = inspect_schema_directory(base).expect("schema doctor report");
        assert!(report.schema_dir_exists);
        assert!(report.schema_dir_is_directory);
        assert!(
            report.missing_files.contains(&EMBEDDED_SCHEMAS[2].filename),
            "expected at least one managed schema to be reported missing"
        );
        assert!(report.mismatched_files.contains(&mismatched.filename));
        assert!(report
            .extra_files
            .contains(&"custom-extra.schema.json".to_string()));
        assert!(report.unreadable_files.is_empty());
    }

    #[test]
    fn schema_sync_plan_fails_when_schema_path_is_not_directory() {
        let temp = TempDir::new().expect("tempdir");
        let base = temp.path();
        let groundwork_dir = base.join(".groundwork");
        fs::create_dir_all(&groundwork_dir).expect("create groundwork dir");
        fs::write(groundwork_dir.join("schemas"), "not-a-dir").expect("write schemas file");

        let err = plan_schema_sync(base).expect_err("expected error");
        assert_eq!(
            err.to_string(),
            "error: .groundwork/schemas exists but is not a directory"
        );
    }

    #[test]
    fn schema_sync_plan_fails_when_artifacts_path_is_not_directory() {
        let temp = TempDir::new().expect("tempdir");
        let base = temp.path();
        let groundwork_dir = base.join(".groundwork");
        fs::create_dir_all(&groundwork_dir).expect("create groundwork dir");
        fs::create_dir_all(groundwork_dir.join("schemas")).expect("create schemas dir");
        fs::write(groundwork_dir.join("artifacts"), "not-a-dir").expect("write artifacts file");

        let err = plan_schema_sync(base).expect_err("expected error");
        assert_eq!(
            err.to_string(),
            "error: .groundwork/artifacts exists but is not a directory"
        );
    }

    #[test]
    fn schema_sync_treats_non_utf8_content_as_drift_and_overwrites() {
        let temp = TempDir::new().expect("tempdir");
        let base = temp.path();
        let schema_dir = base.join(SCHEMA_DIR);
        let artifacts_dir = base.join(ARTIFACTS_DIR);
        fs::create_dir_all(&schema_dir).expect("create schema dir");
        fs::create_dir_all(&artifacts_dir).expect("create artifacts dir");

        let schema = EMBEDDED_SCHEMAS[0];
        fs::write(schema_dir.join(schema.filename), [0xff, 0xfe, 0xfd]).expect("write non-utf8");

        let plan = plan_schema_sync(base).expect("schema sync plan");
        let action = plan
            .files
            .iter()
            .find(|f| f.filename == schema.filename)
            .expect("schema plan entry exists")
            .action;
        assert_eq!(action, SchemaFileAction::Update);

        apply_schema_sync(base, &plan).expect("schema sync apply");
        let written = fs::read(schema_dir.join(schema.filename)).expect("read healed schema");
        assert_eq!(written, schema.content.as_bytes());
    }

    #[test]
    fn schema_directory_inspection_marks_non_utf8_schema_unreadable() {
        let temp = TempDir::new().expect("tempdir");
        let base = temp.path();
        let schema_dir = base.join(SCHEMA_DIR);
        fs::create_dir_all(&schema_dir).expect("create schema dir");

        let unreadable = EMBEDDED_SCHEMAS[0];
        fs::write(schema_dir.join(unreadable.filename), [0xff, 0xfe]).expect("write non-utf8");

        let report = inspect_schema_directory(base).expect("schema doctor report");
        assert!(report.unreadable_files.contains(&unreadable.filename));
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

    #[test]
    fn lock_round_trips_with_issue_sync() {
        let lock = InstallLock {
            version: 1,
            installer_version: "0.1.0".to_string(),
            installed_at: "2026-01-01T00:00:00Z".to_string(),
            sk: LockSk {
                mode: SkMode::Binary,
                version: "1.0.0".to_string(),
            },
            issue_sync: Some(LockIssueSync {
                mode: IssueSyncMode::Binary,
                version: "0.3.0".to_string(),
            }),
            entries: vec![],
        };

        let toml_str = toml::to_string_pretty(&lock).expect("serialize");
        assert!(toml_str.contains("[issue_sync]"));
        let parsed: InstallLock = toml::from_str(&toml_str).expect("deserialize");
        let is = parsed.issue_sync.expect("issue_sync present");
        assert_eq!(is.mode, IssueSyncMode::Binary);
        assert_eq!(is.version, "0.3.0");
    }

    #[test]
    fn lock_round_trips_go_install_mode() {
        let lock = InstallLock {
            version: 1,
            installer_version: "0.1.0".to_string(),
            installed_at: "2026-01-01T00:00:00Z".to_string(),
            sk: LockSk {
                mode: SkMode::Npx,
                version: "1.0.0".to_string(),
            },
            issue_sync: Some(LockIssueSync {
                mode: IssueSyncMode::GoInstall,
                version: "0.5.0".to_string(),
            }),
            entries: vec![],
        };

        let toml_str = toml::to_string_pretty(&lock).expect("serialize");
        assert!(toml_str.contains("mode = \"go-install\""));
        let parsed: InstallLock = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(parsed.issue_sync.unwrap().mode, IssueSyncMode::GoInstall);
    }

    #[test]
    fn lock_round_trips_without_issue_sync() {
        let lock = InstallLock {
            version: 1,
            installer_version: "0.1.0".to_string(),
            installed_at: "2026-01-01T00:00:00Z".to_string(),
            sk: LockSk {
                mode: SkMode::Npx,
                version: "1.0.0".to_string(),
            },
            issue_sync: None,
            entries: vec![],
        };

        let toml_str = toml::to_string_pretty(&lock).expect("serialize");
        assert!(!toml_str.contains("issue_sync"));
        assert!(toml_str.contains("mode = \"npx\""));
        let parsed: InstallLock = toml::from_str(&toml_str).expect("deserialize");
        assert!(parsed.issue_sync.is_none());
        assert_eq!(parsed.sk.mode, SkMode::Npx);
    }

    #[test]
    fn old_lock_without_issue_sync_deserializes() {
        let old_toml = r#"
version = 1
installer_version = "0.1.0"
installed_at = "2026-01-01T00:00:00Z"

[sk]
mode = "binary"
version = "1.0.0"

[[entries]]
alias = "ground"
origin = "local"
source = "gh"
repo = "pentaxis93/groundwork"
path = "skills/ground"
skill = "ground"
"#;

        let parsed: InstallLock = toml::from_str(old_toml).expect("deserialize old lock");
        assert!(parsed.issue_sync.is_none());
        assert_eq!(parsed.sk.mode, SkMode::Binary);
        assert_eq!(parsed.entries.len(), 1);
    }

    #[test]
    fn skill_target_option_detected_when_present() {
        let help = r#"
Usage: sk sync [options]

Options:
  --dry-run          Plan changes without modifying files
  --skill-target     Choose skill target naming
  --non-interactive  Run without prompts
"#;
        assert!(supports_skill_target_option(help));
    }

    #[test]
    fn skill_target_option_not_detected_when_absent() {
        let help = r#"
Usage: sk sync [options]

Options:
  --dry-run          Plan changes without modifying files
  --non-interactive  Run without prompts
"#;
        assert!(!supports_skill_target_option(help));
    }

    #[test]
    fn issue_sync_status_reports_never_when_line_says_never() {
        let status = r#"
Repository: pentaxis93/groundwork
Last full pull: never

No local changes
"#;
        assert!(issue_sync_status_is_never_pulled(status));
    }

    #[test]
    fn issue_sync_status_not_never_when_timestamp_present() {
        let status = r#"
Repository: pentaxis93/groundwork
Last full pull: 2026-03-06T12:34:56Z

No local changes
"#;
        assert!(!issue_sync_status_is_never_pulled(status));
    }

    #[test]
    fn strict_init_issue_sync_status_complete_when_timestamp_present() {
        let status = r#"
Repository: pentaxis93/groundwork
Last full pull: 2026-03-06T12:34:56Z

No local changes
"#;
        assert!(issue_sync_status_has_completed_pull(status));
    }

    #[test]
    fn strict_init_issue_sync_status_incomplete_when_never() {
        let status = r#"
Repository: pentaxis93/groundwork
Last full pull: never

No local changes
"#;
        assert!(!issue_sync_status_has_completed_pull(status));
    }

    #[test]
    fn strict_init_issue_sync_status_incomplete_when_missing_line() {
        let status = r#"
Repository: pentaxis93/groundwork
No local changes
"#;
        assert!(!issue_sync_status_has_completed_pull(status));
    }

    #[test]
    fn pull_error_hint_detects_missing_project_scope() {
        let stderr = "gh: Your token has not been granted the required scopes to execute this query. The 'title' field requires one of the following scopes: ['read:project'].";
        let hint = issue_sync_pull_remediation_hint(stderr).expect("scope hint");
        assert!(hint.contains("gh auth refresh -h github.com -s read:project"));
    }

    #[test]
    fn pull_error_hint_detects_invalid_token() {
        let stderr = "The token in /home/user/.config/gh/hosts.yml is invalid.";
        let hint = issue_sync_pull_remediation_hint(stderr).expect("auth hint");
        assert!(hint.contains("gh auth login -h github.com"));
    }

    #[test]
    fn pull_error_hint_none_for_unclassified_error() {
        let stderr = "some unexpected output";
        assert!(issue_sync_pull_remediation_hint(stderr).is_none());
    }

    #[test]
    fn parse_version_line_uses_first_non_empty_trimmed_line() {
        let parsed = parse_version_line("\n\n  gh-issue-sync 0.3.0\nbuild info");
        assert_eq!(parsed.as_deref(), Some("gh-issue-sync 0.3.0"));
    }

    #[test]
    fn strict_version_capture_rejects_empty_output() {
        let err = capture_required_version("sk", Ok("\n\n".to_string())).expect_err("must fail");
        assert!(err.to_string().contains("version output is empty"));
    }

    #[test]
    fn strict_version_capture_surfaces_command_failures() {
        let err =
            capture_required_version("gh-issue-sync", Err(anyhow!("boom"))).expect_err("must fail");
        assert!(err
            .to_string()
            .contains("failed to capture gh-issue-sync version"));
    }

    #[test]
    fn release_asset_maps_linux_amd64() {
        let asset =
            issue_sync_release_asset_for("linux", "x86_64").expect("linux amd64 should map");
        assert_eq!(asset.name, "gh-issue-sync-linux-amd64");
        assert_eq!(
            asset.sha256,
            "095749599f9d6c81d91765bd16b14d5c76215fd1d3ef6bccf5d005efb2d9b84e"
        );
    }

    #[test]
    fn release_asset_rejects_unsupported_target() {
        assert!(issue_sync_release_asset_for("macos", "x86_64").is_none());
    }

    #[test]
    fn trusted_commit_check_accepts_exact_match() {
        assert!(check_trusted_commit(
            "ec257c4057c32230478e5c1b86347177134e1469",
            "ec257c4057c32230478e5c1b86347177134e1469"
        )
        .is_ok());
    }

    #[test]
    fn trusted_commit_check_rejects_mismatch() {
        let err = check_trusted_commit(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "ec257c4057c32230478e5c1b86347177134e1469",
        )
        .expect_err("must reject mismatch");
        assert!(err.to_string().contains("trusted commit mismatch"));
    }
}
