// This file will be replaced by the runner

//! SDMS Lab 0
//!
//! In this lab, you will implement a (simplified) Disk Manager and a (simplified) Buffer Manager.
//! Look into the disk and buffer directories for the tasks.
//!
//! This file contains some common type definitions for both tasks.

use std::fmt::Display;

pub mod buffer;
pub mod disk;

/// 1 Kibibyte constant.
pub const KIBI_BYTES: usize = 1024;

/// Size of a raw page in bytes.
pub const PAGE_SIZE: usize = 4 * KIBI_BYTES;

/// Number of frames in buffer pool.
pub const BUFFER_POOL_SIZE: usize = 1024;

/// Newtype pattern for PageID
///
/// We use the newtype pattern to implement PageID.
/// See https://www.lurklurk.org/effective-rust/newtype.html
/// Using this pattern (instead of an alias) avoids that incorrect IDs are assigned as PageIDs.
/// E.g., with a type alias of usize, nothing prevents us from assigning, e.g., a FrameID as a
/// PageID.
///
/// PageID is required to compute the offset of a data page on disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Default, Ord)]
pub struct PageID(pub usize);

/// Implement the display trait for PageID.
/// This generates an implementation for .to_string() for user-facing output.
impl Display for PageID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type for frame indices
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Ord, Default)]
pub struct FrameID(pub usize);

/// Implement the display trait for FrameID.
/// This generates an implementation for .to_string() for user-facing output.
impl Display for FrameID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
