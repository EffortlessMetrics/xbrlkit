//! Benchmarks for the HTML, context, and streaming validation paths.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use sec_profile_types::{NumericRules, ProfilePack};
use std::fmt::Write as _;
use validation_run::{
    validate_context_completeness_streaming, validate_contexts, validate_html_members,
};

fn benchmark_profile() -> ProfilePack {
    ProfilePack {
        id: "benchmark-profile".to_string(),
        label: "Benchmark profile".to_string(),
        required_facts: vec!["dei:Fact0".to_string(), "dei:Fact1".to_string()],
        numeric_rules: Some(NumericRules::default()),
        ..ProfilePack::default()
    }
}

fn inline_html(fact_count: usize) -> String {
    let mut html = String::from(
        r#"<html xmlns:ix="http://www.xbrl.org/2013/inlineXBRL" xmlns:dei="http://xbrl.sec.gov/dei/2024"><body>"#,
    );
    for index in 0..fact_count {
        let _ = write!(
            html,
            r#"<ix:nonNumeric name="dei:Fact{index}">Value {index}</ix:nonNumeric>"#
        );
    }
    html.push_str("</body></html>");
    html
}

fn xbrl_instance(context_count: usize, fact_count: usize) -> String {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?><xbrl xmlns="http://www.xbrl.org/2003/instance" xmlns:us-gaap="http://fasb.org/us-gaap/2024" xmlns:xbrli="http://www.xbrl.org/2003/instance">"#,
    );
    for index in 0..context_count {
        let _ = write!(
            xml,
            r#"<xbrli:context id="ctx-{index}"><xbrli:entity><xbrli:identifier scheme="http://www.sec.gov/CIK">0001234567</xbrli:identifier></xbrli:entity><xbrli:period><xbrli:instant>2024-12-31</xbrli:instant></xbrli:period></xbrli:context>"#
        );
    }
    for index in 0..fact_count {
        let context = index % context_count.max(1);
        let _ = write!(
            xml,
            r#"<us-gaap:Revenue contextRef="ctx-{context}">{}</us-gaap:Revenue>"#,
            index * 1000
        );
    }
    xml.push_str("</xbrl>");
    xml
}

fn bench_validation_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation_pipeline");
    let profile = benchmark_profile();

    for size in [10, 50, 100] {
        let html = inline_html(size);
        let members = vec![("member-a.html", html.as_str())];
        group.bench_function(format!("validate_html_members_{size}_facts"), |benchmark| {
            benchmark.iter(|| validate_html_members(black_box(&members), black_box(&profile)));
        });
    }

    for context_count in [10, 50, 100] {
        let xml = xbrl_instance(context_count, 0);
        group.bench_function(
            format!("validate_contexts_{context_count}_contexts"),
            |benchmark| {
                benchmark.iter(|| validate_contexts(black_box(&xml)));
            },
        );
    }

    for (context_count, fact_count) in [(10, 50), (50, 200), (100, 500)] {
        let xml = xbrl_instance(context_count, fact_count);
        group.bench_function(
            format!(
                "validate_context_completeness_streaming_{context_count}_contexts_{fact_count}_facts"
            ),
            |benchmark| {
                benchmark.iter(|| {
                    validate_context_completeness_streaming(black_box(&xml), black_box(100))
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_validation_pipeline);
criterion_main!(benches);
