//! Benchmarks for DTS (Discoverable Taxonomy Set) resolution hot path.
//!
//! Tests `build_dts` and `nonstandard_entry_points` with synthetic profile packs.

#![allow(clippy::format_push_string)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use sec_profile_types::{AcceptedTaxonomies, InlineRules, ProfilePack};
use taxonomy_dts::{build_dts, nonstandard_entry_points};
use taxonomy_types::NamespaceMapping;

/// Create a synthetic profile pack with `n` accepted namespaces.
fn synthetic_profile(n: usize) -> ProfilePack {
    let namespaces: Vec<NamespaceMapping> = (0..n)
        .map(|i| NamespaceMapping {
            prefix: format!("ns{i}"),
            uri: format!("http://example.org/ns{i}/2024"),
        })
        .collect();

    let standard_taxonomy_uris: Vec<String> = namespaces.iter().map(|ns| ns.uri.clone()).collect();

    ProfilePack {
        id: "bench-profile".to_string(),
        label: "Benchmark profile".to_string(),
        forms: vec!["10-K".to_string()],
        enabled_rule_families: vec!["dts".to_string()],
        inline_rules: InlineRules::default(),
        accepted_taxonomies: AcceptedTaxonomies {
            years: vec![2024],
            namespaces,
        },
        standard_taxonomy_uris,
        required_facts: vec![],
        numeric_rules: None,
    }
}

/// Create entry points that match a subset of the profile namespaces.
fn entry_points(count: usize) -> Vec<String> {
    (0..count)
        .map(|i| format!("http://example.org/ns{i}/2024"))
        .collect()
}

fn bench_dts_resolution(c: &mut Criterion) {
    let mut group = c.benchmark_group("dts_resolution");

    for size in [10, 50, 100] {
        let profile = synthetic_profile(size);
        let eps = entry_points(size);

        group.bench_function(format!("build_dts_{size}namespaces"), |b| {
            b.iter(|| {
                let dts = build_dts(black_box(&profile), black_box(eps.clone()));
                assert_eq!(dts.entry_points.len(), size);
            });
        });

        group.bench_function(format!("nonstandard_entry_points_{size}namespaces"), |b| {
            let dts = build_dts(&profile, eps.clone());
            b.iter(|| {
                let missing = nonstandard_entry_points(black_box(&dts), black_box(&profile));
                assert!(missing.is_empty());
            });
        });
    }

    // Stress test: mixed-year detection
    let mixed_eps = vec![
        "http://example.org/ns/2023".to_string(),
        "http://example.org/ns/2024".to_string(),
        "http://example.org/ns/2025".to_string(),
    ];
    group.bench_function("mixed_taxonomy_years_3entries", |b| {
        b.iter(|| {
            let mixed = taxonomy_dts::mixed_taxonomy_years(black_box(&mixed_eps));
            assert!(mixed);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_dts_resolution);
criterion_main!(benches);
