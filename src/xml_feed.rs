use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "feed")]
pub struct XmlFeed {
    pub link: Vec<Link>,
    pub id: XmlContentString,

    #[serde(rename = "channelId")]
    pub channel_id: XmlContentString,

    #[serde(rename = "playlistId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_id: Option<XmlContentString>,

    pub title: XmlContentString,
    pub author: Author,
    pub published: XmlContentDateTime,

    #[serde(rename = "entry")]
    pub videos: Option<Vec<XmlVideo>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XmlContentString {
    #[serde(default)]
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct XmlContentu32 {
    #[serde(rename = "$value")]
    value: u32,
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
pub struct XmlVideo {
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
