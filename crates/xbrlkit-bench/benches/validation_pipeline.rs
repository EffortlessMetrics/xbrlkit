//! Benchmarks for the validation pipeline hot path.
//!
//! Tests `validate_html_members`, `validate_contexts`, and
//! `validate_context_completeness_streaming` with synthetic fixtures.

#![allow(clippy::format_push_string)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use sec_profile_types::{
    AcceptedTaxonomies, InlineRules, NegativeValueRules, NumericRules, ProfilePack,
};
use validation_run::{
    validate_context_completeness_streaming, validate_contexts, validate_html_members,
};

/// Build a synthetic SEC profile pack for validation benchmarks.
fn synthetic_validation_profile() -> ProfilePack {
    ProfilePack {
        id: "sec/efm-77/opco".to_string(),
        label: "Benchmark validation profile".to_string(),
        forms: vec!["10-K".to_string()],
        enabled_rule_families: vec![
            "inline_restrictions".to_string(),
            "required_facts".to_string(),
        ],
        inline_rules: InlineRules {
            banned_elements: vec!["ix:fraction".to_string(), "ix:tuple".to_string()],
            banned_attributes: vec!["xml:base".to_string()],
        },
        accepted_taxonomies: AcceptedTaxonomies {
            years: vec![2024],
            namespaces: vec![],
        },
        standard_taxonomy_uris: vec![],
        required_facts: vec![
            "dei:EntityRegistrantName".to_string(),
            "dei:EntityCentralIndexKey".to_string(),
            "dei:DocumentType".to_string(),
            "dei:DocumentPeriodEndDate".to_string(),
        ],
        numeric_rules: Some(NumericRules {
            negative_value_rules: NegativeValueRules {
                prohibited_concepts: vec!["us-gaap:Assets".to_string()],
            },
            ..Default::default()
        }),
    }
}

/// Build a synthetic inline XBRL HTML member with `n` facts.
fn synthetic_inline_html(n: usize) -> String {
    let mut html = r#"<html xmlns:ix="http://www.xbrl.org/2013/inlineXBRL" xmlns:dei="http://xbrl.sec.gov/dei/2024"><body>"#.to_string();

    for i in 0..n {
        html.push_str(&format!(
            r#"<ix:nonNumeric name="dei:Fact{i}">Value {i}</ix:nonNumeric>"#
        ));
    }

    // Required DEI facts
    html.push_str(r#"<ix:nonNumeric name="dei:EntityRegistrantName">Example Corp</ix:nonNumeric>"#);
    html.push_str(r#"<ix:nonNumeric name="dei:EntityCentralIndexKey">0001234567</ix:nonNumeric>"#);
    html.push_str(r#"<ix:nonNumeric name="dei:DocumentType">10-K</ix:nonNumeric>"#);
    html.push_str(r#"<ix:nonNumeric name="dei:DocumentPeriodEndDate">2024-12-31</ix:nonNumeric>"#);

    html.push_str("</body></html>");
    html
}

/// Build a synthetic XBRL instance with `n` contexts and `m` facts.
fn synthetic_xbrl_instance(contexts: usize, facts: usize) -> String {
    let mut xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xbrl xmlns="http://www.xbrl.org/2003/instance" xmlns:us-gaap="http://fasb.org/us-gaap/2024" xmlns:xbrli="http://www.xbrl.org/2003/instance">"#.to_string();

    for i in 0..contexts {
        xml.push_str(&format!(
            r#"
    <xbrli:context id="ctx-{i}">
        <xbrli:entity>
            <xbrli:identifier scheme="http://www.sec.gov/CIK">0001234567</xbrli:identifier>
        </xbrli:entity>
        <xbrli:period>
            <xbrli:instant>2024-12-31</xbrli:instant>
        </xbrli:period>
    </xbrli:context>"#
        ));
    }

    for i in 0..facts {
        let ctx = format!("ctx-{}", i % contexts.max(1));
        xml.push_str(&format!(
            r#"
    <us-gaap:Revenue contextRef="{ctx}" unitRef="usd" decimals="-3">{value}</us-gaap:Revenue>"#,
            value = i * 1000
        ));
    }

    xml.push_str("\n</xbrl>");
    xml
}

fn bench_validation_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation_pipeline");
    let profile = synthetic_validation_profile();

    // Benchmark validate_html_members at different scales
    for size in [10, 50, 100] {
        let html = synthetic_inline_html(size);
        let members: Vec<(&str, &str)> = vec![("member-a.html", &html)];

        group.bench_function(format!("validate_html_members_{size}facts"), |b| {
            b.iter(|| {
                let run = validate_html_members(black_box(&members), black_box(&profile));
                // Sanity check: should not have errors for valid synthetic data
                let has_errors = run.report.findings.iter().any(|f| f.severity == "error");
                assert!(!has_errors, "unexpected errors: {:?}", run.report.findings);
            });
        });
    }

    // Benchmark context parsing
    for ctx_count in [10, 50, 100] {
        let xml = synthetic_xbrl_instance(ctx_count, 0);
        group.bench_function(format!("validate_contexts_{ctx_count}contexts"), |b| {
            b.iter(|| {
                let result = validate_contexts(black_box(&xml));
                assert!(result.is_ok());
                let (set, _) = result.unwrap();
                assert_eq!(set.len(), ctx_count);
            });
        });
    }

    // Benchmark streaming context completeness at different scales
    for (ctx_count, fact_count) in [(10, 50), (50, 200), (100, 500)] {
        let xml = synthetic_xbrl_instance(ctx_count, fact_count);
        group.bench_function(
            format!("validate_context_completeness_streaming_{ctx_count}ctx_{fact_count}facts"),
            |b| {
                b.iter(|| {
                    let findings =
                        validate_context_completeness_streaming(black_box(&xml), black_box(100));
                    // All facts reference valid contexts in synthetic data
                    let errors: Vec<_> =
                        findings.iter().filter(|f| f.severity == "error").collect();
                    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_validation_pipeline);
criterion_main!(benches);
