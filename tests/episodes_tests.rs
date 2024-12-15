#[cfg(test)]
mod tests {
    use rss_reader::Episode;
    use std::time::Duration;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_feed() {
        let feed_content = fs::read_to_string("tests/test-feed.rss")
            .expect("Failed to read test RSS feed");
        
        let channel = rss::Channel::read_from(feed_content.as_bytes())
            .expect("Failed to parse RSS feed");
        
        let items = channel.items();
        assert!(!items.is_empty(), "Feed should contain items");

        // Test first episode (Sam Aaron episode)
        let first_item = items.first().unwrap();
        let episode = Episode::from_item(first_item.clone())
            .expect("Failed to create episode from item");
        
        assert_eq!(episode.title, "Programming As An Expressive Instrument (with Sam Aaron)");
        assert_eq!(episode.audio_url.unwrap(), 
            "https://redirect.zencastr.com/r/episode/6751276a51560f45d2201d41/size/158427784/audio-files/619e48a9649c44004c5a44e8/d724ade9-cf25-482d-9583-a90188659626.mp3");
        assert_eq!(episode.duration, Some(Duration::from_secs(6601))); // 1:50:01
    }

    #[test]
    fn test_parse_duration() {
        use rss_reader::episodes::parse_duration;

        // Test HH:MM:SS format
        assert_eq!(
            parse_duration("01:30:45"),
            Some(Duration::from_secs(5445)) // 1h 30m 45s
        );

        // Test MM:SS format
        assert_eq!(
            parse_duration("45:30"),
            Some(Duration::from_secs(2730)) // 45m 30s
        );

        // Test seconds format
        assert_eq!(
            parse_duration("6601"),
            Some(Duration::from_secs(6601)) // 1h 50m 1s
        );

        // Test invalid format
        assert_eq!(parse_duration("invalid"), None);
    }

    #[test]
    fn test_read_rss_feeds() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let feed_path = dir.path().join("test-feeds.txt");
        let mut file = File::create(&feed_path)?;

        writeln!(file, "# This is a comment")?;
        writeln!(file, "// Another comment")?;
        writeln!(file, "-- SQL style comment")?;
        writeln!(file, "")?;
        writeln!(file, "https://feed1.com/rss")?;
        writeln!(file, "  https://feed2.com/rss  ")?;
        writeln!(file, "https://feed3.com/rss")?;

        let feeds = rss_reader::read_rss_feeds(feed_path.to_str().unwrap())?;
        
        assert_eq!(feeds.len(), 3);
        assert_eq!(feeds[0], "https://feed1.com/rss");
        assert_eq!(feeds[1], "https://feed2.com/rss");
        assert_eq!(feeds[2], "https://feed3.com/rss");
        
        Ok(())
    }
}
