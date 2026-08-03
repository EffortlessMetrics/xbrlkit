//! Benchmarks for profile-aware DTS construction and entry-point checks.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use sec_profile_types::{AcceptedTaxonomies, ProfilePack};
use taxonomy_dts::{build_dts, mixed_taxonomy_years, nonstandard_entry_points};
use taxonomy_types::NamespaceMapping;

fn synthetic_profile(size: usize) -> ProfilePack {
    let namespaces = (0..size)
        .map(|index| NamespaceMapping {
            prefix: format!("ns{index}"),
            uri: format!("http://example.org/ns{index}/2024"),
        })
        .collect::<Vec<_>>();
    let standard_taxonomy_uris = namespaces
        .iter()
        .map(|namespace| namespace.uri.clone())
        .collect();

    ProfilePack {
        id: "bench-profile".to_string(),
        label: "Benchmark profile".to_string(),
        accepted_taxonomies: AcceptedTaxonomies {
            namespaces,
            ..AcceptedTaxonomies::default()
        },
        standard_taxonomy_uris,
        ..ProfilePack::default()
    }
}

fn entry_points(size: usize) -> Vec<String> {
    (0..size)
        .map(|index| format!("http://example.org/ns{index}/2024"))
        .collect()
}

fn bench_dts_resolution(c: &mut Criterion) {
    let mut group = c.benchmark_group("dts_resolution");

    for size in [10, 50, 100] {
        let profile = synthetic_profile(size);
        let points = entry_points(size);
        group.bench_function(format!("build_dts_{size}_entry_points"), |benchmark| {
            benchmark.iter(|| build_dts(black_box(&profile), black_box(points.clone())));
        });

        let dts = build_dts(&profile, points);
        group.bench_function(
            format!("nonstandard_entry_points_{size}_entry_points"),
            |benchmark| {
                benchmark.iter(|| nonstandard_entry_points(black_box(&dts), black_box(&profile)));
            },
        );
    }

    let mixed_points = vec![
        "http://example.org/ns/2023".to_string(),
        "http://example.org/ns/2024".to_string(),
        "http://example.org/ns/2025".to_string(),
    ];
    group.bench_function("mixed_taxonomy_years_3_entry_points", |benchmark| {
        benchmark.iter(|| mixed_taxonomy_years(black_box(&mixed_points)));
    });

    group.finish();
}

criterion_group!(benches, bench_dts_resolution);
criterion_main!(benches);
