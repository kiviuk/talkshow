use anyhow::Result;
use crate::episodes::Episode;
use rodio::{Decoder, OutputStream, Sink, Source};
use rodio::source::SeekError;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use reqwest;
use std::io::Cursor;

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
            current_position: Arc::new(Mutex::new(Duration::from_secs(0))),
            duration: Arc::new(Mutex::new(None)),
            cached_audio_bytes: Arc::new(Mutex::new(None)),
            is_playing: Arc::new(Mutex::new(false)),
        })
    }

    pub fn play(&mut self, episode: &Episode) -> Result<()> {
        // Stop any existing playback
        let _ = self.stop();

        // Clear cached audio
        self.clear_cached_audio();

        // Get the audio URL
        let audio_url = episode.audio_url
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Episode has no audio URL"))?;

        // Download the audio file
        let response = reqwest::blocking::get(audio_url)?;
        let audio_bytes = response.bytes()?;

        // Create cursor from audio bytes
        let cursor = Cursor::new(audio_bytes.to_vec());
        let file = BufReader::new(cursor);
        
        // Decode the audio
        let source = Decoder::new(file)?;

        // Set duration
        let duration = source.total_duration();
        *self.duration.lock().unwrap() = duration;

        // Create a new sink
        let sink = Sink::try_new(&self.stream_handle)?;

        // Reset position
        *self.current_position.lock().unwrap() = Duration::from_secs(0);

        // Append source to sink
        sink.append(source);

        // Store sink and file
        *self.sink.lock().unwrap() = Some(sink);
        *self.current_file.lock().unwrap() = Some(PathBuf::from(audio_url));

        // Cache audio bytes
        *self.cached_audio_bytes.lock().unwrap() = Some(audio_bytes.to_vec());

        Ok(())
    }

    pub fn play_from_position(&mut self, position: Duration) -> Result<()> {
        // Ensure position is within total duration
        let total_duration = self.duration.lock().unwrap().unwrap_or(Duration::from_secs(0));
        let adjusted_position = position.min(total_duration);

        // Check if we have an existing sink
        if let Some(sink) = self.sink.lock().unwrap().as_mut() {
            // Attempt to seek to the desired position
            if let Err(_) = sink.try_seek(adjusted_position) {
                // If seeking is not supported or fails, do nothing
                return Ok(());
            }
            // Seeking was successful
            *self.current_position.lock().unwrap() = adjusted_position;
        }

        Ok(())
    }

    pub fn skip(&mut self, seconds: i64) -> Result<()> {
        // Get current position
        let current_pos = self.current_position();
        
        // Calculate new position
        let new_pos = if seconds >= 0 {
            // Forward skip
            let total_duration = self.duration().unwrap_or(Duration::from_secs(0));
            (current_pos + Duration::from_secs(seconds as u64)).min(total_duration)
        } else {
            // Backward skip
            current_pos.checked_sub(Duration::from_secs((-seconds) as u64))
                .unwrap_or(Duration::from_secs(0))
        };
        
        // Play from new position
        self.play_from_position(new_pos)?;
        
        // Print skip direction
        if seconds > 0 {
            println!("⏩ Skipped forward {} seconds", seconds);
        } else if seconds < 0 {
            println!("⏪ Skipped backward {} seconds", -seconds);
        }
        
        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
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

    pub fn resume(&mut self) -> Result<()> {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            sink.play();
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

    pub fn volume_up(&mut self, step: f32) -> Result<()> {
        if let Some(sink) = self.sink.lock().unwrap().as_mut() {
            let current_volume = sink.volume();
            sink.set_volume(current_volume + step);
            println!("🔊 Volume increased to {:.1}", sink.volume());
        }
        Ok(())
    }

    pub fn volume_down(&mut self, step: f32) -> Result<()> {
        if let Some(sink) = self.sink.lock().unwrap().as_mut() {
            let current_volume = sink.volume();
            let new_volume = (current_volume - step).max(0.0);
            sink.set_volume(new_volume);
            println!("🔉 Volume decreased to {:.1}", new_volume);
        }
        Ok(())
    }

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
