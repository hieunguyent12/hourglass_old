use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self},
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, widgets::TableState, Terminal};

mod action;
mod ui;

use action::Action;

enum View {
    Task(Action),
    Issues,
}

#[derive(Clone)]
struct Task {
    id: i32,
    description: String,
    completed: bool,
    age: Instant,
}

pub struct Hourglass {
    should_quit: bool,
    input: String,

    view: View,

    table_state: TableState,

    tasks: Vec<Task>,
}

impl Hourglass {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            input: String::new(),
            view: View::Task(Action::View),
            tasks: vec![
                Task {
                    id: 1,
                    description: String::from("Take out the trash"),
                    completed: false,
                    age: Instant::now(),
                },
                Task {
                    id: 2,
                    description: String::from("Do the dishes"),
                    completed: false,
                    age: Instant::now(),
                },
                Task {
                    id: 3,
                    description: String::from("Do Laundry"),
                    completed: false,
                    age: Instant::now(),
                },
            ],
            table_state: TableState::default(),
        }
    }

    pub fn start_tui() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(terminal)
    }

    pub fn pause_tui() -> io::Result<()> {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        terminal.show_cursor()?;
        Ok(())
    }

    pub fn run<B: tui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        // println!("Test: {:?}", self.items);

        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(250);
        // how is rust able to run an infinite loop without crashing?

        loop {
            terminal.draw(|f| {
                ui::build_ui(f, self);
            })?;

            // wtf is the point of this?
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            // the poll method will halt the loop to wait a certain amount of time (based on timeout) for an event to occur before moving on
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key);
                }
            }

            if self.should_quit {
                return Ok(());
            }

            // why?
            // without this line, the program will consume very high CPU
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.tasks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.table_state.select(Some(i))
    }
    fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tasks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn toggle_task_status(&mut self) {
        if let Some(i) = self.table_state.selected() {
            if let Some(task) = self.tasks.get_mut(i) {
                task.completed = !task.completed;
            }
        }
    }

    fn add_task(&mut self) {
        let description = self.input.clone();

        self.input = String::new();

        self.tasks.push(Task {
            id: (self.tasks.len() + 1) as i32,
            description,
            completed: false,
            age: Instant::now(),
        })
    }

    fn update_task(&mut self) {
        if let Some(i) = self.table_state.selected() {
            if let Some(task) = self.tasks.get_mut(i) {
                task.description = self.input.clone();
            }
        }

        self.input = String::new();
    }

    fn remove_task(&mut self) {
        if let Some(index) = self.table_state.selected() {
            self.tasks.remove(index);
        }
    }

    fn handle_input(&mut self, key_event: KeyEvent) {
        // we handle input differently based on the current view
        match &self.view {
            View::Task(action) => match action {
                Action::View => self.handle_key_for_task_view(key_event.code),
                _ => self.update_command_input(key_event.code),
            },

            View::Issues => {}
        }
    }

    fn update_command_input(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::Enter => match &self.view {
                View::Task(action) => match action {
                    Action::Add => {
                        self.add_task();

                        self.view = View::Task(Action::View)
                    }
                    Action::Update => {
                        self.update_task();

                        self.view = View::Task(Action::View)
                    }
                    _ => {}
                },

                View::Issues => {}
            },
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Esc => {
                self.input = String::new();
                self.view = View::Task(Action::View);
            }
            _ => {}
        }
    }

    fn handle_key_for_task_view(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char(c) => match c {
                'q' => self.should_quit = true,
                'j' => self.next(),
                'k' => self.previous(),
                'd' => self.toggle_task_status(),
                'a' => self.view = View::Task(Action::Add),
                'u' => self.view = View::Task(Action::Update),
                'x' => self.remove_task(),
                _ => {}
            },
            KeyCode::Down => self.next(),
            KeyCode::Up => self.previous(),
            _ => {}
        }
    }
}
