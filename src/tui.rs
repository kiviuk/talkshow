use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Style, Color},
    text::Text,
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode},
};
use std::io::{stdout, Stdout};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    left_items: Vec<String>,
    right_items: Vec<String>,
}

impl Tui {
    pub fn new(left_items: Vec<String>, right_items: Vec<String>) -> Result<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;

        Ok(Self { 
            terminal, 
            left_items, 
            right_items 
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.draw()?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        self.cleanup()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(frame.area());

            // Convert left items to ListItems with colored text
            let left_list_items: Vec<ListItem> = self.left_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i % 2 == 0 { 
                        Style::default().fg(Color::Green) 
                    } else { 
                        Style::default().fg(Color::LightGreen) 
                    };
                    ListItem::new(Text::styled(item.clone(), style))
                })
                .collect();

            // Convert right items to ListItems with colored text
            let right_list_items: Vec<ListItem> = self.right_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i % 2 == 0 { 
                        Style::default().fg(Color::Blue) 
                    } else { 
                        Style::default().fg(Color::LightBlue) 
                    };
                    ListItem::new(Text::styled(item.clone(), style))
                })
                .collect();

            // Create left pane list
            let left_pane = List::new(left_list_items)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Podcasts"));
            
            // Create right pane list
            let right_pane = List::new(right_list_items)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Episodes"));

            // Render panes
            frame.render_widget(left_pane, layout[0]);
            frame.render_widget(right_pane, layout[1]);
        })?;

        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
