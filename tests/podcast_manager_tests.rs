use rss_reader::podcast_manager::{Podcast, PodcastManager, load_podcasts};
use rss_reader::episodes::Episode;
use anyhow::Result;

#[test]
fn test_podcast_manager() {
    let mut manager = PodcastManager::new();

    // Create some dummy episodes
    let podcast1_episodes = vec![
        Episode {
            title: "Podcast 1 Episode 1".to_string(),
            ..Default::default()
        },
        Episode {
            title: "Podcast 1 Episode 2".to_string(),
            ..Default::default()
        },
    ];

    let podcast2_episodes = vec![
        Episode {
            title: "Podcast 2 Episode 1".to_string(),
            ..Default::default()
        },
    ];

    // Create podcasts
    let podcast1 = Podcast::new(
        "https://example.com/podcast1".to_string(), 
        "First Awesome Podcast".to_string(), 
        podcast1_episodes
    );

    let podcast2 = Podcast::new(
        "https://example.com/podcast2".to_string(), 
        "Second Cool Podcast".to_string(), 
        podcast2_episodes
    );

    // Add podcasts
    manager.add_podcast(podcast1);
    manager.add_podcast(podcast2);

    // Test listing podcast URLs
    let podcast_urls = manager.list_podcast_urls();
    assert_eq!(podcast_urls.len(), 2);
    assert!(podcast_urls.contains(&"https://example.com/podcast1".to_string()));
    assert!(podcast_urls.contains(&"https://example.com/podcast2".to_string()));

    // Test listing podcast titles
    let podcast_titles = manager.list_podcast_titles();
    assert_eq!(podcast_titles.len(), 2);
    assert!(podcast_titles.contains(&"First Awesome Podcast".to_string()));
    assert!(podcast_titles.contains(&"Second Cool Podcast".to_string()));

    // Test getting episodes
    let podcast1_eps = manager.get_episodes("https://example.com/podcast1");
    assert!(podcast1_eps.is_some());
    assert_eq!(podcast1_eps.unwrap().len(), 2);

    let podcast2_eps = manager.get_episodes("https://example.com/podcast2");
    assert!(podcast2_eps.is_some());
    assert_eq!(podcast2_eps.unwrap().len(), 1);
}

#[test]
fn test_load_podcasts() -> Result<()> {
    // Mock read_rss_feeds function
    let mock_read_rss_feeds = |_: &str| -> Result<Vec<String>> {
        Ok(vec![
            "https://example.com/podcast1".to_string(),
            "https://example.com/podcast2".to_string(),
        ])
    };

    // Mock fetch_episodes function
    let mock_fetch_episodes = |feed_url: &str| -> Result<Vec<Episode>> {
        // Create mock episodes based on feed URL
        Ok(vec![
            Episode {
                title: format!("{} - Episode 1", feed_url),
                ..Default::default()
            },
            Episode {
                title: format!("{} - Episode 2", feed_url),
                ..Default::default()
            }
        ])
    };

    // Create a podcast manager and load podcasts
    let mut podcast_manager = PodcastManager::new();
    load_podcasts("dummy_path", &mut podcast_manager, mock_read_rss_feeds, mock_fetch_episodes)?;

    // Verify the podcasts were loaded correctly
    assert_eq!(podcast_manager.list_podcast_urls().len(), 2);
    assert!(podcast_manager.list_podcast_urls().contains(&"https://example.com/podcast1".to_string()));
    assert!(podcast_manager.list_podcast_urls().contains(&"https://example.com/podcast2".to_string()));

    // Verify episodes were fetched and added
    let podcast1 = podcast_manager.get_podcast("https://example.com/podcast1").unwrap();
    assert_eq!(podcast1.episodes().len(), 2);
    assert_eq!(podcast1.title(), "https://example.com/podcast1 - Episode 1");

    let podcast2 = podcast_manager.get_podcast("https://example.com/podcast2").unwrap();
    assert_eq!(podcast2.episodes().len(), 2);
    assert_eq!(podcast2.title(), "https://example.com/podcast2 - Episode 1");

    Ok(())
}
