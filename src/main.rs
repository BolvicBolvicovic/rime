#![feature(str_split_whitespace_remainder)]
#![feature(let_chains)]
mod app;
use clap::{command, Arg, ArgAction};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers}, execute, terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::Backend, prelude::{CrosstermBackend, Terminal},
};
use std::io::{stderr, Result};
use crate::app::*;

fn init_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stderr>>> {
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    Ok(terminal)
}

fn get_handle() -> Vec<(Result<std::fs::File>, String)> {
    let matches = command!()
        .arg(Arg::new("file").action(ArgAction::Append))
        .get_matches();
    let args = matches
        .get_many::<String>("file")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    let mut vec_files = vec![];
    for result in args {
        vec_files.push((std::fs::File::options()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(result), result.to_owned()));
    }
    vec_files
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
)-> Result<bool> {
    terminal.draw(|frame| ui(app, frame))?;
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreenMode::Main => match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    _ => (),
                },
                CurrentScreenMode::Config => {

                },
                CurrentScreenMode::File(index) => {
                    match &app.current_editing {
                        CurrentEditing::Page if key.kind == KeyEventKind::Press => 
                            match key.code {
                                KeyCode::Char('i') if key.modifiers == KeyModifiers::ALT => app.files[index].undo_tree.move_cursor_up(),
                                KeyCode::Char('k') if key.modifiers == KeyModifiers::ALT => app.files[index].undo_tree.move_cursor_down(),
                                KeyCode::Char('j') if key.modifiers == KeyModifiers::ALT => app.files[index].undo_tree.move_cursor_left(),
                                KeyCode::Char('l') if key.modifiers == KeyModifiers::ALT => app.files[index].undo_tree.move_cursor_right(),
                                KeyCode::Esc => app.current_editing = CurrentEditing::Selecting,
                                KeyCode::Backspace => app.files[index].undo_tree.del_char(),
                                KeyCode::Enter => app.files[index].undo_tree.add_newspace(),
                                KeyCode::Char(c) => app.files[index].undo_tree.add_char(c),
                                _ => (),
                            },
                        CurrentEditing::Command(string) => match key.code {
                            KeyCode::Esc => app.current_editing = CurrentEditing::Selecting,
                            KeyCode::Enter => app.execute_command(string.clone()),
                            KeyCode::Backspace => app.current_editing = CurrentEditing::Command(string[..(if string.len() > 0 {string.len() - 1} else {0})].to_owned()),
                            KeyCode::Char(c) => app.current_editing = CurrentEditing::Command(string.to_owned() + &c.to_string()),
                            _ => (),
                        },
                        CurrentEditing::Selecting => match key.code {
                            KeyCode::Char('i') if key.modifiers == KeyModifiers::ALT => app.files[index].undo_tree.move_cursor_up(),
                            KeyCode::Char('k') if key.modifiers == KeyModifiers::ALT => app.files[index].undo_tree.move_cursor_down(),
                            KeyCode::Char('j') if key.modifiers == KeyModifiers::ALT => app.files[index].undo_tree.move_cursor_left(),
                            KeyCode::Char('l') if key.modifiers == KeyModifiers::ALT => app.files[index].undo_tree.move_cursor_right(),
                            KeyCode::Char('i') => {
                                let (text, cursor) = if let Some(node) = &app.files[index].undo_tree.current {
                                    (node.borrow().text.clone(), node.borrow().cursor.clone())
                                } else {("".to_owned(), Cursor::new(0, 1))};
                                app.files[index].undo_tree.add_node(text, cursor);
                                app.current_editing = CurrentEditing::Page;
                            }
                            KeyCode::Char('R') => app.files[index].undo_tree.redo(),
                            KeyCode::Char('u') => app.files[index].undo_tree.undo(),
                            KeyCode::Char(':') => app.current_editing = CurrentEditing::Command("".to_string()),
                            _ => if let KeyCode::Char(c) = key.code {
                                app.current_editing = CurrentEditing::Listening(c)
                            },
                        }
                        CurrentEditing::Listening(c) => match c {
                            'g' => match key.code {
                                KeyCode::Char('t') => {
                                    app.current_screen = if app.files.len() > index + 1 {
                                        CurrentScreenMode::File(index + 1)
                                    } else {
                                        CurrentScreenMode::File(0)
                                    };
                                    app.current_editing = CurrentEditing::Selecting;
                                },
                                KeyCode::Char('T') => {
                                    app.current_screen = if index as i32 - 1 > - 1 {
                                        CurrentScreenMode::File(index - 1)
                                    } else {
                                        CurrentScreenMode::File(app.files.len() - 1)
                                    };
                                    app.current_editing = CurrentEditing::Selecting;
                                },
                                _ => app.current_editing = CurrentEditing::Selecting,
                            },
                            _ => app.current_editing = CurrentEditing::Selecting,
                        }
                        _ => (),
                    }
                }
            }
        }
        terminal.draw(|frame| ui(app, frame))?;
    }
}

fn main() -> Result<()>{
    let mut terminal = init_terminal()?;
    let mut app = App::new();
    let vec_files = get_handle();
    for (file, name) in vec_files {
        if file.is_ok() {
            app.open_file(file.unwrap(), name);
        }
    }
    let _res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}
