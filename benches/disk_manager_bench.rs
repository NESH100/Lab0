// This file will be replaced by the runner
use criterion::{Criterion, criterion_group, criterion_main};

use std::collections::VecDeque;

use rand::{RngCore, SeedableRng, rngs::StdRng};
use sdms_lab_0::PageID;
use sdms_lab_0::disk::DiskManager;
use uuid::Uuid;

// Benchmarks the allocation and freeing of pages of the DiskManager (roughly 50/50 distribution)
fn bench_alloc_free_rand(c: &mut Criterion) {}

criterion_group!(benches, bench_alloc_free_rand);
criterion_main!(benches);
