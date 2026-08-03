//! Benchmarks for taxonomy loading, including local I/O and recursive parsing.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::fmt::Write as _;
use std::path::PathBuf;
use taxonomy_loader::TaxonomyLoader;

const XLINK_NAMESPACE: &str = "http://www.w3.org/1999/xlink";

struct TaxonomyFixture {
    directory: PathBuf,
    entrypoint: String,
}

impl Drop for TaxonomyFixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

fn schema_document(index: usize, schema_count: usize, groups: usize) -> String {
    let prefix = format!("bench{index}");
    let namespace = format!("http://example.org/{prefix}/2024");
    let mut schema = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"
            xmlns:xbrli="http://www.xbrl.org/2001/instance"
            xmlns:xbrldt="http://xbrl.org/2005/xbrldt"
            xmlns:link="http://www.xbrl.org/2003/linkbase"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            xmlns:{prefix}="{namespace}"
            targetNamespace="{namespace}"
            elementFormDefault="qualified">
<link:linkbaseRef xlink:type="simple" xlink:href="{prefix}_def.xml"
                  xlink:role="http://www.xbrl.org/2003/role/definitionLinkbaseRef"/>
"#
    );

    if index + 1 < schema_count {
        let next_prefix = format!("bench{}", index + 1);
        let next_namespace = format!("http://example.org/{next_prefix}/2024");
        let _ = writeln!(
            schema,
            "<xsd:import namespace=\"{next_namespace}\" schemaLocation=\"{next_prefix}.xsd\"/>"
        );
    }

    for group in 0..groups {
        let _ = writeln!(
            schema,
            "<xsd:element id=\"{prefix}_Table{group}\" name=\"Table{group}\" substitutionGroup=\"xbrldt:hypercubeItem\" type=\"xbrli:stringItemType\" abstract=\"true\"/>"
        );
        let _ = writeln!(
            schema,
            "<xsd:element id=\"{prefix}_Axis{group}\" name=\"Axis{group}\" substitutionGroup=\"xbrldt:dimensionItem\" type=\"xbrli:stringItemType\" abstract=\"true\"/>"
        );
        let _ = writeln!(
            schema,
            "<xsd:element id=\"{prefix}_Domain{group}\" name=\"Domain{group}\" type=\"xbrli:domainItemType\" abstract=\"true\"/>"
        );
        let _ = writeln!(
            schema,
            "<xsd:element id=\"{prefix}_Member{group}\" name=\"Member{group}\" type=\"xbrli:domainItemType\"/>"
        );
    }

    schema.push_str("</xsd:schema>\n");
    schema
}

fn definition_linkbase(index: usize, groups: usize) -> String {
    let prefix = format!("bench{index}");
    let mut linkbase = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<link:linkbase xmlns:link="http://www.xbrl.org/2003/linkbase"
               xmlns:xlink="{XLINK_NAMESPACE}">
  <link:definitionLink xlink:type="extended" xlink:role="http://www.xbrl.org/2003/role/link">
"#
    );

    for group in 0..groups {
        let _ = writeln!(
            linkbase,
            "    <link:loc xlink:type=\"locator\" xlink:href=\"{prefix}.xsd#{prefix}_Table{group}\" xlink:label=\"Table{group}\"/>"
        );
        let _ = writeln!(
            linkbase,
            "    <link:loc xlink:type=\"locator\" xlink:href=\"{prefix}.xsd#{prefix}_Axis{group}\" xlink:label=\"Axis{group}\"/>"
        );
        let _ = writeln!(
            linkbase,
            "    <link:loc xlink:type=\"locator\" xlink:href=\"{prefix}.xsd#{prefix}_Domain{group}\" xlink:label=\"Domain{group}\"/>"
        );
        let _ = writeln!(
            linkbase,
            "    <link:loc xlink:type=\"locator\" xlink:href=\"{prefix}.xsd#{prefix}_Member{group}\" xlink:label=\"Member{group}\"/>"
        );
        let _ = writeln!(
            linkbase,
            "    <link:definitionArc xlink:type=\"arc\" xlink:arcrole=\"http://xbrl.org/int/dim/arcrole/hypercube-dimension\" xlink:from=\"Table{group}\" xlink:to=\"Axis{group}\" order=\"1\"/>"
        );
        let _ = writeln!(
            linkbase,
            "    <link:definitionArc xlink:type=\"arc\" xlink:arcrole=\"http://xbrl.org/int/dim/arcrole/dimension-domain\" xlink:from=\"Axis{group}\" xlink:to=\"Domain{group}\" order=\"1\"/>"
        );
        let _ = writeln!(
            linkbase,
            "    <link:definitionArc xlink:type=\"arc\" xlink:arcrole=\"http://xbrl.org/int/dim/arcrole/domain-member\" xlink:from=\"Domain{group}\" xlink:to=\"Member{group}\" order=\"1\"/>"
        );
    }

    linkbase.push_str("  </link:definitionLink>\n</link:linkbase>\n");
    linkbase
}

fn prepare_fixture(schema_count: usize, groups: usize) -> Option<TaxonomyFixture> {
    let directory = std::env::temp_dir().join(format!(
        "xbrlkit-bench-taxonomy-{}-{schema_count}-{groups}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    if let Err(error) = std::fs::create_dir_all(&directory) {
        eprintln!("benchmark fixture setup failed: {error}");
        return None;
    }

    for index in 0..schema_count {
        let prefix = format!("bench{index}");
        let schema_path = directory.join(format!("{prefix}.xsd"));
        let linkbase_path = directory.join(format!("{prefix}_def.xml"));
        if let Err(error) =
            std::fs::write(&schema_path, schema_document(index, schema_count, groups))
        {
            eprintln!("benchmark schema setup failed: {error}");
            let _ = std::fs::remove_dir_all(&directory);
            return None;
        }
        if let Err(error) = std::fs::write(&linkbase_path, definition_linkbase(index, groups)) {
            eprintln!("benchmark linkbase setup failed: {error}");
            let _ = std::fs::remove_dir_all(&directory);
            return None;
        }
    }

    let entrypoint = directory.join("bench0.xsd");
    let entrypoint = entrypoint.to_string_lossy().into_owned();
    let taxonomy = TaxonomyLoader::new().load(&entrypoint);
    let expected_relationships = schema_count * groups;
    match taxonomy {
        Ok(taxonomy)
            if taxonomy.hypercubes.len() == expected_relationships
                && taxonomy
                    .hypercubes
                    .values()
                    .all(|hypercube| !hypercube.dimensions.is_empty())
                && taxonomy.dimension_domains.len() == expected_relationships
                && taxonomy
                    .domains
                    .values()
                    .all(|domain| !domain.members.is_empty()) =>
        {
            Some(TaxonomyFixture {
                directory,
                entrypoint,
            })
        }
        Ok(taxonomy) => {
            eprintln!(
                "benchmark fixture loaded unexpected relationships: hypercubes={}, dimensions={}, domains={}",
                taxonomy.hypercubes.len(),
                taxonomy.dimension_domains.len(),
                taxonomy.domains.len()
            );
            let _ = std::fs::remove_dir_all(&directory);
            None
        }
        Err(error) => {
            eprintln!("benchmark fixture validation failed: {error}");
            let _ = std::fs::remove_dir_all(&directory);
            None
        }
    }
}

fn bench_taxonomy_loader(c: &mut Criterion) {
    let mut group = c.benchmark_group("taxonomy_loader");
    let fixtures = [(2, 20), (4, 50), (8, 100)]
        .into_iter()
        .filter_map(|(schema_count, groups)| {
            prepare_fixture(schema_count, groups).map(|fixture| (schema_count, groups, fixture))
        })
        .collect::<Vec<_>>();

    for (schema_count, groups, fixture) in &fixtures {
        group.bench_with_input(
            BenchmarkId::new("load", format!("{schema_count}schemas-{groups}groups")),
            &fixture.entrypoint,
            |benchmark, entrypoint| {
                benchmark.iter(|| TaxonomyLoader::new().load(black_box(entrypoint.as_str())));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_taxonomy_loader);
criterion_main!(benches);
