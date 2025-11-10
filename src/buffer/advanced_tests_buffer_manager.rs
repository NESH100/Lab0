// This file will be replaced by the runner
// Still you can add tests here for local testing

#[cfg(test)]
mod advanced {
    use crate::buffer::buffer_manager::*;
    use crate::buffer::*;

    /// Pin a page.
    /// Unpin this page twice
    /// Check that the page is in the buffer pool
    /// Check that the pin count is exactly 0
    #[test]
    fn test_pin_unpin_twice() -> Result<(), BufferManagerError> {
        unimplemented!();
    }

    /// Pin less than BUFFER_POOL_SIZE pages.
    /// Unpin a subset of these pages.
    /// Check if the FrameDescriptors are set correctly.
    /// Especially if the PageIDs are correct, if the overall number of pins is correct and if the
    /// dirty bits are correct.
    #[test]
    fn test_pin_less_than_max() -> Result<(), BufferManagerError> {
        unimplemented!();
    }

    /// Pin exactly BUFFER_POOL_SIZE random pages. No page should be evicted.
    /// Select pages to pin at least 2 or 3 times.
    /// Checks if pin counter and frame matches expected.
    /// (start with using frame 0, 1, 2... and so on)
    #[test]
    fn test_pin_unpin() -> Result<(), BufferManagerError> {
        unimplemented!();
    }

    /// Pin exactly BUFFER_POOL_SIZE random pages. No page should be evicted.
    /// Check pin counter and dirty flag.
    /// Unpin ever page unchanged.
    /// Check pin counter and dirty flag.
    /// Pin every page again.
    /// Change the content of every page.
    /// Unpin with changed content.
    /// Check pin count and dirty flag.
    #[test]
    fn test_pin_unpin_dirty_and_not_dirty() -> Result<(), BufferManagerError> {
        unimplemented!();
    }

    /// Creates frame descriptors for 3 pages with IDs 1 to 3.
    /// Call on_pin on all frame descriptors.
    /// Check that on_pin does not touch attributes it should not change
    /// Call on_pin on FrameID(0)
    /// Call replace and check the result.
    #[test]
    fn test_lru_replacement_strategy() -> Result<(), BufferManagerError> {
        unimplemented!();
    }

    /// Creates frame descriptors for 3 pages with IDs 1 to 3.
    /// Call on_pin on all frame descriptors.
    /// Check that on_pin does not touch attributes it should not change
    /// Call replace, check the result, and then call on_pin on the victim.
    /// Call replace, check the result, and then call on_pin on the victim.
    /// Call on_pin on FrameID(2)
    /// Call replace, check the result, and then call on_pin on the victim.
    /// Call on_pin on FrameID(0)
    /// Call replace, check the result, and then call on_pin on the victim.
    /// Call on_pin on all frame descriptors.
    /// Call replace, check the result, and then call on_pin on the victim.
    #[test]
    fn test_clock_replacement_strategy() -> Result<(), BufferManagerError> {
        unimplemented!();
    }

    /// Compares BufferManager state with reference implementation.
    /// Pick initial set of BUFFER_POOL_SIZE pages.
    /// Generate random sequence of pin and unpin operations.
    /// Execute operations and compare with reference implementation
    /// using LRU as ReplacementStrategy.
    /// Also checks DiskManager state. Expect write back and eviction on pin.
    #[test]
    fn test_lru_with_write_back() -> Result<(), BufferManagerError> {
        unimplemented!();
    }

    /// Compares BufferManager state with reference implementation.
    /// Pick initial set of BUFFER_POOL_SIZE pages.
    /// Generate random sequence of pin and unpin operations.
    /// Execute operations and compare with reference implementation
    /// using Clock as ReplacementStrategy.
    /// Also checks DiskManager state. Expect write back and eviction on pin.
    #[test]
    fn test_clock_with_write_back() -> Result<(), BufferManagerError> {
        unimplemented!();
    }
}
