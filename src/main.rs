mod algorithms;

use self::algorithms::{merge_sort::merge_sort, quick_sort::quick_sort};

use core::fmt;
use std::{
    io,
    time::{Duration, Instant},
};

use rand::{distributions::Uniform, Rng};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{BarChart, Block, Borders},
    Frame, Terminal,
};

struct App<'a> {
    data: Vec<u64>,
    keys: Vec<&'a str>,
    current: Algorithm,
    _available_algorithms: Vec<Algorithm>,
}

enum Algorithm {
    MergeSort,
    QuickSort,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Algorithm::MergeSort => write!(f, "Merge Sort"),
            Algorithm::QuickSort => write!(f, "Quick Sort"),
        }
    }
}

impl<'a> App<'a> {
    fn new(size: usize, range: u64) -> App<'a> {
        let mut app = App {
            data: vec![],
            keys: vec![
                // TODO: This is an almighty workaround the fact that I have no idea how to
                // generate this list dynamically e.g.
                // let keys = (0..range).map(|i| format!("B{}",i).as_ref()).collect();
                "B1", "B2", "B3", "B4", "B5", "B6", "B7", "B8", "B9", "B10", "B11", "B12", "B13",
                "B14", "B15", "B16", "B17", "B18", "B19", "B20", "B21", "B22", "B23", "B24", "B25",
            ],
            current: Algorithm::MergeSort,
            _available_algorithms: vec![Algorithm::MergeSort, Algorithm::QuickSort],
        };

        let mut rng = rand::thread_rng();
        app.data = (0..size)
            .map(|_| rng.sample(Uniform::new(0, range)))
            .collect();
        return app;
    }

    fn _set_algorithm(&mut self, algorithm: Algorithm) {
        self.current = algorithm;
    }

    // Rather than returning the final value each algorithm could return a list of steps
    // which would be the value at each point before then end.
    fn run_algorithm_tick(&mut self) {
        let data = self.data.as_mut_slice();
        let mut steps = vec![];
        match self.current {
            Algorithm::MergeSort => merge_sort(data, &mut steps),
            Algorithm::QuickSort => quick_sort(data),
        };
        self.data = data.to_vec();
    }

    fn on_tick(&mut self) {
        self.run_algorithm_tick();
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(90)].as_ref())
        .split(f.size());

    let data: Vec<(&str, u64)> = app
        .data
        .iter()
        .map(|&i| (app.keys[i as usize], i))
        .collect();

    let barchart = BarChart::default()
        .block(
            Block::default()
                .title(app.current.to_string())
                .borders(Borders::ALL),
        )
        .bar_width(3)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::Gray))
        .value_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .label_style(Style::default().fg(Color::White))
        .data(data.as_slice());

    f.render_widget(barchart, chunks[0]);
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let app = App::new(20, 20);
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
