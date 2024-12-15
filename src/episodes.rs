use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Duration;
use anyhow::{Result, Context};
use rss::Channel;
use reqwest::blocking::Client;

#[derive(Debug)]
pub struct Episode {
    pub title: String,
    pub link: Option<String>,
    pub description: Option<String>,
    pub pub_date: Option<String>,
    pub duration: Option<String>,
    pub audio_url: Option<String>,
}

pub fn read_rss_feeds(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File not found: {}", file_path)
        ));
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut feeds = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();
        if !trimmed.is_empty() && !is_comment(trimmed) {
            feeds.push(trimmed.to_string());
        }
    }

    Ok(feeds)
}

fn is_comment(line: &str) -> bool {
    line.starts_with('#') || line.starts_with("--") || line.starts_with("//")
}

pub fn fetch_episodes(feed_url: &str) -> Result<Vec<Episode>> {
    // Create a blocking HTTP client with timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .context("Failed to create HTTP client")?;
    
    // Fetch the RSS feed content
    let content = client.get(feed_url)
        .send()
        .context("Failed to fetch RSS feed")?
        .bytes()
        .context("Failed to read response body")?;
    
    // Parse the RSS feed
    let channel: Channel = Channel::read_from(&content[..])
        .context("Failed to parse RSS feed")?;
    
    // Convert items to our Episode struct
    let episodes: Vec<Episode> = channel.items()
        .iter()
        .map(|item| Episode {
            title: item.title().map_or("Untitled".to_string(), ToString::to_string),
            link: item.link().map(String::from),
            description: item.description().map(String::from),
            pub_date: item.pub_date().map(String::from),
            duration: item.itunes_ext()
                .and_then(|itunes| itunes.duration())
                .map(String::from),
            audio_url: item.enclosure().map(|e| e.url().to_string()),
        })
        .collect();

    if episodes.is_empty() {
        anyhow::bail!("No episodes found in the feed");
    }

    Ok(episodes)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Unit tests can go here if needed
}
