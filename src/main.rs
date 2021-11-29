use std::io::stdout;
use std::io;
use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
  terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
  }
};

use tui::{
  backend::{Backend, CrosstermBackend},
  Terminal,
  widgets::{Block, Borders},
  layout::{Layout, Direction, Constraint},
  Frame
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
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(75)
            ].as_ref()
        )
        .split(f.size());
    let block = Block::default()
         .title("Filters")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default()
         .title("Issues")
         .borders(Borders::ALL);

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

  let mut i = 0;
  loop {
    terminal.draw(|f| {
      let main_screen = MainScreen::new();
      main_screen.draw(f)
    })?;
    i = i + 1;
    if i == 1000 {
      break;
    }
  }

  terminal.show_cursor()?;
  disable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

  Ok(())
}
