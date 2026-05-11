//! Events flowing over the MPSC channel into the main event loop.

use super::{Article, UpdateInfo};

/// Events flowing over the MPSC channel into the main event loop.
#[derive(Debug)]
pub enum AppEvent {
    /// A key input from the user.
    Input(crossterm::event::KeyEvent),
    /// Periodic tick for UI updates and animations.
    Tick,
    /// Result of a background feed fetch: (feed_idx, Result<(articles, xml_updated_secs), error>).
    FeedFetched(usize, Result<(Vec<Article>, Option<i64>), String>),
    /// Result of background feed-title fetch during the AddFeed flow.
    FeedTitleFetched(Result<String, String>),
    /// A newer version of brochure is available on crates.io.
    UpdateAvailable(UpdateInfo),
    /// An image has been downloaded for display in the article detail.
    ImageDownloaded(String, Result<Vec<u8>, String>),
}
