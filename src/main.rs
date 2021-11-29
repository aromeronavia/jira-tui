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
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, List, ListItem, ListState, Row, Table, TableState},
    Frame, Terminal,
};

struct MainScreen;

impl MainScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw<B>(self, f: &mut Frame<B>, app: &mut App)
    where
        B: Backend,
    {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
            .split(f.size());
        let filter_list = List::new(
            app.filters.iter().map(|filter| ListItem::new(*filter)).collect::<Vec<ListItem>>()
        )
            .block(Block::default().title("Filters").borders(Borders::ALL));
        f.render_widget(filter_list, chunks[0]);

        let rows = app.rows.iter().map(|item| {
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.iter().map(|c| Cell::from(*c));
            Row::new(cells).height(height as u16)
        });

        let table = Table::new(rows)
            .style(Style::default().fg(Color::White))
            .header(
                Row::new(vec!["ID", "Title", "Assignee", "Status"])
                    .style(Style::default().fg(Color::LightBlue))
                    .bottom_margin(1),
            )
            .block(Block::default().title("Tickets").borders(Borders::ALL))
            .widths(&[
                Constraint::Percentage(10),
                Constraint::Percentage(30),
                Constraint::Percentage(15),
                Constraint::Percentage(10),
            ])
            .column_spacing(5)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>");

        f.render_stateful_widget(table, chunks[1], &mut app.table_state);
    }
}

struct App<'a> {
    filters_state: ListState,
    filters: Vec<&'a str>,
    table_state: TableState,
    rows: Vec<Vec<&'a str>>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        App {
            filters_state: ListState::default(),
            filters: vec!["Team Elric", "Assigned to Me", "Frontline"],
            table_state,
            rows: vec![
                vec![
                    "JT-42",
                    "Create new main layout",
                    "Alberto Romero",
                    "Backlog",
                ],
                vec![
                    "JT-69",
                    "Integration tests",
                    "Alberto Romero",
                    "In Progress",
                ],
                vec!["JT-42", "Mock Database", "Alberto Romero", "Backlog"],
                vec![
                    "JT-124",
                    "Migrate to React Navigator v6",
                    "Huichops",
                    "Done",
                ],
            ],
        }
    }
    pub fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.rows.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.rows.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
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

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let main_screen = MainScreen::new();
            main_screen.draw(f, &mut app);
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
                    KeyCode::Char('j') => app.next(),
                    KeyCode::Char('k') => app.previous(),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
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
