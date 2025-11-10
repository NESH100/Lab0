// This file will be replaced by the runner

//! Simplified Disk Manager of a databases system
//!
//! Represents a data structure to track persisted data on disk.
//! Provides interfaces to store and retrieve pages from disk.
//! It encapsulates the logic required to interact with the disk/the operating system.

use crate::{PAGE_SIZE, PageID};
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use thiserror::Error;

/// Type for byte arrays representing raw pages.
pub type RawPage = [u8; PAGE_SIZE];

/// DiskManager-related Errors
///
/// Errors that can happen during interaction with the disk.
/// We use a specific error enum here to make disk-related error explicit.
#[derive(Error, Debug)]
pub enum DiskManagerError {
    #[error("invalid page ID: {0}!")]
    InvalidPageID(PageID),
    #[error(transparent)]
    IOError(#[from] io::Error),
}

/// The DiskManager is used to store and retrieve persisted database data
///
/// Data is stored in fix-sized blocks of bytes, called pages.
/// In our simplified implementation, these pages are stored within a single file.
/// The DiskManager keeps track of used and unused pages.
#[derive(Debug)]
pub struct DiskManager {
    /// Handle to a database file on disk
    file: File,
    /// Highest allocated [`PageID`] + 1. Never decreases.
    next_free: PageID,
    /// Used to keep track of pages that are not in use anymore
    ///
    /// Add freed pages to `free_list`
    free_list: VecDeque<PageID>,
}

// The tests
mod advanced_tests_disk_manager;
mod basic_tests_disk_manager;

// The implementations
pub mod disk_manager;
