use color_eyre::{eyre::Context, owo_colors::OwoColorize, Help, SectionExt};
use hyper::body::Buf;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::video::Video;

#[derive(Serialize, Deserialize, Debug, Clone, derive_builder::Builder)]
pub struct Feed {
    pub title: String,
    pub author: String,
    pub channel_id: String,
    pub playlist_id: Option<String>,
    pub url: String,
    pub published: chrono::DateTime<chrono::Utc>,
    pub videos: Option<Vec<Video>>,
}

impl Feed {
    pub async fn new(uri: &str) -> Result<Self, color_eyre::Report> {

        let uri = uri.parse::<hyper::http::Uri>()?;

        let https = hyper_tls::HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);

        let body = client.get(uri).await?;
        let body = hyper::body::to_bytes(body)
            .await
            .wrap_err("Failed to read body as bytes.")?;

        let body_str = String::from_utf8(body.clone().into_iter().collect::<Vec<u8>>())?;

        let mut de = serde_xml_rs::Deserializer::new_from_reader(body.reader())
            .non_contiguous_seq_elements(true);

        let feed = XmlFeed::deserialize(&mut de)
            .wrap_err("Failed to deserialize body as XML.")
            .with_section(move || {
                format!("{}", body_str.bold().yellow()).header("Retrieved Body:")
            })?;

        // there is always a channel_id
        let channel_id = match feed.channel_id.value.is_empty() {
            false => feed.channel_id.value,
            true => feed
                .author
                .uri
                .value
                .split("/channel/")
                .collect::<Vec<&str>>()[1]
                .to_string(),
        };

        // there is not always a playlist_id
        let playlist_id = match feed.playlist_id {
            Some(playlist_id) => Some(match playlist_id.value.is_empty() {
                false => playlist_id.value,
                true => feed
                    .link
                    .iter()
                    .find(|link| link.rel == "self")
                    .ok_or(color_eyre::eyre::eyre!(
                        "This Feed does not have a link with \"rel\" == \"self\""
                    ))?
                    .href
                    .split("playlist_id=")
                    .collect::<Vec<&str>>()[1]
                    .to_string(),
            }),
            None => None,
        };

        let videos = feed.videos;
        let videos: Option<Vec<Video>> = videos.map(|vid_vec| {
            vid_vec
                .into_iter()
                .map(|xml_video| Video::from(xml_video))
                .collect()
        });

        Ok(Feed {
            title: feed.title.value,
            author: feed.author.name.value,
            channel_id,
            playlist_id,
            url: feed.author.uri.value,
            published: feed.published.value,
            videos,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "feed")]
struct XmlFeed {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XmlContentString {
    #[serde(default)]
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
