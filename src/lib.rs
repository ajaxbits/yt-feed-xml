use std::collections::HashMap;

use async_trait::async_trait;
use hyper::{
    body::{self, Buf},
    Client,
};
use hyper_tls::HttpsConnector;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "feed")]
struct Feed {
    link: Vec<Link>,
    id: XmlContentString,

    #[serde(rename = "channelId")]
    channel_id: XmlContentString,

    #[serde(rename = "playlistId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    playlist_id: Option<XmlContentString>,

    title: XmlContentString,
    author: Author,
    published: XmlContentDateTime,

    #[serde(rename = "entry")]
    videos: Option<Vec<XmlVideo>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, derive_builder::Builder)]
pub struct Channel {
    id: String,
    title: String,
    author: String,
    channel_url: String,
    published: chrono::DateTime<chrono::Utc>,
    videos: Option<Vec<Video>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, derive_builder::Builder)]
pub struct Playlist {
    id: String,
    title: String,
    channel_id: String,
    author: String,
    url: String,
    published: chrono::DateTime<chrono::Utc>,
    videos: Option<Vec<Video>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    id: String,
    url: String,
    channel_id: String,
    title: String,
    author: String,
    author_url: String,
    published: chrono::DateTime<chrono::Utc>,
    updated: chrono::DateTime<chrono::Utc>,
    group: MediaGroup,
}

#[async_trait]
trait Generate {
    async fn new(id: &str) -> Self;
}

#[async_trait]
impl Generate for Feed {
    async fn new(uri: &str) -> Self {
        let uri = uri
            .parse::<hyper::http::Uri>()
            .expect("could not parse url as Uri for some reason");

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let body = client.get(uri).await.expect("could not get feed for id");
        let body = body::to_bytes(body).await.unwrap().reader();

        let mut de =
            serde_xml_rs::Deserializer::new_from_reader(body).non_contiguous_seq_elements(true);

        Feed::deserialize(&mut de).expect("could not deseralize xml!")
    }
}

impl From<XmlVideo> for Video {
    fn from(video: XmlVideo) -> Self {
        Video {
            id: video.video_id.value,
            url: video.link.href,
            channel_id: video.channel_id.value,
            title: video.title.value,
            author: video.author.name.value,
            author_url: video.author.uri.value,
            published: video.published.value,
            updated: video.updated.value,
            group: video.group,
        }
    }
}

#[async_trait]
impl Generate for Channel {
    async fn new(id: &str) -> Self {
        let uri = format!(
            "https://www.youtube.com/feeds/videos.xml?channel_id={}",
            &id
        );

        let feed: Feed = Feed::new(&uri).await;

        let videos = feed
            .videos
            .map(|videos| videos.into_iter().map(Video::from).collect());

        Channel {
            id: feed.id.value,
            title: feed.title.value,
            author: feed.author.name.value,
            channel_url: feed.author.uri.value,
            published: feed.published.value,
            videos,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct XmlContentString {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct XmlContentu32 {
    #[serde(rename = "$value")]
    value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct XmlContentDateTime {
    #[serde(rename = "$value")]
    value: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    rel: String,
    href: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Author {
    name: XmlContentString,
    uri: XmlContentString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct XmlVideo {
    id: XmlContentString,
    video_id: XmlContentString,
    channel_id: XmlContentString,
    title: XmlContentString,
    link: Link,
    author: Author,
    published: XmlContentDateTime,
    updated: XmlContentDateTime,
    group: MediaGroup,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MediaGroup {
    title: XmlContentString,
    content: MediaContent,
    thumbnail: MediaThumbnail,
    description: Option<XmlContentString>,
    community: MediaCommunity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MediaContent {
    url: String,
    r#type: String,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MediaThumbnail {
    url: String,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct MediaCommunity {
    star_rating: MediaStarRating,
    statistics: MediaStatistics,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MediaStarRating {
    count: u32,
    average: Decimal,
    min: u32,
    max: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MediaStatistics {
    views: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_linus() {
        let linus = Feed::new("UCXuqSBlHAE6Xw-yeJA0Tunw").await;
        assert_eq!(linus.id.value, "yt:channel:UCXuqSBlHAE6Xw-yeJA0Tunw");
        assert_eq!(linus.channel_id.value, "UCXuqSBlHAE6Xw-yeJA0Tunw");
        assert_eq!(linus.title.value, "Linus Tech Tips");
    }

    #[tokio::test]
    #[should_panic]
    async fn test_linus_missing_playlist() {
        let linus = Feed::new("UCXuqSBlHAE6Xw-yeJA0Tunw").await;
        let _panic = linus.playlist_id.unwrap();
    }
}
