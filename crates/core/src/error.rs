//! Core error types for Project Brain.
//!
//! All crates should map their internal errors to `BrainError`
//! for a consistent error interface.

use thiserror::Error;

/// The unified error type for Project Brain operations.
#[derive(Debug, Error)]
pub enum BrainError {
    /// The requested entity was not found.
    #[error("Entity not found: {entity_type} with id '{id}'")]
    NotFound {
        /// The type of entity (e.g., "node", "relationship").
        entity_type: String,
        /// The ID that was looked up.
        id: String,
    },

    /// A duplicate entity already exists.
    #[error("Duplicate {entity_type}: '{id}' already exists")]
    Duplicate {
        /// The type of entity.
        entity_type: String,
        /// The conflicting ID.
        id: String,
    },

    /// A validation error (e.g., invalid node type, missing required field).
    #[error("Validation error: {message}")]
    Validation {
        /// Human-readable description of the validation failure.
        message: String,
    },

    /// A storage/database error.
    #[error("Storage error: {message}")]
    Storage {
        /// Human-readable description of the storage failure.
        message: String,
    },

    /// An error during serialization or deserialization.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// A generic internal error.
    #[error("Internal error: {message}")]
    Internal {
        /// Human-readable description.
        message: String,
    },
}

impl BrainError {
    /// Create a NotFound error.
    pub fn not_found(entity_type: impl Into<String>, id: impl Into<String>) -> Self {
        Self::NotFound {
            entity_type: entity_type.into(),
            id: id.into(),
        }
    }

    /// Create a Duplicate error.
    pub fn duplicate(entity_type: impl Into<String>, id: impl Into<String>) -> Self {
        Self::Duplicate {
            entity_type: entity_type.into(),
            id: id.into(),
        }
    }

    /// Create a Validation error.
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    /// Create a Storage error.
    pub fn storage(message: impl Into<String>) -> Self {
        Self::Storage {
            message: message.into(),
        }
    }

    /// Create an Internal error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }
}

/// Convenience type alias for Results using BrainError.
pub type BrainResult<T> = Result<T, BrainError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_found_error_message() {
        let err = BrainError::not_found("node", "abc-123");
        assert_eq!(err.to_string(), "Entity not found: node with id 'abc-123'");
    }

    #[test]
    fn duplicate_error_message() {
        let err = BrainError::duplicate("relationship", "rel-1");
        assert_eq!(
            err.to_string(),
            "Duplicate relationship: 'rel-1' already exists"
        );
    }

    #[test]
    fn validation_error_message() {
        let err = BrainError::validation("title cannot be empty");
        assert_eq!(err.to_string(), "Validation error: title cannot be empty");
    }

    #[test]
    fn storage_error_message() {
        let err = BrainError::storage("database locked");
        assert_eq!(err.to_string(), "Storage error: database locked");
    }

    #[test]
    fn serde_json_error_converts() {
        let bad_json = "not json";
        let result: Result<serde_json::Value, _> = serde_json::from_str(bad_json);
        let brain_err: BrainError = result.unwrap_err().into();
        assert!(brain_err.to_string().starts_with("Serialization error:"));
    }
}
