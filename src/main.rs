use anyhow::Result;
use rss_reader::{read_rss_feeds, fetch_episodes, stream_episode_audio};

fn main() -> Result<()> {
    // Read RSS feed URLs from file
    let feeds = read_rss_feeds("rss-db.txt")?;
    
    // Fetch episodes from the first feed
    if let Some(feed_url) = feeds.first() {
        println!("Fetching episodes from: {}", feed_url);
        let episodes = fetch_episodes(feed_url)?;
        
        // Print available episodes
        println!("\nAvailable Episodes:");
        for (i, episode) in episodes.iter().enumerate() {
            println!("{}. {}", i + 1, episode.title);
        }
        
        // Play the first episode
        if let Some(episode) = episodes.first() {
            println!("\nPlaying first episode...");
            stream_episode_audio(episode)?;
        } else {
            println!("No episodes found!");
        }
    } else {
        println!("No RSS feeds found in rss-db.txt!");
    }
    
    Ok(())
}
