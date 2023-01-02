use serde::Deserialize;
use serde::Serialize;

use crate::Feed;
use crate::Video;

#[derive(Serialize, Deserialize, Debug, Clone, derive_builder::Builder)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub author: String,
    pub channel_id: String,
    pub url: String,
    pub published: chrono::DateTime<chrono::Utc>,
    pub videos: Option<Vec<Video>>,
}

impl Playlist {
    pub async fn new(id: &str) -> Self {
        let uri = format!(
            "https://www.youtube.com/feeds/videos.xml?playlist_id={}",
            &id
        );

        let feed: Feed = Feed::new(&uri).await;
        feed.into()
    }
}

impl From<Feed> for Playlist {
    fn from(f: Feed) -> Self {
        Self {
            id: f.playlist_id.expect("all Playlists have a playlist_id"),
            title: f.title,
            author: f.author,
            channel_id: f.channel_id,
            url: f.url,
            published: f.published,
            videos: f.videos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sinclair_lore_playlist() {
        let sinclair_lore_va_masq = Playlist::new("PLOIA4n5j7KcYj52DQ9orEBJDA9IqBTB3I").await;
        assert_eq!(
            sinclair_lore_va_masq.id,
            "PLOIA4n5j7KcYj52DQ9orEBJDA9IqBTB3I"
        );
        assert_eq!(sinclair_lore_va_masq.channel_id, "UCH6IMeS2HVdTJZU4BlN6ODg");
        assert_eq!(
            sinclair_lore_va_masq.title,
            "Vampire the Masquerade ► Down Under by Night | Actual Play"
        );
    }
}
