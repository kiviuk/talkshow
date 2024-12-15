use anyhow::Result;
use rss_reader::{read_rss_feeds, fetch_episodes, AudioControl};

mod tui;

fn main() -> Result<()> {
    // Option to launch TUI
    let mut tui = tui::Tui::new()?;
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
    
    let episode_num: usize = input.trim().parse()?;
    if episode_num > 0 && episode_num <= episodes.len() {
        let selected_episode = &episodes[episode_num - 1];
        
        // Display episode details
        println!("\n--- Episode Details ---");
        println!("{}", rss_reader::Episode::pretty_print(selected_episode));
        
        // Play the episode
        let mut audio_control = AudioControl::new()?;
        audio_control.play_episode(selected_episode)?;
    }
    
    Ok(())
}
