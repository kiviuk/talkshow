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
    SkipForward(i64),
    SkipBackward(i64),
    VolumeUp(f32),
    VolumeDown(f32),
    Quit,
    Ignore,
}

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    sink: Arc<Mutex<Option<Sink>>>,
    current_file: Arc<Mutex<Option<PathBuf>>>,
    current_position: Arc<Mutex<Duration>>,
    duration: Arc<Mutex<Option<Duration>>>,
}
pub trait AudioPlayerTrait {
    fn play(&mut self, episode: &Episode) -> Result<()>;
    fn resume(&mut self) -> Result<()>;
    fn pause(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn skip(&mut self, seconds: i64) -> Result<()>;
    fn adjust_volume(&mut self, step: f32) -> Result<()>;
}

impl AudioPlayerTrait for AudioPlayer {
    fn play(&mut self, episode: &Episode) -> Result<()> {
        AudioPlayer::play(self, episode)
    }

    fn resume(&mut self) -> Result<()> {
        AudioPlayer::resume(self)
    }

    fn pause(&mut self) -> Result<()> {
        AudioPlayer::pause(self)
    }

    fn stop(&mut self) -> Result<()> {
        AudioPlayer::stop(self)
    }

    fn skip(&mut self, seconds: i64) -> Result<()> {
        AudioPlayer::skip(self, seconds)
    }

    fn adjust_volume(&mut self, step: f32) -> Result<()> {
        AudioPlayer::adjust_volume(self, step)
    }
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        Ok(Self {
            _stream,
            stream_handle,
            sink: Arc::new(Mutex::new(None)),
            current_file: Arc::new(Mutex::new(None)),
            current_position: Arc::new(Mutex::new(Duration::default())),
            duration: Arc::new(Mutex::new(None)),
        })
    }

    pub fn play(&mut self, episode: &Episode) -> Result<()> {
        // Stop playback, clear previous cached audio, and validate the URL
        self.stop()?;
        
        let audio_url = episode.audio_url.as_ref().ok_or_else(|| anyhow!("Episode has no audio URL"))?;

        // Download and decode audio
        let audio_bytes = reqwest::blocking::get(audio_url)?.bytes()?.to_vec();
        let source = Decoder::new(BufReader::new(Cursor::new(audio_bytes.clone())))?;

        // Setup playback and store state
        let sink = Sink::try_new(&self.stream_handle)?;
        *self.duration.lock().unwrap() = source.total_duration();
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
}