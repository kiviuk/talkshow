use rss_reader::episodes::{read_rss_feeds, fetch_episodes};
use std::io::Write;
use tempfile::NamedTempFile;

// fn print_episode_summary(episode: &Episode) {
//     println!("Title: {}", episode.title);
//     println!("Link: {}", episode.link.as_deref().unwrap_or("None"));
//     println!("Publication Date: {}", episode.pub_date.as_deref().unwrap_or("None"));
//     println!("Duration: {}", episode.duration.map(|d| format!("{:?}", d)).unwrap_or_else(|| "None".to_string()));
//     println!("Description: {}", episode.description.as_deref().unwrap_or("None"));
// }

#[test]
fn test_read_rss_feeds_happy_path() {
    // Create a temporary file
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    
    // Write some test data to the file
    writeln!(temp_file, "https://example.com/feed1.rss").expect("Failed to write to temp file");
    writeln!(temp_file, "# This is a comment").expect("Failed to write comment");
    writeln!(temp_file, "https://example.com/feed2.rss").expect("Failed to write to temp file");
    writeln!(temp_file, "  // Another comment").expect("Failed to write comment");
    
    // Flush and sync the file to ensure all data is written
    temp_file.flush().expect("Failed to flush temp file");
    temp_file.as_file().sync_all().expect("Failed to sync temp file");
    
    // Get the path of the temporary file
    let file_path = temp_file.path();
    
    // Read RSS feeds from the temporary file
    let feeds = read_rss_feeds(file_path.to_str().expect("Failed to convert path to string"))
        .expect("Failed to read RSS feeds");
    
    // Assert that comments are filtered out
    assert_eq!(feeds.len(), 2, "Should have 2 non-comment feed URLs");
    assert_eq!(feeds[0], "https://example.com/feed1.rss");
    assert_eq!(feeds[1], "https://example.com/feed2.rss");
}

#[test]
fn test_read_rss_feeds_empty_lines() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "\n\nhttps://example.com/feed1.rss\n\n\n  \nhttps://example.com/feed2.rss\n\n").unwrap();
    
    let feeds = read_rss_feeds(temp_file.path().to_str().unwrap()).unwrap();
    
    assert_eq!(feeds.len(), 2, "Should ignore empty lines and whitespace-only lines");
    assert_eq!(feeds[0], "https://example.com/feed1.rss", "First feed URL should be preserved");
    assert_eq!(feeds[1], "https://example.com/feed2.rss", "Second feed URL should be preserved");
}

#[test]
fn test_read_rss_feeds_file_not_found() {
    let result = read_rss_feeds("nonexistent_file.txt");
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.downcast_ref::<std::io::Error>().map_or(false, |io_err| io_err.kind() == std::io::ErrorKind::NotFound), 
                "Expected a 'file not found' error, got: {}", e);
    }
}

#[test]
fn test_read_rss_feeds_with_comments() {
    let mut temp_file = NamedTempFile::new().unwrap();
    
    writeln!(temp_file, "https://feed1.com/rss").unwrap();
    writeln!(temp_file, "# This is a comment").unwrap();
    writeln!(temp_file, "https://feed2.com/rss").unwrap();
    writeln!(temp_file, "-- Another comment").unwrap();
    writeln!(temp_file, "https://feed3.com/rss").unwrap();
    writeln!(temp_file, "// Yet another comment").unwrap();
    writeln!(temp_file, "https://feed4.com/rss").unwrap();
    
    let feeds = read_rss_feeds(temp_file.path().to_str().unwrap()).unwrap();
    
    assert_eq!(feeds.len(), 4);
    assert_eq!(feeds[0], "https://feed1.com/rss");
    assert_eq!(feeds[1], "https://feed2.com/rss");
    assert_eq!(feeds[2], "https://feed3.com/rss");
    assert_eq!(feeds[3], "https://feed4.com/rss");
}

#[test]
fn test_fetch_episodes_real_feed() {
    let feed_url = "https://feeds.zencastr.com/f/oSn1i316.rss";
    match fetch_episodes(feed_url) {
        Ok(episodes) => {
            println!("\nFound {} episodes in feed {}", episodes.len(), feed_url);
            
            // Verify we got episodes
            assert!(!episodes.is_empty(), "Expected at least one episode");
            
            // Check the first episode has all required fields
            let first_episode = &episodes[0];
            
            // Title should never be empty
            assert!(!first_episode.title.is_empty(), "Episode should have a non-empty title");
            
            // Link should be present and be a valid URL
            assert!(first_episode.link.is_some(), "Episode should have a link");
            if let Some(link) = &first_episode.link {
                assert!(link.starts_with("http"), "Link should be a valid URL");
            }
            
            // Publication date should be present
            assert!(first_episode.pub_date.is_some(), "Episode should have a publication date");
            if let Some(date) = &first_episode.pub_date {
                assert!(!date.is_empty(), "Publication date should not be empty");
            }
            
            // Description should be present and non-empty
            assert!(first_episode.description.is_some(), "Episode should have a description");
            if let Some(desc) = &first_episode.description {
                assert!(!desc.is_empty(), "Description should not be empty");
            }
            
            // Duration might be present (it's optional but common in podcasts)
            if let Some(duration) = &first_episode.duration {
                assert!(!duration.is_zero(), "If duration is present, it should not be zero");
            }

            // Print summary for debugging
            for episode in &episodes {
                println!("\nEpisode:");
                println!("Title: {}", episode.title);
                println!("Link: {}", episode.link.as_deref().unwrap_or("None"));
                println!("Publication Date: {}", episode.pub_date.as_deref().unwrap_or("None"));
                println!("Duration: {}", episode.duration.map(|d| format!("{:?}", d)).unwrap_or_else(|| "None".to_string()));
                println!("Description: {}", episode.description.as_deref().unwrap_or("None"));
            }
        }
        Err(e) => {
            panic!("Failed to fetch episodes: {}", e);
        }
    }
}

#[test]
fn test_fetch_episodes_invalid_url() {
    let result = fetch_episodes("https://invalid.url/feed.rss");
    assert!(result.is_err());
}
