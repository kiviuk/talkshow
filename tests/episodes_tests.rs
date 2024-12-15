#[cfg(test)]
mod tests {
    use rss_reader::Episode;
    use std::time::Duration;
    use std::fs;

    #[test]
    fn test_parse_feed() {
        // Use the test-feed.rss file in the tests directory
        let feed_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test-feed.rss");
        
        let feed_content = fs::read_to_string(&feed_path)
            .expect("Failed to read test feed file");
        
        let channel = rss::Channel::read_from(feed_content.as_bytes())
            .expect("Failed to parse RSS feed");
        
        let episodes: Vec<Episode> = channel.items()
            .iter()
            .map(|item| Episode::from_item(item.clone()).expect("Failed to create episode"))
            .collect();
        
        assert!(!episodes.is_empty(), "Should parse at least one episode");
        
        // Basic checks on the first episode
        let first_episode = &episodes[0];
        println!("Episode title: {}", first_episode.title);
        println!("Episode audio URL: {:?}", first_episode.audio_url);
        assert!(first_episode.title.contains("Sam"), "Episode title should contain 'Sam'");
        assert!(first_episode.audio_url.is_some(), "Episode should have an audio URL");
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
}
