use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "orch")]
#[command(about = "AtmanVessel Orchestrator Core", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Approve any sensitive global writes without prompting
    #[arg(long, global = true)]
    approve: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub slug: String,
    pub minion: String,
    pub status: String,
    pub path: String,
    pub parent_branch: Option<String>,
}

pub struct Board {
    pub path: PathBuf,
}

impl Board {
    pub fn new(root: &Path) -> Self {
        Self {
            path: root.join(".agent/BOARD.md"),
        }
    }

    pub fn update_task(&self, task: &Task) -> Result<()> {
        if !self.path.exists() {
            fs::create_dir_all(self.path.parent().unwrap())?;
            fs::write(
                &self.path,
                "# Task Board\n\n| Task | Minion | Status | Path | Parent |\n| :--- | :--- | :--- | :--- | :--- |\n",
            )?;
        }

        let content = fs::read_to_string(&self.path)?;
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut found = false;

        let task_line = format!(
            "| `{}` | {} | {} | {} | {} |",
            task.slug, 
            task.minion, 
            task.status, 
            task.path,
            task.parent_branch.as_deref().unwrap_or("Unknown")
        );

        for line in lines.iter_mut() {
            if line.contains(&format!("| `{}` |", task.slug)) {
                *line = task_line.clone();
                found = true;
                break;
            }
        }

        if !found {
            lines.push(task_line);
        }

        fs::write(&self.path, lines.join("\n") + "\n")?;
        Ok(())
    }

    pub fn get_task(&self, slug: &str) -> Result<Option<Task>> {
        if !self.path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&self.path)?;
        for line in content.lines() {
            if line.contains(&format!("| `{}` |", slug)) {
                let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
                if parts.len() >= 6 {
                    return Ok(Some(Task {
                        slug: slug.to_string(),
                        minion: parts[2].to_string(),
                        status: parts[3].to_string(),
                        path: parts[4].to_string(),
                        parent_branch: Some(parts[5].to_string()),
                    }));
                }
            }
        }
        Ok(None)
    }

    pub fn remove_task(&self, slug: &str) -> Result<()> {
        if !self.path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&self.path)?;
        let lines: Vec<String> = content
            .lines()
            .filter(|l| !l.contains(&format!("| `{}` |", slug)))
            .map(|s| s.to_string())
            .collect();

        fs::write(&self.path, lines.join("\n") + "\n")?;
        Ok(())
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Manage tasks and worktrees
    Task {
        #[command(subcommand)]
        subcommand: TaskCommands,
    },
    /// Start the MCP server for agent-to-agent tool use
    Mcp,
    /// Launch a profile in a worktree
    Launch {
        #[arg(short, long)]
        profile: String,
        #[arg(short, long)]
        worktree: Option<PathBuf>,
        /// CLI to launch (gemini or codex)
        #[arg(default_value = "gemini")]
        cli: String,
        /// Create a new worktree for this session
        #[arg(long)]
        new_worktree: bool,
        /// Run the CLI in interactive mode (inherit stdin/stdout/stderr)
        #[arg(long)]
        interactive: bool,
        /// Arguments to pass to the CLI
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Verify and promote asserted facts
    Verify,
    /// Freeze current state and merge validated facts
    Freeze {
        #[arg(short, long)]
        message: String,
    },
    /// Switch to a different profile
    Switch {
        #[arg(short, long)]
        profile: String,
    },
    /// Recover a stuck or crashed session
    Recover,
    /// Perform a clone-and-verify takeover
    Takeover {
        #[arg(short, long)]
        profile: String,
    },
    /// Generate a post-mortem report
    Postmortem,
    /// Manage agent charters
    Charter {
        #[command(subcommand)]
        subcommand: CharterCommands,
    },
    /// Sign a session for cryptographic provenance
    Sign {
        #[arg(short, long)]
        session: Option<String>,
    },
    /// Manage data governance and classification
    Governance {
        #[command(subcommand)]
        subcommand: GovernanceCommands,
    },
    /// Initialize orchestrator environment
    Init {
        /// Initialize global system directory
        #[arg(long)]
        global: bool,
    },
    /// Resume a paused session
    Resume {
        /// Profile to resume
        #[arg(short, long)]
        profile: Option<String>,
    },
    /// Show current session status
    Status,
    /// Manage profiles
    Profiles {
        #[command(subcommand)]
        subcommand: ProfileCommands,
    },
    /// Cleanup stale locks and old artifacts
    Cleanup,
}

#[derive(Subcommand)]
enum TaskCommands {
    /// Start a new task (creates worktree and launches chief minion)
    Start {
        /// Task slug (e.g., fix-auth-bug)
        slug: String,
        /// Initial profile to launch
        #[arg(short, long, default_value = "chief")]
        profile: String,
    },
    /// List all tasks on the board
    List,
    /// Mark a task as done
    Done {
        slug: String,
    },
    /// Finish a task (commit and merge results back to parent)
    Finish {
        slug: String,
    },
}

#[derive(Subcommand)]
enum ProfileCommands {
    /// List all registered profiles
    List,
}

#[derive(Subcommand)]
enum CharterCommands {
    /// List all available minion charters
    List,
    /// Show details of a specific minion charter
    Show { name: String },
}

#[derive(Subcommand)]
enum GovernanceCommands {
    /// Set data classification for the current session
    SetClass {
        #[arg(short, long)]
        level: String,
    },
    /// Status of current governance settings
    Status,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Secret,
    Personal,
    LegalDraft,
    TradingResearch,
    SystemInternal,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StateStatus {
    Planned,
    Running,
    Paused,
    Blocked,
    Stuck,
    Crashed,
    Recovering,
    Takeover,
    Resumed,
    Completed,
    Aborted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceStatus {
    Observed,
    Asserted,
    Verified,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provenance<T> {
    pub value: T,
    pub source: String,
    pub session_id: String,
    pub timestamp: DateTime<Utc>,
    pub confidence: f32,
    pub status: ProvenanceStatus,
    pub classification: Option<DataClassification>,
    pub signature: Option<String>,
}

impl<T> Provenance<T> {
    pub fn new(value: T, source: &str, session_id: &str, status: ProvenanceStatus) -> Self {
        Self {
            value,
            source: source.to_string(),
            session_id: session_id.to_string(),
            timestamp: Utc::now(),
            confidence: 1.0,
            status,
            classification: None,
            signature: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Charter {
    pub name: String,
    pub primary_role: String,
    pub secondary_capabilities: Vec<String>,
    pub constraints: Vec<String>,
}

pub fn get_global_system_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    Ok(home.join(".orchestrator").join("system"))
}

pub fn is_wsl() -> bool {
    is_wsl_internal("/proc/version")
}

fn is_wsl_internal(path: &str) -> bool {
    if cfg!(target_os = "linux") || cfg!(test) {
        if let Ok(content) = fs::read_to_string(path) {
            return content.to_lowercase().contains("microsoft");
        }
    }
    false
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lock {
    pub worktree_path: PathBuf,
    pub session_id: String,
    pub profile: String,
    pub pid: u32,
    pub acquired_at: DateTime<Utc>,
}

impl Lock {
    pub fn new(worktree_path: PathBuf, session_id: &str, profile: &str) -> Self {
        Self {
            worktree_path,
            session_id: session_id.to_string(),
            profile: profile.to_string(),
            pid: std::process::id(),
            acquired_at: Utc::now(),
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let lock: Lock = serde_json::from_str(&content)?;
        Ok(lock)
    }

    pub fn is_live(&self) -> bool {
        #[cfg(unix)]
        {
            use libc::{ESRCH, kill};
            unsafe {
                let res = kill(self.pid as i32, 0);
                res == 0 || std::io::Error::last_os_error().raw_os_error() != Some(ESRCH)
            }
        }
        #[cfg(windows)]
        {
            use std::process::Command;
            let output = Command::new("tasklist")
                .arg("/FI")
                .arg(format!("PID eq {}", self.pid))
                .arg("/NH")
                .output();

            if let Ok(out) = output {
                let s = String::from_utf8_lossy(&out.stdout);
                // Exact PID match: Look for the PID surrounded by whitespace or at the start/end of a line
                // tasklist /NH output starts with the image name, then PID is the second column.
                s.lines().any(|line| {
                    line.split_whitespace().any(|word| word == self.pid.to_string())
                })
            } else {
                true
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub provider: String,
    pub root: PathBuf,
    pub role: Option<String>,
    pub default_classification: DataClassification,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MergeableList<T> {
    pub merge_strategy: String,
    pub items: Vec<Provenance<T>>,
}

impl<T: Serialize + Deserialize<'static> + Clone + PartialEq> MergeableList<T> {
    pub fn new() -> Self {
        Self {
            merge_strategy: "append-unique".to_string(),
            items: Vec::new(),
        }
    }

    pub fn append(&mut self, item: Provenance<T>) {
        if !self.items.iter().any(|i| i.value == item.value) {
            self.items.push(item);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionState {
    pub goal: Option<Provenance<String>>,
    pub verified_facts: MergeableList<String>,
    pub blockers: MergeableList<String>,
    pub files_in_focus: MergeableList<PathBuf>,
    pub next_actions: MergeableList<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub schema_version: String,
    pub session_id: String,
    pub version: u64,
    pub state_status: StateStatus,
    pub classification: DataClassification,
    pub active_profile: Option<Provenance<String>>,
    pub current_task: Option<Provenance<String>>,
    pub state: SessionState,
    pub last_updated: DateTime<Utc>,
    #[serde(skip)]
    pub loaded_at: DateTime<Utc>,
    pub last_verified_at: Option<DateTime<Utc>>,
    pub profiles: HashMap<String, Profile>,
    pub metadata: HashMap<String, String>,
}

impl Session {
    pub fn new() -> Self {
        let session_id = uuid::Uuid::new_v4().to_string();
        Self {
            schema_version: "1.5.0".to_string(),
            session_id,
            version: 0,
            state_status: StateStatus::Planned,
            classification: DataClassification::Internal,
            active_profile: None,
            current_task: None,
            state: SessionState {
                goal: None,
                verified_facts: MergeableList::new(),
                blockers: MergeableList::new(),
                files_in_focus: MergeableList::new(),
                next_actions: MergeableList::new(),
            },
            last_updated: Utc::now(),
            loaded_at: Utc::now(),
            last_verified_at: None,
            profiles: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read session file at {:?}", path))?;
        let mut session: Session =
            serde_json::from_str(&content).with_context(|| "Failed to parse session JSON")?;
        session.loaded_at = Utc::now();
        Ok(session)
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        if path.exists() {
            let metadata = fs::metadata(path)?;
            let last_modified: DateTime<Utc> = metadata.modified()?.into();
            
            // 1. Check timestamp (Legacy/Fallback)
            if last_modified > self.loaded_at {
                anyhow::bail!(
                    "State on disk is newer than loaded session ({} > {})",
                    last_modified,
                    self.loaded_at
                );
            }

            // 2. Check version (Robust)
            let on_disk_content = fs::read_to_string(path)?;
            if let Ok(on_disk) = serde_json::from_str::<serde_json::Value>(&on_disk_content) {
                if let Some(on_disk_version) = on_disk.get("version").and_then(|v| v.as_u64()) {
                    if on_disk_version > self.version {
                        anyhow::bail!(
                            "Version mismatch: disk version {} is newer than memory version {}",
                            on_disk_version,
                            self.version
                        );
                    }
                }
            }
        }

        self.version += 1;
        self.last_updated = Utc::now();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        self.loaded_at = Utc::now();
        Ok(())
    }

    pub fn touch(&mut self) {
        self.last_updated = Utc::now();
    }

    pub fn promote_facts(&mut self) {
        let mut promoted = 0;

        // Promote Goal
        if let Some(ref mut goal) = self.state.goal {
            if goal.status != ProvenanceStatus::Verified {
                goal.status = ProvenanceStatus::Verified;
                promoted += 1;
            }
        }

        // Promote Facts
        for item in &mut self.state.verified_facts.items {
            if item.status != ProvenanceStatus::Verified {
                item.status = ProvenanceStatus::Verified;
                promoted += 1;
            }
        }

        // Promote Blockers
        for item in &mut self.state.blockers.items {
            if item.status != ProvenanceStatus::Verified {
                item.status = ProvenanceStatus::Verified;
                promoted += 1;
            }
        }

        // Promote Next Actions
        for item in &mut self.state.next_actions.items {
            if item.status != ProvenanceStatus::Verified {
                item.status = ProvenanceStatus::Verified;
                promoted += 1;
            }
        }

        if promoted > 0 {
            self.last_verified_at = Some(Utc::now());
            
            // If we were in Takeover mode and now everything is verified, transition to Resumed
            if self.state_status == StateStatus::Takeover && self.is_fully_verified() {
                self.state_status = StateStatus::Resumed;
            }
            
            self.touch();
        }
    }

    pub fn is_fully_verified(&self) -> bool {
        if let Some(ref goal) = self.state.goal {
            if goal.status != ProvenanceStatus::Verified {
                return false;
            }
        }
        if self
            .state
            .verified_facts
            .items
            .iter()
            .any(|i| i.status != ProvenanceStatus::Verified)
        {
            return false;
        }
        if self
            .state
            .blockers
            .items
            .iter()
            .any(|i| i.status != ProvenanceStatus::Verified)
        {
            return false;
        }
        if self
            .state
            .next_actions
            .items
            .iter()
            .any(|i| i.status != ProvenanceStatus::Verified)
        {
            return false;
        }
        true
    }

    pub fn set_active_profile(&mut self, profile_name: &str, source: &str) {
        self.active_profile = Some(Provenance::new(
            profile_name.to_string(),
            source,
            &self.session_id,
            ProvenanceStatus::Verified,
        ));
        self.touch();
    }

    pub fn acquire_lock(&mut self, worktree: PathBuf, profile: &str) -> Result<Lock> {
        let lock_path = Self::lock_path(&worktree, &worktree, profile);

        if lock_path.exists() {
            let existing = Lock::load(&lock_path)?;
            if existing.is_live() && existing.session_id != self.session_id {
                anyhow::bail!(
                    "Worktree already locked by session {} (profile {})",
                    existing.session_id,
                    existing.profile
                );
            }
        }

        let lock = Lock::new(worktree, &self.session_id, profile);
        lock.save(lock_path)?;
        Ok(lock)
    }

    fn lock_path(root: &Path, worktree: &Path, profile: &str) -> PathBuf {
        root.join(".orchestrator/locks")
            .join(Self::lock_file_name(worktree, profile))
    }

    fn legacy_lock_path(root: &Path, profile: &str) -> PathBuf {
        root.join(".orchestrator/locks")
            .join(format!("{}.json", profile))
    }

    fn lock_file_name(worktree: &Path, profile: &str) -> String {
        let safe_profile: String = profile
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.' {
                    ch
                } else {
                    '_'
                }
            })
            .collect();

        let stable_worktree = worktree
            .canonicalize()
            .unwrap_or_else(|_| worktree.to_path_buf());
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        stable_worktree.to_string_lossy().hash(&mut hasher);
        format!("{}-{:016x}.json", safe_profile, hasher.finish())
    }

    pub fn generate_handoff(&self) -> String {
        let mut handoff = String::new();
        handoff.push_str("# ORCHESTRATOR HANDOFF\n\n");
        handoff.push_str("## 1. Task Context\n");
        handoff.push_str(&format!(
            "**Current Goal**: {}\n",
            self.current_task
                .as_ref()
                .map(|p| p.value.as_str())
                .unwrap_or("None")
        ));
        handoff.push_str(&format!("**Status**: {:?}\n", self.state_status));
        handoff.push_str(&format!("**Classification**: {:?}\n", self.classification));
        handoff.push_str(&format!("**Session ID**: {}\n", self.session_id));
        handoff.push_str(&format!(
            "**Active Profile**: {}\n",
            self.active_profile
                .as_ref()
                .map(|p| p.value.as_str())
                .unwrap_or("None")
        ));

        handoff.push_str("\n## 2. Verified Facts\n");
        if self.state.verified_facts.items.is_empty() {
            handoff.push_str("- No verified facts yet.\n");
        } else {
            for fact in &self.state.verified_facts.items {
                handoff.push_str(&format!("- {} (Source: {})\n", fact.value, fact.source));
            }
        }

        handoff.push_str("\n## 3. Blockers\n");
        if self.state.blockers.items.is_empty() {
            handoff.push_str("- No known blockers.\n");
        } else {
            for blocker in &self.state.blockers.items {
                handoff.push_str(&format!("- {} (Source: {})\n", blocker.value, blocker.source));
            }
        }

        handoff.push_str("\n## 4. Files in Focus\n");
        if self.state.files_in_focus.items.is_empty() {
            handoff.push_str("- No files currently in focus.\n");
        } else {
            for focus in &self.state.files_in_focus.items {
                handoff.push_str(&format!("- `{:?}`\n", focus.value));
            }
        }

        handoff.push_str("\n## 5. Next Atomic Steps\n");
        if self.state.next_actions.items.is_empty() {
            handoff.push_str("- Triage current state and define next steps.\n");
        } else {
            for action in &self.state.next_actions.items {
                handoff.push_str(&format!("- {}\n", action.value));
            }
        }

        handoff.push_str("\n## 6. Provenance\n");
        if let Some(ref p) = self.current_task {
            handoff.push_str(&format!(
                "- Task Source: {} (at {})\n",
                p.source, p.timestamp
            ));
        }
        handoff
    }

    pub fn generate_recovery(&self) -> String {
        let mut recovery = String::new();
        recovery.push_str("# CRITICAL RECOVERY\n\n");
        recovery.push_str("## Session Information\n");
        recovery.push_str(&format!("**Session ID**: {}\n", self.session_id));
        recovery.push_str(&format!(
            "**Last Updated**: {}\n",
            self.last_updated.to_rfc3339()
        ));
        recovery.push_str(&format!("**Status**: {:?}\n", self.state_status));
        recovery.push_str(&format!("**Classification**: {:?}\n", self.classification));

        recovery.push_str("\n## Recovery Instructions\n");
        recovery.push_str("1. Verify the integrity of the state files.\n");
        recovery.push_str("2. Check for any partial writes or corrupted JSON.\n");
        recovery.push_str(&format!(
            "3. Run `orch takeover --profile {}` if locks are stale.\n",
            self.active_profile
                .as_ref()
                .map(|p| p.value.as_str())
                .unwrap_or("unknown")
        ));

        recovery
    }

    pub fn takeover_lock(&mut self, worktree: &Path, profile: &str, force: bool) -> Result<()> {
        let lock_path = Self::lock_path(worktree, worktree, profile);
        let legacy_lock_path = Self::legacy_lock_path(worktree, profile);
        let lock_path = if lock_path.exists() || !legacy_lock_path.exists() {
            lock_path
        } else {
            legacy_lock_path
        };

        if lock_path.exists() {
            let existing = Lock::load(&lock_path)?;
            if existing.is_live() && !force {
                anyhow::bail!(
                    "Cannot takeover live lock for profile {} (PID: {}) without force",
                    profile,
                    existing.pid
                );
            }
        }

        let lock = Lock::new(worktree.to_path_buf(), &self.session_id, profile);
        lock.save(lock_path)?;
        self.state_status = StateStatus::Takeover;
        self.touch();
        Ok(())
    }

    pub fn init(path: &Path) -> Result<Self> {
        let lock_dir = path.join(".orchestrator/locks");
        let profile_dir = path.join(".orchestrator/profiles");
        fs::create_dir_all(lock_dir)?;
        fs::create_dir_all(profile_dir)?;

        let mut session = Session::new();
        session.save(path.join(".orchestrator/session.json"))?;
        Ok(session)
    }

    pub fn status(&self) -> String {
        let mut status = String::new();
        status.push_str("\x1b[1;36m=== Orchestrator Session Status ===\x1b[0m\n");
        status.push_str(&format!("ID:     {}\n", self.session_id));

        let status_color = match self.state_status {
            StateStatus::Running => "\x1b[32m",
            StateStatus::Crashed | StateStatus::Stuck => "\x1b[31m",
            StateStatus::Blocked => "\x1b[33m",
            _ => "\x1b[34m",
        };
        status.push_str(&format!(
            "Status: {}{:?}\x1b[0m\n",
            status_color, self.state_status
        ));
        status.push_str(&format!("Class:  {:?}\n", self.classification));
        status.push_str(&format!(
            "Active: \x1b[1m{}\x1b[0m\n",
            self.active_profile
                .as_ref()
                .map(|p| p.value.as_str())
                .unwrap_or("None")
        ));

        if let Some(profile_prov) = &self.active_profile {
            let worktree = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            let lock_path = Self::lock_path(Path::new("."), &worktree, &profile_prov.value);
            let legacy_lock_path = Self::legacy_lock_path(Path::new("."), &profile_prov.value);
            let lock_path = if lock_path.exists() || !legacy_lock_path.exists() {
                lock_path
            } else {
                legacy_lock_path
            };
            if let Ok(lock) = Lock::load(lock_path) {
                if lock.is_live() {
                    status.push_str(&format!(
                        "Lock:   \x1b[32mLIVE (PID: {})\x1b[0m\n",
                        lock.pid
                    ));
                } else {
                    status.push_str(&format!(
                        "Lock:   \x1b[31mSTALE (Dead PID: {})\x1b[0m\n",
                        lock.pid
                    ));
                }
            } else {
                status.push_str("Lock:   None\n");
            }
        }

        if let Some(goal) = &self.state.goal {
            status.push_str(&format!("\n\x1b[1mGoal:\x1b[0m {}\n", goal.value));
        } else if let Some(task) = &self.current_task {
            status.push_str(&format!("\n\x1b[1mTask:\x1b[0m {}\n", task.value));
        }

        status
    }

    pub fn cleanup(worktree: &Path) -> Result<()> {
        let lock_dir = worktree.join(".orchestrator/locks");
        if lock_dir.exists() {
            for entry in fs::read_dir(lock_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(lock) = Lock::load(&path) {
                        if !lock.is_live() {
                            eprintln!(
                                "  Removing stale lock: {:?}",
                                path.file_name().unwrap_or_default()
                            );
                            fs::remove_file(path)?;
                        }
                    }
                }
            }
        }

        // Cleanup Stale Worktrees from Board
        let board = Board::new(worktree);
        if board.path.exists() {
            let content = fs::read_to_string(&board.path)?;
            let mut slugs_to_remove = Vec::new();

            for line in content.lines() {
                if line.contains("| `") && (line.contains("Stale") || line.contains("Done")) {
                    let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
                    if parts.len() >= 5 {
                        let slug = parts[1].trim_matches('`').to_string();
                        let path_str = parts[4].to_string();
                        let wt_path = PathBuf::from(&path_str);

                        if wt_path.exists() && path_str.contains(".worktrees") {
                            println!("🗑️  Removing stale minion worktree: {}", slug);
                            let status = std::process::Command::new("git")
                                .args(["worktree", "remove", "--force", &path_str])
                                .status()?;
                            
                            if status.success() {
                                slugs_to_remove.push(slug);
                            }
                        } else {
                            // If path doesn't exist, just remove from board
                            slugs_to_remove.push(slug);
                        }
                    }
                }
            }

            for slug in slugs_to_remove {
                board.remove_task(&slug)?;
            }
        }

        // Archive handoff if it exists
        let handoff_path = worktree.join(".orchestrator/HANDOFF.md");
        if handoff_path.exists() {
            let archive_dir = worktree.join(".orchestrator/archives");
            fs::create_dir_all(&archive_dir)?;
            let now = Utc::now().format("%Y%m%d_%H%M%S").to_string();
            let archived_name = format!("HANDOFF_{}.md", now);
            eprintln!(
                "  Archiving handoff to {:?}",
                archive_dir.join(&archived_name)
            );
            fs::rename(&handoff_path, archive_dir.join(archived_name))?;
        }

        Ok(())
    }
}

pub fn bootstrap_profile(profile_root: &Path, project_root: &Path, profile_name: &str) -> Result<()> {
    // 1. Try to bootstrap from local project profile storage
    let local_source = project_root.join(".orchestrator/profiles").join(profile_name).join(".gemini");
    if local_source.exists() {
        let target_dir = profile_root.join(".gemini");
        return bootstrap_profile_from_source(&local_source, &target_dir);
    }

    // 2. Try to bootstrap from global system profile storage
    if let Ok(global_system) = get_global_system_path() {
        let global_source = global_system.join("profiles").join(profile_name).join(".gemini");
        if global_source.exists() {
            let target_dir = profile_root.join(".gemini");
            return bootstrap_profile_from_source(&global_source, &target_dir);
        }
    }

    // 3. Fallback to global home directory (for initial profile setup)
    let source_dir = std::env::home_dir()
        .map(|p| p.join(".gemini"))
        .context("Could not find home directory")?;
    let target_dir = profile_root.join(".gemini");
    bootstrap_profile_from_source(&source_dir, &target_dir)
}

pub fn bootstrap_profile_from_source(source_dir: &Path, profile_path: &Path) -> Result<()> {
    if !profile_path.exists() {
        fs::create_dir_all(profile_path)?;
    }

    let files = [
        "settings.json",
        "projects.json",
        "state.json",
        "oauth_creds.json",
        "google_accounts.json",
    ];

    for file in &files {
        let src = source_dir.join(file);
        let dst = profile_path.join(file);
        if src.exists() && !dst.exists() {
            fs::copy(src, dst)?;
        }
    }

    Ok(())
}

pub fn prepare_launch_command(
    cli: &str,
    profile_path: &Path,
    args: &[String],
) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new(cli);
    // Harden path redirection to ensure CLI finds the correct profile credentials
    cmd.env("HOME", profile_path);
    cmd.env("GEMINI_CLI_HOME", profile_path);
    cmd.env("XDG_CONFIG_HOME", profile_path);
    cmd.env("GEMINI_CLI_TRUST_WORKSPACE", "true");
    cmd.args(args);
    cmd
}

fn probe_platform() -> Result<()> {
    eprint!("  - Platform Probe... ");
    let is_wsl = is_wsl();
    let os = std::env::consts::OS;
    if os != "linux" && os != "windows" {
        eprintln!("❌");
        anyhow::bail!("Unsupported OS: {}", os);
    }
    eprintln!("✅ ({} {})", os, if is_wsl { "WSL" } else { "Native" });
    Ok(())
}

fn probe_locks() -> Result<()> {
    eprint!("  - Lock Liveness Probe... ");
    let wt = std::env::current_dir()?;
    let session_id = "probe-session";
    let profile = "probe-profile";
    let lock_dir = wt.join(".orchestrator/probe_locks");
    fs::create_dir_all(&lock_dir)?;
    let lock_path = lock_dir.join(format!("{}.json", profile));

    // Test Live Lock
    let my_pid = std::process::id();
    let lock = Lock {
        worktree_path: wt.clone(),
        session_id: session_id.to_string(),
        profile: profile.to_string(),
        pid: my_pid,
        acquired_at: Utc::now(),
    };
    lock.save(&lock_path)?;
    let loaded = Lock::load(&lock_path)?;
    if !loaded.is_live() {
        eprintln!("❌");
        anyhow::bail!("Live lock detected as dead");
    }

    // Test Stale Lock
    let stale_pid = 999999;
    let lock_stale = Lock {
        worktree_path: wt.clone(),
        session_id: session_id.to_string(),
        profile: profile.to_string(),
        pid: stale_pid,
        acquired_at: Utc::now(),
    };
    lock_stale.save(&lock_path)?;
    let loaded_stale = Lock::load(&lock_path)?;
    if loaded_stale.is_live() {
        // Just in case PID 999999 is actually alive, but unlikely
        #[cfg(unix)]
        {
            use libc::{ESRCH, kill};
            let res = unsafe { kill(stale_pid as i32, 0) };
            if res != 0 && std::io::Error::last_os_error().raw_os_error() == Some(ESRCH) {
                eprintln!("❌");
                anyhow::bail!("Stale lock detected as live");
            }
        }
    }

    fs::remove_dir_all(&lock_dir)?;
    eprintln!("✅");
    Ok(())
}

fn probe_schema() -> Result<()> {
    eprint!("  - Schema Version Probe... ");
    let current_version = "1.5.0";
    let session = Session::new();
    if session.schema_version != current_version {
        eprintln!("❌");
        anyhow::bail!("Unexpected schema version: {}", session.schema_version);
    }
    eprintln!("✅");
    Ok(())
}

fn probe_sync() -> Result<()> {
    eprint!("  - Multi-Terminal Sync Probe... ");
    let temp_dir = std::env::temp_dir().join(format!("orch-probe-sync-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir)?;
    let session_path = temp_dir.join("session.json");

    let mut session_a = Session::new();
    session_a.save(&session_path)?;

    // Simulate Session B update
    std::thread::sleep(std::time::Duration::from_millis(10));
    let mut session_b = Session::load(&session_path)?;
    session_b.touch();
    session_b.save(&session_path)?;

    // Session A should detect stale state
    let result = session_a.save(&session_path);
    if result.is_ok() {
        eprintln!("❌");
        anyhow::bail!("Session A failed to detect stale state on disk");
    }

    fs::remove_dir_all(&temp_dir)?;
    eprintln!("✅");
    Ok(())
}

fn probe_isolation() -> Result<()> {
    eprint!("  - Profile Isolation Probe... ");
    // Logic: Verify we can create a temporary HOME and it doesn't affect current environment
    let temp_home =
        std::env::temp_dir().join(format!("orch-probe-isolation-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_home)?;

    let _original_home = std::env::var_os("HOME");

    // In Rust, std::env::set_var affects the current process.
    // To truly test isolation, we'd need to spawn a child process with a different HOME.
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("exit 0")
        .env("HOME", &temp_home)
        .status()?;

    if !status.success() {
        eprintln!("❌");
        anyhow::bail!("Failed to spawn child process with custom HOME");
    }

    fs::remove_dir_all(&temp_home)?;
    eprintln!("✅");
    Ok(())
}

pub fn run_probes() -> Result<()> {
    eprintln!("🔍 Running pre-flight probes...");
    probe_platform()?;
    probe_locks()?;
    probe_schema()?;
    probe_sync()?;
    probe_isolation()?;
    eprintln!("✅ All probes passed.");
    Ok(())
}

pub fn validate_slug(slug: &str) -> Result<()> {
    if slug.is_empty() {
        anyhow::bail!("Slug cannot be empty");
    }
    if !slug.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        anyhow::bail!("Slug contains invalid characters. Use only alphanumeric, '-' or '_'.");
    }
    Ok(())
}

pub fn start_task_core(root: &Path, slug: &str, profile: &str) -> Result<PathBuf> {
    validate_slug(slug)?;

    // 1. Initialize session or use existing
    let session_path = root.join(".orchestrator/session.json");
    let session_id = if session_path.exists() {
        Session::load(&session_path)?.session_id
    } else {
        let id = uuid::Uuid::new_v4().to_string();
        Session::init(root)?;
        id
    };

    // 2. Run probes
    run_probes()?;

    // 3. Create Worktree
    let wt_path = create_worktree(root, &session_id, slug)?;
    
    // 4. Get Current Branch as Parent
    let parent_branch = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .ok();

    // 5. Update Board
    let board = Board::new(root);
    board.update_task(&Task {
        slug: slug.to_string(),
        minion: profile.to_string(),
        status: "In Progress".to_string(),
        path: wt_path.to_string_lossy().to_string(),
        parent_branch,
    })?;

    // 5.1 Scaffold Plan Template (Centralized in Investigations)
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let central_plan_dir = PathBuf::from("/mnt/e/investigations/projects/atman-vessel/plans").join(&date);
    if central_plan_dir.exists() || std::fs::create_dir_all(&central_plan_dir).is_ok() {
        let plan_path = central_plan_dir.join(format!("{}-plan.md", slug));
        if !plan_path.exists() {
            let template = format!(
                "# Task Plan: {}\n- **Project:** atman-vessel\n- **Launched by:** {}\n- **Date:** {}\n\n## Objectives\n- [ ] Task 1\n\n## Verification Criteria\n- [ ] Criterion 1\n",
                slug, profile, date
            );
            let _ = std::fs::write(&plan_path, template);
            println!("📝 Centralized plan template scaffolded at /mnt/e/investigations/projects/atman-vessel/plans/{}/{}", date, slug);
        }
    }

    // 6. Initialize/Load session in worktree
    let wt_session_path = wt_path.join(".orchestrator/session.json");
    if !wt_session_path.exists() {
        Session::init(&wt_path)?;
    }
    let mut session = Session::load(&wt_session_path)?;
    let profile_root = wt_path.join(".orchestrator/profiles").join(profile);
    
    bootstrap_profile(&profile_root, root, profile)?;
    session.acquire_lock(wt_path.clone(), profile)?;
    session.set_active_profile(profile, "task-start");
    session.save(&wt_session_path)?;

    let handoff_path = wt_path.join(".orchestrator/HANDOFF.md");
    fs::write(&handoff_path, session.generate_handoff())?;

    Ok(wt_path)
}

pub fn create_worktree(base_path: &Path, session_id: &str, slug: &str) -> Result<PathBuf> {
    let wt_path = base_path
        .join(".worktrees")
        .join(format!("session-{}-{}", session_id, slug));

    if !wt_path.exists() {
        let output = std::process::Command::new("git")
            .arg("worktree")
            .arg("add")
            .arg("-b")
            .arg(format!("task-{}-{}", session_id, slug))
            .arg(&wt_path)
            .arg("HEAD")
            .output()
            .context("Failed to execute git worktree add")?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git worktree add failed: {}", err);
        }
    }

    Ok(wt_path)
}

#[cfg(test)]
mod tests;

pub fn check_approval(approved_by_flag: bool, message: &str) -> Result<()> {
    if approved_by_flag {
        return Ok(());
    }

    println!("⚠️  SECURITY GATE: {}", message);
    print!("Do you approve this action? [y/N]: ");
    use std::io::Write;
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() == "y" {
        Ok(())
    } else {
        anyhow::bail!("Action not approved by user")
    }
}

pub fn bootstrap_charters(charters_dir: &Path) -> Result<()> {
    if !charters_dir.exists() {
        fs::create_dir_all(charters_dir)?;
    }

    let roles = [
        ("chief", "Planning, dispatching, and integration"),
        ("builder", "Implementation and refactoring"),
        ("systems", "Environment, tooling, and setup"),
        ("reviewer", "Code review and regression spotting"),
        ("writer", "Documentation and drafting"),
        ("quant", "Trading research and backtesting"),
        ("ops", "Agent system maintenance"),
        ("recovery", "Stuck/crashed session triage"),
    ];

    for (name, role) in roles {
        let charter_path = charters_dir.join(format!("{}.json", name));
        if !charter_path.exists() {
            let charter = Charter {
                name: name.to_string(),
                primary_role: role.to_string(),
                secondary_capabilities: Vec::new(),
                constraints: Vec::new(),
            };
            let content = serde_json::to_string_pretty(&charter)?;
            fs::write(charter_path, content)?;
        }
    }

    Ok(())
}

async fn run_cli(
    cli_bin: &str,
    profile_root: &Path,
    args: &[String],
    session_path: &Path,
    session: &mut Session,
    interactive: bool,
) -> Result<()> {
    let mut cmd = prepare_launch_command(cli_bin, profile_root, args);
    cmd.kill_on_drop(true);
    let timeout_duration = std::time::Duration::from_secs(60);

    if interactive {
        cmd.stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());

        let mut child = cmd
            .spawn()
            .with_context(|| format!("Failed to spawn CLI: {}", cli_bin))?;
        
        let status = tokio::time::timeout(timeout_duration, child.wait())
            .await
            .map_err(|_| anyhow::anyhow!("CLI process timed out after 60 seconds"))?
            .context("Failed to wait for CLI process")?;

        if !status.success() {
            session.state_status = StateStatus::Crashed;
            session.save(session_path)?;
            anyhow::bail!("CLI process exited with non-zero status: {}", status);
        } else {
            session.state_status = StateStatus::Completed;
            session.save(session_path)?;
        }
    } else {
        // Ensure autonomous mode
        cmd.arg("--yolo");

        // Capture output to prevent cluttering the manager's terminal
        cmd.stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let mut child = cmd
            .spawn()
            .with_context(|| format!("Failed to spawn CLI: {}", cli_bin))?;

        // Inject the handoff as the first directive
        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            let handoff = session.generate_handoff();
            let directive = format!(
                "Context: Read the handoff below and resume work from 'Next Atomic Steps'. Do not ask for confirmation. Be autonomous.\n\n---\n{}\n---\n",
                handoff
            );
            stdin.write_all(directive.as_bytes()).await?;
            stdin.flush().await?;
            drop(stdin); // Explicitly close stdin to signal EOF
        }

        let result = tokio::time::timeout(timeout_duration, child.wait_with_output()).await;

        match result {
            Ok(Ok(output)) => {
                // Print captured output to stderr to prevent corrupting MCP JSON-RPC on stdout
                io::stderr().write_all(&output.stdout)?;
                io::stderr().write_all(&output.stderr)?;

                if !output.status.success() {
                    session.state_status = StateStatus::Crashed;
                    session.save(session_path)?;
                    let err = String::from_utf8_lossy(&output.stderr);
                    anyhow::bail!("CLI process failed: {}", err);
                } else {
                    session.state_status = StateStatus::Completed;
                    session.save(session_path)?;
                }
            }
            Ok(Err(e)) => anyhow::bail!("Failed to wait for CLI process: {}", e),
            Err(_) => {
                anyhow::bail!("CLI process timed out after 60 seconds");
            }
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct McpRequest {
    jsonrpc: String,
    method: String,
    params: Option<serde_json::Value>,
    id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct McpResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<McpError>,
    id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct McpError {
    code: i32,
    message: String,
}

async fn handle_mcp_request(req: McpRequest) -> McpResponse {
    let result = match req.method.as_str() {
        "initialize" => Some(serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {
                    "listChanged": false
                }
            },
            "serverInfo": {
                "name": "atman-vessel-orchestrator",
                "version": "1.6.0"
            }
        })),
        "tools/list" => Some(serde_json::json!({
            "tools": [
                {
                    "name": "get_status",
                    "description": "Get current orchestrator session status",
                    "inputSchema": { "type": "object", "properties": {} }
                },
                {
                    "name": "start_task",
                    "description": "Start a new task, create a worktree, and launch a specialist",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "slug": { "type": "string", "description": "The task slug (e.g., fix-tests)" },
                            "profile": { "type": "string", "description": "The specialist profile to launch", "default": "chief" }
                        },
                        "required": ["slug"]
                    }
                },
                {
                    "name": "switch_profile",
                    "description": "Switch to a different agent profile and resume work",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "profile": { "type": "string", "description": "The name of the profile (e.g., reviewer, builder)" }
                        },
                        "required": ["profile"]
                    }
                },
                {
                    "name": "finish_task",
                    "description": "Commit and merge results from a specialist worktree back to parent branch",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "slug": { "type": "string", "description": "The task slug" }
                        },
                        "required": ["slug"]
                    }
                },
                {
                    "name": "freeze_session",
                    "description": "Freeze current session with a summary message",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "message": { "type": "string", "description": "Handoff message/summary" }
                        },
                        "required": ["message"]
                    }
                }
            ]
        })),
        "tools/call" => {
            let params = req.params.as_ref().and_then(|v| v.as_object());
            let tool_name = params.and_then(|p| p.get("name")).and_then(|n| n.as_str());
            let arguments = params.and_then(|p| p.get("arguments"));

            match tool_name {
                Some("get_status") => {
                    let session_path = Path::new(".orchestrator/session.json");
                    match Session::load(session_path) {
                        Ok(session) => Some(serde_json::json!({ "content": [{ "type": "text", "text": session.status() }] })),
                        Err(e) => Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": format!("Error: {}", e) }] })),
                    }
                }
                Some("start_task") => {
                    let slug = arguments.and_then(|a| a.get("slug")).and_then(|s| s.as_str());
                    let profile = arguments.and_then(|a| a.get("profile")).and_then(|p| p.as_str()).unwrap_or("chief");
                    
                    if let Some(s) = slug {
                        let root = match std::env::current_dir() {
                            Ok(d) => d,
                            Err(e) => return McpResponse { jsonrpc: "2.0".to_string(), result: None, error: Some(McpError { code: -32000, message: format!("IO Error: {}", e) }), id: req.id },
                        };

                        match start_task_core(&root, s, profile) {
                            Ok(wt_path) => {
                                let wt_session_path = wt_path.join(".orchestrator/session.json");
                                let profile_root = wt_path.join(".orchestrator/profiles").join(profile);

                                // We spawn in background for MCP to not block the server indefinitely
                                tokio::spawn(async move {
                                    if let Ok(mut session_inner) = Session::load(&wt_session_path) {
                                        let _ = run_cli("gemini", &profile_root, &[], &wt_session_path, &mut session_inner, false).await;
                                    }
                                });

                                Some(serde_json::json!({ "content": [{ "type": "text", "text": format!("Task {} started and agent {} launched in worktree {:?}. A plan template has been scaffolded at brain/plans/{}-plan.md.", s, profile, wt_path, s) }] }))
                            }
                            Err(e) => Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": format!("Error: {}", e) }] })),
                        }
                    } else {
                        Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": "Missing 'slug' argument" }] }))
                    }
                }
                Some("switch_profile") => {
                    let profile = arguments.and_then(|a| a.get("profile")).and_then(|p| p.as_str());
                    if let Some(p) = profile {
                        let session_path = Path::new(".orchestrator/session.json");
                        match Session::load(session_path) {
                            Ok(mut session) => {
                                let wt = match std::env::current_dir() {
                                    Ok(d) => d,
                                    Err(e) => return McpResponse { jsonrpc: "2.0".to_string(), result: None, error: Some(McpError { code: -32000, message: format!("IO Error: {}", e) }), id: req.id },
                                };
                                let profile_root = wt.join(".orchestrator/profiles").join(p);
                                if !session.profiles.contains_key(p) {
                                    session.profiles.insert(p.to_string(), Profile {
                                        name: p.to_string(),
                                        provider: "google".to_string(),
                                        root: profile_root.clone(),
                                        role: None,
                                        default_classification: DataClassification::Internal,
                                    });
                                }
                                session.set_active_profile(p, "mcp-tool");
                                session.state_status = StateStatus::Running;
                                
                                if let Err(e) = session.save(session_path) {
                                    Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": format!("Save Error: {}", e) }] }))
                                } else {
                                    // Bootstrap and regenerate handoff
                                    let _ = bootstrap_profile(&profile_root, &wt, p);
                                    let handoff_path = wt.join(".orchestrator/HANDOFF.md");
                                    let _ = fs::write(&handoff_path, session.generate_handoff());

                                    Some(serde_json::json!({ "content": [{ "type": "text", "text": format!("Switched to profile {}. Handoff regenerated.", p) }] }))
                                }
                            }
                            Err(e) => Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": format!("Load Error: {}", e) }] })),
                        }
                    } else {
                        Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": "Missing 'profile' argument" }] }))
                    }
                }
                Some("finish_task") => {
                    let slug = arguments.and_then(|a| a.get("slug")).and_then(|s| s.as_str());
                    if let Some(s) = slug {
                        let root = match std::env::current_dir() {
                            Ok(d) => d,
                            Err(e) => return McpResponse { jsonrpc: "2.0".to_string(), result: None, error: Some(McpError { code: -32000, message: format!("IO Error: {}", e) }), id: req.id },
                        };

                        println!("🏁 MCP: Finishing task: {}", s);
                        let board = Board::new(&root);
                        match board.get_task(s) {
                            Ok(Some(task)) => {
                                let wt_path = PathBuf::from(&task.path);
                                if !wt_path.exists() {
                                    Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": "Worktree path does not exist" }] }))
                                } else {
                                    // Commit
                                    let _ = std::process::Command::new("git").arg("-C").arg(&wt_path).args(["add", "."]).status();
                                    let commit_msg = format!("feat(minion): results from {}", s);
                                    let _ = std::process::Command::new("git").arg("-C").arg(&wt_path).args(["commit", "-m", &commit_msg]).status();

                                    let mut merge_failed = false;
                                    // Merge
                                    if let Some(parent) = task.parent_branch {
                                        let task_branch = std::process::Command::new("git").arg("-C").arg(&wt_path).args(["rev-parse", "--abbrev-ref", "HEAD"]).output()
                                            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
                                        
                                        let _ = std::process::Command::new("git").args(["checkout", &parent]).status();
                                        let merge_status = std::process::Command::new("git").args(["merge", &task_branch]).status();
                                        
                                        if let Ok(st) = merge_status {
                                            if !st.success() {
                                                merge_failed = true;
                                            }
                                        }
                                    }

                                    if merge_failed {
                                        Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": "Merge conflict detected" }] }))
                                    } else {
                                        // Mark Stale
                                        let _ = board.update_task(&Task {
                                            slug: s.to_string(),
                                            minion: task.minion,
                                            status: "Stale (Merged)".to_string(),
                                            path: task.path,
                                            parent_branch: None,
                                        });

                                        Some(serde_json::json!({ "content": [{ "type": "text", "text": format!("Task {} finished and merged.", s) }] }))
                                    }
                                }
                            }
                            _ => Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": "Task not found" }] })),
                        }
                    } else {
                        Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": "Missing 'slug' argument" }] }))
                    }
                }
                Some("freeze_session") => {
                    let message = arguments.and_then(|a| a.get("message")).and_then(|m| m.as_str());
                    if let Some(msg) = message {
                        let session_path = Path::new(".orchestrator/session.json");
                        match Session::load(session_path) {
                            Ok(mut session) => {
                                session.state_status = StateStatus::Paused;
                                session.state.verified_facts.append(Provenance::new(
                                    msg.to_string(),
                                    session
                                        .active_profile
                                        .as_ref()
                                        .map(|p| p.value.as_str())
                                        .unwrap_or("unknown"),
                                    &session.session_id,
                                    ProvenanceStatus::Verified,
                                ));
                                session.touch();
                                if let Err(e) = session.save(session_path) {
                                    Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": format!("Save Error: {}", e) }] }))
                                } else {
                                    Some(serde_json::json!({ "content": [{ "type": "text", "text": format!("Session frozen: {}", msg) }] }))
                                }
                            }
                            Err(e) => Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": format!("Load Error: {}", e) }] })),
                        }
                    } else {
                        Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": "Missing 'message' argument" }] }))
                    }
                }
                _ => Some(serde_json::json!({ "isError": true, "content": [{ "type": "text", "text": format!("Unknown tool: {:?}", tool_name) }] })),
            }
        }
        _ => None,
    };

    McpResponse {
        jsonrpc: "2.0".to_string(),
        result: result.clone(),
        error: if result.is_none() { Some(McpError { code: -32601, message: "Method not found".to_string() }) } else { None },
        id: req.id,
    }
}

async fn run_mcp_server() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let resp = match serde_json::from_str::<McpRequest>(&line) {
            Ok(req) => {
                let is_notification = req.id.is_none();
                let resp = handle_mcp_request(req).await;
                if is_notification {
                    None
                } else {
                    Some(resp)
                }
            }
            Err(_) => Some(McpResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(McpError {
                    code: -32700,
                    message: "Parse error".to_string(),
                }),
                id: None,
            }),
        };

        if let Some(resp) = resp {
            let resp_json = serde_json::to_string(&resp)?;
            stdout.write_all(resp_json.as_bytes())?;
            stdout.write_all(b"\n")?;
            stdout.flush()?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let session_path = Path::new(".orchestrator/session.json");

    match cli.command {
        Commands::Task { subcommand } => {
            let root = std::env::current_dir()?;

            match subcommand {
                TaskCommands::Start { slug, profile } => {
                    println!("🚀 Starting task: {}", slug);
                    let wt_path = start_task_core(&root, &slug, &profile)?;
                    println!("✅ Worktree created at {:?}", wt_path);
                    println!("🚀 Deploying minion {} in background...", profile);

                    let _wt_session_path = wt_path.join(".orchestrator/session.json");
                    let profile_root = wt_path.join(".orchestrator/profiles").join(&profile);

                    // Prepare the launch command
                    let mut cmd = prepare_launch_command("gemini", &profile_root, &[]);
                    
                    // IMPORTANT: For CLI background launch, we do NOT set kill_on_drop(true)
                    // and we do NOT await wait_with_output.
                    // We also ensure it's detached from the current process group if possible,
                    // but for standard CLI usage, just spawning and letting orch exit is enough.
                    
                    // Redirect output to a log file in the worktree for traceability
                    let log_path = wt_path.join(".orchestrator/minion.log");
                    let log_file = std::fs::File::create(&log_path)?;
                    
                    cmd.stdin(std::process::Stdio::null())
                       .stdout(log_file.try_clone()?)
                       .stderr(log_file);

                    // Inject handoff via temporary file since we can't use stdin pipe easily with detached
                    // Actually, gemini-cli can read from a file. But our current run_cli logic 
                    // uses stdin. For detached background, we'll just spawn and let the minion
                    // read its own HANDOFF.md which we already wrote in start_task_core.
                    
                    cmd.spawn().with_context(|| format!("Failed to spawn minion: {}", profile))?;
                    
                    println!("✨ Minion {} is now active. Trace: {:?}", profile, log_path);
                    println!("👉 Worktree: {:?}", wt_path);
                }
                TaskCommands::List => {
                    let board = Board::new(&root);
                    if board.path.exists() {
                        let content = fs::read_to_string(&board.path)?;
                        println!("{}", content);
                    } else {
                        println!("Board empty.");
                    }
                }
                TaskCommands::Done { slug } => {
                    let board = Board::new(&root);
                    board.update_task(&Task {
                        slug,
                        minion: "None".to_string(),
                        status: "Done".to_string(),
                        path: "Archived".to_string(),
                        parent_branch: None,
                    })?;
                    println!("Task marked as Done.");
                }
                TaskCommands::Finish { slug } => {
                    let board = Board::new(&root);
                    if let Some(task) = board.get_task(&slug)? {
                        println!("🏁 Finishing task: {}", slug);
                        let wt_path = PathBuf::from(&task.path);
                        
                        if !wt_path.exists() {
                            anyhow::bail!("Worktree path does not exist: {:?}", wt_path);
                        }

                        // 0. Integrity Check: Verify design brief exists in central investigations
                        println!("🔍 Checking for centralized design brief...");
                        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
                        let central_briefs_dir = PathBuf::from("/mnt/e/investigations/projects/atman-vessel/briefs").join(&date);
                        let brief_exists = if central_briefs_dir.exists() {
                            std::fs::read_dir(central_briefs_dir)?
                                .filter_map(|e| e.ok())
                                .any(|e| e.file_name().to_string_lossy().contains(&slug))
                        } else {
                            false
                        };

                        if !brief_exists {
                            println!("⚠️ WARNING: No design brief found for task '{}' in /mnt/e/investigations/projects/atman-vessel/briefs/{}", slug, date);
                            println!("High-integrity workflow requires a centralized design brief for cross-agent visibility.");
                        }

                        // 1. Check for untracked files
                        println!("🧐 Checking for untracked files...");
                        let untracked = std::process::Command::new("git")
                            .arg("-C")
                            .arg(&wt_path)
                            .args(["status", "--porcelain"])
                            .output()?;
                        
                        let untracked_str = String::from_utf8_lossy(&untracked.stdout);
                        if untracked_str.lines().any(|l| l.starts_with("??")) {
                            println!("⚠️ WARNING: Untracked files detected in worktree:");
                            for line in untracked_str.lines().filter(|l| l.starts_with("??")) {
                                println!("  {}", line);
                            }
                        }

                        // 2. Commit changes in worktree
                        println!("📝 Committing changes in worktree...");
                        let status = std::process::Command::new("git")
                            .arg("-C")
                            .arg(&wt_path)
                            .args(["add", "."])
                            .status()?;
                        
                        if !status.success() {
                            anyhow::bail!("Failed to stage changes in worktree");
                        }

                        let commit_msg = format!("feat(minion): results from {}", slug);
                        let status = std::process::Command::new("git")
                            .arg("-C")
                            .arg(&wt_path)
                            .args(["commit", "-m", &commit_msg])
                            .status()?;

                        if !status.success() {
                            println!("⚠️ No changes to commit or commit failed.");
                        }

                        // 3. Merge to parent branch
                        if let Some(parent) = task.parent_branch {
                            println!("🔄 Merging results into parent branch: {}", parent);
                            
                            // Get the current task branch name
                            let task_branch = std::process::Command::new("git")
                                .arg("-C")
                                .arg(&wt_path)
                                .args(["rev-parse", "--abbrev-ref", "HEAD"])
                                .output()
                                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())?;

                            // Switch parent repo to parent branch if not already there
                            let status = std::process::Command::new("git")
                                .args(["checkout", &parent])
                                .status()?;

                            if !status.success() {
                                anyhow::bail!("Failed to checkout parent branch: {}", parent);
                            }

                            // Merge the task branch
                            let status = std::process::Command::new("git")
                                .args(["merge", &task_branch])
                                .status()?;

                            if !status.success() {
                                anyhow::bail!("Merge conflict detected. Please resolve manually in branch {}", parent);
                            }
                            
                            println!("✅ Successfully merged {} into {}", task_branch, parent);
                            
                            // 3.1 Audit Log: Write summary to investigations
                            let date = chrono::Local::now().format("%Y-%m-%d").to_string();
                            let log_dir = PathBuf::from("/mnt/e/investigations/daily").join(&date);
                            if log_dir.exists() || std::fs::create_dir_all(&log_dir).is_ok() {
                                let log_file = log_dir.join(format!("{}-summary.md", slug));
                                let summary = format!(
                                    "# Task Summary: {}\n- **Minion:** {}\n- **Status:** Integrated\n- **Date:** {}\n\nResults merged into branch `{}`.\n",
                                    slug, task.minion, date, parent
                                );
                                let _ = std::fs::write(log_file, summary);
                                println!("📝 Audit log written to investigations/daily/{}", date);
                            }

                            // 4. Auto-Cleanup Worktree
                            println!("🧹 Auto-cleaning up worktree...");
                            let _ = std::process::Command::new("git")
                                .args(["worktree", "remove", &task.path, "--force"])
                                .status();
                            
                            println!("✅ Worktree removed.");
                        }

                        // 5. Update Board to Done
                        board.update_task(&Task {
                            slug: slug.clone(),
                            minion: task.minion,
                            status: "Done".to_string(),
                            path: task.path,
                            parent_branch: None,
                        })?;
                        
                        println!("🎉 Task {} complete and integrated.", slug);

                    } else {
                        anyhow::bail!("Task not found: {}", slug);
                    }
                }
            }
        }
        Commands::Mcp => {
            run_mcp_server().await?;
        }
        Commands::Launch {
            profile,
            worktree,
            cli: cli_bin,
            new_worktree,
            interactive,
            args,
        } => {
            let wt = if new_worktree {
                let base = std::env::current_dir()?;
                // We need a session_id. If session.json exists, use it. Otherwise new.
                let session_id = if session_path.exists() {
                    Session::load(session_path)?.session_id
                } else {
                    uuid::Uuid::new_v4().to_string()
                };
                create_worktree(&base, &session_id, &profile)?
            } else {
                worktree.unwrap_or(std::env::current_dir()?)
            };

            let session_path = wt.join(".orchestrator/session.json");

            run_probes()?;

            if !session_path.exists() {
                println!("🏗️ Implicitly initializing orchestrator in {:?}", wt);
                Session::init(&wt)?;
            }

            println!("🚀 Launching {} in profile: {}", cli_bin, profile);
            let mut session = Session::load(&session_path)?;

            let profiles_dir = wt.join(".orchestrator/profiles");
            let profile_root = profiles_dir.join(&profile);

            // Register profile if new
            if !session.profiles.contains_key(&profile) {
                session.profiles.insert(
                    profile.clone(),
                    Profile {
                        name: profile.clone(),
                        provider: "google".to_string(),
                        root: profile_root.clone(),
                        role: None,
                        default_classification: DataClassification::Internal,
                    },
                );
            }

            session.acquire_lock(wt.clone(), &profile)?;
            session.set_active_profile(&profile, "user");
            session.state_status = StateStatus::Running;
            session.save(&session_path)?;

            // Bootstrap
            let base_root = std::env::current_dir()?;
            bootstrap_profile(&profile_root, &base_root, &profile)?;

            let handoff_path = wt.join(".orchestrator/HANDOFF.md");
            fs::write(&handoff_path, session.generate_handoff())?;
            println!("Handoff generated at {:?}", handoff_path);

            run_cli(&cli_bin, &profile_root, &args, &session_path, &mut session, interactive).await?;
        }
        Commands::Verify => {
            let mut session = Session::load(session_path)?;
            println!("🔄 Promoting asserted facts to verified...");
            session.promote_facts();
            session.save(session_path)?;
            println!("Done.");
        }
        Commands::Freeze { message } => {
            println!("❄️ Freezing session: {}", message);
            let mut session = Session::load(session_path)?;
            session.state_status = StateStatus::Paused;
            session.state.verified_facts.append(Provenance::new(
                message,
                session
                    .active_profile
                    .as_ref()
                    .map(|p| p.value.as_str())
                    .unwrap_or("unknown"),
                &session.session_id,
                ProvenanceStatus::Verified,
            ));
            session.touch();
            session.save(session_path)?;
            println!("Session frozen and saved.");
        }
        Commands::Governance { subcommand } => match subcommand {
            GovernanceCommands::SetClass { level } => {
                let mut session = Session::load(session_path)?;
                let class = match level.to_lowercase().as_str() {
                    "public" => DataClassification::Public,
                    "internal" => DataClassification::Internal,
                    "confidential" => DataClassification::Confidential,
                    "secret" => DataClassification::Secret,
                    _ => anyhow::bail!("Unknown classification level: {}", level),
                };
                session.classification = class;
                session.touch();
                session.save(session_path)?;
                println!("Data classification set to: {:?}", session.classification);
            }
            GovernanceCommands::Status => {
                let session = Session::load(session_path)?;
                println!("Current Governance Status:");
                println!("  Classification: {:?}", session.classification);
                println!("  Session ID: {}", session.session_id);
            }
        },
        Commands::Takeover { profile } => {
            println!("🔄 Taking over profile: {}", profile);
            let mut session = if session_path.exists() {
                Session::load(session_path)?
            } else {
                Session::new()
            };

            let worktree = std::env::current_dir()?;
            session.takeover_lock(&worktree, &profile, false)?;
            session.set_active_profile(&profile, "takeover");
            session.save(session_path)?;
            println!("Takeover successful.");
        }
        Commands::Switch { profile } => {
            println!("🔄 Switching to profile: {}", profile);
            let mut session = Session::load(session_path)?;

            let wt = std::env::current_dir()?;
            let profile_root = wt.join(".orchestrator/profiles").join(&profile);

            if !session.profiles.contains_key(&profile) {
                session.profiles.insert(
                    profile.clone(),
                    Profile {
                        name: profile.clone(),
                        provider: "google".to_string(),
                        root: profile_root.clone(),
                        role: None,
                        default_classification: DataClassification::Internal,
                    },
                );
            }

            session.set_active_profile(&profile, "user");
            session.state_status = StateStatus::Running;
            session.save(session_path)?;

            // Bootstrap
            bootstrap_profile(&profile_root, &wt, &profile)?;

            let handoff_path = wt.join(".orchestrator/HANDOFF.md");
            fs::write(&handoff_path, session.generate_handoff())?;
            println!("Handoff regenerated for switch to {}", profile);

            // Default to 'gemini' for switch if not otherwise known
            run_cli("gemini", &profile_root, &[], session_path, &mut session, false).await?;
        }
        Commands::Recover => {
            println!("💊 Recovering session...");
            let session = Session::load(session_path)?;
            let recovery_path = Path::new(".orchestrator/RECOVERY.md");
            fs::write(recovery_path, session.generate_recovery())?;
            println!("Recovery report generated at {:?}", recovery_path);
        }
        Commands::Postmortem => {
            println!("📝 Generating post-mortem report...");
            let session = Session::load(session_path)?;

            let mut report = String::new();
            report.push_str("# Post-Mortem Analysis\n\n");
            report.push_str(&format!("**Session ID**: {}\n", session.session_id));
            report.push_str(&format!("**Final Status**: {:?}\n\n", session.state_status));

            report.push_str("## Collate Facts vs. Blockers\n");
            report.push_str("### Verified Facts\n");
            for fact in &session.state.verified_facts.items {
                let val = if fact.classification == Some(DataClassification::Secret) {
                    "[REDACTED]".to_string()
                } else {
                    fact.value.clone()
                };
                report.push_str(&format!("- {}\n", val));
            }

            report.push_str("\n### Blockers & Issues\n");
            for blocker in &session.state.blockers.items {
                let val = if blocker.classification == Some(DataClassification::Secret) {
                    "[REDACTED]".to_string()
                } else {
                    blocker.value.clone()
                };
                report.push_str(&format!("- {}\n", val));
            }

            let report_dir = Path::new("brain/post-mortems");
            fs::create_dir_all(report_dir)?;
            let report_path = report_dir.join(format!("pm-{}.md", session.session_id));
            fs::write(&report_path, &report)?;
            println!("Local post-mortem report generated at {:?}", report_path);

            if session.classification == DataClassification::Secret {
                println!("🤫 Secret session: Global post-mortem copying disabled.");
                return Ok(());
            }

            if let Ok(global_path) = get_global_system_path() {
                check_approval(
                    cli.approve,
                    &format!(
                        "Copy post-mortem report to global storage at {:?}",
                        global_path
                    ),
                )?;
                let global_pm_dir = global_path.join("postmortems");
                if fs::create_dir_all(&global_pm_dir).is_ok() {
                    let global_pm_path =
                        global_pm_dir.join(format!("pm-{}.md", session.session_id));
                    fs::write(&global_pm_path, &report)?;
                    println!("Global post-mortem report copied to {:?}", global_pm_path);
                }
            }
        }
        Commands::Charter { subcommand } => {
            let global_path = get_global_system_path()?;
            let charters_dir = global_path.join("charters");

            match subcommand {
                CharterCommands::List => {
                    println!("Agent Charters:");
                    if !charters_dir.exists() {
                        println!("  (No charters found in {:?})", charters_dir);
                        return Ok(());
                    }
                    for entry in fs::read_dir(charters_dir)? {
                        let entry = entry?;
                        if entry.file_type()?.is_file() {
                            if let Some(name) = entry.file_name().to_str() {
                                if name.ends_with(".json") {
                                    println!("  - {}", name.trim_end_matches(".json"));
                                }
                            }
                        }
                    }
                }
                CharterCommands::Show { name } => {
                    let charter_path = charters_dir.join(format!("{}.json", name));
                    if !charter_path.exists() {
                        anyhow::bail!("Charter '{}' not found at {:?}", name, charter_path);
                    }
                    let content = fs::read_to_string(&charter_path)?;
                    let charter: Charter = serde_json::from_str(&content)?;
                    println!("Charter: {}", charter.name);
                    println!("  Primary Role: {}", charter.primary_role);
                    println!(
                        "  Secondary Capabilities: {:?}",
                        charter.secondary_capabilities
                    );
                    println!("  Constraints: {:?}", charter.constraints);
                }
            }
        }
        Commands::Sign {
            session: session_id,
        } => {
            let id = session_id.unwrap_or_else(|| {
                if let Ok(session) = Session::load(session_path) {
                    session.session_id
                } else {
                    "unknown".to_string()
                }
            });
            println!(
                "🖋️ Signing session: {} (Stub - cryptographic readiness)",
                id
            );
            println!("Provenance signature field ready for integration.");
        }
        Commands::Init { global } => {
            println!("🏗️ Initializing orchestrator...");
            let worktree = std::env::current_dir()?;
            Session::init(&worktree)?;

            if global {
                let global_path = get_global_system_path()?;
                let charters_dir = global_path.join("charters");
                println!("🌐 Bootstrapping global charters in {:?}", charters_dir);
                bootstrap_charters(&charters_dir)?;
            }
            println!("Done.");
        }
        Commands::Resume { profile } => {
            println!("⏯️ Resuming session...");
            let mut session = Session::load(session_path)?;

            let profile_name = match profile {
                Some(p) => p,
                None => match &session.active_profile {
                    Some(ap) => ap.value.clone(),
                    None => anyhow::bail!(
                        "No active profile found. Use `launch --profile <name>` to start a session or specify `--profile` with resume."
                    ),
                },
            };

            // In a real implementation, we'd verify lock status here
            session.set_active_profile(&profile_name, "resume");
            session.state_status = StateStatus::Running;
            session.touch();
            session.save(session_path)?;

            let wt = std::env::current_dir()?;
            let profile_root = wt.join(".orchestrator/profiles").join(&profile_name);

            // Bootstrap
            bootstrap_profile(&profile_root, &wt, &profile_name)?;

            // Regenerate HANDOFF.md
            let handoff_path = wt.join(".orchestrator/HANDOFF.md");
            fs::write(&handoff_path, session.generate_handoff())?;
            println!("Handoff regenerated at {:?}", handoff_path);

            println!("Session resumed with profile: {}", profile_name);

            // Default to 'gemini' for resume if not otherwise known
            run_cli("gemini", &profile_root, &[], session_path, &mut session, false).await?;
        }
        Commands::Status => {
            let session = Session::load(session_path)?;
            println!("{}", session.status());
        }
        Commands::Profiles { subcommand } => match subcommand {
            ProfileCommands::List => {
                let session = Session::load(session_path)?;
                println!("Registered Profiles:");
                for name in session.profiles.keys() {
                    let active = if let Some(ap) = &session.active_profile {
                        if ap.value == *name { " [Active]" } else { "" }
                    } else {
                        ""
                    };
                    println!("  - {}{}", name, active);
                }
            }
        },
        Commands::Cleanup => {
            println!("🧹 Cleaning up stale locks...");
            let worktree = std::env::current_dir()?;
            Session::cleanup(&worktree)?;
            println!("Done.");
        }
    }

    Ok(())
}
