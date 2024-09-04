use std::ops::{AddAssign, BitXorAssign, SubAssign};
use std::path::PathBuf;

use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::widgets::TableState;

use crate::error::Error;
use crate::ui::directory::{DirectoryContent, DirectoryEntryType};
use crate::ui::flow::{extract_flows, Flows};

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

    pub fn should_exit(&self) -> bool {
        matches!(self.state, State::Exit)
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
            KeyCode::Up => { self.cursor_up(); }
            KeyCode::Down => { self.cursor_down(); }
            KeyCode::Left => { self.tab_left(); }
            KeyCode::Right => { self.tab_right(); }
            KeyCode::Enter => { self.enter(); }
            KeyCode::Esc => { self.exit(); }
            KeyCode::Backspace => { self.backspace(); }
            _ => return Ok(()),
        };

        Ok(())
    }

    fn cursor_up(&mut self) {
        match &mut self.state {
            State::Browse { index: 0, .. } | State::View { index: 0, .. } => {}
            State::Browse { index, .. } | State::View { index, .. } => { index.sub_assign(1); }
            State::Exit => {}
        }
    }

    fn cursor_down(&mut self) {
        match &mut self.state {
            State::Browse { index, current_directory_content, .. } => {
                if (current_directory_content.len() - 1).gt(index) { index.add_assign(1); }
            }
            State::View { index, .. } => {
                index.add_assign(1);
            }
            State::Exit => {}
        }
    }

    fn tab_left(&mut self) {
        if let State::View { flow_index, index,.. } = &mut self.state { 
            if *flow_index > 0usize {
                flow_index.sub_assign(1);
                index.bitxor_assign(*index);
            }
        }
    }

    fn tab_right(&mut self) {
        if let State::View { flow_index, flows, index, .. } = &mut self.state { 
            if *flow_index < (flows.len() - 1) {
                flow_index.add_assign(1);
                index.bitxor_assign(*index);
            }
        }
    }
    
    fn enter(&mut self) {
        match &self.state {
            State::Browse { index, current_directory, current_directory_content } => {
                let entry = current_directory_content.get(*index).expect("we ensure index is always inside 0..current_directory_content.len(), so this should never throw");
                let new_path = current_directory.join(entry.file_name());
                match entry.entry_type() {
                    DirectoryEntryType::Directory => {
                        let Ok(content) = DirectoryContent::read(&new_path) else {
                            return;
                        };

                        self.state = State::Browse { index: 0, current_directory: new_path, current_directory_content: content }
                    }
                    DirectoryEntryType::File | DirectoryEntryType::SymbolicLink => {
                        let Ok(flows) = extract_flows(new_path) else {
                            return;
                        };

                        self.state = State::View {
                            current_directory: current_directory.to_path_buf(),
                            current_file: entry.display_name(),
                            index: 0,
                            flow_index: 0,
                            flows,
                        }
                    }
                }
            }
            State::View { .. } => {}
            State::Exit => {}
        }
    }

    fn exit(&mut self) {
        self.state = State::Exit
    }

    fn backspace(&mut self) {
        match &self.state {
            State::Browse { current_directory, .. } => {
                let new_path = current_directory.parent().unwrap_or(current_directory);
                let Ok(content) = DirectoryContent::read(new_path) else {
                    return;
                };

                self.state = State::Browse { index: 0, current_directory: new_path.to_path_buf(), current_directory_content: content }
            }
            State::View { current_directory, .. } => {
                self.state = State::Browse { index: 0, current_directory: current_directory.to_path_buf(), current_directory_content: DirectoryContent::read(current_directory).unwrap() }
            }
            State::Exit => {}
        }
    }
}

pub enum State {
    Browse { current_directory: PathBuf, current_directory_content: DirectoryContent, index: usize },
    View { current_directory: PathBuf, current_file: String, index: usize, flow_index: usize, flows: Flows },
    Exit,
}
