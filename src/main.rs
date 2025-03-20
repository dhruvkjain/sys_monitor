use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, List, Paragraph},
};
use std::{io, thread, time::Duration};
use sysinfo::{Disks, Networks, System};

enum Event<I> {
    Input(I),
    Tick,
}

// struct App {
//     sys: System,
//     cpu_data: Vec<(f64, f64)>,
//     time: f64,
// }

// impl App {
//     fn new() -> App {
//         App {
//             sys: System::new_all(),
//             cpu_data: Vec::new(),
//             time: 0.0,
//         }
//     }

//     fn update(&mut self) {
//         self.sys.refresh_cpu_all();
//         let cpu_usage = self.sys.global_cpu_usage() as f64;
//         self.cpu_data.push((self.time, cpu_usage));
//         self.time += 1.0;

//         // Keep only the last 100 data points
//         if self.cpu_data.len() > 100 {
//             self.cpu_data.remove(0);
//         }
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout: io::Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create a channel to receive events
    let (tx, rx) = std::sync::mpsc::channel();
    let tick_rate = Duration::from_secs(1);
    thread::spawn(move || {
        let mut last_tick = std::time::Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = std::time::Instant::now();
            }
        }
    });

    // Initialize system struct
    let mut sys = System::new_all();
    // let mut app = App::new();

    loop {
        // Update system information
        sys.refresh_all();

        // Draw the user interface
        terminal.draw(|f| {
            let size = f.area();

            // Layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(size);

            // CPU Information
            let cpu = sys.global_cpu_usage() ;
            let cpu_text = vec![Line::from(format!("CPU Usage: {:.2}%", cpu))];
            let cpu_block =
                Paragraph::new(cpu_text).block(Block::default().borders(Borders::ALL).title("CPU"));
            f.render_widget(cpu_block, chunks[0]);

            // Memory Information
            let memory_text: Vec<Line<'_>> = vec![
                Line::from(format!("Total Memory: {:.2} GB", (sys.total_memory())as f64/(1073741824)as f64)),
                Line::from(format!("Used Memory: {:.2} GB", (sys.used_memory())as f64/(1073741824)as f64)),
                Line::from(format!("Total Swap: {:.2} GB", (sys.total_swap())as f64/(1073741824)as f64)),
                Line::from(format!("Used Swap: {:.2} GB", (sys.used_swap())as f64/(1073741824)as f64)),
            ];
            let memory_block = Paragraph::new(memory_text)
                .block(Block::default().borders(Borders::ALL).title("Memory"));
            f.render_widget(memory_block, chunks[1]);

            // Disk Information
            let disk_text: Vec<Line> = Disks::new_with_refreshed_list()
                .iter()
                .map(|disk| {
                    Line::from(format!(
                        "{}: {:.2}/{:.2} GB",
                        disk.name().to_string_lossy(),
                        disk.available_space()as f64/(1073741824)as f64,
                        disk.total_space()as f64/(1073741824)as f64
                    ))
                })
                .collect();
            let disk_block =
                List::new(disk_text).block(Block::default().borders(Borders::ALL).title("Disks"));
            f.render_widget(disk_block, chunks[2]);

            // Network Information
            let network_text: Vec<Line> = Networks::new_with_refreshed_list()
                .iter()
                .map(|(interface_name, data)| {
                    Line::from(format!(
                        "{}: received {} B, transmitted {} B",
                        interface_name,
                        data.received(),
                        data.transmitted()
                    ))
                })
                .collect();
            let network_block = List::new(network_text)
                .block(Block::default().borders(Borders::ALL).title("Networks"));
            f.render_widget(network_block, chunks[3]);

            // System Information
            let system_text = vec![
                Line::from(format!(
                    "System Name: {}",
                    System::name().unwrap_or_default()
                )),
                Line::from(format!(
                    "Kernel Version: {}",
                    System::kernel_version().unwrap_or_default()
                )),
                Line::from(format!(
                    "OS Version: {}",
                    System::os_version().unwrap_or_default()
                )),
                Line::from(format!(
                    "Host Name: {}",
                    System::host_name().unwrap_or_default()
                )),
            ];
            let system_block = Paragraph::new(system_text)
                .block(Block::default().borders(Borders::ALL).title("System"));
            f.render_widget(system_block, chunks[4]);
        })?;

        // Handle input
        match rx.recv()? {
            Event::Input(event) => {
                if event.code == KeyCode::Char('q') {
                    break;
                }
            }
            Event::Tick => {}
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
