use std::fs::FileType as OsFileType;
use std::path::{Path, PathBuf};

use crossterm::ExecutableCommand;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::*;
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders};

use crate::error::Error;
use crate::ui::context::Context;

mod color;
mod color_set;
mod context;
mod style;
mod table;

pub fn run() -> Result<(), Error> {
    let mut context = Context::new()?;

    enable_raw_mode().map_err(Error::TuiSetup)?;
    std::io::stdout()
        .execute(EnterAlternateScreen)
        .map_err(Error::TuiSetup)?;

    let mut terminal =
        Terminal::new(CrosstermBackend::new(std::io::stdout())).map_err(Error::TuiSetup)?;

    while !context.should_exit() {
        context.update();
        terminal
            .draw(|frame| {
                draw_ui(frame, &mut context);
            })
            .map_err(Error::TuiDraw)?;
        context.handle_input()?;
    }

    disable_raw_mode().map_err(Error::TuiTeardown)?;
    std::io::stdout()
        .execute(LeaveAlternateScreen)
        .map_err(Error::TuiTeardown)?;

    Ok(())
}

fn draw_ui(frame: &mut Frame, context: &mut Context) {
    let constraints = [
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ];
    let main_layout = Layout::new(Direction::Vertical, constraints).split(frame.area());

    draw_header(frame, context, main_layout[0]);
    draw_body(frame, context, main_layout[1]);
    draw_footer(frame, &context.state, main_layout[2]);
}

fn draw_header(frame: &mut Frame, context: &Context, area: Rect) {
    let title = match &context.state {
        State::Browse { current_directory, current_directory_content, index } => current_directory.to_string_lossy().to_string(),
        State::View { current_directory, current_file, index } => current_file,
        State::Exit => String::new(),
    };

    let width = area.width as usize - 2usize;
    let title_start = if title.len() < width {
        0usize
    } else {
        title.len() - width
    };

    let header = Block::new()
        .title(format!(" {} ", &title[title_start..]))
        .style(style::HEADER)
        .borders(Borders::TOP);

    frame.render_widget(header, area);
}

fn draw_body(frame: &mut Frame, context: &mut Context, area: Rect) {
    match &mut context.state {
        State::Browse { current_directory, current_directory_content, index } =>
            table::draw(frame, area, ["", "", ""].into_iter(), current_directory_content.iter(), *index, &mut context.table_state),
        State::View { current_directory, current_file, index } => {}
        State::Exit => {}
    }
}

fn draw_footer(frame: &mut Frame, state: &State, area: Rect) {
    let instructions = "[↑] UP [↓] DOWN [ESC] EXIT [↵] ENTER";

    let header = Block::new()
        .title(format!(" {} ", instructions))
        .style(style::HEADER)
        .borders(Borders::TOP);

    frame.render_widget(header, area);
}

enum State {
    Browse { current_directory: PathBuf, current_directory_content: DirectoryContent, index: usize },
    View { current_directory: PathBuf, current_file: String, index: usize },
    Exit,
}

struct DirectoryContent {
    content: Vec<(FileType, String)>,
}

impl DirectoryContent {
    fn read(directory: impl AsRef<Path>) -> Result<Self, Error> {
        let content = std::fs::read_dir(directory)
            .map_err(Error::ReadDirContent)?
            .filter_map(Result::ok)
            .filter_map(|e| e.file_type().map(|t| (t, e.file_name())).ok())
            .filter_map(|(os_file_type, file_name)| FileType::try_from(os_file_type).map(|file_type| (file_type, file_name)).ok())
            .map(|(file_type, file_name)| (style_from_file_type(file_type), file_name))
            .collect();

        Ok(DirectoryContent { content })
    }
}

fn style_from_file_type(file_type: FileType) -> Style {
    match file_type {
        FileType::File => style::file(),
        FileType::Directory => style::directory(),
        FileType::SymbolicLink => style::symbolic_link(),
    }
}

enum FileType {
    File,
    Directory,
    SymbolicLink,
}

impl TryFrom<OsFileType> for FileType {
    type Error = Error;

    fn try_from(value: OsFileType) -> Result<Self, Self::Error> {
        let file_type = match (value.is_file(), value.is_dir(), value.is_symlink()) {
            (true, false, false) => Self::File,
            (false, true, false) => Self::Directory,
            (false, false, true) => Self::SymbolicLink,
            _ => return Err(Error::FileTypeConversion)
        };

        Ok(file_type)
    }
}