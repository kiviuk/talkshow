use rss_reader::keyboard_controls::Controls;
use rss_reader::PlayerCommand;

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
