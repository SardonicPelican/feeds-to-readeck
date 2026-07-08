// Copyright 2025
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Benchmarks for the CPU-bound work `feeds-to-readeck` performs on every run:
//! parsing downloaded RSS/Atom feeds and building Readeck API endpoint URLs.

use divan::{Bencher, black_box};
use feeds_to_readeck::feed::Feed;
use feeds_to_readeck::readeck::bookmarks_endpoint;
use url::Url;

fn main() {
    divan::main();
}

/// Builds a syntactically valid RSS 2.0 feed with `n` items.
fn generate_rss(n: usize) -> String {
    let mut items = String::new();
    for i in 0..n {
        items.push_str(&format!(
            "<item>\
                <title>Example item number {i}</title>\
                <link>https://example.com/articles/{i}</link>\
                <guid>https://example.com/articles/{i}</guid>\
                <description>A reasonably sized description for item {i} \
                so the parser has some content to work through.</description>\
                <pubDate>Wed, 01 Jan 2025 00:00:00 GMT</pubDate>\
            </item>"
        ));
    }
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
        <rss version=\"2.0\">\
            <channel>\
                <title>Example RSS Feed</title>\
                <link>https://example.com</link>\
                <description>An example feed used for benchmarking.</description>\
                {items}\
            </channel>\
        </rss>"
    )
}

/// Builds a syntactically valid Atom feed with `n` entries.
fn generate_atom(n: usize) -> String {
    let mut entries = String::new();
    for i in 0..n {
        entries.push_str(&format!(
            "<entry>\
                <title>Example entry number {i}</title>\
                <id>urn:uuid:00000000-0000-0000-0000-{i:012}</id>\
                <updated>2025-01-01T00:00:00Z</updated>\
                <link rel=\"alternate\" href=\"https://example.com/articles/{i}\"/>\
                <summary>A reasonably sized summary for entry {i} \
                so the parser has some content to work through.</summary>\
            </entry>"
        ));
    }
    format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\
        <feed xmlns=\"http://www.w3.org/2005/Atom\">\
            <title>Example Atom Feed</title>\
            <id>urn:uuid:00000000-0000-0000-0000-000000000000</id>\
            <updated>2025-01-01T00:00:00Z</updated>\
            {entries}\
        </feed>"
    )
}

/// Parsing an Atom feed: `Feed::from_str` succeeds on the first attempt.
#[divan::bench(args = [16, 128])]
fn parse_atom(bencher: Bencher, entries: usize) {
    let data = generate_atom(entries);
    bencher.bench(|| black_box(&data).parse::<Feed>().unwrap());
}

/// Parsing an RSS feed: `Feed::from_str` first fails as Atom, then succeeds as
/// RSS. This exercises the exact fallback path the application uses.
#[divan::bench(args = [16, 128])]
fn parse_rss(bencher: Bencher, entries: usize) {
    let data = generate_rss(entries);
    bencher.bench(|| black_box(&data).parse::<Feed>().unwrap());
}

/// Building the `/api/bookmarks` endpoint URL, done once per pushed entry.
#[divan::bench]
fn build_bookmarks_endpoint(bencher: Bencher) {
    let base = Url::parse("https://readeck.example.com/subpath").unwrap();
    bencher.bench(|| bookmarks_endpoint(black_box(&base)));
}
