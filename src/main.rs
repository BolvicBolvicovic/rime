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
use std::{io::{stderr, Result}, os::fd::{AsRawFd, RawFd}};
use crate::app::*;

fn init_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stderr>>> {
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    Ok(terminal)
}

fn get_handle() -> Option<(RawFd, String)> {
    let matches = command!()
        .arg(Arg::new("file"))
        .get_matches();
    let args = matches.get_one::<String>("file");
    if let None = args {
        return None;
    }
    match std::fs::File::open(args.unwrap()) {
        Ok(file) => Some((file.as_raw_fd(), args.unwrap().to_owned())),
        Err(_) => None,
    }
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
)-> Result<bool> {
    loop {
        terminal.draw(|frame| ui(app, frame))?;
        if let Event::Key(key) = event::read()? {
            dbg!(key.code);
        }
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
                        CurrentEditing::Page => match key.code {
                            KeyCode::Esc => app.current_editing = CurrentEditing::Selecting,
                            _ => (),
                        }, //TODO:add the char to the String
                        CurrentEditing::Command => match key.code {
                            KeyCode::Esc => app.current_editing = CurrentEditing::Selecting,
                            _ => (),
                        }, //TODO:add the char to Command
                        CurrentEditing::Selecting => match key.code {
                            KeyCode::Char('i') => app.current_editing = CurrentEditing::Page,
                            KeyCode::Char(':') => app.current_editing = CurrentEditing::Command,
                            //Temporary: Need to add quit to command
                            KeyCode::Char('q') => return Ok(true),
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
                    }
                }
            }
        }
    }
}

fn main() -> Result<()>{
    let mut terminal = init_terminal()?;
    let mut app = App::new();
    {
        let (handle, name) = get_handle().unwrap_or((-1, "nil".to_string()));
        if handle != -1 {
            app.open_file(handle, name);
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
