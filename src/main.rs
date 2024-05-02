mod app;
use clap::{command, Arg};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind}, execute, terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    }, ExecutableCommand
};
use ratatui::{
    backend::Backend, prelude::{CrosstermBackend, Stylize, Terminal}, widgets::Paragraph, Frame
};
use std::{io::{Write, stderr, Result}, os::fd::{AsRawFd, RawFd}};
use crate::app::*;

fn init_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stderr>>> {
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    Ok(terminal)
}

fn get_handle() -> (Result<std::fs::File>, String) {
    let matches = command!()
        .arg(Arg::new("file"))
        .get_matches();
    let args = matches.get_one::<String>("file");
    if let None = args {
        return (Err(std::io::Error::from(std::io::ErrorKind::NotFound)), "Not Found".to_owned());
    }
    (std::fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(args.unwrap()),
    args.unwrap().to_string())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
)-> Result<bool> {
    terminal.draw(|frame| ui(app, frame))?;
    loop {
        /*if let Event::Key(key) = event::read()? {
            dbg!(key.code);
        }*/
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
                    match app.current_editing {
                        CurrentEditing::Page if key.kind == KeyEventKind::Press => 
                            match key.code {
                                KeyCode::Esc => app.current_editing = CurrentEditing::Selecting,
                                KeyCode::Backspace => app.files[index].undo_tree.del_char(),
                                KeyCode::Enter => app.files[index].undo_tree.add_newspace(),
                                KeyCode::Char(c) => app.files[index].undo_tree.add_char(c),
                                _ => (),
                            },
                        CurrentEditing::Command(_char) => match key.code {
                            KeyCode::Esc => app.current_editing = CurrentEditing::Selecting,
                            KeyCode::Char('q') => {
                                match app.quit_file() {
                                    Ok(_) => return Ok(true),
                                    Err(e) => dbg!(e.to_string()),
                                };
                                return Ok(true);
                            }
                            KeyCode::Char('w') => {
                                app.save_file();
                                app.current_editing = CurrentEditing::Command('w')
                            },
                            _ => (),
                        }, //TODO:add the char to Command
                        CurrentEditing::Selecting => match key.code {
                            KeyCode::Char('i') => {
                                let text = if let Some(node) = &app.files[index].undo_tree.current {
                                    node.borrow().text.clone()
                                } else {"".to_owned()};
                                app.files[index].undo_tree.add_node(text);
                                app.current_editing = CurrentEditing::Page;
                            }
                            KeyCode::Char('R') => app.files[index].undo_tree.redo(),
                            KeyCode::Char('u') => app.files[index].undo_tree.undo(),
                            KeyCode::Char(':') => app.current_editing = CurrentEditing::Command(' '),
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
    let (file, name) = get_handle();
    if file.is_ok() {
        app.open_file(file.unwrap(), name);
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
