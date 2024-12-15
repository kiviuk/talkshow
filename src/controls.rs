use std::io::{self, BufRead};
use std::time::{Duration, Instant};

const COOLDOWN: Duration = Duration::from_millis(50);

#[derive(Debug, PartialEq)]
pub enum PlayerCommand {
    Play,
    Pause,
    Stop,
    SkipForward(u64),    // seconds
    SkipBackward(u64),   // seconds
    VolumeUp(f32),       // volume increment
    VolumeDown(f32),     // volume decrement
    Quit,
    Unknown,
}

pub struct Controls {
    skip_amount: u64,
    volume_step: f32,
    last_command_time: Instant,
}

impl Controls {
    pub fn new(skip_amount: u64, volume_step: f32) -> Self {
        Self { 
            skip_amount,
            volume_step,
            last_command_time: Instant::now() - (COOLDOWN + Duration::from_millis(1)), // Safely past cooldown
        }
    }

    fn translate_command(&self, input: &str) -> PlayerCommand {
        match input {
            "p" => PlayerCommand::Pause,
            "q" => PlayerCommand::Quit,
            "+" => PlayerCommand::VolumeUp(self.volume_step),
            "-" => PlayerCommand::VolumeDown(self.volume_step),
            "f" => PlayerCommand::SkipForward(self.skip_amount),
            "b" => PlayerCommand::SkipBackward(self.skip_amount),
            _ => PlayerCommand::Unknown,
        }
    }

    pub fn process_input(&mut self) -> io::Result<Option<PlayerCommand>> {
        let stdin = io::stdin();
        let mut buffer = String::new();
        
        // Read a line from stdin
        if stdin.lock().read_line(&mut buffer)? == 0 {
            return Ok(None);
        }

        let command = self.translate_command(buffer.trim());
        if command == PlayerCommand::Unknown {
            return Ok(None);
        }

        let now = Instant::now();
        if now.duration_since(self.last_command_time) >= COOLDOWN {
            self.last_command_time = now;
            Ok(Some(command))
        } else {
            Ok(None)
        }
    }

    pub fn print_help(&self) {
        println!("\nPlayback Controls:");
        println!("  p: Play/Pause");
        println!("  q: Stop and quit");
        println!("  +: Volume up by {:.1}", self.volume_step);
        println!("  -: Volume down by {:.1}", self.volume_step);
        println!("  f: Skip forward {} seconds", self.skip_amount);
        println!("  b: Skip backward {} seconds", self.skip_amount);
        println!("\nPress Enter after each command");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    fn setup_controls() -> Controls {
        Controls::new(10, 0.1)
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
        thread::sleep(COOLDOWN + Duration::from_millis(10));
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
        
        assert_eq!(controls.translate_command("f"), PlayerCommand::SkipForward(10));
        assert_eq!(controls.translate_command("b"), PlayerCommand::SkipBackward(10));
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
        
        assert_eq!(controls.translate_command(""), PlayerCommand::Unknown);
        assert_eq!(controls.translate_command("x"), PlayerCommand::Unknown);
        assert_eq!(controls.translate_command("random"), PlayerCommand::Unknown);
    }
}
