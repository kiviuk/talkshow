use anyhow::Result;
use rss_reader::{audio_player, fetch_episodes, read_rss_feeds, AudioControl};

mod tui;

fn main() -> Result<()> {
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

    // Option to launch TUI
    let mut tui = tui::Tui::new(left_items, right_items)?;
    tui.run()?;

    let feed_url = read_rss_feeds("rss-db.txt")?
        .first()
        .ok_or_else(|| anyhow::anyhow!("No feeds found"))?
        .clone();
    
    let episodes = fetch_episodes(&feed_url)?;
    
    // List episodes
    for (i, episode) in episodes.iter().enumerate() {
        println!("{}. {}", i + 1, episode.title);
    }
    
    // Get episode selection
    println!("\nSelect episode (1-{}):", episodes.len());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let mut audio_player = audio_player::AudioPlayer::new()?;
    
    let episode_num: usize = input.trim().parse()?;
    if episode_num > 0 && episode_num <= episodes.len() {
        let selected_episode = &episodes[episode_num - 1];
        
        // Display episode details
        println!("\n--- Episode Details ---");
        println!("{}", rss_reader::Episode::pretty_print(selected_episode));
        
        // Play the episode
        let audio_control: AudioControl = AudioControl::new();
        audio_control.play_episode(&mut audio_player, selected_episode)?;
    }
    
    Ok(())
}
