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
