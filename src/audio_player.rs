use anyhow::Result;
use crate::episodes::Episode;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct AudioPlayer {
    sink: Arc<Mutex<Option<Sink>>>,
    _stream: OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    current_file: Arc<Mutex<Option<PathBuf>>>,
    current_position: Arc<Mutex<Duration>>,
    duration: Arc<Mutex<Option<Duration>>>,
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
        })
    }

    pub fn play(&mut self, episode: &Episode) -> Result<()> {
        // Stop any existing playback
        self.stop();

        // Open the audio file
        let audio_url = episode.audio_url
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Episode has no audio URL"))?;
        let file = BufReader::new(File::open(audio_url)?);
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

        Ok(())
    }

    pub fn play_from_position(&mut self, position: Duration) -> Result<()> {
        // Stop any existing playback
        self.stop();

        // Ensure position is within total duration
        let total_duration = self.duration.lock().unwrap().unwrap_or(Duration::from_secs(0));
        let clamped_position = position.min(total_duration);

        // Open the audio file
        let current_file = self.current_file.lock().unwrap()
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No current audio file"))?
            .clone();
        let file = BufReader::new(File::open(&current_file)?);
        let source = Decoder::new(file)?;

        // Create a new sink
        let sink = Sink::try_new(&self.stream_handle)?;

        // Skip to the desired position and append to sink
        let skipped_source = source.skip_duration(clamped_position);
        sink.append(skipped_source);

        // Update current position
        *self.current_position.lock().unwrap() = clamped_position;

        // Store sink
        *self.sink.lock().unwrap() = Some(sink);

        Ok(())
    }

    pub fn skip(&mut self, seconds: i64) -> Result<()> {
        // Get current position
        let current_pos = self.current_position.lock().unwrap();
        
        // Calculate new position
        let new_pos = if seconds >= 0 {
            *current_pos + Duration::from_secs(seconds as u64)
        } else {
            current_pos.saturating_sub(Duration::from_secs(seconds.unsigned_abs()))
        };

        // Play from the new position
        drop(current_pos);
        self.play_from_position(new_pos)?;

        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        if let Some(sink) = &*self.sink.lock().unwrap() {
            sink.pause();
        }
        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        if let Some(sink) = &*self.sink.lock().unwrap() {
            sink.play();
        }
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        // Clear the sink
        if let Some(sink) = self.sink.lock().unwrap().take() {
            sink.stop();
        }
        
        // Reset position
        *self.current_position.lock().unwrap() = Duration::from_secs(0);

        Ok(())
    }

    pub fn volume_up(&mut self, step: f32) -> Result<()> {
        if let Some(sink) = &*self.sink.lock().unwrap() {
            let current_volume = sink.volume();
            sink.set_volume(current_volume + step);
        }
        Ok(())
    }

    pub fn volume_down(&mut self, step: f32) -> Result<()> {
        if let Some(sink) = &*self.sink.lock().unwrap() {
            let current_volume = sink.volume();
            sink.set_volume((current_volume - step).max(0.0));
        }
        Ok(())
    }

    pub fn current_position(&self) -> Duration {
        *self.current_position.lock().unwrap()
    }

    pub fn duration(&self) -> Option<Duration> {
        *self.duration.lock().unwrap()
    }
}
