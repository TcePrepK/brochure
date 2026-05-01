//! Core domain types: Feed, Article, and user-persistent data structures.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use super::CategoryId;

/// A single RSS/Atom feed source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    /// Display name of the feed.
    pub title: String,
    /// URL to the feed's RSS/Atom document.
    pub url: String,
    /// Which category this feed belongs to. None = root / uncategorized.
    #[serde(default)]
    pub category_id: Option<CategoryId>,
    /// Display order among siblings (lower = first).
    #[serde(default)]
    pub order: usize,
    /// Count of unread articles in this feed (runtime, not persisted).
    #[serde(skip, default)]
    pub unread_count: usize,
    /// Articles fetched from this feed (runtime, not persisted).
    #[serde(skip, default)]
    pub articles: Vec<Article>,
    /// Whether this feed has been fetched at least once (runtime, not persisted).
    #[serde(skip, default)]
    pub fetched: bool,
    /// Last fetch error message, if any.
    #[serde(skip, default)]
    pub fetch_error: Option<String>,
    /// Unix timestamp (seconds) from the feed's own `<updated>` / `<lastBuildDate>` field.
    #[serde(skip)]
    pub feed_updated_secs: Option<i64>,
    /// Unix timestamp (seconds) of our last successful fetch of this feed.
    #[serde(default)]
    pub last_fetched_secs: Option<i64>,
}

/// A single article entry from a feed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    /// Article title.
    pub title: String,
    /// Brief description or summary from the feed (not persisted if content is saved).
    pub description: String,
    /// URL to the original article.
    pub link: String,
    /// Whether the user has marked this article as read.
    pub is_read: bool,
    /// Whether this article has been saved to a category. Runtime flag; not persisted on Article.
    #[serde(default)]
    pub is_saved: bool,
    /// Full article text (populated by readability fetch or saved from content field).
    #[serde(default)]
    pub content: String,
    /// URL to a hero image for the article.
    #[serde(default)]
    pub image_url: Option<String>,
    /// Name of the feed this article was fetched from (set at fetch time).
    #[serde(default)]
    pub source_feed: String,
    /// Unix timestamp (seconds) of when the article was published.
    #[serde(default)]
    pub published_secs: Option<i64>,
}

/// Default serde function that returns `true`.
fn default_true() -> bool {
    true
}

/// A user-defined category for saved articles.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SavedCategory {
    /// Unique identifier for this category.
    pub id: u32,
    /// Display name of the category.
    pub name: String,
}

/// An article saved by the user into a named category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedArticle {
    /// The article that was saved.
    pub article: Article,
    /// ID of the category this article was saved to.
    pub category_id: u32,
}

/// User-specific persistent data (read/starred state).
#[derive(Serialize, Deserialize, Default)]
pub struct UserData {
    /// Set of article links marked as read by the user.
    pub read_links: HashSet<String>,
    /// Articles saved by the user into named categories.
    #[serde(default)]
    pub saved_articles: Vec<SavedArticle>,
    /// User-defined categories for organizing saved articles.
    #[serde(default)]
    pub saved_categories: Vec<SavedCategory>,
    /// Legacy field: populated when reading old user_data.json. Migrated on load, never re-written.
    #[serde(default, skip_serializing)]
    pub starred_articles: Vec<Article>,
    /// Whether to save full article content when fetching (vs. description only).
    #[serde(default)]
    pub save_article_content: bool,
    /// Whether to use rounded borders in the UI.
    #[serde(default)]
    pub border_rounded: bool,
    /// Whether to eagerly fetch full article content when viewing an article.
    #[serde(default = "default_true")]
    pub eager_article_fetch: bool,
    /// Whether to automatically fetch feeds when the app starts.
    #[serde(default = "default_true")]
    pub auto_fetch_on_start: bool,
}
