//! Core library for the Threadfuck interpreter.

pub mod error;
pub mod op;
pub mod parser;
pub mod program;
pub mod tape;

pub use error::ThreadfuckError;
pub use op::Op;
pub use parser::parse_program;
pub use program::Program;
pub use tape::Tape;