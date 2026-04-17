use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "orch")]
#[command(about = "AtmanVessel Orchestrator Core", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch a profile in a worktree
    Launch {
        #[arg(short, long)]
        profile: String,
        #[arg(short, long)]
        worktree: Option<PathBuf>,
        /// CLI to launch (gemini or codex)
        #[arg(default_value = "gemini")]
        cli: String,
        /// Arguments to pass to the CLI
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
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
}

#[derive(Subcommand)]
enum CharterCommands {
    /// List all available agent charters
    List,
    /// Show details of a specific agent charter
    Show {
        name: String,
    },
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
            use libc::{kill, ESRCH};
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
                s.contains(&self.pid.to_string())
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
            schema_version: "1.4.0".to_string(),
            session_id,
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
        let mut session: Session = serde_json::from_str(&content)
            .with_context(|| "Failed to parse session JSON")?;
        session.loaded_at = Utc::now();
        Ok(session)
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        if path.exists() {
            let metadata = fs::metadata(path)?;
            let last_modified: DateTime<Utc> = metadata.modified()?.into();
            if last_modified > self.loaded_at {
                anyhow::bail!("State on disk is newer than loaded session ({} > {})", last_modified, self.loaded_at);
            }
        }

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
        let lock_dir = Path::new(".orchestrator/locks");
        let lock_name = format!("{}.json", profile);
        let lock_path = lock_dir.join(lock_name);
        
        if lock_path.exists() {
            let existing = Lock::load(&lock_path)?;
            if existing.is_live() && existing.session_id != self.session_id {
                anyhow::bail!("Worktree already locked by session {} (profile {})", existing.session_id, existing.profile);
            }
        }

        let lock = Lock::new(worktree, &self.session_id, profile);
        lock.save(lock_path)?;
        Ok(lock)
    }

    pub fn generate_handoff(&self) -> String {
        let mut handoff = String::new();
        handoff.push_str("# ORCHESTRATOR HANDOFF\n\n");
        handoff.push_str("## 1. Task Context\n");
        handoff.push_str(&format!("**Current Goal**: {}\n", self.current_task.as_ref().map(|p| p.value.as_str()).unwrap_or("None")));
        handoff.push_str(&format!("**Status**: {:?}\n", self.state_status));
        handoff.push_str(&format!("**Classification**: {:?}\n", self.classification));
        handoff.push_str(&format!("**Session ID**: {}\n", self.session_id));
        handoff.push_str(&format!("**Active Profile**: {}\n", self.active_profile.as_ref().map(|p| p.value.as_str()).unwrap_or("None")));
        
        handoff.push_str("\n## 2. Verified Facts\n");
        for fact in &self.state.verified_facts.items {
            handoff.push_str(&format!("- {} (Source: {})\n", fact.value, fact.source));
        }

        handoff.push_str("\n## 3. Provenance\n");
        if let Some(ref p) = self.current_task {
            handoff.push_str(&format!("- Task Source: {} (at {})\n", p.source, p.timestamp));
        }
        handoff
    }

    pub fn generate_recovery(&self) -> String {
        let mut recovery = String::new();
        recovery.push_str("# CRITICAL RECOVERY\n\n");
        recovery.push_str("## Session Information\n");
        recovery.push_str(&format!("**Session ID**: {}\n", self.session_id));
        recovery.push_str(&format!("**Last Updated**: {}\n", self.last_updated.to_rfc3339()));
        recovery.push_str(&format!("**Status**: {:?}\n", self.state_status));
        recovery.push_str(&format!("**Classification**: {:?}\n", self.classification));
        
        recovery.push_str("\n## Recovery Instructions\n");
        recovery.push_str("1. Verify the integrity of the state files.\n");
        recovery.push_str("2. Check for any partial writes or corrupted JSON.\n");
        recovery.push_str(&format!("3. Run `orch takeover --profile {}` if locks are stale.\n", 
            self.active_profile.as_ref().map(|p| p.value.as_str()).unwrap_or("unknown")));
        
        recovery
    }

    pub fn takeover_lock(&mut self, worktree: &Path, profile: &str, force: bool) -> Result<()> {
        let lock_dir = worktree.join(".orchestrator/locks");
        let lock_name = format!("{}.json", profile);
        let lock_path = lock_dir.join(lock_name);
        
        if lock_path.exists() {
            let existing = Lock::load(&lock_path)?;
            if existing.is_live() && !force {
                anyhow::bail!("Cannot takeover live lock for profile {} (PID: {}) without force", profile, existing.pid);
            }
        }

        let lock = Lock::new(worktree.to_path_buf(), &self.session_id, profile);
        lock.save(lock_path)?;
        self.state_status = StateStatus::Takeover;
        self.touch();
        Ok(())
    }
}

pub fn bootstrap_profile(profile_root: &Path) -> Result<()> {
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
        "google_accounts.json",
        "oauth_creds.json",
        "settings.json",
        "projects.json",
        "state.json",
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

pub fn prepare_launch_command(cli: &str, profile_path: &Path, args: &[String]) -> std::process::Command {
    let mut cmd = std::process::Command::new(cli);
    cmd.env("HOME", profile_path);
    cmd.args(args);
    cmd
}

#[cfg(test)]
mod tests;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let session_path = Path::new(".orchestrator/session.json");

    match cli.command {
        Commands::Launch { profile, worktree, cli, args } => {
            println!("🚀 Launching {} in profile: {}", cli, profile);
            let mut session = if session_path.exists() {
                Session::load(session_path)?
            } else {
                Session::new()
            };
            
            let wt = worktree.unwrap_or(std::env::current_dir()?);
            let profiles_dir = wt.join(".orchestrator/profiles");
            let profile_root = profiles_dir.join(&profile);
            
            // Register profile if new
            if !session.profiles.contains_key(&profile) {
                session.profiles.insert(profile.clone(), Profile {
                    name: profile.clone(),
                    provider: "google".to_string(),
                    root: profile_root.clone(),
                    role: None,
                    default_classification: DataClassification::Internal,
                });
            }

            session.acquire_lock(wt.clone(), &profile)?;
            session.set_active_profile(&profile, "user");
            session.state_status = StateStatus::Running;
            session.save(session_path)?;
            
            // Bootstrap
            bootstrap_profile(&profile_root)?;

            let handoff_path = wt.join(".orchestrator/HANDOFF.md");
            fs::write(&handoff_path, session.generate_handoff())?;
            println!("Handoff generated at {:?}", handoff_path);

            let mut cmd = prepare_launch_command(&cli, &profile_root, &args);
            
            cmd.stdin(std::process::Stdio::inherit())
               .stdout(std::process::Stdio::inherit())
               .stderr(std::process::Stdio::inherit());

            let mut child = cmd.spawn().with_context(|| format!("Failed to spawn CLI: {}", cli))?;
            let status = child.wait().context("Failed to wait for CLI process")?;
            
            if !status.success() {
                session.state_status = StateStatus::Crashed;
                session.save(session_path)?;
                anyhow::bail!("CLI process exited with non-zero status: {}", status);
            } else {
                session.state_status = StateStatus::Completed;
                session.save(session_path)?;
            }
        }
        Commands::Freeze { message } => {
            println!("❄️ Freezing session: {}", message);
            let mut session = Session::load(session_path)?;
            session.state_status = StateStatus::Paused;
            session.state.verified_facts.append(Provenance::new(
                message,
                session.active_profile.as_ref().map(|p| p.value.as_str()).unwrap_or("unknown"),
                &session.session_id,
                ProvenanceStatus::Verified,
            ));
            session.touch();
            session.save(session_path)?;
            println!("Session frozen and saved.");
        }
        Commands::Governance { subcommand } => {
            match subcommand {
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
            }
        }
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
                session.profiles.insert(profile.clone(), Profile {
                    name: profile.clone(),
                    provider: "google".to_string(),
                    root: profile_root.clone(),
                    role: None,
                    default_classification: DataClassification::Internal,
                });
            }
            
            session.set_active_profile(&profile, "user");
            session.save(session_path)?;
            println!("Active profile switched to: {}", profile);
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
                report.push_str(&format!("- {}\n", fact.value));
            }
            
            report.push_str("\n### Blockers & Issues\n");
            for blocker in &session.state.blockers.items {
                report.push_str(&format!("- {}\n", blocker.value));
            }
            
            let report_dir = Path::new("brain/post-mortems");
            fs::create_dir_all(report_dir)?;
            let report_path = report_dir.join(format!("pm-{}.md", session.session_id));
            fs::write(&report_path, &report)?;
            println!("Local post-mortem report generated at {:?}", report_path);
            
            if let Ok(global_path) = get_global_system_path() {
                let global_pm_dir = global_path.join("postmortems");
                if fs::create_dir_all(&global_pm_dir).is_ok() {
                    let global_pm_path = global_pm_dir.join(format!("pm-{}.md", session.session_id));
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
                    println!("  Secondary Capabilities: {:?}", charter.secondary_capabilities);
                    println!("  Constraints: {:?}", charter.constraints);
                }
            }
        }
        Commands::Sign { session: session_id } => {
            let id = session_id.unwrap_or_else(|| {
                if let Ok(session) = Session::load(session_path) {
                    session.session_id
                } else {
                    "unknown".to_string()
                }
            });
            println!("🖋️ Signing session: {} (Stub - cryptographic readiness)", id);
            println!("Provenance signature field ready for integration.");
        }
    }

    Ok(())
}
