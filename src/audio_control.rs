use anyhow::Result;
use crate::audio_player::{AudioPlayer, PlayerCommand};
use crate::keyboard_controls::{KeyboardControls, CooldownHandler};
use crate::episodes::Episode;

use crate::keyboard_controls::{VOLUME_STEP, SKIP_SECONDS};

pub struct AudioControl {
    player: AudioPlayer,
}

impl AudioControl {
    pub fn new() -> Result<Self> {
        Ok(Self {
            player: AudioPlayer::new()?,
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
        println!("  +: Volume up ({:.1} step)", VOLUME_STEP);
        println!("  -: Volume down ({:.1} step)", VOLUME_STEP);
        println!("  f: Skip forward {} seconds", SKIP_SECONDS);
        println!("  b: Skip backward {} seconds", SKIP_SECONDS);
        println!("\nPress Enter after each command");
    }

    pub fn run(&mut self) -> Result<()> {
        let mut cooldown_handler = CooldownHandler::new();
        loop {
            let command: PlayerCommand = KeyboardControls::get_user_input(&mut cooldown_handler);
            match command {
                PlayerCommand::Quit => break,
                PlayerCommand::Ignore => continue,
                _ => self.process_command(command)?,
            }
        }
        Ok(())
    }

    pub fn process_command(&mut self, command: PlayerCommand) -> Result<()> {
        match command {
            PlayerCommand::Play => self.player.resume()?,
            PlayerCommand::Pause => self.player.pause()?,
            PlayerCommand::Stop => self.player.stop()?,
            PlayerCommand::SkipForward(seconds) => self.player.skip(seconds)?,
            PlayerCommand::SkipBackward(seconds) => self.player.skip(-seconds)?,
            PlayerCommand::VolumeUp(step) => self.player.adjust_volume(step)?,
            PlayerCommand::VolumeDown(step) => self.player.adjust_volume(-step)?,
            PlayerCommand::Quit => return Ok(()),
            PlayerCommand::Ignore => (),
        }
        Ok(())
    }
}
