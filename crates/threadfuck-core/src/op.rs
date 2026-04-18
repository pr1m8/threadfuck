//! Instruction set for the Threadfuck language.
//!
//! Purpose
//! -------
//! Defines the core operation set for Threadfuck and provides conversion
//! from source characters into typed operations.
//!
//! Design
//! ------
//! Threadfuck preserves Brainfuck's core operations and extends them with
//! a minimal concurrency-oriented instruction set. Each valid source
//! character maps to exactly one ``Op`` variant.

use crate::error::ThreadfuckError;

/// A single parsed threadfuck instruction.
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum Op { 
    /// Move the current task's data pointer one cell to the right.
    MoveRight,
    
    /// Move the current task's data pointer one cell to the left. 
    MoveLeft,
    
    /// Increment the value in the current tape cell. 
    Increment,
    
    /// Decrement the value in the current tape cell.
    Decrement,

    /// Write the current tape cell to the output stream.
    Output,

    /// Read one byte into the current tape cell.
    Input,

    /// Begin a conditional loop.
    LoopStart,

    /// End a conditional loop.
    LoopEnd,

    /// Spawn a new task from the current execution context.
    ForkTask,

    /// Yield execution voluntarily.
    Yield,

    /// Halt the current task.
    Halt,
}