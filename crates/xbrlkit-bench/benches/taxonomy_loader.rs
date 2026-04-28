//! Benchmarks for taxonomy loading hot path.
//!
//! Tests the full `TaxonomyLoader::load()` path including file I/O,
//! XSD schema parsing, linkbase parsing, and recursive import resolution.

#![allow(clippy::format_push_string)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::io::Write;
use taxonomy_loader::TaxonomyLoader;

/// Returns a synthetic XSD schema with dimension elements and imports.
fn make_schema(name: &str, imports: &[&str], elements: usize) -> String {
    let mut schema = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"
            xmlns:xbrli="http://www.xbrl.org/2001/instance"
            xmlns:xbrldt="http://xbrl.org/2005/xbrldt"
            xmlns:{name}="http://example.org/{name}/2024"
            targetNamespace="http://example.org/{name}/2024"
            elementFormDefault="qualified">
"#
    );

    for import in imports {
        schema.push_str(&format!(
            r#"    <xsd:import namespace="http://example.org/{import}/2024" schemaLocation="{import}.xsd"/>"#,
        ));
    }

    for i in 0..elements {
        let suffix = if i % 3 == 0 {
            r#"substitutionGroup="xbrldt:hypercubeItem""#
        } else if i % 3 == 1 {
            r#"substitutionGroup="xbrldt:dimensionItem""#
        } else {
            r#"type="xbrli:stringItemType""#
        };
        schema.push_str(&format!(
            r#"    <xsd:element id="{name}_Elem{i}" name="Elem{i}" {suffix} xbrli:periodType="duration" abstract="true"/>"#,
        ));
    }

    schema.push_str("</xsd:schema>");
    schema
}

/// Returns a synthetic definition linkbase with arcs.
fn make_linkbase(schema_name: &str, arcs: usize) -> String {
    let mut lb = r#"<?xml version="1.0" encoding="UTF-8"?>
<link:linkbase xmlns:link="http://www.xbrl.org/2003/linkbase"
               xmlns:xlink="http://www.w3.org/1999/xlink">
    <link:definitionLink xlink:type="extended" xlink:role="http://www.xbrl.org/2003/role/link">"#
    .to_string();

    // Locators
    for i in 0..arcs {
        lb.push_str(&format!(
            r#"        <link:loc xlink:type="locator" xlink:href="{schema_name}.xsd#{schema_name}_Elem{i}" xlink:label="Elem{i}"/>"#,
        ));
    }

    // Arcs: chain hypercube -> dimension -> domain -> member
    for i in 0..arcs.saturating_sub(1) {
        let arcrole = if i % 4 == 0 {
            "http://xbrl.org/int/dim/arcrole/hypercube-dimension"
        } else if i % 4 == 1 {
            "http://xbrl.org/int/dim/arcrole/dimension-domain"
        } else if i % 4 == 2 {
            "http://xbrl.org/int/dim/arcrole/domain-member"
        } else {
            "http://xbrl.org/int/dim/arcrole/all"
        };
        lb.push_str(&format!(
            r#"        <link:definitionArc xlink:type="arc" xlink:arcrole="{arcrole}" xlink:from="Elem{i}" xlink:to="Elem{i_plus}" order="1"/>"#,
            i_plus = i + 1
        ));
    }

    lb.push_str("    </link:definitionLink>\n</link:linkbase>");
    lb
}

/// Write synthetic taxonomy fixtures to a temp directory.
fn setup_taxonomy_fixture(
    dir: &std::path::Path,
    schemas: usize,
    elements_per_schema: usize,
    arcs: usize,
) {
    let schema_names: Vec<String> = (0..schemas).map(|i| format!("schema{i}")).collect();

    for (idx, name) in schema_names.iter().enumerate() {
        let imports: Vec<&str> = if idx > 0 {
            vec![&schema_names[idx - 1]]
        } else {
            vec![]
        };

        let schema = make_schema(name, &imports, elements_per_schema);
        let schema_path = dir.join(format!("{name}.xsd"));
        let mut f = std::fs::File::create(&schema_path).unwrap();
        f.write_all(schema.as_bytes()).unwrap();

        let linkbase = make_linkbase(name, arcs);
        let lb_path = dir.join(format!("{name}_def.xml"));
        let mut f = std::fs::File::create(&lb_path).unwrap();
        f.write_all(linkbase.as_bytes()).unwrap();
    }
}

fn bench_taxonomy_loader(c: &mut Criterion) {
    let mut group = c.benchmark_group("taxonomy_loader");

    // Small fixture
    let small_dir = std::env::temp_dir().join("xbrlkit_bench_small");
    let _ = std::fs::remove_dir_all(&small_dir);
    std::fs::create_dir_all(&small_dir).unwrap();
    setup_taxonomy_fixture(&small_dir, 2, 20, 10);
    let small_entry = small_dir.join("schema0.xsd").to_string_lossy().to_string();

    group.bench_function("small_2schemas_20elements", |b| {
        b.iter(|| {
            let loader = TaxonomyLoader::new();
            let result = loader.load(black_box(&small_entry));
            assert!(result.is_ok(), "{result:?}");
        });
    });

    // Medium fixture
    let medium_dir = std::env::temp_dir().join("xbrlkit_bench_medium");
    let _ = std::fs::remove_dir_all(&medium_dir);
    std::fs::create_dir_all(&medium_dir).unwrap();
    setup_taxonomy_fixture(&medium_dir, 4, 50, 25);
    let medium_entry = medium_dir.join("schema0.xsd").to_string_lossy().to_string();

    group.bench_function("medium_4schemas_50elements", |b| {
        b.iter(|| {
            let loader = TaxonomyLoader::new();
            let result = loader.load(black_box(&medium_entry));
            assert!(result.is_ok(), "{result:?}");
        });
    });

    // Large fixture
    let large_dir = std::env::temp_dir().join("xbrlkit_bench_large");
    let _ = std::fs::remove_dir_all(&large_dir);
    std::fs::create_dir_all(&large_dir).unwrap();
    setup_taxonomy_fixture(&large_dir, 8, 100, 50);
    let large_entry = large_dir.join("schema0.xsd").to_string_lossy().to_string();

    group.bench_function("large_8schemas_100elements", |b| {
        b.iter(|| {
            let loader = TaxonomyLoader::new();
            let result = loader.load(black_box(&large_entry));
            assert!(result.is_ok(), "{result:?}");
        });
    });

    // Also benchmark schema parsing in isolation (CPU-bound, no I/O)
    // Note: schema::parse_schema is private; we approximate by loading from a
    // single pre-loaded string via the public API by writing a temp file.
    let single_schema_dir = std::env::temp_dir().join("xbrlkit_bench_single");
    let _ = std::fs::remove_dir_all(&single_schema_dir);
    std::fs::create_dir_all(&single_schema_dir).unwrap();
    let schema_content = make_schema("bench", &[], 100);
    let schema_path = single_schema_dir.join("bench.xsd");
    std::fs::write(&schema_path, schema_content).unwrap();
    let schema_entry = schema_path.to_string_lossy().to_string();

    group.bench_function("parse_schema_only_100elements", |b| {
        b.iter(|| {
            let loader = TaxonomyLoader::new();
            let result = loader.load(black_box(&schema_entry));
            assert!(result.is_ok(), "{result:?}");
        });
    });

    group.finish();
}

criterion_group!(benches, bench_taxonomy_loader);
criterion_main!(benches);
