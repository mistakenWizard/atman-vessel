use super::*;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn setup_test_dir(name: &str) -> PathBuf {
    let path = std::env::current_dir().unwrap().join("target").join(format!("test_{}", name));
    if path.exists() {
        fs::remove_dir_all(&path).ok();
    }
    fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn test_stale_state_detection() {
    let test_dir = setup_test_dir("stale_state");
    let session_path = test_dir.join("session.json");

    let session = Session::new();
    session.save(&session_path).expect("Initial save failed");

    // Load session in memory (T1)
    let session_in_mem = Session::load(&session_path).expect("Load failed");
    
    // Simulate another process updating the session on disk (T2 > T1)
    // We wait a bit to ensure the timestamp is different (if granularity is low)
    thread::sleep(Duration::from_millis(100));
    let mut session_on_disk = Session::load(&session_path).expect("Second load failed");
    session_on_disk.touch();
    session_on_disk.save(&session_path).expect("On-disk save failed");

    // Attempting to save session_in_mem should now fail because it's stale
    let result = session_in_mem.save(&session_path);
    assert!(result.is_err(), "Should have failed to save stale session");
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("State on disk is newer"), "Error message '{}' should mention stale state", err_msg);

    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_recovery_artifact_integrity() {
    let mut session = Session::new();
    session.session_id = "test-session-123".to_string();
    session.state_status = StateStatus::Crashed;
    session.touch();

    // This method is defined in Phase 3 design 2.2
    let recovery_md = session.generate_recovery();
    
    assert!(recovery_md.contains("test-session-123"), "Recovery should contain session_id");
    assert!(recovery_md.contains(&session.last_updated.to_rfc3339()), "Recovery should contain timestamp");
    assert!(recovery_md.contains("CRITICAL RECOVERY"), "Recovery should have a clear header");
}

#[test]
fn test_takeover_lock_replacement() {
    let test_dir = setup_test_dir("takeover_lock");
    // The lock directory is relative to the worktree root in actual usage, 
    // but here we'll mock the paths as needed by the takeover logic.
    let lock_dir = test_dir.join(".orchestrator/locks");
    fs::create_dir_all(&lock_dir).unwrap();
    
    let profile = "test-profile";
    let lock_path = lock_dir.join(format!("{}.json", profile));

    // 1. Create a "live" lock (using our own PID)
    let my_pid = std::process::id();
    let session1 = Session::new();
    let mut lock = Lock::new(test_dir.clone(), &session1.session_id, profile);
    lock.pid = my_pid; 
    lock.save(&lock_path).expect("Failed to save initial lock");

    // 2. Attempt takeover without force - should fail since it's "live"
    let mut session2 = Session::new(); 
    // We expect a method takeover_lock in Session to handle the core logic
    let result = session2.takeover_lock(&test_dir, profile, false);
    assert!(result.is_err(), "Takeover of live lock without force should fail");

    // 3. Create a "stale" lock (using a likely non-existent PID)
    let stale_pid = 999999; // Mocking a stale PID
    let mut lock_stale = Lock::new(test_dir.clone(), &session1.session_id, profile);
    lock_stale.pid = stale_pid;
    lock_stale.save(&lock_path).expect("Failed to save stale lock");

    // 4. Attempt takeover of stale lock without force - should succeed
    let result_stale = session2.takeover_lock(&test_dir, profile, false);
    assert!(result_stale.is_ok(), "Takeover of stale lock should succeed even without force. Error: {:?}", result_stale.err());

    // 5. Verify new lock has new session_id
    let new_lock = Lock::load(&lock_path).expect("Failed to load new lock");
    assert_eq!(new_lock.session_id, session2.session_id);
    
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_auth_bootstrapping_integrity() {
    let test_dir = setup_test_dir("auth_bootstrapping");
    let source_dir = test_dir.join("source_gemini");
    let target_dir = test_dir.join("target_profile");
    
    fs::create_dir_all(&source_dir).unwrap();
    
    let files = [
        "google_accounts.json",
        "oauth_creds.json",
        "settings.json",
        "projects.json",
        "state.json",
    ];
    
    for file in &files {
        fs::write(source_dir.join(file), format!("content for {}", file)).unwrap();
    }
    
    bootstrap_profile_from_source(&source_dir, &target_dir).expect("Bootstrapping failed");
    
    for file in &files {
        let target_file = target_dir.join(file);
        assert!(target_file.exists(), "File {} missing in target", file);
        let content = fs::read_to_string(target_file).unwrap();
        assert_eq!(content, format!("content for {}", file));
    }
    
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_auth_bootstrapping_no_overwrite() {
    let test_dir = setup_test_dir("auth_no_overwrite");
    let source_dir = test_dir.join("source_gemini");
    let target_dir = test_dir.join("target_profile");
    
    fs::create_dir_all(&source_dir).unwrap();
    fs::create_dir_all(&target_dir).unwrap();
    
    let file = "google_accounts.json";
    fs::write(source_dir.join(file), "new content").unwrap();
    fs::write(target_dir.join(file), "original content").unwrap();
    
    bootstrap_profile_from_source(&source_dir, &target_dir).expect("Bootstrapping failed");
    
    let content = fs::read_to_string(target_dir.join(file)).unwrap();
    assert_eq!(content, "original content", "Should not have overwritten existing file");
    
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_command_environment_redirection() {
    let test_dir = setup_test_dir("env_redirection");
    let profile_root = test_dir.join("profile_abc");
    fs::create_dir_all(&profile_root).unwrap();
    
    let cmd = prepare_launch_command("gemini", &profile_root, &[]);
    
    let envs: Vec<_> = cmd.get_envs().collect();
    let home_env = envs.iter().find(|(k, _)| k.to_str() == Some("HOME"));
    
    assert!(home_env.is_some(), "HOME environment variable not set");
    let (_, val) = home_env.unwrap();
    assert_eq!(val.unwrap(), profile_root.as_os_str());
    
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_pre_launch_state_transition() {
    let test_dir = setup_test_dir("state_transition");
    let session_path = test_dir.join("session.json");
    
    let mut session = Session::new();
    session.state_status = StateStatus::Planned;
    session.save(&session_path).unwrap();
    
    // Reload to update loaded_at and avoid "state on disk is newer" error
    let mut session = Session::load(&session_path).unwrap();
    session.state_status = StateStatus::Running;
    session.save(&session_path).unwrap();
    
    // Verify on disk
    let saved_session = Session::load(&session_path).unwrap();
    assert_eq!(saved_session.state_status, StateStatus::Running);
    
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_pre_launch_handoff_generation() {
    let test_dir = setup_test_dir("handoff_generation");
    let handoff_path = test_dir.join("HANDOFF.md");
    
    let mut session = Session::new();
    session.current_task = Some(Provenance::new("test task".to_string(), "user", &session.session_id, ProvenanceStatus::Verified));
    
    // Simulate pre-launch logic
    fs::write(&handoff_path, session.generate_handoff()).unwrap();
    
    // Verify handoff exists and contains expected content
    assert!(handoff_path.exists());
    let content = fs::read_to_string(&handoff_path).unwrap();
    assert!(content.contains("test task"));
    assert!(content.contains(&session.session_id));
    
    fs::remove_dir_all(test_dir).ok();
}
