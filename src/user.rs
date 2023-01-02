use serde::{Deserialize, Serialize};

use crate::{Feed, Video};

#[derive(Serialize, Deserialize, Debug, Clone, derive_builder::Builder)]
pub struct User {
    pub id: String,
    pub title: String,
    pub author: String,
    pub url: String,
    pub published: chrono::DateTime<chrono::Utc>,
    pub videos: Option<Vec<Video>>,
}

impl User {
    pub async fn new(user_name: &str) -> Self {
        let uri = format!(
            "https://www.youtube.com/feeds/videos.xml?user={}",
            &user_name
        );

        let feed: Feed = Feed::new(&uri).await;
        feed.into()
    }
}

impl From<Feed> for User {
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
    async fn test_cgpgrey_user() {
        let cgpgrey_user = User::new("cgpgrey").await;
        assert_eq!(cgpgrey_user.id, "UC2C_jShtL725hvbm1arSV9w");
        assert_eq!(cgpgrey_user.title, "CGP Grey");
    }

    #[tokio::test]
    #[should_panic]
    async fn test_cgpgrey_user_missing_playlist() {
        let cgpgrey_user = Feed::new("https://www.youtube.com/feeds/videos.xml?user=cgpgrey").await;
        let _panic = cgpgrey_user.playlist_id.unwrap();
    }
}
