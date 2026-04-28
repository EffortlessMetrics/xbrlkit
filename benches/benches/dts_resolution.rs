use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

fn bench_dts_resolution(c: &mut Criterion) {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .expect("benchmark crate is inside workspace")
        .to_path_buf();

    let profile =
        sec_profile_types::load_profile_from_workspace(&workspace_root, "sec/efm-77/opco")
            .expect("profile should load");

    // Entry points from the profile's accepted taxonomies
    let entry_points: Vec<String> = profile
        .accepted_taxonomies
        .namespaces
        .iter()
        .map(|ns| ns.uri.clone())
        .collect();

    c.bench_function("dts_resolution", |b| {
        b.iter(|| {
            let dts = taxonomy_dts::build_dts(black_box(&profile), black_box(entry_points.clone()));
            black_box(dts);
        });
    });
}

criterion_group!(benches, bench_dts_resolution);
criterion_main!(benches);
