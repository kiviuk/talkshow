use rss_reader::audio_control::process_command;
use rss_reader::audio_player::{AudioPlayerTrait, PlayerCommand};
use rss_reader::episodes::Episode;
use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;

// Mock AudioPlayer for testing
struct MockAudioPlayer {
    actions: Rc<RefCell<Vec<String>>>,
}

impl AudioPlayerTrait for MockAudioPlayer {
    fn play(&mut self, episode: &Episode) -> Result<()> {
        self.actions.borrow_mut().push(format!("play: {}", episode.title));
        Ok(())
    }

    fn pause(&mut self) -> Result<()> {
        self.actions.borrow_mut().push("pause".to_string());
        Ok(())
    }

    fn resume(&mut self) -> Result<()> {
        self.actions.borrow_mut().push("resume".to_string());
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        self.actions.borrow_mut().push("stop".to_string());
        Ok(())
    }

    fn skip(&mut self, seconds: i64) -> Result<()> {
        self.actions.borrow_mut().push(format!("skip: {}", seconds));
        Ok(())
    }

    fn adjust_volume(&mut self, step: f32) -> Result<()> {
        self.actions.borrow_mut().push(format!("volume: {}", step));
        Ok(())
    }
}

impl MockAudioPlayer {
    fn new() -> Self {
        Self {
            actions: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn get_actions(&self) -> Vec<String> {
        self.actions.borrow().clone()
    }
}

#[test]
fn test_process_command() {
    let test_cases = vec![
        (PlayerCommand::Pause, "pause"),
        (PlayerCommand::Play, "resume"),
        (PlayerCommand::Stop, "stop"),
        (PlayerCommand::SkipForward(10), "skip: 10"),
        (PlayerCommand::SkipBackward(10), "skip: -10"),
        (PlayerCommand::VolumeUp(0.1), "volume: 0.1"),
        (PlayerCommand::VolumeDown(0.1), "volume: -0.1"),
    ];

    for (command, expected_action) in test_cases {
        let mut player = MockAudioPlayer::new();
        
        // Process the command
        let result = process_command(&mut player, command.clone());
        
        // Check the result
        assert!(result.is_ok(), "Command processing failed: {:?}", command);
        
        // Check the action
        assert_eq!(
            player.get_actions(), 
            vec![expected_action], 
            "Failed for command: {:?}", 
            command
        );
    }
}

#[test]
fn test_process_command_ignore() {
    let mut player = MockAudioPlayer::new();
    
    // Test Ignore and Quit commands do nothing
    let ignore_commands = vec![
        PlayerCommand::Ignore,
        PlayerCommand::Quit,
    ];

    for command in ignore_commands {
        let result = process_command(&mut player, command.clone());
        
        // Check the result
        assert!(result.is_ok(), "Command processing failed: {:?}", command);
        
        // Check no actions were taken
        assert!(
            player.get_actions().is_empty(), 
            "Unexpected actions for command: {:?}", 
            command
        );
    }
}
