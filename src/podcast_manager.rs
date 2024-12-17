use std::collections::HashMap;
use crate::episodes::Episode;

pub struct PodcastManager {
    podcasts: HashMap<String, Vec<Episode>>,
}

impl PodcastManager {
    pub fn new() -> Self {
        Self {
            podcasts: HashMap::new(),
        }
    }

    pub fn add_podcast(&mut self, name: String, episodes: Vec<Episode>) {
        self.podcasts.insert(name, episodes);
    }

    pub fn list_podcasts(&self) -> Vec<String> {
        self.podcasts.keys().cloned().collect()
    }

    pub fn get_episodes(&self, podcast_name: &str) -> Option<&Vec<Episode>> {
        self.podcasts.get(podcast_name)
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

        // Add podcasts
        manager.add_podcast("Podcast 1".to_string(), podcast1_episodes);
        manager.add_podcast("Podcast 2".to_string(), podcast2_episodes);

        // Test listing podcasts
        let podcasts = manager.list_podcasts();
        assert_eq!(podcasts.len(), 2);
        assert!(podcasts.contains(&"Podcast 1".to_string()));
        assert!(podcasts.contains(&"Podcast 2".to_string()));

        // Test getting episodes
        let podcast1_eps = manager.get_episodes("Podcast 1");
        assert!(podcast1_eps.is_some());
        assert_eq!(podcast1_eps.unwrap().len(), 2);

        let podcast2_eps = manager.get_episodes("Podcast 2");
        assert!(podcast2_eps.is_some());
        assert_eq!(podcast2_eps.unwrap().len(), 1);
    }
}
