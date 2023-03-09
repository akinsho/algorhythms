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

struct App {
    data: Vec<u64>,
    _available_algorithms: Vec<Algorithm>,
    current: Algorithm,
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

fn __merge(left: Vec<u64>, right: Vec<u64>) -> Vec<u64> {
    let mut result = Vec::new();
    let mut l = 0;
    let mut r = 0;
    while left[l] <= right[r] {
        if left[l] < right[r] {
            result.push(left[l]);
            l += 1;
        } else {
            result.push(right[r]);
            r += 1;
        }
    }
    while l < left.len() {
        result.push(left[l]);
        l += 1;
    }
    while r < right.len() {
        result.push(right[r]);
        r += 1;
    }
    return result;
}

/// step 1: start
///
/// step 2: declare array and left, right, mid variable
///
/// step 3: perform merge function.
/// ```
///     if left > right
///         return
///     mid=(left+right)/2
///     mergesort(array, left, mid)
///     mergesort(array, mid+1, right)
///     merge(array, left, mid, right)
///```
/// step 4: Stop
fn merge_sort(data: Vec<u64>) -> Vec<u64> {
    let left_index = 0;
    let right_index = data.len() - 1;
    if data[left_index] > data[right_index] {
        return data;
    }
    let mid = (left_index + right_index) / 2;
    let left = data[left_index..mid].to_vec();
    let right = data[mid + 1..right_index].to_vec();
    let next_left = merge_sort(left);
    let next_right = merge_sort(right);
    return __merge(next_left, next_right);
}

fn quick_sort(_data: Vec<u64>) -> Vec<u64> {
    todo!()
}

impl App {
    fn new(size: usize, range: u64) -> App {
        let mut rng = rand::thread_rng();
        let data = (0..size)
            .map(|i| {
                let s = format!("B{}", i);
                (s, rng.sample(Uniform::new(0, range)))
            })
            .collect();
        return App {
            data,
            current: Algorithm::MergeSort,
            _available_algorithms: vec![Algorithm::MergeSort, Algorithm::QuickSort],
        };
    }

    fn _set_algorithm(&mut self, algorithm: Algorithm) {
        self.current = algorithm;
    }

    fn run_algorithm_tick(&mut self) -> Vec<u64> {
        let data = self.data.clone();
        return match self.current {
            Algorithm::MergeSort => merge_sort(data),
            Algorithm::QuickSort => quick_sort(data),
        };
    }

    fn on_tick(&mut self) {
        let next_data = self.run_algorithm_tick();
        self.data = next_data;
    }

    fn get_data<'a>(&'a self) -> Vec<(&'a str, &'a u64)> {
        let d: Vec<_> = self
            .data
            .iter()
            .map(|i| {
                let str = format!("B{}", i).as_str();
                (str, i)
            })
            .collect();
        return d;
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(90)].as_ref())
        .split(f.size());

    let data = app.get_data();

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
