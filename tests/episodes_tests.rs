use rss_reader::{read_rss_feeds, fetch_episodes, Episode};
use std::io::Write;
use tempfile::NamedTempFile;

fn print_episode_summary(episode: &Episode) {
    println!("Title: {}", episode.title);
    println!("Link: {}", episode.link.as_deref().unwrap_or("None"));
    println!("Publication Date: {}", episode.pub_date.as_deref().unwrap_or("None"));
    println!("Duration: {}", episode.duration.as_deref().unwrap_or("None"));
    println!("Description: {}", episode.description.as_deref().unwrap_or("None"));
}

#[test]
fn test_read_rss_feeds_happy_path() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "https://example.com/feed1.rss").unwrap();
    writeln!(temp_file, "https://example.com/feed2.rss").unwrap();
    
    let feeds = read_rss_feeds(temp_file.path().to_str().unwrap()).unwrap();
    
    assert_eq!(feeds.len(), 2);
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
        assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
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
    writeln!(temp_file, "// Code style comment").unwrap();
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
                assert!(!duration.is_empty(), "If duration is present, it should not be empty");
            }

            // Print summary for debugging
            for (i, episode) in episodes.iter().enumerate() {
                println!("\nEpisode {}:", i + 1);
                print_episode_summary(episode);
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
