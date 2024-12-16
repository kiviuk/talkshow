use anyhow::Result;
use crate::audio_player::PlayerCommand;
use crate::keyboard_controls::{KeyboardControls, CooldownHandler};
use crate::episodes::Episode;
use crate::audio_player::AudioPlayerTrait;

pub struct AudioControl {
    keyboard_controls: KeyboardControls,
}

impl AudioControl {

    pub fn new() -> Self {
        Self {
            keyboard_controls: KeyboardControls::new(),
        }
    }

    pub fn play_episode<T: AudioPlayerTrait>(&self, player: &mut T, episode: &Episode,) -> Result<()> {
        player.play(episode)?;
        self.keyboard_controls.print_help();
        self.run(player)  
    }

    pub fn run<T: AudioPlayerTrait>(&self, player: &mut T) -> Result<()> {
        let mut cooldown_handler: CooldownHandler = CooldownHandler::new();
        loop {
            let command: PlayerCommand = KeyboardControls::get_user_input(&mut cooldown_handler);
            match command {
                PlayerCommand::Quit => break,
                PlayerCommand::Ignore => continue,
                _ => self.process_command(player, command)?,
            }
        }
        Ok(())
    }

    pub fn process_command<T: AudioPlayerTrait>(
        &self,
        player: &mut T,
        command: PlayerCommand,
    ) -> Result<()> {
        match command {
            PlayerCommand::Pause => player.pause()?,
            PlayerCommand::Play => player.resume()?,
            PlayerCommand::Stop => player.stop()?,
            PlayerCommand::SkipForward(seconds) => player.skip(seconds)?,
            PlayerCommand::SkipBackward(seconds) => player.skip(-seconds)?,
            PlayerCommand::VolumeUp(step) => player.adjust_volume(step)?,
            PlayerCommand::VolumeDown(step) => player.adjust_volume(-step)?,
            _ => (),
        }
        Ok(())
    }
}
