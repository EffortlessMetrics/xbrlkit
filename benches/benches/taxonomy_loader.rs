use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

/// Generates a synthetic XSD schema with `n` hypercube/dimension element pairs.
fn synthetic_schema(n: usize) -> String {
    let mut elements = String::new();
    for i in 0..n {
        elements.push_str(&format!(
            r#"    <xsd:element id="us-gaap_Table{i}" name="Table{i}" substitutionGroup="xbrldt:hypercubeItem" type="xbrli:stringItemType" abstract="true"/>"#,
        ));
        elements.push_str(&format!(
            r#"    <xsd:element id="us-gaap_Axis{i}" name="Axis{i}" substitutionGroup="xbrldt:dimensionItem" type="xbrli:stringItemType" abstract="true"/>"#,
        ));
    }
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"
            xmlns:xbrli="http://www.xbrl.org/2001/instance"
            xmlns:xbrldt="http://xbrl.org/2005/xbrldt"
            xmlns:us-gaap="http://fasb.org/us-gaap/2024"
            targetNamespace="http://fasb.org/us-gaap/2024"
            elementFormDefault="qualified">
    <xsd:import namespace="http://www.xbrl.org/2001/instance"
              schemaLocation="http://www.xbrl.org/2001/xbrl-instance-2001-12-31.xsd"/>
    <xsd:import namespace="http://xbrl.org/2005/xbrldt"
              schemaLocation="http://www.xbrl.org/2005/xbrldt-2005.xsd"/>
{elements}
</xsd:schema>
"#
    )
}

/// Generates a synthetic definition linkbase with `n` hypercube-dimension arcs
/// and `n` dimension-domain arcs, plus `2n` domain-member arcs.
fn synthetic_linkbase(n: usize) -> String {
    let mut locs = String::new();
    let mut arcs = String::new();

    for i in 0..n {
        locs.push_str(&format!(
            r#"        <link:loc xlink:type="locator" xlink:href="schema.xsd#us-gaap_Table{i}" xlink:label="Table{i}"/>"#,
        ));
        locs.push_str(&format!(
            r#"        <link:loc xlink:type="locator" xlink:href="schema.xsd#us-gaap_Axis{i}" xlink:label="Axis{i}"/>"#,
        ));
        locs.push_str(&format!(
            r#"        <link:loc xlink:type="locator" xlink:href="schema.xsd#us-gaap_Domain{i}" xlink:label="Domain{i}"/>"#,
        ));
        locs.push_str(&format!(
            r#"        <link:loc xlink:type="locator" xlink:href="schema.xsd#us-gaap_MemberA{i}" xlink:label="MemberA{i}"/>"#,
        ));
        locs.push_str(&format!(
            r#"        <link:loc xlink:type="locator" xlink:href="schema.xsd#us-gaap_MemberB{i}" xlink:label="MemberB{i}"/>"#,
        ));

        arcs.push_str(&format!(
            r#"        <link:definitionArc xlink:type="arc" xlink:arcrole="http://xbrl.org/int/dim/arcrole/hypercube-dimension" xlink:from="Table{i}" xlink:to="Axis{i}" order="1"/>"#,
        ));
        arcs.push_str(&format!(
            r#"        <link:definitionArc xlink:type="arc" xlink:arcrole="http://xbrl.org/int/dim/arcrole/dimension-domain" xlink:from="Axis{i}" xlink:to="Domain{i}" order="1"/>"#,
        ));
        arcs.push_str(&format!(
            r#"        <link:definitionArc xlink:type="arc" xlink:arcrole="http://xbrl.org/int/dim/arcrole/domain-member" xlink:from="Domain{i}" xlink:to="MemberA{i}" order="1"/>"#,
        ));
        arcs.push_str(&format!(
            r#"        <link:definitionArc xlink:type="arc" xlink:arcrole="http://xbrl.org/int/dim/arcrole/domain-member" xlink:from="Domain{i}" xlink:to="MemberB{i}" order="2"/>"#,
        ));
    }

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<link:linkbase xmlns:link="http://www.xbrl.org/2003/linkbase"
               xmlns:xlink="http://www.w3.org/1999/xlink">
    <link:definitionLink xlink:type="extended" xlink:role="http://www.xbrl.org/2003/role/link">
{locs}
{arcs}
    </link:definitionLink>
</link:linkbase>
"#
    )
}

fn bench_schema_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("schema_parse");
    for size in [10, 50, 100] {
        let schema = synthetic_schema(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &schema, |b, s| {
            b.iter(|| {
                let mut taxonomy = taxonomy_loader::DimensionTaxonomy::new();
                taxonomy_loader::parse_schema(black_box(s), black_box(&mut taxonomy)).unwrap();
            });
        });
    }
    group.finish();
}

fn bench_linkbase_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("linkbase_parse");
    for size in [10, 50, 100] {
        let linkbase = synthetic_linkbase(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &linkbase, |b, lb| {
            b.iter(|| {
                let mut taxonomy = taxonomy_loader::DimensionTaxonomy::new();
                // Pre-seed hypercubes so the arcs can link them
                for i in 0..size {
                    taxonomy.add_hypercube(taxonomy_loader::Hypercube {
                        qname: format!("us-gaap:Table{i}"),
                        dimensions: std::collections::BTreeMap::new(),
                        label: None,
                    });
                }
                taxonomy_loader::parse_definition_linkbase(black_box(lb), black_box(&mut taxonomy))
                    .unwrap();
            });
        });
    }
    group.finish();
}

fn bench_full_taxonomy_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("taxonomy_load");
    for size in [10, 50] {
        let schema = synthetic_schema(size);
        let linkbase = synthetic_linkbase(size);

        // Write to temp directory once per size
        let temp_dir = std::env::temp_dir().join(format!("xbrlkit_bench_taxonomy_{size}"));
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();
        std::fs::write(temp_dir.join("schema.xsd"), &schema).unwrap();
        std::fs::write(temp_dir.join("schema_def.xml"), &linkbase).unwrap();

        // Modify schema to reference the local linkbase instead of HTTP ones,
        // and remove imports that would hit the network
        let local_schema = schema
            .replace(
                "schemaLocation=\"http://www.xbrl.org/2001/xbrl-instance-2001-12-31.xsd\"",
                "schemaLocation=\"xbrl-instance.xsd\"",
            )
            .replace(
                "schemaLocation=\"http://www.xbrl.org/2005/xbrldt-2005.xsd\"",
                "schemaLocation=\"xbrldt.xsd\"",
            );
        // Write stub imports so the loader doesn't network
        std::fs::write(temp_dir.join("xbrl-instance.xsd"), r#"<?xml version="1.0"?><xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema" targetNamespace="http://www.xbrl.org/2001/instance"/>"#).unwrap();
        std::fs::write(temp_dir.join("xbrldt.xsd"), r#"<?xml version="1.0"?><xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema" targetNamespace="http://xbrl.org/2005/xbrldt"/>"#).unwrap();
        std::fs::write(temp_dir.join("schema.xsd"), local_schema).unwrap();

        let schema_path = temp_dir.join("schema.xsd");
        let path_str = schema_path.to_string_lossy().to_string();

        group.bench_with_input(BenchmarkId::from_parameter(size), &path_str, |b, p| {
            b.iter(|| {
                let loader = taxonomy_loader::TaxonomyLoader::new();
                let _ = black_box(loader.load(p));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_schema_parse, bench_linkbase_parse, bench_full_taxonomy_load);
criterion_main!(benches);
