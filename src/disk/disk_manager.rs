use crate::disk::*;
use crate::{PAGE_SIZE, PageID};
use std::{
    collections::VecDeque,
    fs::OpenOptions,
    io::{self, Read, Seek, Write},
};

impl DiskManager {
    /// Create a DiskManager to manage a database file on disk
    ///
    /// Always start with an empty file. Empty the file if it already exists.
    /// `next_free` should start at `PageID(1)`
    ///
    /// # Errors
    ///
    /// Will return [`io::Error`] if opening `filename` returns an error.
    pub fn new(filename: &str) -> Result<Self, io::Error> {
        todo!()
    }

    /// Get a PageID for a new page, either from the `free_list` or using `next_free`.
    pub fn allocate(&mut self) -> PageID {
        todo!()
    }

    /// Mark the given page id as free
    ///
    /// This allows reusing the page id for new pages.
    ///
    /// # Errors
    /// Returns [`DiskManagerError::InvalidPageID`] if `page_id` is not in the
    /// interval of allocated pages or if the page is already on the free list.
    pub fn free(&mut self, page_id: PageID) -> Result<(), DiskManagerError> {
        todo!()
    }

    /// Reads a page from the database file on disk
    ///
    /// PageID serves as an offset to the position of the page in the file.
    ///
    /// # Errors
    /// - Returns [`DiskManagerError::InvalidPageID`] if `page_id` is not in the
    ///   interval of allocated pages or if the page is on the free list.
    /// - Return [`DiskManagerError::IOError`] if file operations return an [`io::Error`].
    pub fn read(&mut self, page_id: PageID, buf: &mut RawPage) -> Result<(), DiskManagerError> {
        todo!()
    }

    /// Writes page to the database file on disk
    ///
    /// PageID serves as an offset to the position of the page in the file.
    /// Ensure all data is written to disk by calling fsync.
    ///
    /// # Errors
    /// - Returns [`DiskManagerError::InvalidPageID`] if `page_id` is not in the
    ///   interval of allocated pages or if the page is on the free list.
    /// - Return [`DiskManagerError::IOError`] if file operations return an [`io::Error`].
    pub fn write(&mut self, page_id: PageID, buf: &RawPage) -> Result<(), DiskManagerError> {
        todo!()
    }
}
