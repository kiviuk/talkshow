use anyhow::{Result, anyhow};
use crate::episodes::Episode;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::{BufReader, Cursor};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use reqwest;

#[derive(Debug, PartialEq)]
pub enum PlayerCommand {
    Play,
    Pause,
    Stop,
    SkipForward,
    SkipBackward,
    VolumeUp(f32),
    VolumeDown(f32),
    Quit,
    Ignore,
}

pub struct AudioPlayer {
    sink: Arc<Mutex<Option<Sink>>>,
    _stream: OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    current_file: Arc<Mutex<Option<PathBuf>>>,
    current_position: Arc<Mutex<Duration>>,
    duration: Arc<Mutex<Option<Duration>>>,
    cached_audio_bytes: Arc<Mutex<Option<Vec<u8>>>>,
    is_playing: Arc<Mutex<bool>>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        Ok(Self {
            sink: Arc::new(Mutex::new(None)),
            _stream,
            stream_handle,
            current_file: Arc::new(Mutex::new(None)),
            current_position: Arc::new(Mutex::new(Duration::default())),
            duration: Arc::new(Mutex::new(None)),
            cached_audio_bytes: Arc::new(Mutex::new(None)),
            is_playing: Arc::new(Mutex::new(false)),
        })
    }

    pub fn play(&mut self, episode: &Episode) -> Result<()> {
        // Stop playback, clear previous cached audio, and validate the URL
        self.stop()?;
        self.clear_cached_audio();
        let audio_url = episode.audio_url.as_ref().ok_or_else(|| anyhow::anyhow!("Episode has no audio URL"))?;

        // Download and decode audio
        let audio_bytes = reqwest::blocking::get(audio_url)?.bytes()?.to_vec();
        let source = Decoder::new(BufReader::new(Cursor::new(audio_bytes.clone())))?;

        // Setup playback and store state
        let sink = Sink::try_new(&self.stream_handle)?;
        *self.duration.lock().unwrap() = source.total_duration(); // Cache duration
        *self.cached_audio_bytes.lock().unwrap() = Some(audio_bytes); // Cache audio bytes
        *self.current_file.lock().unwrap() = Some(PathBuf::from(audio_url));
        *self.current_position.lock().unwrap() = Duration::default();

        // Start playback
        sink.append(source);
        *self.sink.lock().unwrap() = Some(sink);
        Ok(())
    }

    pub fn play_from_position(&mut self, position: Duration) -> Result<()> {
        // Ensure position is within total duration
        let total_duration = self.duration()
            .ok_or_else(|| anyhow!("Failed to get duration"))?;
        let adjusted_position = position.min(total_duration);

        // Reuse cached audio for seeking
        let audio_bytes = {
            let cached = self.cached_audio_bytes.lock().unwrap();
            cached
                .as_ref()
                .ok_or_else(|| anyhow!("No cached audio bytes found for seeking"))?
                .clone()
        };
    
        // Create a new decoder for the cached audio stream
        let mut decoder = Decoder::new(BufReader::new(Cursor::new(audio_bytes)))?;
        
        // Attempt to seek to the required position
        if decoder.try_seek(adjusted_position).is_err() {
            return Err(anyhow!(
                "Seeking failed. The audio format may not support seeking."
            ));
        }
    
        // Replace the Sink's current source
        if let Some(sink) = self.sink.lock().unwrap().as_mut() {
            sink.stop();           // Stop the sink
            sink.append(decoder);  // Append the new seeked source
            sink.play();           // Resume playback
        }
    
        // Update current position
        *self.current_position.lock().unwrap() = adjusted_position;
        Ok(())
    }

    pub fn play_from_position_2(&mut self, position: Duration) -> Result<()> {
        // Ensure position is within total duration
        let total_duration = self.duration()
            .ok_or_else(|| anyhow!("Failed to get duration"))?;
        let adjusted_position = position.min(total_duration);

        // Try to seek in the existing sink
        if let Some(sink) = self.sink.lock().unwrap().as_mut() {
            if sink.try_seek(adjusted_position).is_err() {
                // If seeking fails, log a warning but continue
                println!("Warning: Seeking not supported or failed");
            }
        }

        // Update current position
        *self.current_position.lock().unwrap() = adjusted_position;
        Ok(())
    }

    pub fn skip(&mut self, seconds: i64) -> Result<()> {
        let current_pos = self.current_position();
        let total_duration = self.duration().map_or(Duration::from_secs(0), |d| d);
        let new_pos = match seconds.is_positive() {
            true => (current_pos + Duration::from_secs(seconds as u64)).min(total_duration),
            false => current_pos.saturating_sub(Duration::from_secs(-seconds as u64)),
        };
    
        // Use play_from_position for seeking
        self.play_from_position(new_pos)?;
    
        // Print skip message
        println!(
            "{} {} seconds. New position: {:?}",
            if seconds >= 0 { "⏩ Skipped forward" } else { "⏪ Skipped backward" },
            seconds.abs(),
            new_pos
        );
        Ok(())
    }    

    pub fn resume(&mut self) -> Result<()> {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            sink.play();
        }
        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        let sink = self.sink.lock().unwrap();
        if let Some(sink) = sink.as_ref() {
            if sink.is_paused() {
                sink.play();
                println!("▶️ Resumed playback");
            } else {
                sink.pause();
                println!("⏸️ Paused playback");
            }
        }
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        if let Some(sink) = self.sink.lock().unwrap().take() {
            sink.stop();
            println!("⏹️ Stopped playback");
        }
        Ok(())
    }

    pub fn adjust_volume(&mut self, step: f32) -> Result<()> {
        if let Some(sink) = self.sink.lock().unwrap().as_mut() {
            sink.set_volume((sink.volume() + step).max(0.0));
            println!(
                "🔊 Volume {} to {:.1}",
                if step > 0.0 { "increased" } else { "decreased" },
                sink.volume()
            );
        }
        Ok(())
    }

    // Helpers
    pub fn current_position(&self) -> Duration {
        *self.current_position.lock().unwrap()
    }

    pub fn duration(&self) -> Option<Duration> {
        *self.duration.lock().unwrap()
    }

    pub fn clear_cached_audio(&self) {
        *self.cached_audio_bytes.lock().unwrap() = None;
    }
}