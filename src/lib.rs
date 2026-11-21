//! SigmaOS buildbtw component.
//!
//! Rust implementation of `archlinux/buildbtw`: the service that builds new
//! versions of Arch packages in clean chroots and publishes results.
//!
//! Ported core: the build-job model — job states, scheduling, chroot build
//! command planning, and result packaging — matching upstream's pipeline
//! (detect new pkgver → schedule build → verify → report).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod job;
pub mod queue;

pub use error::BuildbtwError;
pub use job::{BuildJob, JobState, RepoArch};
pub use queue::{JobQueue, ScheduleReport};
