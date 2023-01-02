use hyper::{
    body::{self, Buf},
    Client,
};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use xml_feed::{XmlFeed, XmlVideo};

mod channel;
mod playlist;
mod xml_feed;

#[derive(Serialize, Deserialize, Debug, Clone, derive_builder::Builder)]
struct Feed {
    title: String,
    author: String,
    channel_id: String,
    playlist_id: Option<String>,
    url: String,
    published: chrono::DateTime<chrono::Utc>,
    videos: Option<Vec<Video>>,
}

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

impl Feed {
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

        let feed = XmlFeed::deserialize(&mut de).expect("could not deseralize xml!");

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
                    .unwrap()
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

        Feed {
            title: feed.title.value,
            author: feed.author.name.value,
            channel_id,
            playlist_id,
            url: feed.author.uri.value,
            published: feed.published.value,
            videos,
        }
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
            description: match video.group.description {
                Some(s) => s.value,
                None => String::new(),
            },
            thumbnail: video.group.thumbnail.url,
            views: video.group.community.statistics.views,
        }
    }
}
