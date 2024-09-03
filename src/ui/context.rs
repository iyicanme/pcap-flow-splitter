use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::widgets::TableState;

use crate::error::Error;
use crate::ui::{DirectoryContent, State};

pub struct Context {
    pub state: State,
    pub table_state: TableState,
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        let current_directory = std::env::current_dir().map_err(Error::ReadEnv)?;
        let current_directory_content = DirectoryContent::read(&current_directory)?;

        Ok(Self { state: State::Browse { current_directory, current_directory_content, index: 0 }, table_state: TableState::default() })
    }

    pub fn update(&self) {}

    pub fn should_exit(&self) -> bool {
        match self.state {
            State::Exit => true,
            _ => false,
        }
    }

    pub fn handle_input(&mut self) -> Result<(), Error> {
        if !event::poll(std::time::Duration::from_millis(5)).map_err(Error::TuiReadInput)? {
            return Ok(());
        }

        let Event::Key(key) = event::read().map_err(Error::TuiReadInput)? else {
            return Ok(());
        };

        if key.kind != event::KeyEventKind::Press {
            return Ok(());
        }

        match key.code {
            KeyCode::Up => {
                self.cursor_up();
            }
            KeyCode::Down => {
                self.cursor_down();
            }
            KeyCode::Enter => {
                self.enter();
            }
            KeyCode::Esc => {
                self.exit();
            }
            _ => return Ok(()),
        };

        Ok(())
    }

    fn cursor_up(&mut self) {}

    fn cursor_down(&mut self) {}

    fn enter(&mut self) {}

    fn exit(&mut self) {}
}
