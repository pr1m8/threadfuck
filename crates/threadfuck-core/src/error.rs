//! Error types for the Threadfuck core crate.
//!
//! Purpose
//! -------
//! Defines the shared error type used across parsing and execution layers.
//!
//! Design
//! ------
//! The core crate uses a single error enum so that parser, program-building,
//! and runtime code can share a common error surface while still preserving
//! precise failure reasons.

use thiserror::Error;

/// Error type for Threadfuck parsing and execution
#[derive(Debug,Error,PartialEq,Eq)]
pub enum ThreadfuckError {
    /// A source character was not a valid opcode.
    #[error("invalid opcode: {0:?}")]
    InvalidOpcode(char),

    // A loop end was encountered without a matching loop start.
    #[error("unmatched loop end at instruction index")]
    UnmatchedLoopEnd {index: usize},

    /// One or more loop starts were not closed.
    #[error("unmatched loop start at instruction index")]
    UnmatchedLoopStart {index: usize},
}