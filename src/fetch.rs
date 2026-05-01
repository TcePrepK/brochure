//! Async network I/O for brochure: feed fetching, image URL extraction, and Readability-based
//! article content retrieval. All HTTP requests share a single lazily-initialised client.

use crate::models::{Article, ReleaseNote, UpdateInfo};
use std::sync::OnceLock;

/// Returns the shared, lazily-initialised HTTP client used for all outgoing requests.
fn http_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .user_agent("brochure/0.1 (RSS reader)")
            .build()
            .expect("failed to build HTTP client")
    })
}

/// Returns the compiled regex used to extract the first `https` image URL from HTML content.
fn img_url_re() -> &'static regex::Regex {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    RE.get_or_init(|| regex::Regex::new(r#"<img[^>]+src=["'](https?://[^"']+)["']"#).unwrap())
}

/// Strips a UTF-8 BOM (`EF BB BF`) from the start of a byte slice if one is present.
fn strip_bom(bytes: &[u8]) -> &[u8] {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &bytes[3..]
    } else {
        bytes
    }
}

/// Fetch and parse a single RSS/Atom feed URL into a list of articles.
/// Returns `(articles, xml_updated_secs)` where `xml_updated_secs` is the feed-level
/// `<updated>` / `<lastBuildDate>` timestamp as Unix seconds, if present.
pub async fn fetch_feed(url: &str) -> Result<(Vec<Article>, Option<i64>), String> {
    let bytes = http_client()
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    let parsed = feed_rs::parser::parse(strip_bom(&bytes)).map_err(|e| {
        let msg = e.to_string();
        if msg.contains("no element") || msg.contains("unable to parse feed") {
            "URL is not a valid RSS/Atom feed".to_string()
        } else {
            format!("Failed to parse feed: {msg}")
        }
    })?;
    let xml_updated_secs = parsed.updated.map(|dt| dt.timestamp());

    let articles = parsed
        .entries
        .into_iter()
        .map(|entry| {
            let title = entry
                .title
                .map(|t| t.content)
                .unwrap_or_else(|| "No Title".to_string());
            let description = entry
                .summary
                .map(|s| s.content)
                .unwrap_or_else(|| "No Description".to_string());
            let link = entry
                .links
                .into_iter()
                .next()
                .map(|l| l.href)
                .unwrap_or_default();
            let html_content = entry
                .content
                .and_then(|c| c.body)
                .unwrap_or_else(|| description.clone());
            let image_url = img_url_re()
                .captures(&html_content)
                .map(|caps| caps[1].to_string());
            let content = html2md::parse_html(&html_content);
            let published_secs = entry.published.or(entry.updated).map(|dt| dt.timestamp());

            Article {
                title,
                description,
                link,
                is_read: false,
                is_saved: false,
                content,
                image_url,
                source_feed: String::new(), // filled in by on_feed_fetched in main.rs
                published_secs,
                is_archived: false,
            }
        })
        .collect();

    Ok((articles, xml_updated_secs))
}

/// Fetch just the feed title from a URL (used for AddFeed title auto-fill).
pub async fn fetch_feed_title(url: &str) -> Result<String, String> {
    let bytes = http_client()
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    let parsed = feed_rs::parser::parse(strip_bom(&bytes)).map_err(|e| e.to_string())?;
    Ok(parsed.title.map(|t| t.content).unwrap_or_default())
}

/// Fetch and extract readable article content from a URL using Mozilla's Readability algorithm.
pub async fn fetch_readable_content(url: &str) -> Result<String, String> {
    let bytes = http_client()
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    let parsed_url = reqwest::Url::parse(url).map_err(|_| "Invalid URL".to_string())?;
    let mut cursor = std::io::Cursor::new(bytes);
    readability::extractor::extract(&mut cursor, &parsed_url)
        .map(|product| product.content)
        .map_err(|e| format!("Readability error: {e}"))
}

/// Fetch the latest published versions of brochure from crates.io and GitHub releases.
/// Filters GitHub releases to only those newer than the current version, sorted newest-first.
/// Returns `Some(UpdateInfo)` if any newer versions exist, `None` if already up to date.
/// GitHub release fetch failures are graceful — falls back to returning `Some` with empty releases list.
pub async fn check_latest_version() -> Option<UpdateInfo> {
    // Step 1: Crates.io check (early exit if version matches)
    let resp = http_client()
        .get("https://crates.io/api/v1/crates/brochure")
        .header(
            "User-Agent",
            concat!("brochure/", env!("CARGO_PKG_VERSION"), " (version-check)"),
        )
        .send()
        .await
        .ok()?;
    let text = resp.text().await.ok()?;
    let json: serde_json::Value = serde_json::from_str(&text).ok()?;
    let crates_latest = json["crate"]["newest_version"].as_str()?.to_string();
    let current = env!("CARGO_PKG_VERSION");
    if crates_latest == current {
        return None;
    }

    // Step 2: GitHub releases fetch (returns array, graceful failure)
    let releases = if let Ok(gh_resp) = http_client()
        .get("https://api.github.com/repos/Sylviromi/brochure/releases")
        .header(
            "User-Agent",
            concat!("brochure/", env!("CARGO_PKG_VERSION"), " (version-check)"),
        )
        .send()
        .await
    {
        if let Ok(gh_text) = gh_resp.text().await {
            if let Ok(gh_json) = serde_json::from_str::<serde_json::Value>(&gh_text) {
                if let Some(releases_array) = gh_json.as_array() {
                    let mut release_notes = Vec::new();

                    for release in releases_array {
                        // Parse tag_name and strip leading 'v'
                        let tag_name = match release["tag_name"].as_str() {
                            Some(tag) => tag.strip_prefix('v').unwrap_or(tag),
                            None => continue,
                        };

                        // Step 3: Version filtering - only keep newer versions
                        if !is_newer_version(tag_name, current) {
                            continue;
                        }

                        // Step 4: Parse release into ReleaseNote
                        let date = release["published_at"]
                            .as_str()
                            .map(|s| s.chars().take(10).collect::<String>())
                            .unwrap_or_default();

                        let highlights = if let Some(body) = release["body"].as_str() {
                            let mut highlights_vec = Vec::new();
                            let mut in_highlights_section = false;
                            for line in body.lines() {
                                if line.starts_with("## Highlights") {
                                    in_highlights_section = true;
                                } else if line.starts_with("## ") {
                                    in_highlights_section = false;
                                } else if in_highlights_section && line.starts_with("- ") {
                                    highlights_vec.push(line[2..].to_string());
                                }
                            }
                            highlights_vec
                        } else {
                            Vec::new()
                        };

                        release_notes.push(ReleaseNote {
                            version: tag_name.to_string(),
                            date,
                            highlights,
                        });
                    }

                    // Step 5: Sort newest-first by semver
                    release_notes.sort_by(|a, b| {
                        let a_semver = parse_semver(&a.version);
                        let b_semver = parse_semver(&b.version);
                        b_semver.cmp(&a_semver)
                    });

                    release_notes
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    // Return None if no newer versions, otherwise return Some with releases list
    if releases.is_empty() {
        None
    } else {
        Some(UpdateInfo { releases })
    }
}

/// Parse a semantic version string into (major, minor, patch) tuple.
fn parse_semver(v: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    Some((
        parts[0].parse().ok()?,
        parts[1].parse().ok()?,
        parts[2].parse().ok()?,
    ))
}

/// Check if `tag_version` is strictly newer than `current_version` using semver comparison.
fn is_newer_version(tag_version: &str, current_version: &str) -> bool {
    match (parse_semver(tag_version), parse_semver(current_version)) {
        (Some(tag_semver), Some(current_semver)) => tag_semver > current_semver,
        _ => false,
    }
}
