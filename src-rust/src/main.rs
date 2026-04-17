use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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
            // Simple check if process exists on Windows
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
                true // Fallback to safe side
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub provider: String, // "gemini" | "codex"
    pub root: PathBuf,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MergeableList<T> {
    pub merge_strategy: String, // "append-unique"
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

    pub fn set_task(&mut self, task: &str, source: &str) {
        self.current_task = Some(Provenance::new(
            task.to_string(),
            source,
            &self.session_id,
            ProvenanceStatus::Asserted,
        ));
        self.touch();
    }
    
    pub fn add_fact(&mut self, fact: &str, source: &str, status: ProvenanceStatus) {
        self.state.verified_facts.append(Provenance::new(
            fact.to_string(),
            source,
            &self.session_id,
            status,
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
            println!("Replacing stale or same-session lock.");
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
    println!("AtmanVessel Orchestrator Core MVP v1.4.0");

    let session_path = Path::new(".orchestrator/session.json");
    let mut session = if session_path.exists() {
        println!("Loading existing session...");
        Session::load(session_path)?
    } else {
        println!("Initializing new session...");
        let mut s = Session::new();
        s.set_active_profile("chief", "system");
        s.set_task("Phase 3: Worktree Safety Implementation", "user");
        s.add_fact("Environment assumptions validated via probes", "chief", ProvenanceStatus::Verified);
        s.add_fact("Rust core scaffolding complete", "builder", ProvenanceStatus::Verified);
        s.add_fact("Provenance-aware State Core complete", "builder", ProvenanceStatus::Verified);
        s
    };

    println!("Session ID: {}", session.session_id);
    
    println!("Attempting to acquire lock for profile 'builder'...");
    let lock = session.acquire_lock(std::env::current_dir()?, "builder")?;
    println!("Lock acquired: {:?}", lock);

    session.touch();
    session.save(session_path)?;
    
    println!("\n--- Handoff Artifact ---");
    println!("{}", session.generate_handoff());
    println!("------------------------\n");

    Ok(())
}
