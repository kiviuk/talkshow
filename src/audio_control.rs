use anyhow::Result;
use crate::audio_player::{AudioPlayerTrait, PlayerCommand};
use crate::keyboard_controls::{KeyboardControls, CooldownHandler, Cooldown};
use crate::episodes::Episode;
use std::io;

pub fn play_episode<T: AudioPlayerTrait>(
    player: &mut T,
    episode: &Episode,
) -> Result<()> {
    player.play(episode)?;
    KeyboardControls::print_help();
    
    let stdin = io::stdin();
    let mut stdin_locked = stdin.lock();
    let get_stdin_command = |cooldown_handler: &mut CooldownHandler| {
        let mut handler = cooldown_handler.clone();
        get_next_command(&mut handler, &mut stdin_locked)
    };
    
    run(player, get_stdin_command)
}

pub fn run<T: AudioPlayerTrait>(
    player: &mut T,
    mut get_command: impl FnMut(&mut CooldownHandler) -> PlayerCommand
) -> Result<()> {
    let mut cooldown_handler: CooldownHandler = CooldownHandler::new();

    loop {
        let command = get_command(&mut cooldown_handler);
        match command {
            PlayerCommand::Quit => break,
            PlayerCommand::Ignore => continue,
            _ => process_command(player, command)?,
        }
    }
    Ok(())
}

pub fn get_next_command<T: Cooldown, R: io::BufRead>(
    cooldown_handler: &mut T, 
    input: &mut R
) -> PlayerCommand {
    KeyboardControls::get_user_input(cooldown_handler, input)
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