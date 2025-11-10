// This file will be replaced by the runner
// Still you can add tests here for local testing

#[cfg(test)]
mod basic {
    use crate::buffer::buffer_manager::*;
    use crate::buffer::*;
    use crate::{BUFFER_POOL_SIZE, PageID};
    use rand::{RngCore, SeedableRng, rng, rngs::StdRng};
    use std::{cell::RefCell, rc::Rc};

    fn rand_page(rng: &mut StdRng) -> MaterializedPage {
        let mut page = MaterializedPage::default();

        rng.fill_bytes(&mut page.1);

        page
    }

    #[test]
    fn test_pin_and_unpin() -> Result<(), BufferManagerError> {
        let seed = [
            255, 28, 175, 55, 9, 97, 124, 42, 250, 188, 255, 17, 39, 17, 184, 154, 229, 247, 80,
            146, 42, 241, 192, 229, 66, 56, 82, 183, 50, 21, 77, 187,
        ];
        let mut rng = StdRng::from_seed(seed);

        let mut pages = Vec::new();
        for _ in 0..=BUFFER_POOL_SIZE {
            pages.push(rand_page(&mut rng));
        }
        let disk_manager = Rc::new(RefCell::new(DummyDiskManager { pages }));
        let mut buffer_manager =
            BufferManager::new(disk_manager.clone(), DummyReplacementStrategy {});

        // pin every page once
        for i in 1..=BUFFER_POOL_SIZE {
            buffer_manager.pin(PageID(i))?;
        }

        // check frame descriptor for every page
        for (i, frame_descriptor) in buffer_manager.frame_descriptors.iter().enumerate() {
            assert_eq! {frame_descriptor.pin_count, 1, "Expected a pin count of 1 for PID {}", frame_descriptor.page_id};
            assert! {!frame_descriptor.dirty, "Page PID {} should not be dirty", frame_descriptor.page_id};
            assert_eq! {frame_descriptor.page_id, PageID(i + 1), "Expected page PID {} at FrameID {}", frame_descriptor.page_id, i};
        }

        // unpin every page in reverse order
        for i in (1..=BUFFER_POOL_SIZE).rev() {
            buffer_manager.unpin(PageID(i), false);
        }

        // check frame descriptor
        for (i, frame_descriptor) in buffer_manager.frame_descriptors.iter().enumerate() {
            assert_eq! {frame_descriptor.pin_count, 0, "Expected a pin count of 0 for PID {}", frame_descriptor.page_id};
            assert! {!frame_descriptor.dirty, "Expected dirty bit to be false for PID {} at FrameID {}", frame_descriptor.page_id, i};
            assert_eq! {frame_descriptor.page_id, PageID(i + 1), "Expected page PID {} at FrameID {}", frame_descriptor.page_id, i};
        }

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_panic_if_page_not_loaded() {
        let mut seed_rng = rng();
        let mut seed = [0u8; 32];
        seed_rng.fill_bytes(&mut seed);
        println!("Seed: {seed:?}");
        let mut rng = StdRng::from_seed(seed);

        let mut pages = Vec::new();
        for _ in 0..=BUFFER_POOL_SIZE {
            pages.push(rand_page(&mut rng));
        }
        let disk_manager = Rc::new(RefCell::new(DummyDiskManager {
            pages: pages.clone(),
        }));
        let mut buffer_manager =
            BufferManager::new(disk_manager.clone(), DummyReplacementStrategy {});

        // pin every page once, return to fail test if an error occurs
        for i in 1..=BUFFER_POOL_SIZE {
            if buffer_manager.pin(PageID(i)).is_err() {
                return;
            }
        }

        // unpin page not loaded in buffer manager
        buffer_manager.unpin(PageID(BUFFER_POOL_SIZE + 1), false)
    }

    #[test]
    fn test_error_propagation() {
        // InvalidPageID
        {
            let pages = Vec::new();
            let disk_manager = Rc::new(RefCell::new(DummyDiskManager { pages }));
            let mut buffer_manager =
                BufferManager::new(disk_manager.clone(), DummyReplacementStrategy {});

            assert_eq! {Err(BufferManagerError::InvalidPageID(PageID(0))), buffer_manager.pin(PageID(0))};
        }
        // AllPagesPinned for LRU
        {
            let mut seed_rng = rng();
            let mut seed = [0u8; 32];
            seed_rng.fill_bytes(&mut seed);
            println!("Seed: {seed:?}");
            let mut rng = StdRng::from_seed(seed);

            let mut pages = Vec::new();
            for _ in 0..=BUFFER_POOL_SIZE + 1 {
                pages.push(rand_page(&mut rng));
            }
            let disk_manager = Rc::new(RefCell::new(DummyDiskManager { pages }));
            let replacement = LRUReplacementStrategy::default();
            let mut buffer_manager = BufferManager::new(disk_manager.clone(), replacement);

            for i in 1..=BUFFER_POOL_SIZE {
                buffer_manager.pin(PageID(i)).unwrap();
            }

            assert_eq! {Err(BufferManagerError::AllPagesPinned), buffer_manager.pin(PageID(BUFFER_POOL_SIZE + 1))};
        }
        // AllPagesPinned for Clock
        {
            let mut seed_rng = rng();
            let mut seed = [0u8; 32];
            seed_rng.fill_bytes(&mut seed);
            println!("Seed: {seed:?}");
            let mut rng = StdRng::from_seed(seed);

            let mut pages = Vec::new();
            for _ in 0..=BUFFER_POOL_SIZE + 1 {
                pages.push(rand_page(&mut rng));
            }
            let disk_manager = Rc::new(RefCell::new(DummyDiskManager { pages }));
            let replacement = ClockReplacementStrategy::default();
            let mut buffer_manager = BufferManager::new(disk_manager.clone(), replacement);

            for i in 1..=BUFFER_POOL_SIZE {
                buffer_manager.pin(PageID(i)).unwrap();
            }

            assert_eq! {Err(BufferManagerError::AllPagesPinned), buffer_manager.pin(PageID(BUFFER_POOL_SIZE + 1))};
        }
    }
}
