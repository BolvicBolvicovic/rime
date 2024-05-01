use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Style}, text::Text, widgets::{Block, Borders, Paragraph}, Frame};

use crate::{app::App, CurrentScreenMode};

pub fn ui(app: &App, frame: &mut Frame) {
    let page = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3)
        ])
        .split(frame.size());
    
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title_str = &("rime".to_owned() + ( if let CurrentScreenMode::File(index) = app.current_screen {
                                                app.files[index].name.as_str()
                                            } else {""}));
    let title = Paragraph::new(Text::styled(
        title_str,
        Style::default().fg(Color::Rgb(183, 65, 14)),
    ))
    .block(title_block);
    frame.render_widget(title, page[0])
}
