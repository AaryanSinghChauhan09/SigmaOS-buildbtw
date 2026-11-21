//! Error types for the buildbtw component.

/// Errors that can occur in buildbtw job handling.
#[derive(Debug, thiserror::Error)]
pub enum BuildbtwError {
    /// A job referenced an unknown package or repo.
    #[error("unknown package: {0}")]
    UnknownPackage(String),

    /// Invalid job state transition.
    #[error("invalid state transition: {0} -> {1}")]
    InvalidTransition(String, String),

    /// I/O error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
