use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Widget};

use crate::ui::flow::Flow;

pub fn draw(frame: &mut Frame, area: Rect, data: &Flow) {
    let buffer = frame.buffer_mut();

    let block = Block::bordered();
    let inner = block.inner(area);

    block.render(area, buffer);

    let constraints = [Constraint::Min(0); 6];
    let areas = Layout::new(Direction::Horizontal, constraints).split(inner);

    draw_paragraph(buffer, areas[0], &["Initiator:".to_string(), "Respondent:".to_string(), "Protocol:".to_string(), "Packet count:".to_string()]);
    draw_paragraph(buffer, areas[1], &[data.initiator.to_string(), data.respondent.to_string(), data.protocol.to_string(), data.packet_count.to_string()]);

    draw_paragraph(buffer, areas[2], &["Total size:".to_string(), "Average size:".to_string(), "Minimum size:".to_string(), "Maximum size:".to_string()]);
    draw_paragraph(buffer, areas[3], &[data.total_size.to_string(), data.average_size.to_string(), data.minimum_size.to_string(), data.maximum_size.to_string()]);

    draw_paragraph(buffer, areas[4], &["Flow duration:".to_string(), "Average inter-arrival time:".to_string(), "Minimum inter-arrival time:".to_string(), "Maximum inter-arrival time:".to_string()]);
    draw_paragraph(buffer, areas[5], &[data.flow_duration.to_string(), data.average_interarrival_time.to_string(), data.minimum_interarrival_time.to_string(), data.maximum_interarrival_time.to_string()]);
}

pub fn draw_paragraph<'a>(buffer: &mut Buffer, area: Rect, text: &[String]) {
    let lines = text.iter().map(|s| s.as_str()).map(Line::from).collect::<Vec<Line>>();
    Paragraph::new(lines).render(area, buffer);
}