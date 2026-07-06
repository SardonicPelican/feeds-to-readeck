// Copyright 2016 Francis Gagné
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Benchmarks for the feed parsing hot path.
//!
//! `feeds-to-pocket` downloads RSS and Atom feeds and parses them on every
//! sync. The parsing dispatch mirrors `Feed::from_str` in `src/main.rs`: try
//! to parse the body as Atom first, then fall back to RSS. These benchmarks
//! exercise that same dispatch against representative feed payloads.

use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Parses a feed body the same way the application does: attempt Atom first,
/// then fall back to RSS.
fn parse_feed(body: &str) {
    match body.parse::<atom_syndication::Feed>() {
        Ok(feed) => {
            black_box(feed);
        }
        Err(_) => {
            let channel = body.parse::<rss::Channel>().expect("valid RSS feed");
            black_box(channel);
        }
    }
}

fn build_atom_feed(entries: usize) -> String {
    let mut feed = String::from(
        r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Example Atom Feed</title>
  <link href="https://example.com/"/>
  <link rel="alternate" href="https://example.com/index.html"/>
  <updated>2024-01-01T00:00:00Z</updated>
  <id>urn:uuid:60a76c80-d399-11d9-b93C-0003939e0af6</id>
"#,
    );
    for i in 0..entries {
        feed.push_str(&format!(
            r#"  <entry>
    <title>Entry number {i}</title>
    <link rel="alternate" href="https://example.com/posts/{i}"/>
    <id>urn:uuid:1225c695-cfb8-4ebb-aaaa-{i:012}</id>
    <updated>2024-01-01T00:00:00Z</updated>
    <summary>This is the summary for entry number {i}. It contains a fair amount of text to make parsing representative of real-world feeds with prose content.</summary>
  </entry>
"#,
        ));
    }
    feed.push_str("</feed>\n");
    feed
}

fn build_rss_feed(items: usize) -> String {
    let mut feed = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Example RSS Feed</title>
    <link>https://example.com/</link>
    <description>An example RSS feed for benchmarking.</description>
    <lastBuildDate>Mon, 01 Jan 2024 00:00:00 GMT</lastBuildDate>
"#,
    );
    for i in 0..items {
        feed.push_str(&format!(
            r#"    <item>
      <title>Item number {i}</title>
      <link>https://example.com/posts/{i}</link>
      <description>This is the description for item number {i}. It contains a fair amount of text to make parsing representative of real-world feeds with prose content.</description>
      <pubDate>Mon, 01 Jan 2024 00:00:00 GMT</pubDate>
      <guid>https://example.com/posts/{i}</guid>
    </item>
"#,
        ));
    }
    feed.push_str("  </channel>\n</rss>\n");
    feed
}

fn bench_feed_parsing(c: &mut Criterion) {
    let atom_feed = build_atom_feed(50);
    let rss_feed = build_rss_feed(50);

    let mut group = c.benchmark_group("feed_parsing");

    // Atom parses on the first attempt.
    group.bench_function("atom", |b| b.iter(|| parse_feed(black_box(&atom_feed))));

    // RSS fails the Atom attempt and falls back, matching the app's dispatch.
    group.bench_function("rss", |b| b.iter(|| parse_feed(black_box(&rss_feed))));

    group.finish();
}

criterion_group!(benches, bench_feed_parsing);
criterion_main!(benches);
