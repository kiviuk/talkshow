use log::{debug, info};
use std::collections::HashMap;
use std::io::BufRead;
use std::time::{Duration, Instant};
use crate::audio_player::PlayerCommand;

const COOLDOWN: Duration = Duration::from_millis(250);
pub const VOLUME_STEP: f32 = 0.1;
pub const SKIP_SECONDS: i64 = 10;

lazy_static::lazy_static! {
    static ref COMMAND_MAP: HashMap<&'static str, PlayerCommand> = {
        let mut map = HashMap::new();
        map.insert("p", PlayerCommand::Pause);
        map.insert("q", PlayerCommand::Quit);
        map.insert("+", PlayerCommand::VolumeUp(VOLUME_STEP));
        map.insert("-", PlayerCommand::VolumeDown(VOLUME_STEP));
        map.insert("f", PlayerCommand::SkipForward(SKIP_SECONDS));
        map.insert("b", PlayerCommand::SkipBackward(SKIP_SECONDS));
        map
    };
}

pub trait Cooldown {
    fn update_command_time(&mut self);
    fn is_cooldown_active(&self, cooldown: Duration) -> bool;
}

#[derive(Clone)]
pub struct CooldownHandler {
    last_command_time: Instant,
}

impl Cooldown for CooldownHandler {
    fn update_command_time(&mut self) {
        self.last_command_time = Instant::now();
    }

    fn is_cooldown_active(&self, cooldown: Duration) -> bool {
        Instant::now() < self.last_command_time + cooldown
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
pub struct KeyboardControls;

impl KeyboardControls {
    pub fn new() -> Self { Self { } }

    pub fn get_user_input<T: Cooldown, R: BufRead>(
        cooldown_handler: &mut T,
        reader: &mut R,
    ) -> PlayerCommand {
        // Read user input safely
        let input = match Self::read_input(reader) {
            Ok(valid_input) => valid_input,
            Err(_) => return PlayerCommand::Ignore,
        };

        // Ignore empty input
        if input.trim().is_empty() {
            return PlayerCommand::Ignore;
        }

        // Translate input into a command
        let command = Self::translate_command(&input);

        debug!("Cooldown active: {}", cooldown_handler.is_cooldown_active(COOLDOWN));

        // Check cooldown
        if cooldown_handler.is_cooldown_active(COOLDOWN) {
            return PlayerCommand::Ignore;
        }

        // Update last command time for valid commands
        if !matches!(command, PlayerCommand::Ignore) {
            cooldown_handler.update_command_time();
        }

        command
    }


    /// Reads user input from the provided reader
    fn read_input<R: BufRead>(reader: &mut R) -> Result<String, std::io::Error> {
        let mut input = String::new();
        reader.read_line(&mut input)?;
        Ok(input)
    }

    /// Translates input string into the corresponding `PlayerCommand`
    pub fn translate_command(input: &str) -> PlayerCommand {
        COMMAND_MAP.get(input.trim()).cloned().unwrap_or(PlayerCommand::Ignore)
    }

    pub fn print_help() {
        for (shortcut, _command) in COMMAND_MAP.iter() {
            println!("{}", shortcut);
        }
    }

}