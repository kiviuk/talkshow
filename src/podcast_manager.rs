use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use crate::episodes::Episode;

#[derive(Debug, Clone)]
pub struct Podcast {
    feed_url: String,
    title: String,
    episodes: Vec<Episode>,
    time_added: SystemTime,
    last_updated: Option<SystemTime>,
    total_episodes: usize,
    description: Option<String>,
    author: Option<String>,
}

impl Podcast {
    pub fn new(feed_url: String, title: String, episodes: Vec<Episode>) -> Self {
        Self {
            feed_url,
            title,
            total_episodes: episodes.len(),
            episodes,
            time_added: SystemTime::now(),
            last_updated: None,
            description: None,
            author: None,
        }
    }

    pub fn feed_url(&self) -> &str {
        &self.feed_url
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn episodes(&self) -> &[Episode] {
        &self.episodes
    }

    pub fn time_added(&self) -> SystemTime {
        self.time_added
    }
}

pub struct PodcastManager {
    podcasts: HashMap<String, Podcast>,
}

impl PodcastManager {
    pub fn new() -> Self {
        Self {
            podcasts: HashMap::new(),
        }
    }

    pub fn add_podcast(&mut self, podcast: Podcast) {
        self.podcasts.insert(podcast.feed_url().to_string(), podcast);
    }

    pub fn list_podcast_urls(&self) -> Vec<String> {
        self.podcasts.keys().cloned().collect()
    }

    pub fn list_podcast_titles(&self) -> Vec<String> {
        self.podcasts.values().map(|podcast| podcast.title().to_string()).collect()
    }

    pub fn get_podcast(&self, feed_url: &str) -> Option<&Podcast> {
        self.podcasts.get(feed_url)
    }

    pub fn get_episodes(&self, feed_url: &str) -> Option<&[Episode]> {
        self.podcasts.get(feed_url).map(|podcast| podcast.episodes())
    }
}

// Example usage and tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::episodes::Episode;

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
}
