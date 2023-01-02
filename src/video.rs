use serde::{Deserialize, Serialize};

use crate::xml_feed::XmlVideo;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub thumbnail: String,
    pub published: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub url: String,
    pub author_url: String,
    pub channel_id: String,
    pub views: u64,
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
            description: match video.group.description {
                Some(s) => s.value,
                None => String::new(),
            },
            thumbnail: video.group.thumbnail.url,
            views: video.group.community.statistics.views,
        }
    }
}
