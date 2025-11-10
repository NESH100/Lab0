// This file will be replaced by the runner
// Still you can add tests here for local testing

#[cfg(test)]
mod advanced {
    use crate::disk::*;

    /// Write and overwrite pages with random data using your write function.
    /// Test with valid and invalid PageIDs.
    /// Checks if the pages are written correctly to the file by reading it (with a test-specific
    /// read function) and comparing it with the expected file.
    #[test]
    fn write_rand_data() -> Result<(), DiskManagerError> {
        unimplemented!();
    }

    /// Write a database file with a test-specific write function, read it with your read function.
    /// Test with valid and invalid PageIDs.
    /// Compare read pages to the ones written before.
    #[test]
    fn read_rand_data() -> Result<(), DiskManagerError> {
        unimplemented!();
    }

    /// Set the disk manager to a given state (next_free and free_list).
    /// Allocate new pages and check if the disk manager behaves as expected with respect to
    /// - Returned page id
    /// - Changes to next_free
    /// - Changes to the free_list
    #[test]
    fn alloc_pages() -> Result<(), DiskManagerError> {
        unimplemented!();
    }

    /// Set the disk manager to a given state.
    /// Free pages and check if the disk manager behaves as expected with respect to
    /// - Returned page id
    /// - Changes to next_free
    /// - Changes to the free_list
    #[test]
    fn free_pages() -> Result<(), DiskManagerError> {
        unimplemented!();
    }

    /// Set the disk manager to a given state. Allocate and free pages and check if the disk manager
    /// behaves as expected with respect to
    /// - Returned page id
    /// - Changes to next_free
    /// - Changes to the free_list
    ///
    /// This test does not contain messages on every assertion. Check that your code passes the
    /// previous tests first. Then write your own tests if necessary.
    #[test]
    fn alloc_free_pages() -> Result<(), DiskManagerError> {
        unimplemented!();
    }
}
