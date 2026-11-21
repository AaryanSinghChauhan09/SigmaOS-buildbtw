//! Job queue + scheduling report, mirroring upstream buildbtw worker model.

use std::collections::BTreeMap;

use crate::job::{BuildJob, JobState, RepoArch};

/// Outcome of a scheduling pass.
#[derive(Debug, Default)]
pub struct ScheduleReport {
    /// Jobs scheduled this pass.
    pub scheduled: Vec<u64>,
    /// Jobs skipped (already in flight or done).
    pub skipped: Vec<u64>,
    /// Jobs failed to schedule.
    pub failed: Vec<(u64, String)>,
}

/// In-memory job queue keyed by job ID.
#[derive(Debug, Default)]
pub struct JobQueue {
    jobs: BTreeMap<u64, BuildJob>,
    next_id: u64,
}

impl JobQueue {
    /// Empty queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enqueue a pending job for `pkgbase` at `new_version`.
    pub fn enqueue(&mut self, pkgbase: &str, targets: Vec<RepoArch>, new_version: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.jobs.insert(id, BuildJob::new(id, pkgbase, targets, new_version));
        id
    }

    /// Get a job by ID.
    pub fn get(&self, id: u64) -> Option<&BuildJob> {
        self.jobs.get(&id)
    }

    /// Get a mutable job by ID.
    pub fn get_mut(&mut self, id: u64) -> Option<&mut BuildJob> {
        self.jobs.get_mut(&id)
    }

    /// All jobs in a given state.
    pub fn by_state(&self, state: JobState) -> Vec<&BuildJob> {
        self.jobs.values().filter(|j| j.state == state).collect()
    }

    /// Pending jobs ready to schedule.
    pub fn pending(&self) -> Vec<&BuildJob> {
        self.by_state(JobState::Pending)
    }

    /// Schedule all pending jobs (mark Building), returning a report.
    ///
    /// `max_concurrent` caps how many may be in-flight; extras stay pending.
    pub fn schedule_pending(&mut self, max_concurrent: usize) -> ScheduleReport {
        let mut report = ScheduleReport::default();
        let in_flight = self.by_state(JobState::Building).len();
        let mut slots = max_concurrent.saturating_sub(in_flight);
        let pending_ids: Vec<u64> = self.pending().iter().map(|j| j.id).collect();
        for id in pending_ids {
            if slots == 0 {
                report.skipped.push(id);
                continue;
            }
            if let Some(job) = self.jobs.get_mut(&id) {
                match job.transition(JobState::Building) {
                    Ok(()) => {
                        report.scheduled.push(id);
                        slots -= 1;
                    }
                    Err(e) => report.failed.push((id, e.to_string())),
                }
            }
        }
        report
    }

    /// Jobs needing attention: failed or built-but-unverified.
    pub fn needs_attention(&self) -> Vec<&BuildJob> {
        self.jobs
            .values()
            .filter(|j| matches!(j.state, JobState::Failed | JobState::Built))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::RepoArch;

    #[test]
    fn test_enqueue_and_schedule() {
        let mut q = JobQueue::new();
        let ra = RepoArch { repo: "extra".into(), arch: "x86_64".into() };
        q.enqueue("foo", vec![ra], "1.0");
        q.enqueue("bar", vec![], "2.0");
        let report = q.schedule_pending(1);
        assert_eq!(report.scheduled.len(), 1);
        assert_eq!(report.skipped.len(), 1);
        assert_eq!(q.by_state(JobState::Building).len(), 1);
    }

    #[test]
    fn test_needs_attention() {
        let mut q = JobQueue::new();
        let id = q.enqueue("foo", vec![], "1.0");
        q.schedule_pending(4);
        q.get_mut(id).unwrap().transition(JobState::Built).unwrap();
        q.get_mut(id).unwrap().transition(JobState::Failed).is_err(); // Built->Failed legal
        let attn = q.needs_attention();
        assert_eq!(attn.len(), 1);
    }
}
