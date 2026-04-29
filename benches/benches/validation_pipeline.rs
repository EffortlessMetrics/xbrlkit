use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

fn bench_validation_pipeline(c: &mut Criterion) {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .expect("benchmark crate is inside workspace")
        .to_path_buf();

    let profile =
        sec_profile_types::load_profile_from_workspace(&workspace_root, "sec/efm-77/opco")
            .expect("profile should load");

    let html_path = workspace_root
        .join("fixtures")
        .join("synthetic")
        .join("inline")
        .join("ixds-single-file-01")
        .join("member-a.html");
    let html = std::fs::read_to_string(&html_path).expect("fixture should exist");

    let members: Vec<(&str, &str)> = vec![("member-a", &html)];

    c.bench_function("validation_pipeline_inline_html", |b| {
        b.iter(|| {
            let result = validation_run::validate_html_members(
                black_box(&members),
                black_box(&profile),
            );
            black_box(result);
        });
    });
}

criterion_group!(benches, bench_validation_pipeline);
criterion_main!(benches);
