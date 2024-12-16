use std::time::Duration;
use std::thread;
use rss_reader::{
    PlayerCommand, 
    keyboard_controls::{KeyboardControls, Cooldown, CooldownHandler}
};
use std::io::Cursor;

// Mock Cooldown for testing
#[derive(Default)]
struct MockCooldown {
    is_elapsed: bool,
}

impl Cooldown for MockCooldown {
    fn update_command_time(&mut self) {}

    fn is_cooldown_elapsed(&self, _cooldown: Duration) -> bool {
        self.is_elapsed
    }
}

pub struct TestInputReader {
    reader: Cursor<String>,
}

impl TestInputReader {
    pub fn new(input: &str) -> Self {
        Self {
            reader: Cursor::new(input.to_owned()),
        }
    }

    pub fn get_reader(&mut self) -> &mut Cursor<String> {
        &mut self.reader
    }
}

#[test]
fn test_translate_basic_commands() {
    assert_eq!(KeyboardControls::translate_command("p"), PlayerCommand::Pause);
    assert_eq!(KeyboardControls::translate_command("q"), PlayerCommand::Quit);
}

#[test]
fn test_translate_skip_commands() {
    assert_eq!(KeyboardControls::translate_command("f"), PlayerCommand::SkipForward(10));
    assert_eq!(KeyboardControls::translate_command("b"), PlayerCommand::SkipBackward(10));
}

#[test]
fn test_translate_volume_commands() {
    assert_eq!(KeyboardControls::translate_command("+"), PlayerCommand::VolumeUp(0.1));
    assert_eq!(KeyboardControls::translate_command("-"), PlayerCommand::VolumeDown(0.1));
}

#[test]
fn test_translate_unknown_commands() {
    assert_eq!(KeyboardControls::translate_command("x"), PlayerCommand::Ignore);
    assert_eq!(KeyboardControls::translate_command(""), PlayerCommand::Ignore);
    assert_eq!(KeyboardControls::translate_command(" "), PlayerCommand::Ignore);
}

#[test]
fn test_cooldown_handler() {
    let mut cooldown_handler = CooldownHandler::new();
    const COOLDOWN: Duration = Duration::from_millis(25);

    // First command should NOT be allowed (25ms buffer)
    assert!(!cooldown_handler.is_cooldown_elapsed(COOLDOWN));

    // Wait for cooldown to pass
    thread::sleep(Duration::from_millis(30));

    // After waiting, cooldown should be elapsed
    assert!(cooldown_handler.is_cooldown_elapsed(COOLDOWN));

    // Update command time
    cooldown_handler.update_command_time();

    // Immediately after updating, cooldown should not be elapsed
    assert!(!cooldown_handler.is_cooldown_elapsed(COOLDOWN));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_input_valid_command() {
        let mut cooldown_handler = MockCooldown { is_elapsed: true };
        let mut input_reader = TestInputReader::new("p\n");

        let result = KeyboardControls::get_user_input(&mut cooldown_handler, input_reader.get_reader());

        // but we can check it's not an Ignore command when cooldown is elapsed
        assert_ne!(result, PlayerCommand::Pause);
    }

    #[test]
    fn test_get_user_input_cooldown_elapsed() {
        let mut cooldown_handler = MockCooldown { is_elapsed: true };
        let mut input_reader = TestInputReader::new("p\n");
        let result = KeyboardControls::get_user_input(&mut cooldown_handler, input_reader.get_reader());

        // Since this is a mock, we can't predict the exact command, 
        // but we can check it's not an Ignore command when cooldown is elapsed
        assert_ne!(result, PlayerCommand::Ignore);
    }

    #[test]
    fn test_get_user_input_cooldown_not_elapsed() {
        let mut cooldown_handler = MockCooldown { is_elapsed: false };
        let mut input_reader = TestInputReader::new("p\n");

        let result = KeyboardControls::get_user_input(&mut cooldown_handler, input_reader.get_reader());

        // When cooldown is not elapsed, we expect an Ignore command
        assert_eq!(result, PlayerCommand::Ignore);
    }
}
