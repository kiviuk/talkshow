use anyhow::Result;
use env_logger::Env;
use log::{info, error};
use rss_reader::{
    audio_player::AudioPlayer, 
    fetch_episodes, 
    read_rss_feeds, 
    play_episode,
    episodes::pretty_print
};

mod tui;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Starting RSS Reader Application");

    // Fetch and read RSS feeds
    let feeds = match read_rss_feeds("rss-db.txt") {
        Ok(feeds) => {
            info!("Successfully read RSS feeds");
            feeds
        },
        Err(e) => {
            error!("Failed to read RSS feeds: {}", e);
            return Err(e);
        }
    };

    // Fetch episodes
    let episodes = match fetch_episodes(&feeds.first().ok_or_else(|| anyhow::anyhow!("No feeds found"))?) {
        Ok(episodes) => {
            info!("Successfully fetched episodes");
            episodes
        },
        Err(e) => {
            error!("Failed to fetch episodes: {}", e);
            return Err(e);
        }
    };

    // Pretty print episodes
    for episode in &episodes {
        println!("{}", pretty_print(episode));
    }

    // Example lists for TUI
    let left_items = vec![
        "Podcast 1".to_string(), 
        "Podcast 2".to_string(), 
        "Podcast 3".to_string()
    ];
    let right_items = vec![
        "Episode A".to_string(), 
        "Episode B".to_string(), 
        "Episode C".to_string()
    ];

    // Initialize TUI
    let mut tui = tui::Tui::new(left_items, right_items)?;
    
    info!("Launching Terminal User Interface");
    tui.run()?;

    // List episodes
    for (i, episode) in episodes.iter().enumerate() {
        println!("{}. {}", i + 1, episode.title);
    }
    
    // Get episode selection
    println!("\nSelect episode (1-{}):", episodes.len());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let mut audio_player = AudioPlayer::new()?;
    
    let episode_num: usize = input.trim().parse()?;
    if episode_num > 0 && episode_num <= episodes.len() {
        let selected_episode = &episodes[episode_num - 1];
        
        // Display episode details
        println!("\n--- Episode Details ---");
        println!("{}", pretty_print(selected_episode));
        
        // Play the episode
        play_episode(&mut audio_player, selected_episode)?;
    }
    
    Ok(())
}
