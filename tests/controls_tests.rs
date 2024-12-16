use std::time::Duration;
use std::thread;
use rss_reader::{
    PlayerCommand, 
    keyboard_controls::{KeyboardControls, Cooldown, CooldownHandler}
};

// Mock Cooldown for testing
struct MockCooldown {
    is_elapsed: bool,
}

impl Cooldown for MockCooldown {
    fn update_command_time(&mut self) {}

    fn is_cooldown_elapsed(&self, _cooldown: Duration) -> bool {
        self.is_elapsed
    }
}

#[test]
fn test_translate_basic_commands() {
    let controls = KeyboardControls::new();
    
    assert_eq!(controls.translate_command("p"), PlayerCommand::Pause);
    assert_eq!(controls.translate_command("q"), PlayerCommand::Quit);
}

#[test]
fn test_translate_skip_commands() {
    let controls = KeyboardControls::new();
    
    assert_eq!(controls.translate_command("f"), PlayerCommand::SkipForward);
    assert_eq!(controls.translate_command("b"), PlayerCommand::SkipBackward);
}

#[test]
fn test_translate_volume_commands() {
    let controls = KeyboardControls::new();
    
    assert_eq!(controls.translate_command("+"), PlayerCommand::VolumeUp(0.1));
    assert_eq!(controls.translate_command("-"), PlayerCommand::VolumeDown(0.1));
}

#[test]
fn test_translate_unknown_commands() {
    let controls = KeyboardControls::new();
    
    assert_eq!(controls.translate_command("x"), PlayerCommand::Ignore);
    assert_eq!(controls.translate_command(""), PlayerCommand::Ignore);
    assert_eq!(controls.translate_command(" "), PlayerCommand::Ignore);
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

#[test]
fn test_get_user_input_valid_command() {
    let controls = KeyboardControls::new();
    let mut cooldown_handler = MockCooldown { is_elapsed: true };

    let result = controls.get_user_input(&mut cooldown_handler);

    assert!(result.is_ok());
}

#[test]
fn test_get_user_input_cooldown_elapsed() {
    let controls = KeyboardControls::new();
    let mut cooldown_handler = MockCooldown { is_elapsed: true };

    let result = controls.get_user_input(&mut cooldown_handler);
    assert!(result.is_ok());
}

#[test]
fn test_get_user_input_cooldown_not_elapsed() {
    let controls = KeyboardControls::new();
    let mut cooldown_handler = MockCooldown { is_elapsed: false };

    let result = controls.get_user_input(&mut cooldown_handler);
    assert!(result.is_ok());
}
