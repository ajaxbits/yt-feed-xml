#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use rust_decimal::Decimal;
    use serde::Deserialize;

    use super::YtFeed;

    #[test]
    fn test_basic_feed() {
        let vihart = include_str!("../test/vihart.xml");
        let mut de = serde_xml_rs::Deserializer::new_from_reader(vihart.as_bytes())
            .non_contiguous_seq_elements(true);
        let vihart = YtFeed::deserialize(&mut de).unwrap();

        assert_eq!(vihart.id.value, "yt:channel:UCOGeU-1Fig3rrDjhm9Zs_wg");
        assert_eq!(vihart.channel_id.value, "UCOGeU-1Fig3rrDjhm9Zs_wg");
        assert_eq!(vihart.title.value, "Vihart");
        assert_eq!(vihart.author.name.value, "Vihart");
        assert_eq!(
            vihart.author.uri.value,
            "https://www.youtube.com/channel/UCOGeU-1Fig3rrDjhm9Zs_wg"
        );
        assert_eq!(
            vihart.published.value,
            DateTime::parse_from_rfc3339("2009-06-08T02:34:21+00:00").unwrap()
        );
    }
    #[test]
    fn test_single_entry() {
        let vihart = include_str!("../test/vihart.xml");
        let mut de = serde_xml_rs::Deserializer::new_from_reader(vihart.as_bytes())
            .non_contiguous_seq_elements(true);
        let vihart = YtFeed::deserialize(&mut de).unwrap();
        let video = vihart.videos.into_iter().next().unwrap();
        let media = &video.group;

        assert_eq!(video.id.value, "yt:video:Twik7wqdwZU");
        assert_eq!(video.video_id.value, "Twik7wqdwZU");
        assert_eq!(video.channel_id.value, "UCOGeU-1Fig3rrDjhm9Zs_wg");
        assert_eq!(
            video.title.value,
            "only 0.000000001% of people will understand this video"
        );
        assert_eq!(video.link.rel, "alternate");
        assert_eq!(
            video.link.href,
            "https://www.youtube.com/watch?v=Twik7wqdwZU"
        );
        assert_eq!(video.author.name.value, "Vihart");
        assert_eq!(
            video.author.uri.value,
            "https://www.youtube.com/channel/UCOGeU-1Fig3rrDjhm9Zs_wg"
        );
        assert_eq!(
            video.published.value,
            DateTime::parse_from_rfc3339("2022-10-27T04:11:05+00:00").unwrap()
        );
        assert_eq!(
            video.updated.value,
            DateTime::parse_from_rfc3339("2022-10-27T06:56:21+00:00").unwrap()
        );

        assert_eq!(
            media.title.value,
            "only 0.000000001% of people will understand this video"
        );
        assert_eq!(
            media.content.url,
            "https://www.youtube.com/v/Twik7wqdwZU?version=3"
        );
        assert_eq!(media.content.r#type, "application/x-shockwave-flash");
        assert_eq!(media.content.width, 640);
        assert_eq!(media.content.height, 390);
        assert_eq!(
            media.thumbnail.url,
            "https://i1.ytimg.com/vi/Twik7wqdwZU/hqdefault.jpg"
        );
        assert_eq!(media.thumbnail.width, 480);
        assert_eq!(media.thumbnail.height, 360);
        assert_eq!(
            media.description.value,
            "Can you death-of-the-author a math test? How about cultural values? YouTube videos?\n\nThis video references Hank Green's video: https://youtu.be/lBJVyCYuu78\n\nHey, I made a video! Thank you patrons for supporting and encouraging me. I had fun with this one.\n\nUhh there is probably more to say but it's been a while and I forget how to youtube"
        );
        assert_eq!(media.community.star_rating.count, 12923);
        assert_eq!(
            media.community.star_rating.average,
            Decimal::from_str_exact("5.00").unwrap()
        );
        assert_eq!(media.community.star_rating.min, 1);
        assert_eq!(media.community.star_rating.max, 5);
        assert_eq!(media.community.statistics.views, 148559);
    }

    #[test]
    fn test_entry_amounts() {
        let vihart = include_str!("../test/vihart.xml");
        let mut de = serde_xml_rs::Deserializer::new_from_reader(vihart.as_bytes())
            .non_contiguous_seq_elements(true);
        let vihart = YtFeed::deserialize(&mut de).unwrap();
        assert_eq!(vihart.videos.iter().count(), 15);
    }

    #[tokio::test]
    async fn test_grim_beard_id() {
        let url = "https://www.youtube.com/feeds/videos.xml?channel_id=UCNmv1Cmjm3Hk8Vc9kIgv0AQ";
        let resp = reqwest::get(url).await.unwrap();
        let xml = resp.text().await.unwrap();
        let mut de = serde_xml_rs::Deserializer::new_from_reader(xml.as_bytes())
            .non_contiguous_seq_elements(true);

        let grim_beard = YtFeed::deserialize(&mut de).unwrap();
        assert_eq!(grim_beard.videos.iter().count(), 15);
        assert_eq!(grim_beard.id.value, "yt:channel:UCNmv1Cmjm3Hk8Vc9kIgv0AQ");
        assert_eq!(grim_beard.channel_id.value, "UCNmv1Cmjm3Hk8Vc9kIgv0AQ");
        assert_eq!(grim_beard.title.value, "Grim Beard");
        assert_eq!(grim_beard.author.name.value, "Grim Beard");
        assert_eq!(
            grim_beard.author.uri.value,
            "https://www.youtube.com/channel/UCNmv1Cmjm3Hk8Vc9kIgv0AQ"
        );
        assert_eq!(
            grim_beard.published.value,
            DateTime::parse_from_rfc3339("2013-08-14T03:37:55+00:00").unwrap()
        );
    }
}
