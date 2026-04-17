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
    /// Manage data governance and classification
    Governance {
        #[command(subcommand)]
        subcommand: GovernanceCommands,
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
        }
    }
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
            last_verified_at: None,
            profiles: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read session file at {:?}", path))?;
        let session: Session = serde_json::from_str(&content)
            .with_context(|| "Failed to parse session JSON")?;
        Ok(session)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let session_path = Path::new(".orchestrator/session.json");

    match cli.command {
        Commands::Launch { profile, worktree } => {
            println!("🚀 Launching profile: {}", profile);
            let mut session = if session_path.exists() {
                Session::load(session_path)?
            } else {
                Session::new()
            };
            
            let wt = worktree.unwrap_or(std::env::current_dir()?);
            session.acquire_lock(wt, &profile)?;
            session.set_active_profile(&profile, "user");
            session.state_status = StateStatus::Running;
            session.save(session_path)?;
            
            let handoff_path = Path::new(".orchestrator/HANDOFF.md");
            fs::write(handoff_path, session.generate_handoff())?;
            println!("Handoff generated at {:?}", handoff_path);
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
        _ => {
            println!("Command not yet implemented in MVP core.");
        }
    }

    Ok(())
}
