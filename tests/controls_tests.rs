use rss_reader::keyboard_controls::{Controls, CooldownHandler};
use rss_reader::PlayerCommand;
use std::time::Duration;
use std::thread;

#[test]
fn test_translate_basic_commands() {
    let controls = Controls::new();
    
    assert_eq!(controls.translate_command("p"), PlayerCommand::Pause);
    assert_eq!(controls.translate_command("q"), PlayerCommand::Quit);
}

#[test]
fn test_translate_skip_commands() {
    let controls = Controls::new();
    
    assert_eq!(controls.translate_command("f"), PlayerCommand::SkipForward);
    assert_eq!(controls.translate_command("b"), PlayerCommand::SkipBackward);
}

#[test]
fn test_translate_volume_commands() {
    let controls = Controls::new();
    
    assert_eq!(controls.translate_command("+"), PlayerCommand::VolumeUp(0.1));
    assert_eq!(controls.translate_command("-"), PlayerCommand::VolumeDown(0.1));
}

#[test]
fn test_translate_unknown_commands() {
    let controls = Controls::new();
    
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
