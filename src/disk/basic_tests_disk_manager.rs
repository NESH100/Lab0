// This file will be replaced by the runner
// Still you can add tests here for local testing

#[cfg(test)]
mod basic {
    use crate::PAGE_SIZE;
    use crate::disk::*;
    use std::fs::OpenOptions;

    #[test]
    fn write_and_read() -> Result<(), Box<dyn std::error::Error>> {
        let mut dm = DiskManager::new("/tmp/database_rw.dmdb")?;

        assert_eq!(dm.next_free, PageID(1));

        let mut test_page = [0u8; PAGE_SIZE];

        let mut sample = vec![];

        for _ in 0..100 {
            let pid = dm.allocate();
            let mut page = test_page;
            page[0] = pid.0 as u8;
            dm.write(pid, &page)?;
            sample.push((pid, page));
        }

        test_page = [0u8; PAGE_SIZE];

        for (pid, sample_page) in sample {
            dm.read(pid, &mut test_page)?;
            assert_eq!(test_page, sample_page);
        }

        Ok(())
    }

    #[test]
    fn alloc_pages() -> Result<(), DiskManagerError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("/tmp/database_alloc.dmdb")?;

        let next_free = PageID(10);
        let free_list = vec![PageID(5)];

        {
            let mut dm = DiskManager {
                file,
                next_free,
                free_list: free_list.into(),
            };
            assert_eq!(dm.allocate(), PageID(5));
            assert_eq!(dm.next_free, PageID(10));
            assert_eq!(dm.free_list, vec![]);

            assert_eq!(dm.allocate(), PageID(10));
            assert_eq!(dm.next_free, PageID(11));
            assert_eq!(dm.free_list, vec![]);
        }

        Ok(())
    }

    #[test]
    fn free_pages() -> Result<(), DiskManagerError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("/tmp/database_free.dmdb")?;

        let next_free = PageID(10);
        let free_list = vec![PageID(5)];

        {
            let mut dm = DiskManager {
                file,
                next_free,
                free_list: free_list.into(),
            };
            dm.free(PageID(2))?;
            assert_eq!(dm.next_free, PageID(10));
            assert_eq!(dm.free_list, vec![PageID(5), PageID(2)]);

            let result = dm.free(PageID(15));
            assert!(result.is_err());
            let result = result.unwrap_err();
            match result {
                DiskManagerError::InvalidPageID(pid) => assert_eq!(pid, PageID(15)),
                _ => panic!("Expected InvalidPageID error"),
            }
            assert_eq!(dm.next_free, PageID(10));
            assert_eq!(dm.free_list, vec![PageID(5), PageID(2)]);

            dm.free(PageID(9))?;
            assert_eq!(dm.next_free, PageID(10));
            assert_eq!(dm.free_list, vec![PageID(5), PageID(2), PageID(9)]);
        }

        Ok(())
    }

    #[test]
    fn alloc_free_pages() -> Result<(), DiskManagerError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("/tmp/database_alloc_free.dmdb")?;

        let next_free = PageID(10);
        let free_list = vec![PageID(5)];

        {
            let mut dm = DiskManager {
                file,
                next_free,
                free_list: free_list.into(),
            };

            assert_eq!(dm.allocate(), PageID(5));
            assert_eq!(dm.next_free, PageID(10));
            assert_eq!(dm.free_list, vec![]);

            dm.free(PageID(2))?;
            assert_eq!(dm.next_free, PageID(10));
            assert_eq!(dm.free_list, vec![PageID(2)]);

            let result = dm.free(PageID(15));
            assert!(result.is_err());
            let result = result.unwrap_err();
            match result {
                DiskManagerError::InvalidPageID(pid) => assert_eq!(pid, PageID(15)),
                _ => panic!("Expected InvalidPageID error"),
            }
            assert_eq!(dm.next_free, PageID(10));
            assert_eq!(dm.free_list, vec![PageID(2)]);

            assert_eq!(dm.allocate(), PageID(2));
            assert_eq!(dm.next_free, PageID(10));
            assert_eq!(dm.free_list, vec![]);

            assert_eq!(dm.allocate(), PageID(10));
            assert_eq!(dm.next_free, PageID(11));
            assert_eq!(dm.free_list, vec![]);

            dm.free(PageID(10))?;
            assert_eq!(dm.next_free, PageID(11));
            assert_eq!(dm.free_list, vec![PageID(10)]);

            dm.free(PageID(1))?;
            assert_eq!(dm.next_free, PageID(11));
            assert_eq!(dm.free_list, vec![PageID(10), PageID(1)]);
            dm.free(PageID(2))?;
            assert_eq!(dm.next_free, PageID(11));
            assert_eq!(dm.free_list, vec![PageID(10), PageID(1), PageID(2)]);
            assert_eq!(dm.allocate(), PageID(10));
            assert_eq!(dm.next_free, PageID(11));
            assert_eq!(dm.free_list, vec![PageID(1), PageID(2)]);
        }

        Ok(())
    }
}
