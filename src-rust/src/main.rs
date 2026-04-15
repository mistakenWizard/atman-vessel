use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub provider: String, // "gemini" | "codex"
    pub root: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub version: String,
    pub session_id: String,
    pub current_task: Option<String>,
    pub active_profile: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub profiles: HashMap<String, Profile>,
    pub metadata: HashMap<String, String>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            session_id: uuid::Uuid::new_v4().to_string(),
            current_task: None,
            active_profile: None,
            last_updated: Utc::now(),
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

    pub fn set_active_profile(&mut self, profile_name: &str) {
        self.active_profile = Some(profile_name.to_string());
        self.touch();
    }
}

fn main() -> Result<()> {
    println!("AtmanVessel Orchestrator Core MVP v1.0");

    let session_path = Path::new(".orchestrator/session.json");
    let mut session = if session_path.exists() {
        println!("Loading existing session...");
        Session::load(session_path)?
    } else {
        println!("Initializing new session...");
        Session::new()
    };

    println!("Session ID: {}", session.session_id);
    println!("Active Profile: {:?}", session.active_profile.as_deref().unwrap_or("None"));
    
    // Example: updating state (replaces the need for sed)
    session.touch();
    session.save(session_path)?;
    
    println!("State saved. Last updated: {}", session.last_updated);

    Ok(())
}
