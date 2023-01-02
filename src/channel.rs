use color_eyre::eyre::Context;
use serde::Deserialize;
use serde::Serialize;

use crate::video::Video;
use crate::xml_feed::Feed;

#[derive(Serialize, Deserialize, Debug, Clone, derive_builder::Builder)]
pub struct Channel {
    pub id: String,
    pub title: String,
    pub author: String,
    pub url: String,
    pub published: chrono::DateTime<chrono::Utc>,
    pub videos: Option<Vec<Video>>,
}

impl Channel {
    pub async fn new(id: &str) -> Self {
        let uri = format!(
            "https://www.youtube.com/feeds/videos.xml?channel_id={}",
            &id
        );

        let feed: Feed = Feed::new(&uri)
            .await
            .wrap_err("Failed to create Channel.")
            .unwrap();
        feed.into()
    }
}

impl From<Feed> for Channel {
    fn from(f: Feed) -> Self {
        Self {
            id: f.channel_id,
            title: f.title,
            author: f.author,
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
    async fn test_linus() {
        let linus = Channel::new("UCXuqSBlHAE6Xw-yeJA0Tunw").await;
        assert_eq!(linus.id, "UCXuqSBlHAE6Xw-yeJA0Tunw");
        assert_eq!(linus.title, "Linus Tech Tips");
    }

    #[tokio::test]
    #[should_panic]
    async fn test_linus_missing_playlist() {
        let linus = Feed::new(
            "https://www.youtube.com/feeds/videos.xml?channel_id=UCXuqSBlHAE6Xw-yeJA0Tunw",
        )
        .await
        .unwrap();
        let _panic = linus.playlist_id.unwrap();
    }
}
