use anyhow::{Result, Context};
use crate::episodes::Episode;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct AudioPlayer {
    sink: Arc<Mutex<Sink>>,
    _stream: OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    current_source: Arc<Mutex<Option<Vec<u8>>>>,
    position: Arc<Mutex<Duration>>,
    is_playing: Arc<Mutex<bool>>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()
            .context("Failed to open audio output stream")?;
        let sink = Sink::try_new(&stream_handle)
            .context("Failed to create audio sink")?;
        
        Ok(Self {
            sink: Arc::new(Mutex::new(sink)),
            _stream,
            stream_handle,
            current_source: Arc::new(Mutex::new(None)),
            position: Arc::new(Mutex::new(Duration::from_secs(0))),
            is_playing: Arc::new(Mutex::new(false)),
        })
    }

    pub fn play(&self, episode: &Episode) -> Result<()> {
        let audio_url = episode.audio_url
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Episode has no audio URL"))?;

        println!("Starting stream of: {}", episode.title);
        println!("Audio URL: {}", audio_url);
        
        // Create a blocking HTTP client with timeout
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        
        // Get the audio data
        let response = client.get(audio_url)
            .send()
            .context("Failed to fetch audio")?;
        
        // Check content type for supported formats
        let content_type = response.headers()
            .get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("");

        if !is_supported_format(content_type) {
            return Err(anyhow::anyhow!("Unsupported audio format: {}", content_type));
        }

        let bytes = response.bytes()
            .context("Failed to read audio data")?;
        
        // Store the audio data for seeking
        *self.current_source.lock().unwrap() = Some(bytes.to_vec());
        *self.is_playing.lock().unwrap() = true;
        self.play_from_position(Duration::from_secs(0))?;
        
        println!("\nPlayback Controls:");
        println!("  p: Play/Pause");
        println!("  q: Stop and quit");
        println!("  +: Volume up");
        println!("  -: Volume down");
        println!("  f: Skip forward 10 seconds");
        println!("  b: Skip backward 10 seconds");
        println!("\nPress Enter after each command");

        Ok(())
    }

    fn play_from_position(&self, position: Duration) -> Result<()> {
        println!("Playing from position: {:?}", position);
        if let Some(audio_data) = self.current_source.lock().unwrap().as_ref() {
            println!("Audio data size: {} bytes", audio_data.len());
            
            // Create a new decoder for the audio data
            let cursor = Cursor::new(audio_data.clone());
            let source = Decoder::new(cursor)
                .context("Failed to decode audio data")?;

            // Convert samples and buffer for smoother playback
            let source = source
                .skip_duration(position)
                .convert_samples::<f32>()
                .buffered();
            
            // Get current sink state
            let volume;
            let was_paused;
            {
                let sink = self.sink.lock().unwrap();
                volume = sink.volume();
                was_paused = sink.is_paused();
            }
            
            // Create new sink with same state
            let new_sink = Sink::try_new(&self.stream_handle)
                .context("Failed to create new sink")?;
            new_sink.set_volume(volume);
            
            // Append source and restore state
            new_sink.append(source);
            if was_paused {
                new_sink.pause();
            }
            
            // Replace old sink
            *self.sink.lock().unwrap() = new_sink;
            *self.position.lock().unwrap() = position;
            println!("Successfully started playback from new position");
        } else {
            println!("No audio data available");
        }
        Ok(())
    }

    pub fn skip_forward(&self) {
        if *self.is_playing.lock().unwrap() {
            let current_pos = *self.position.lock().unwrap();
            let new_pos = current_pos + Duration::from_secs(10);
            
            match self.play_from_position(new_pos) {
                Ok(_) => println!("⏩ Skipped forward 10 seconds"),
                Err(e) => println!("Failed to skip forward: {}", e),
            }
        }
    }

    pub fn skip_backward(&self) {
        if *self.is_playing.lock().unwrap() {
            let current_pos = *self.position.lock().unwrap();
            let new_pos = current_pos.checked_sub(Duration::from_secs(10))
                .unwrap_or(Duration::from_secs(0));
            
            if let Ok(_) = self.play_from_position(new_pos) {
                println!("⏪ Skipped backward 10 seconds");
            }
        }
    }

    pub fn pause(&self) {
        let mut sink = self.sink.lock().unwrap();
        if sink.is_paused() {
            sink.play();
            println!("▶️ Resumed playback");
        } else {
            sink.pause();
            println!("⏸️ Paused playback");
        }
    }

    pub fn stop(&self) {
        self.sink.lock().unwrap().stop();
        *self.is_playing.lock().unwrap() = false;
        println!("⏹️ Stopped playback");
    }

    pub fn volume_up(&self) {
        let mut sink = self.sink.lock().unwrap();
        sink.set_volume(sink.volume() + 0.1);
        println!("🔊 Volume increased");
    }

    pub fn volume_down(&self) {
        let mut sink = self.sink.lock().unwrap();
        sink.set_volume((sink.volume() - 0.1).max(0.0));
        println!("🔉 Volume decreased");
    }
}

fn is_supported_format(content_type: &str) -> bool {
    matches!(content_type.to_lowercase().as_str(),
        "audio/mpeg" | "audio/mp3" | "audio/ogg" | "audio/wav" | 
        "audio/x-wav" | "audio/wave" | "audio/webm" | "audio/aac" |
        "audio/flac" | "audio/x-flac"
    )
}

pub fn stream_episode_audio(episode: &Episode) -> Result<()> {
    use std::io::{self, BufRead};

    // Create a new audio player
    let player = AudioPlayer::new()?;
    
    // Start playing the episode
    player.play(episode)?;

    // Handle keyboard input
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();
    
    loop {
        buffer.clear();
        if stdin.read_line(&mut buffer)? == 0 {
            break;
        }
        
        match buffer.trim() {
            "p" => player.pause(),
            "q" => {
                player.stop();
                break;
            },
            "+" => player.volume_up(),
            "-" => player.volume_down(),
            "f" => player.skip_forward(),
            "b" => player.skip_backward(),
            _ => (),
        }
    }

    Ok(())
}
