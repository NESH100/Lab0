// This file will be replaced by the runner

//! The buffer manager is a layer between higher database operations and the storage layer.
//!
//! It is used to cache pages in main memory. Thus, frequently used pages are read once from the
//! disk instead of at every access. This module defines two strategies to evict pages from the
//! buffer manager if it is at maximum capacity: Least Recently Used (LRU) and CLOCK.
//!
//! This file will be overridden in the testing and grading pipeline, so although you can change it
//! on your machine and inside the Git repository, these changes will not persist during testing
//! and grading in the pipeline. All files that are overridden by the pipeline are marked with
//! `// This file will be replaced by the runner` on the first line.
//!
//! Implement your solution in `buffer_manager.rs`.

use crate::PageID;
use crate::buffer::buffer_manager::FrameDescriptor;
use crate::buffer::frame_pool::FramePool;
use crate::disk::DiskManagerError;
use thiserror::Error;

/// Errors for BufferManager operations
#[derive(Error, Debug, PartialEq)]
pub enum BufferManagerError {
    #[error("buffer pool ran out of free frames!")]
    AllPagesPinned,
    #[error("invalid page ID: {0}!")]
    InvalidPageID(PageID),
    #[error("an I/O error occurred in the underlying disk manager!")]
    IOError,
    #[error("an unknown error occurred!")]
    Unknown,
}

/// Conversion from relevant [`DiskManagerError`]s to [`BufferManagerError`]s
impl From<DiskManagerError> for BufferManagerError {
    fn from(value: DiskManagerError) -> Self {
        match value {
            DiskManagerError::InvalidPageID(page_id) => BufferManagerError::InvalidPageID(page_id),
            DiskManagerError::IOError(_) => BufferManagerError::IOError,
        }
    }
}

/// The size remaining in a page after subtracting the size of the header from [`crate::PAGE_SIZE`].
pub const DATA_SIZE: usize = crate::PAGE_SIZE - size_of::<PageID>();

/// Page returned or consumed by [`BufferManager`].
///
/// It consists of a PageID as the header and an u8 array of size [`DATA_SIZE`] containing the page
/// data.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MaterializedPage(PageID, [u8; DATA_SIZE]);

/// Implement the `Default` trait for `MaterializedPage` to make it easier to create new pages.
///
/// # Example
/// ```
/// let page = MaterializedPage::default();
/// ```
impl Default for MaterializedPage {
    fn default() -> Self {
        MaterializedPage(PageID(0), [0; DATA_SIZE])
    }
}

impl MaterializedPage {
    /// Create a new Materialized page with the given page ID.
    ///
    /// # Arguments
    ///
    /// * `page_id`: PageID for the new page.
    ///
    /// returns: MaterializedPage
    fn new(page_id: PageID) -> Self {
        MaterializedPage(page_id, [0; DATA_SIZE])
    }
}

/// Trait defining the interface of a [`BufferManager`].
///
/// In this project, your [`BufferManager`] must implement two methods:
///
/// - [`BufferManagerTrait::pin`] for pinning a page in the buffer manager and getting its
///   reference.
/// - [`BufferManagerTrait::unpin`] for unpinning a page in the buffer manager and marking it as
///   dirty if necessary.
///
/// In the coding labs, we will regularly provide you with structs and traits given for tasks that
/// you have to implement. Defining the trait allows us to test your implementation by the
/// interface defined here.
pub trait BufferManagerTrait {
    /// Pins page *pid* and returns a mutable reference to it.
    ///
    /// # Safety
    /// For every [`BufferManagerTrait::pin`] call for a page ID, there must be a corresponding
    /// [`BufferManagerTrait::unpin`] call with the **same** page ID.
    ///
    /// # Error
    /// - Propagates errors from [`DiskManager::write`] and [`DiskManager::read`].
    /// - If all pages are pinned at least once, [`BufferManagerError::AllPagesPinned`] is returned.
    fn pin(&mut self, pid: PageID) -> Result<&mut MaterializedPage, BufferManagerError>;

    /// Unpins page *pid* and sets the dirty bit as given by the *dirty* parameter.
    ///
    /// # Safety
    /// For every [`BufferManagerTrait::unpin`] call for a page ID, there must be a corresponding
    /// [`BufferManagerTrait::pin`] call with the **same** page ID. Otherwise, life-locks can occur.
    ///
    /// # Panics
    /// Panics if page ID of `page` is not loaded in [`BufferManager`].
    fn unpin(&mut self, page_id: PageID, dirty: bool);
}

/// Simplified interface trait for a DiskManager. Allows us to mock the DiskManager in tests.
/// You do not need to implement this.
pub trait DiskManagerTrait {
    /// Read `page_id` into `buf`.
    ///
    /// # Errors
    /// Returns [`DiskManagerError::InvalidPageID`] if `page_id` is not in a valid range.
    fn read(&mut self, page_id: PageID, buf: &mut MaterializedPage)
    -> Result<(), DiskManagerError>;

    /// Write `buf` at the address `page_id`.
    ///
    /// # Errors
    /// Returns [`DiskManagerError::InvalidPageID`] if `page_id` is not in a valid range.
    fn write(&mut self, page_id: PageID, buf: &MaterializedPage) -> Result<(), DiskManagerError>;
}

/// A dummy implementation of [`DiskManagerTrait`] for testing purposes.
#[derive(Debug)]
struct DummyDiskManager {
    pages: Vec<MaterializedPage>,
}

/// A dummy implementation of [`DiskManagerTrait`] for testing purposes.
impl DiskManagerTrait for DummyDiskManager {
    fn read(
        &mut self,
        page_id: PageID,
        buf: &mut MaterializedPage,
    ) -> Result<(), DiskManagerError> {
        let page = self
            .pages
            .get(page_id.0)
            .ok_or(DiskManagerError::InvalidPageID(page_id))?;

        *buf = page.clone();

        Ok(())
    }

    fn write(&mut self, page_id: PageID, buf: &MaterializedPage) -> Result<(), DiskManagerError> {
        let page = self
            .pages
            .get_mut(page_id.0)
            .ok_or(DiskManagerError::InvalidPageID(page_id))?;

        *page = buf.clone();

        Ok(())
    }
}

/// Trait defining the interface of a replacement strategy.
pub trait ReplacementStrategyTrait {
    /// Core method of a replacement strategy. Returns the page ID of the page to be replaced or an
    /// error if no page can be replaced.
    ///
    /// # Arguments
    ///
    /// * `fds`: A mutable reference to frame descriptors of the buffer manager.
    ///
    /// # Returns: Result<PageID, BufferManagerError>
    ///
    /// Returns the PageID to replace next.
    ///
    /// # Errors
    ///
    /// Returns [`BufferManagerError::AllPagesPinned`] if all pages are pinned and cannot be
    /// replaced.
    fn replace(
        &mut self,
        fds: &mut FramePool<FrameDescriptor>,
    ) -> Result<PageID, BufferManagerError>;

    /// Applies the changes of the replacement strategy state and the frame descriptor required by
    /// the replacement strategy on pin.
    ///
    /// Must be called in [BufferManager::pin] to manage the frame descriptor. This is required for
    /// testing.
    ///
    /// # Arguments
    ///
    /// * `frame_descriptor`: A mutable reference to frame descriptor to update.
    ///
    /// returns: ()
    ///
    /// # Important
    /// ONLY change attributes related to the replacement strategy in this function. Do not touch
    /// page_id, pin_count, or dirty!
    fn on_pin(&mut self, frame_descriptor: &mut FrameDescriptor);
}

/// A dummy implementation of [`ReplacementStrategyTrait`] for testing purposes.
#[derive(Default)]
struct DummyReplacementStrategy {}

/// A dummy implementation of [`ReplacementStrategyTrait`] for testing purposes.
impl ReplacementStrategyTrait for DummyReplacementStrategy {
    /// Panics if called
    fn replace(
        &mut self,
        _fds: &mut FramePool<FrameDescriptor>,
    ) -> Result<PageID, BufferManagerError> {
        unimplemented!()
    }

    fn on_pin(&mut self, _frame_descriptor: &mut FrameDescriptor) {}
}

// The tests
mod advanced_tests_buffer_manager;
mod basic_tests_buffer_manager;

// The implementations
pub mod buffer_manager;
mod frame_pool;
