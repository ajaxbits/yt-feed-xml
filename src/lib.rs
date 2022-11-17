use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "feed")]
pub struct YtFeed {
    pub link: Vec<Link>,
    pub id: XmlContentString,
    #[serde(rename = "channelId")]
    pub channel_id: XmlContentString,
    pub title: XmlContentString,
    pub author: Author,
    pub published: XmlContentDateTime,
    #[serde(rename = "entry")]
    pub videos: Vec<Video>,
}

impl YtFeed {
    pub async fn from_channel_id(id: &str) -> Self {
        // YtFeed::deserialize(&mut de).unwrap()
        todo!()
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
    pub description: XmlContentString,
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

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}