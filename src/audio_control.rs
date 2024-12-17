use anyhow::Result;
use crate::audio_player::{AudioPlayerTrait, PlayerCommand};
use crate::keyboard_controls::{KeyboardControls, CooldownHandler};
use crate::episodes::Episode;
use std::io;

pub fn play_episode<T: AudioPlayerTrait>(
    player: &mut T,
    episode: &Episode,
) -> Result<()> {
    player.play(episode)?;
    KeyboardControls::print_help();
    run(player)
}

pub fn run<T: AudioPlayerTrait>(player: &mut T) -> Result<()> {
    let mut cooldown_handler = CooldownHandler::new();

    let stdin = io::stdin();
    let mut stdin_locked = stdin.lock();
    loop {
        let command: PlayerCommand = KeyboardControls::get_user_input(&mut cooldown_handler, &mut stdin_locked);
        match command {
            PlayerCommand::Quit => break,
            PlayerCommand::Ignore => continue,
            _ => process_command(player, command)?,
        }
    }
    Ok(())
}

pub fn process_command<T: AudioPlayerTrait>(
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