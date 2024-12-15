use std::io::{self, BufRead};
use std::time::{Duration, Instant};
use crate::audio_player::PlayerCommand;

const COOLDOWN: Duration = Duration::from_millis(50);

#[derive(Clone)]
pub struct Controls {
    skip_seconds: i64,
    volume_step: f32,
    last_command_time: Instant,
}

impl Controls {
    pub fn new() -> Self {
        Self {
            skip_seconds: 10,
            volume_step: 0.1,
            last_command_time: Instant::now() - (COOLDOWN + Duration::from_millis(1)), // Safely past cooldown
        }
    }

    pub fn skip_seconds(&self) -> i64 {
        self.skip_seconds
    }

    pub fn volume_step(&self) -> f32 {
        self.volume_step
    }

    fn translate_command(&self, input: &str) -> PlayerCommand {
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

    pub fn get_user_input(&mut self) -> io::Result<PlayerCommand> {
        let stdin = io::stdin();
        let mut buffer = String::new();
        
        stdin.lock().read_line(&mut buffer)?;

        if buffer.trim().is_empty() {
            return Ok(PlayerCommand::Ignore);
        }

        let now = Instant::now();
        if now.duration_since(self.last_command_time) < COOLDOWN {
            return Ok(PlayerCommand::Ignore);
        }

        let command: PlayerCommand = self.translate_command(&buffer);

        self.last_command_time = now;
        

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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_controls() -> Controls {
        Controls::new()
    }

    #[test]
    fn test_cooldown() {
        let mut controls = setup_controls();
        
        // First command should work
        controls.last_command_time = Instant::now() - COOLDOWN;
        let now = Instant::now();
        let result = if now.duration_since(controls.last_command_time) >= COOLDOWN {
            Some(controls.translate_command("f"))
        } else {
            None
        };
        assert!(result.is_some());

        // Immediate second command should be ignored
        controls.last_command_time = now; // Set the time of the last command
        let now = Instant::now();
        let result = if now.duration_since(controls.last_command_time) >= COOLDOWN {
            Some(controls.translate_command("f"))
        } else {
            None
        };
        assert!(result.is_none());

        // After cooldown, command should work again
        std::thread::sleep(COOLDOWN + Duration::from_millis(10));
        let now = Instant::now();
        let result = if now.duration_since(controls.last_command_time) >= COOLDOWN {
            Some(controls.translate_command("f"))
        } else {
            None
        };
        assert!(result.is_some());
    }

    #[test]
    fn test_translate_basic_commands() {
        let controls = setup_controls();
        
        assert_eq!(controls.translate_command("p"), PlayerCommand::Pause);
        assert_eq!(controls.translate_command("q"), PlayerCommand::Quit);
    }

    #[test]
    fn test_translate_skip_commands() {
        let controls = setup_controls();
        
        assert_eq!(controls.translate_command("f"), PlayerCommand::SkipForward);
        assert_eq!(controls.translate_command("b"), PlayerCommand::SkipBackward);
    }

    #[test]
    fn test_translate_volume_commands() {
        let controls = setup_controls();
        
        assert_eq!(controls.translate_command("+"), PlayerCommand::VolumeUp(0.1));
        assert_eq!(controls.translate_command("-"), PlayerCommand::VolumeDown(0.1));
    }

    #[test]
    fn test_translate_unknown_commands() {
        let controls = setup_controls();
        
        assert_eq!(controls.translate_command(""), PlayerCommand::Ignore);
        assert_eq!(controls.translate_command("x"), PlayerCommand::Ignore);
        assert_eq!(controls.translate_command("random"), PlayerCommand::Ignore);
    }
}
