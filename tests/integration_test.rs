//! Integration tests for sigmaos-buildbtw.

use buildbtw::{BuildJob, JobQueue, JobState, RepoArch};

#[test]
fn test_job_lifecycle() {
    let mut j = BuildJob::new(1, "foo", vec![], "1.0-1");
    j.transition(JobState::Building).unwrap();
    j.transition(JobState::Built).unwrap();
    j.transition(JobState::Verified).unwrap();
    j.transition(JobState::Published).unwrap();
    assert_eq!(j.state, JobState::Published);
}

#[test]
fn test_queue_scheduling() {
    let mut q = JobQueue::new();
    let ra = RepoArch { repo: "extra".into(), arch: "x86_64".into() };
    q.enqueue("pkg-a", vec![ra.clone()], "1.0");
    q.enqueue("pkg-b", vec![ra], "2.0");
    q.enqueue("pkg-c", vec![], "3.0");
    let r = q.schedule_pending(2);
    assert_eq!(r.scheduled.len(), 2);
    assert_eq!(r.skipped.len(), 1);
    assert_eq!(q.by_state(JobState::Pending).len(), 1);
}

#[test]
fn test_build_commands_per_target() {
    let j = BuildJob::new(1, "foo", vec![
        RepoArch { repo: "extra".into(), arch: "x86_64".into() },
        RepoArch { repo: "multilib".into(), arch: "x86_64".into() },
    ], "1.0");
    let cmds = j.build_commands();
    assert_eq!(cmds.len(), 2);
    assert!(cmds[0].2.contains("extra/x86_64"));
    assert!(cmds[1].2.contains("multilib/x86_64"));
}

#[test]
fn test_retry_failed_job() {
    let mut q = JobQueue::new();
    let id = q.enqueue("x", vec![], "1.0");
    q.schedule_pending(4);
    let j = q.get_mut(id).unwrap();
    j.transition(JobState::Built).unwrap();
    j.transition(JobState::Failed).unwrap();
    j.transition(JobState::Pending).unwrap();
    assert_eq!(q.by_state(JobState::Pending).len(), 1);
}
