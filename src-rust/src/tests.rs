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
