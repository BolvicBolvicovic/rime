use ratatui::{layout::{Constraint, Direction, Layout}, prelude::Span, style::{Color, Style}, text::{Text, Line}, widgets::{Block, Borders, Paragraph}, Frame};

use crate::{app::App, CurrentScreenMode, CurrentEditing};

pub fn ui(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
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

    let title_str = &("rime ".to_owned() + ( if let CurrentScreenMode::File(index) = app.current_screen {
                                                app.files[index].name.as_str()
                                            } else {""}));
    let title = Paragraph::new(Text::styled(
        title_str,
        Style::default().fg(Color::Rgb(183, 65, 14)),
    ))
    .block(title_block)
    .centered();
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
            if let Some(current_text) = app.files[index].undo_tree.show_current_node() {
                let split_text = current_text.split('\n');
                for (i, line) in split_text.enumerate() {
                    //TODO: Add a parser that formats the line based on its component and on the
                    //file type. Maybe look for Line in ratatui.
                    list_items.push(Line::from(Span::styled(
                        format!("{} ~ {}", i, line),
                        Style::default().fg(Color::LightCyan),
                    )));
                }
                let final_text = Text::from(list_items);
                frame.render_widget(final_text, chunks[1]);
            } else {
            };
        },
        _ => (),
    };

}
