// This file will be replaced by the runner

use crate::FrameID;
use std::ops::{Index, IndexMut};

/// A strongly-typed wrapper around [`Box<[T]>`] that allows indexing with [`FrameID`] and
/// [`&FrameID`].
///
/// Is used with both [`MaterializedPage`] (see examples below) and [`FrameDescriptor`].
///
/// Use [`FramePool::iter()`] to get an iterator over the contents.
///
/// Use [`FramePool::iter_mut()`] to get an iterator over the mutable references to the
/// contents.
///
/// ## Usage examples:
/// ```
/// // Create a FramePool of Materialized Pages
/// use sdms_lab_0::buffer::MaterializedPage;
/// let page = MaterializedPage{};
/// let page_vec = vec![page, page, page];
/// let page_pool = FramePool::new(page_vec.into_boxed_slice());
///
/// // Get an immutable reference of a page by its FrameID
/// let page_ref: &MaterializedPage = &page_pool[FrameID(1)];
///
/// // Get a mutable reference of a page by its FrameID
/// let page_mut_ref: &mut MaterializedPage = &mut page_pool[FrameID(2)];
///
/// // Get an immutable reference of a page by a reference to its FrameID
/// let page_ref: &MaterializedPage = &page_pool[&FrameID(1)];
///
/// // Get a mutable reference of a page by a reference to its FrameID
/// let page_mut_ref: &mut MaterializedPage = &mut page_pool[&FrameID(2)];
///
/// // Iterate over all pages
/// for page in page_pool.iter() {
///     // Do something with the page
/// }
///
/// // Iterate over all mutable pages
/// for page in page_pool.iter_mut() {
///     // Do something with the page, including changing it
/// }
/// ```
pub struct FramePool<T> {
    inner: Box<[T]>,
}

impl<T> FramePool<T> {
    /// Creates a new FramePool by moving a boxed slice into it.
    ///
    /// Create the underlying boxed slice by first creating a `Vec<T>` and then calling
    /// [`Vec::into_boxed_slice`]
    ///
    /// ## Example
    /// ```
    /// let page = MaterializedPage{};
    /// let page_vec = vec![page, page, page];
    /// let page_pool = FramePool::new(page_vec.into_boxed_slice());
    /// ```
    pub fn new(inner: Box<[T]>) -> Self {
        FramePool { inner }
    }

    /// Returns the size/length of the FramePool.
    ///
    /// ## Example
    /// ```
    /// let page = MaterializedPage{};
    /// let page_vec = vec![page, page, page];
    /// let page_pool = FramePool::new(page_vec.into_boxed_slice());
    /// assert_eq!(page_pool.len(), 3)
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns an iterator over the elements
    ///
    /// # Example
    /// ```
    /// for page in page_pool.iter() {
    ///     // Do something with the page
    /// }
    /// ```
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.inner.iter()
    }

    /// Returns a mutable iterator over the elements
    ///
    /// # Example
    /// ```
    /// for page in page_pool.iter_mut() {
    ///     // Do something with the page
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.inner.iter_mut()
    }
}

/// Implements non-mutable indexing with [`FrameID`]
///
/// # Examples
/// ```
/// // Get an immutable reference of a page by its FrameID
/// let page_ref: &MaterializedPage = &page_pool[FrameID(1)];
/// ```
impl<T> Index<FrameID> for FramePool<T> {
    type Output = T;

    fn index(&self, index: FrameID) -> &Self::Output {
        &self.inner[index.0]
    }
}

/// Implements mutable indexing with [`FrameID`]
///
/// # Examples
/// ```
/// // Get an immutable reference of a page by its FrameID
/// let page_mut_ref: &mut MaterializedPage = &mut page_pool[FrameID(2)];
/// ```
impl<T> IndexMut<FrameID> for FramePool<T> {
    fn index_mut(&mut self, index: FrameID) -> &mut Self::Output {
        &mut self.inner[index.0]
    }
}

/// Implements non-mutable indexing with [`&FrameID`]
///
/// # Examples
/// ```
/// // Get an immutable reference of a page by its FrameID
/// let page_ref: &MaterializedPage = &page_pool[&FrameID(1)];
/// ```
impl<T> Index<&FrameID> for FramePool<T> {
    type Output = T;

    fn index(&self, index: &FrameID) -> &Self::Output {
        &self.inner[index.0]
    }
}

/// Implements mutable indexing with [`&FrameID`]
///
/// # Examples
/// ```
/// // Get an immutable reference of a page by its FrameID
/// let page_mut_ref: &mut MaterializedPage = &mut page_pool[&FrameID(2)];
/// ```
impl<T> IndexMut<&FrameID> for FramePool<T> {
    fn index_mut(&mut self, index: &FrameID) -> &mut Self::Output {
        &mut self.inner[index.0]
    }
}
