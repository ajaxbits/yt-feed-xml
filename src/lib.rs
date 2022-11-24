use derive_builder::Builder;
use hyper::{
    body::{self, Buf},
    Client,
};
use hyper_tls::HttpsConnector;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
#[serde(rename = "feed")]
pub struct Feed {
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
    videos: Option<Vec<Video>>,
}

impl Feed {
    pub async fn new(id: &str) -> Self {
        let uri = format!(
            "https://www.youtube.com/feeds/videos.xml?channel_id={}",
            &id
        )
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XmlContentString {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XmlContentu32 {
    #[serde(rename = "$value")]
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XmlContentDateTime {
    #[serde(rename = "$value")]
    pub value: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    pub rel: String,
    pub href: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    pub name: XmlContentString,
    pub uri: XmlContentString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: XmlContentString,
    pub video_id: XmlContentString,
    pub channel_id: XmlContentString,
    pub title: XmlContentString,
    pub link: Link,
    pub author: Author,
    pub published: XmlContentDateTime,
    pub updated: XmlContentDateTime,
    pub group: MediaGroup,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaGroup {
    pub title: XmlContentString,
    pub content: MediaContent,
    pub thumbnail: MediaThumbnail,
    pub description: Option<XmlContentString>,
    pub community: MediaCommunity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaContent {
    pub url: String,
    pub r#type: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaThumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaCommunity {
    pub star_rating: MediaStarRating,
    pub statistics: MediaStatistics,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaStarRating {
    pub count: u32,
    pub average: Decimal,
    pub min: u32,
    pub max: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaStatistics {
    pub views: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_linus() {
        let linus = Feed::new("UCXuqSBlHAE6Xw-yeJA0Tunw").await;
        assert_eq!(linus.id.value, "yt:channel:UCXuqSBlHAE6Xw-yeJA0Tunw");
        assert_eq!(linus.channel_id.value, "UCXuqSBlHAE6Xw-yeJA0Tunw");
    }

    #[tokio::test]
    #[should_panic]
    async fn test_linus_missing_playlist() {
        let linus = Feed::new("UCXuqSBlHAE6Xw-yeJA0Tunw").await;
        let _panic = linus.playlist_id.unwrap();
    }
}
