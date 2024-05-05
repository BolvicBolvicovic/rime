use ratatui::{layout::{Constraint, Direction, Layout}, prelude::Span, style::{Color, Style, Styled}, text::{Line, Text}, widgets::{Block, Borders, Paragraph}, Frame};

use crate::{app::App, CurrentScreenMode, CurrentEditing};

pub fn ui(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(3)
        ])
        .split(frame.size());

    let title_block = Block::default()
        .borders(Borders::BOTTOM)
        .style(Style::default());
    let mut title_str = vec![Span::styled("Rime | ", Style::default().fg(Color::Rgb(183, 65, 14)))]; 
    for file in &app.files {
        if let CurrentScreenMode::File(index) = app.current_screen {
            if app.files[index].name == file.name {
                title_str.push(Span::styled(&file.name, Style::default().fg(Color::LightCyan)));
            } else {
                title_str.push(Span::styled(&file.name, Style::default().fg(Color::Rgb(183, 65, 14))));
            }
            title_str.push(Span::styled(" | ", Style::default().fg(Color::Rgb(183, 65, 14))));
        }
    }
    let title = Paragraph::new(Line::from(title_str))
        .block(title_block);
    frame.render_widget(title, chunks[0]);
    
    let mode_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let mode_str = match app.current_screen {
        CurrentScreenMode::File(_) => "File : ".to_owned() + &(match app.current_editing {
            CurrentEditing::Page => "Page".to_owned(),
            CurrentEditing::Command(c) => format!("Command : {}", c),
            CurrentEditing::Selecting => "Selecting".to_owned(),
            CurrentEditing::Listening(c) => format!("Listening : {}", c),
        }),
        CurrentScreenMode::Main => "Main".to_owned(),
        CurrentScreenMode::Config => "Config".to_owned(),
    };

    let mode = Paragraph::new(Text::styled(
        mode_str,
        Style::default().fg(Color::Rgb(183, 65, 14)),
    ))
    .block(mode_block);
    frame.render_widget(mode, chunks[2]);

    match app.current_screen {
       CurrentScreenMode::File(index) => {
            let mut list_items = Vec::<Line>::new();
            let mut num_items = Vec::<Line>::new();
            if let Some((current_text, cursor_index)) = app.files[index].undo_tree.show_current_node() {
                let lines = current_text.lines();
                let mut cursor_line_index = 0;
                let mut found = false;
                for (num, line) in lines.enumerate() {
                    let line_len = line.len();
                    cursor_line_index += line_len;
                    if cursor_line_index > cursor_index && !found {
                        num_items.push(Line::from(Span::styled(num.to_string(), Style::default().fg(Color::LightCyan))));
                        list_items.push(line_to_span(line, Some(line_len - cursor_line_index + cursor_index)));
                        found = true;
                    } else {
                        list_items.push(line_to_span(line, None));
                        num_items.push(Line::from(Span::styled(num.to_string(), Style::default().fg(Color::Rgb(183, 65, 14)))));
                    }
                }
                let final_text = Text::from(list_items);
                let final_lines = Text::from(num_items);
                //TODO: Render line numbers
                frame.render_widget(final_text, chunks[1]);
            } else {
            };
        },
        _ => (),
    };

}

fn line_to_span<'a>(
    line: &'a str, 
    cursor_index: Option<usize>, 
) -> Line {
    let mut formated_line = Vec::new();
    let mut found = false;
    for (i, c) in line.chars().enumerate() {
        if let Some(index) = cursor_index {
            if index == i && !c.is_ascii_whitespace() {
                formated_line.push(Span::styled(c.to_string(), Style::default().fg(Color::Rgb(183, 65, 14)).bg(Color::LightCyan)));
                found = true;
            } else if index == i {
                formated_line.push(Span::styled(c.to_string(), Style::default().bg(Color::Rgb(183, 65, 14))));
                found = true;
            } else {
                formated_line.push(Span::styled(c.to_string(), Style::default().fg(Color::LightCyan)));
            }
        } else {
            formated_line.push(Span::styled(c.to_string(), Style::default().fg(Color::LightCyan)));
        }
    }
    if !found && cursor_index.is_some() {
        formated_line.push(Span::styled(" ", Style::default().bg(Color::Rgb(183, 65, 14))));
    }
    Line::from(formated_line)
}
