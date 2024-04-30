use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph, Frame,
};
use std::io::{stdout, Result};

pub enum CurrentScreenMode {
    Main,
    EditFile,
}

pub enum CurrentEditing {
    Page,
    Command,
}

pub struct App {
    pub text: String,
    pub current_screen: CurrentScreenMode,
    pub current_editing: Option<CurrentEditing>,
}

impl App {
    pub fn new() -> App {
        App {
            text: String::new(),
            current_screen: CurrentScreenMode::Main,
            current_editing: None
        }
    }
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    Ok(terminal)
}

fn draw_main(frame: &mut Frame<'_>) {
    let area = frame.size();
    frame.render_widget(
        Paragraph::new("Hello World")
            .white()
            .on_light_magenta(),
         area
    );
}

fn main() -> Result<()>{
    let mut terminal = init_terminal()?;

    loop {
        terminal.draw(draw_main)?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        _ => (),
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
