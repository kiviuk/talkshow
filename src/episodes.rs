use anyhow::{Context, Result};
use rss::Channel;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Episode {
    pub title: String,
    pub link: Option<String>,
    pub description: Option<String>,
    pub pub_date: Option<String>,
    pub duration: Option<Duration>,
    pub audio_url: Option<String>,
}

impl Episode {
    pub fn from_item(item: rss::Item) -> Option<Self> {
        let title = item.title()?;
        let enclosure = item.enclosure()?;
        let audio_url = if enclosure.mime_type.starts_with("audio/") {
            Some(enclosure.url.to_string())
        } else {
            None
        };

        let duration = item.itunes_ext()
            .and_then(|ext| ext.duration.clone())
            .and_then(|dur| parse_duration(&dur));

        Some(Episode {
            title: title.to_string(),
            audio_url,
            duration,
            link: item.link().map(|s| s.to_string()),
            pub_date: item.pub_date().map(|s| s.to_string()),
            description: item.description().map(|s| s.to_string()),
        })
    }

}

pub fn pretty_print(episode: &Episode) -> String {
    let mut details = Vec::new();
    
    details.push(format!("🎙️  Title: {}", episode.title));
    
    if let Some(link) = &episode.link {
        details.push(format!("🔗  Link: {}", link));
    }
    
    if let Some(pub_date) = &episode.pub_date {
        details.push(format!("📅  Published: {}", pub_date));
    }
    
    if let Some(duration) = episode.duration {
        let hours = duration.as_secs() / 3600;
        let minutes = (duration.as_secs() % 3600) / 60;
        let seconds = duration.as_secs() % 60;
        details.push(format!("⏱️  Duration: {:02}:{:02}:{:02}", hours, minutes, seconds));
    }
    
    if let Some(audio_url) = &episode.audio_url {
        details.push(format!("🎧  Audio URL: {}", audio_url));
    }
    
    if let Some(description) = &episode.description {
        // Truncate description if it's too long
        let truncated_desc = if description.len() > 200 {
            format!("{}...", &description[..200])
        } else {
            description.to_string()
        };
        details.push(format!("📝  Description: {}", truncated_desc));
    }
    
    details.join("\n")
}

pub fn read_rss_feeds(filename: &str) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| !line.trim_start().starts_with('#'))
        .filter(|line| !line.trim_start().starts_with("//"))
        .filter(|line| !line.trim_start().starts_with("--"))
        .map(|line| line.trim().to_string())
        .collect())
}

pub fn fetch_episodes(feed_url: &str) -> Result<Vec<Episode>> {
    let client = reqwest::blocking::Client::new();
    
    let content = client.get(feed_url)
        .send()
        .with_context(|| format!("Failed to fetch RSS feed from {}", feed_url))?
        .bytes()
        .context("Failed to read RSS feed content")?;
    
    let channel = Channel::read_from(&content[..])
        .with_context(|| format!("Failed to parse RSS feed from {}", feed_url))?;
    
    let episodes: Vec<Episode> = channel.items()
        .iter()
        .filter_map(|item| Episode::from_item(item.clone()))
        .collect();
    
    println!("Found {} episodes", episodes.len());
    Ok(episodes)
}

pub fn parse_duration(duration_str: &str) -> Option<Duration> {
    // Try HH:MM:SS format
    if let Some(duration) = parse_hhmmss(duration_str) {
        return Some(duration);
    }
    
    // Try MM:SS format
    if let Some(duration) = parse_mmss(duration_str) {
        return Some(duration);
    }
    
    // Try seconds format
    duration_str.parse::<u64>()
        .ok()
        .map(Duration::from_secs)
}

fn parse_hhmmss(duration_str: &str) -> Option<Duration> {
    let parts: Vec<&str> = duration_str.split(':').collect();
    if parts.len() == 3 {
        let hours = parts[0].parse::<u64>().ok()?;
        let minutes = parts[1].parse::<u64>().ok()?;
        let seconds = parts[2].parse::<u64>().ok()?;
        Some(Duration::from_secs(hours * 3600 + minutes * 60 + seconds))
    } else {
        None
    }
}

fn parse_mmss(duration_str: &str) -> Option<Duration> {
    let parts: Vec<&str> = duration_str.split(':').collect();
    if parts.len() == 2 {
        let minutes = parts[0].parse::<u64>().ok()?;
        let seconds = parts[1].parse::<u64>().ok()?;
        Some(Duration::from_secs(minutes * 60 + seconds))
    } else {
        None
    }
}
