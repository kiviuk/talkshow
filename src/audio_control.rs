use anyhow::Result;
use crate::audio_player::{AudioPlayer, PlayerCommand};
use crate::keyboard_controls::Controls;
use crate::episodes::Episode;

pub struct AudioControl {
    player: AudioPlayer,
    controls: Controls,
}

impl AudioControl {
    pub fn new() -> Result<Self> {
        Ok(Self {
            player: AudioPlayer::new()?,
            controls: Controls::new(),
        })
    }

    pub fn play_episode(&mut self, episode: &Episode) -> Result<()> {
        self.player.play(episode)?;
        self.print_controls();
        self.run()
    }

    fn print_controls(&self) {
        println!("\nPlayback Controls:");
        println!("  p: Play/Pause");
        println!("  q: Stop and quit");
        println!("  +: Volume up ({:.1} step)", self.controls.volume_step());
        println!("  -: Volume down ({:.1} step)", self.controls.volume_step());
        println!("  f: Skip forward {} seconds", self.controls.skip_seconds());
        println!("  b: Skip backward {} seconds", self.controls.skip_seconds());
        println!("\nPress Enter after each command");
    }

    pub fn run(&mut self) -> Result<()> {
        let mut current_controls = self.controls.clone();
        loop {
            match current_controls.get_user_input()? {
                Some((command, updated_controls)) => {
                    current_controls = updated_controls;
                    match command {
                        PlayerCommand::Quit => break,
                        _ => self.process_command(command)?,
                    }
                }
                None => break,
            }
        }
        Ok(())
    }

    pub fn process_command(&mut self, command: PlayerCommand) -> Result<()> {
        match command {
            PlayerCommand::Play => self.player.resume()?,
            PlayerCommand::Pause => self.player.pause()?,
            PlayerCommand::Stop => self.player.stop()?,
            PlayerCommand::SkipForward => {
                let skip_seconds = self.controls.skip_seconds();
                self.player.skip(skip_seconds as i64)?;
            }
            PlayerCommand::SkipBackward => {
                let skip_seconds = self.controls.skip_seconds();
                self.player.skip(-skip_seconds as i64)?;
            }
            PlayerCommand::VolumeUp(step) => self.player.adjust_volume(step)?,
            PlayerCommand::VolumeDown(step) => self.player.adjust_volume(-step)?,
            PlayerCommand::Quit => return Ok(()),
            PlayerCommand::Ignore => (),
        }
        Ok(())
    }
}
