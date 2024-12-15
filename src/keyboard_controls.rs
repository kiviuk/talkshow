use std::io::{self, BufRead};
use std::time::{Duration, Instant};
use crate::audio_player::PlayerCommand;

const COOLDOWN: Duration = Duration::from_millis(250);

pub struct CooldownHandler {
    last_command_time: Instant,
}

impl CooldownHandler {
    pub fn new() -> Self {
        Self {
            last_command_time: Instant::now(),
        }
    }

    pub fn update_command_time(&mut self) {
        self.last_command_time = Instant::now();
    }

    pub fn is_cooldown_elapsed(&self, cooldown: Duration) -> bool {
        let now = Instant::now();
        now.duration_since(self.last_command_time) >= cooldown
    }
}

#[derive(Clone)]
pub struct KeyboardControls {
    skip_seconds: i64,
    volume_step: f32,
}

impl KeyboardControls {
    pub fn new() -> Self {
        Self {
            skip_seconds: 10,
            volume_step: 0.1,
        }
    }

    pub fn skip_seconds(&self) -> i64 {
        self.skip_seconds
    }

    pub fn volume_step(&self) -> f32 {
        self.volume_step
    }

    pub fn translate_command(&self, input: &str) -> PlayerCommand {
        match input.trim() {
            "p" => PlayerCommand::Pause,
            "q" => PlayerCommand::Quit,
            "+" => PlayerCommand::VolumeUp(self.volume_step),
            "-" => PlayerCommand::VolumeDown(self.volume_step),
            "f" => PlayerCommand::SkipForward,
            "b" => PlayerCommand::SkipBackward,
            _ => PlayerCommand::Ignore,
        }
    }

    pub fn get_user_input(&self, cooldown_handler: &mut CooldownHandler) -> io::Result<PlayerCommand> {
        let stdin = io::stdin();
        let mut input = String::new();
    
        // Read user input
        stdin.lock().read_line(&mut input)?;
    
        // Ignore empty input
        if input.trim().is_empty() {
            return Ok(PlayerCommand::Ignore);
        }
    
        // Translate input into a command
        let command = self.translate_command(&input);
    
        // Handle valid commands with cooldown logic
        if matches!(command, PlayerCommand::Ignore) || !cooldown_handler.is_cooldown_elapsed(COOLDOWN) {
            return Ok(PlayerCommand::Ignore);
        }
    
        // Update last command time for valid commands
        cooldown_handler.update_command_time();
        Ok(command)
    }

    pub fn print_help(&self) {
        println!("Available commands:");
        println!("p - Pause");
        println!("q - Quit");
        println!("+ - Volume Up");
        println!("- - Volume Down");
        println!("f - Skip Forward");
        println!("b - Skip Backward");
    }
}