use std::collections::HashMap;
use std::time::SystemTime;
use crate::episodes::Episode;
use anyhow::Result;

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

pub struct PodcastStore {
    podcasts: HashMap<String, Podcast>,
}

impl PodcastStore {
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

pub fn load_podcasts(
    filename: &str, 
    podcast_manager: &mut PodcastStore, 
    read_feeds_fn: impl Fn(&str) -> Result<Vec<String>>,
    fetch_episodes_fn: impl Fn(&str) -> Result<Vec<Episode>>
) -> Result<()> {
    let feed_urls = read_feeds_fn(filename)?;
    
    for feed_url in feed_urls {
        let episodes = fetch_episodes_fn(&feed_url)?;
        
        // Extract title from first episode or use feed URL
        let title = episodes.first()
            .map(|ep| ep.title.clone())
            .unwrap_or_else(|| feed_url.clone());
        
        let podcast = Podcast::new(
            feed_url.clone(), 
            title, 
            episodes
        );
        
        podcast_manager.add_podcast(podcast);
    }
    
    Ok(())
}
