use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::io::stdout;
use std::time::{Duration, Instant};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};

struct MainScreen;

impl MainScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw<B>(self, f: &mut Frame<B>)
    where
        B: Backend,
    {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
            .split(f.size());
        let block = Block::default().title("Filters").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default().title("Issues").borders(Borders::ALL);

        f.render_widget(block, chunks[1]);
    }
}

fn main() -> Result<(), io::Error> {
    start_ui()
}

fn start_ui() -> Result<(), io::Error> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let main_screen = MainScreen::new();
            main_screen.draw(f);
        })?;

        let tick_rate = Duration::from_millis(50);
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    terminal.show_cursor()?;
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}
