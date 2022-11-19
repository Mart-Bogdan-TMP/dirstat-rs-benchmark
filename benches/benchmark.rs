use std::{
    path::Path,
    time::{Instant},
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[cfg(windows)]
static TEST_PATH: &str = "C:\\Windows\\System32";
#[cfg(target_os = "linux")]
static TEST_PATH: &str = "/usr";
#[cfg(target_os = "macos")]
static TEST_PATH: &str = "/System";

pub fn criterion_benchmark(c: &mut Criterion) {
    // warmup, to fill in OS's FS cache
    println!("Warming up FS cache!");
    let start = Instant::now();
    calculate_new_lib(false);
    calculate_new_lib(true);
    calculate_old_lib(false);
    calculate_old_lib(true);
    let done_in = Instant::now() - start;
    println!("Done in {} ms.", done_in.as_secs_f64() * 1000.0);

    let mut g = c.benchmark_group("logical size");

    g.bench_function("new", |b| b.iter(|| calculate_new_lib(true)));
    g.bench_function("old", |b| b.iter(|| calculate_old_lib(true)));

    g.finish();

    let mut g = c.benchmark_group("physical size");

    g.bench_function("new", |b| b.iter(|| calculate_new_lib(false)));
    g.bench_function("old", |b| b.iter(|| calculate_old_lib(false)));

    g.finish();
}

fn calculate_new_lib(apparent: bool) {
    let file_info = dirstat_new::FileInfo::from_path(Path::new(TEST_PATH), apparent).unwrap();
    let volume_id = if let dirstat_new::FileInfo::Directory { volume_id } = file_info {
        volume_id
    } else {
        panic!("not a dir")
    };
    let di =
        dirstat_new::DiskItem::from_analyze(Path::new(TEST_PATH), apparent, volume_id).unwrap();
    black_box(di);
}
fn calculate_old_lib(apparent: bool) {
    let file_info = dirstat_old::FileInfo::from_path(Path::new(TEST_PATH), apparent).unwrap();
    let volume_id = if let dirstat_old::FileInfo::Directory { volume_id } = file_info {
        volume_id
    } else {
        panic!("not a dir")
    };
    let di =
        dirstat_old::DiskItem::from_analyze(Path::new(TEST_PATH), apparent, volume_id).unwrap();
    black_box(di);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
