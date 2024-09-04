use std::borrow::Cow;

use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;

use crate::error::Error;
use crate::ui::context::{Context, State};

mod color;
mod color_set;
mod context;
mod directory;
mod flow;
mod infobox;
mod style;
mod table;
mod tabs;

pub fn run() -> Result<(), Error> {
    let mut context = Context::new()?;

    enable_raw_mode().map_err(Error::TuiSetup)?;
    std::io::stdout()
        .execute(EnterAlternateScreen)
        .map_err(Error::TuiSetup)?;

    let mut terminal =
        Terminal::new(CrosstermBackend::new(std::io::stdout())).map_err(Error::TuiSetup)?;

    while !context.should_exit() {
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
        State::Browse {
            current_directory, ..
        } => current_directory.to_string_lossy(),
        State::View { current_file, .. } => Cow::from(current_file),
        State::Exit => Cow::from(""),
    };

    let title_start = if area.width < 2 || title.len() < (area.width as usize - 2) {
        0usize
    } else {
        title.len() - area.width as usize
    };

    let header = Block::new()
        .title(format!(" {} ", &title[title_start..]))
        .style(style::HEADER)
        .borders(Borders::TOP);

    frame.render_widget(header, area);
}

fn draw_body(frame: &mut Frame, context: &mut Context, table_area: Rect) {
    match &mut context.state {
        State::Browse {
            current_directory_content,
            index,
            ..
        } => {
            context.table_state.select(Some(*index));
            table::draw(
                frame,
                table_area,
                [Constraint::Min(1)].into_iter(),
                ["NAME"].into_iter(),
                current_directory_content.iter(),
                *index,
                &mut context.table_state,
            );
        }
        State::View {
            index,
            flow_index,
            flows,
            ..
        } => {
            context.table_state.select(Some(*index));

            let constraints = [
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(6),
            ];
            let areas = Layout::new(Direction::Vertical, constraints).split(table_area);

            tabs::draw(frame, areas[0], *flow_index, flows.keys());
            table::draw(
                frame,
                areas[1],
                [
                    Constraint::Length(4),
                    Constraint::Min(1),
                    Constraint::Min(1),
                    Constraint::Min(1),
                ]
                .into_iter(),
                ["#", "DIRECTION", "TIMESTAMP", "LENGTH"].into_iter(),
                flows.iter(*flow_index),
                *index,
                &mut context.table_state,
            );
            infobox::draw(frame, areas[2], flows.get(*flow_index));
        }
        State::Exit => {}
    }
}

fn draw_footer(frame: &mut Frame, state: &State, area: Rect) {
    let instructions = match state {
        State::Browse { .. } => " [↑] UP [↓] DOWN [ESC] EXIT [↵] OPEN [BACKSP] GO UP ",
        State::View { .. } => {
            " [↑] UP [↓] DOWN [←] PREVIOUS [→] NEXT [ESC] EXIT [BACKSP] CLOSE FILE "
        }
        State::Exit => "",
    };

    let header = Block::new()
        .title(instructions)
        .style(style::FOOTER)
        .borders(Borders::TOP);

    frame.render_widget(header, area);
}
