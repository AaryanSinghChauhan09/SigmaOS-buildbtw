//! Package build orchestration service
//!
//! SigmaOS component inspired by Arch Linux's `buildbtw` repository.
//! Designed for component isolation, loose coupling, and secure defaults.

use std::fmt;

/// Errors produced by this component. Fail-fast: errors are returned, not panicked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildbtwError {
    /// The requested operation is not valid in the current state.
    InvalidState,
    /// Input failed validation.
    InvalidInput(String),
}

impl fmt::Display for BuildbtwError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidState => write!(f, "invalid state"),
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
        }
    }
}

impl std::error::Error for BuildbtwError {}

/// Core component type. Holds configuration; performs no I/O by default.
#[derive(Debug, Clone, Default)]
pub struct Buildbtw {
    name: String,
}

impl Buildbtw {
    /// Create a new component instance. Fails fast on empty names.
    pub fn new(name: impl Into<String>) -> Result<Self, BuildbtwError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(BuildbtwError::InvalidInput("name must not be empty".into()));
        }
        Ok(Self { name })
    }

    /// Component identifier.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Perform the component's primary action. Returns a deterministic result.
    pub fn run(&self) -> Result<String, BuildbtwError> {
        if self.name.is_empty() {
            return Err(BuildbtwError::InvalidState);
        }
        Ok(format!("{}: ok", self.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_name() {
        // Arrange / Act / Assert
        assert!(matches!(Buildbtw::new("  "), Err(BuildbtwError::InvalidInput(_))));
    }

    #[test]
    fn run_returns_deterministic_result() {
        let c = Buildbtw::new("test").unwrap();
        assert_eq!(c.run().unwrap(), "test: ok");
    }

    #[test]
    fn name_is_accessible() {
        let c = Buildbtw::new("abc").unwrap();
        assert_eq!(c.name(), "abc");
    }
}
