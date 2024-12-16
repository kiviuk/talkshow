use std::io::{self, BufRead};
use std::time::{Duration, Instant};
use crate::audio_player::PlayerCommand;

const COOLDOWN: Duration = Duration::from_millis(250);
pub const VOLUME_STEP: f32 = 0.1;
pub const SKIP_SECONDS: i64 = 10;

pub trait Cooldown {
    fn update_command_time(&mut self);
    fn is_cooldown_elapsed(&self, cooldown: Duration) -> bool;
}

pub struct CooldownHandler {
    last_command_time: Instant,
}

impl Cooldown for CooldownHandler {
    fn update_command_time(&mut self) {
        self.last_command_time = Instant::now();
    }

    fn is_cooldown_elapsed(&self, cooldown: Duration) -> bool {
        Instant::now().duration_since(self.last_command_time) >= cooldown
    }
}

impl CooldownHandler {
    pub fn new() -> Self {
        Self {
            last_command_time: Instant::now(),
        }
    }
}

#[derive(Clone)]
pub struct KeyboardControls {
    skip_seconds: i64,
}

impl KeyboardControls {
    pub fn new() -> Self {
        Self {
            skip_seconds: 10,
        }
    }

    pub fn skip_seconds(&self) -> i64 {
        self.skip_seconds
    }

    pub fn get_user_input<T: Cooldown>(
        cooldown_handler: &mut T
    ) -> PlayerCommand {
        let mut input = String::new();
        
        // Read user input, return Ignore if it fails
        if io::stdin().lock().read_line(&mut input).is_err() {
            return PlayerCommand::Ignore;
        }

        // Ignore empty input
        if input.trim().is_empty() {
            return PlayerCommand::Ignore;
        }

        // Translate input into a command
        let command = Self::translate_command(&input);

        // Handle valid commands with cooldown logic
        if matches!(command, PlayerCommand::Ignore) || !cooldown_handler.is_cooldown_elapsed(COOLDOWN) {
            return PlayerCommand::Ignore;
        }

        // Update last command time for valid commands
        cooldown_handler.update_command_time();
        command
    }


    pub fn translate_command(input: &str) -> PlayerCommand {
        match input.trim() {
            "p" => PlayerCommand::Pause,
            "q" => PlayerCommand::Quit,
            "+" => PlayerCommand::VolumeUp(VOLUME_STEP),
            "-" => PlayerCommand::VolumeDown(VOLUME_STEP),
            "f" => PlayerCommand::SkipForward(SKIP_SECONDS),
            "b" => PlayerCommand::SkipBackward(SKIP_SECONDS),
            _ => PlayerCommand::Ignore,
        }
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