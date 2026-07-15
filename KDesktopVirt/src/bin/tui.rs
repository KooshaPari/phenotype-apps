/*!
 * KVirtualStage Terminal User Interface (TUI)
 * 
 * Interactive terminal interface providing:
 * - Real-time session monitoring
 * - Live automation control
 * - Performance metrics visualization
 * - Session management
 * - Recording controls
 * - Workflow execution
 */

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use kvirtualstage::{KVirtualStageAPI, APISessionInfo};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{
        Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph,
        Tabs, Table, Row, Cell,
    },
};
use std::{
    collections::HashMap,
    io,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tracing::{error, info};

#[derive(Debug, Clone, PartialEq)]
enum AppMode {
    Sessions,
    Automation,
    Recording,
    Metrics,
    Help,
}

#[derive(Debug)]
struct App {
    api: Arc<KVirtualStageAPI>,
    mode: AppMode,
    sessions: Vec<APISessionInfo>,
    selected_session: Option<usize>,
    session_list_state: ListState,
    last_update: Instant,
    status_message: String,
    metrics: AppMetrics,
    automation_commands: Vec<String>,
    should_quit: bool,
    popup_visible: bool,
    popup_content: String,
}

#[derive(Debug, Default)]
struct AppMetrics {
    active_sessions: usize,
    total_automation_commands: usize,
    average_response_time: f64,
    memory_usage: f64,
    cpu_usage: f64,
    uptime: Duration,
}

impl App {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let api = Arc::new(KVirtualStageAPI::new().await?);
        
        Ok(Self {
            api,
            mode: AppMode::Sessions,
            sessions: Vec::new(),
            selected_session: None,
            session_list_state: ListState::default(),
            last_update: Instant::now(),
            status_message: "KVirtualStage TUI started".to_string(),
            metrics: AppMetrics::default(),
            automation_commands: Vec::new(),
            should_quit: false,
            popup_visible: false,
            popup_content: String::new(),
        })
    }

    async fn update_sessions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.sessions = self.api.list_sessions().await?;
        self.metrics.active_sessions = self.sessions.len();
        Ok(())
    }

    fn next_session(&mut self) {
        if self.sessions.is_empty() {
            return;
        }
        
        let i = match self.session_list_state.selected() {
            Some(i) => {
                if i >= self.sessions.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.session_list_state.select(Some(i));
        self.selected_session = Some(i);
    }

    fn previous_session(&mut self) {
        if self.sessions.is_empty() {
            return;
        }
        
        let i = match self.session_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.sessions.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.session_list_state.select(Some(i));
        self.selected_session = Some(i);
    }

    fn get_selected_session(&self) -> Option<&APISessionInfo> {
        self.selected_session.and_then(|i| self.sessions.get(i))
    }

    async fn execute_automation_command(&mut self, command: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(session) = self.get_selected_session() {
            match command {
                "click" => {
                    self.api.click(&session.session_id, None).await?;
                    self.status_message = "Click executed".to_string();
                }
                "move_center" => {
                    self.api.move_cursor(&session.session_id, 400.0, 300.0).await?;
                    self.status_message = "Cursor moved to center".to_string();
                }
                "type_hello" => {
                    self.api.type_text(&session.session_id, "Hello from KVirtualStage TUI!").await?;
                    self.status_message = "Text typed".to_string();
                }
                _ => {
                    self.status_message = format!("Unknown command: {}", command);
                }
            }
            self.automation_commands.push(format!("{}: {}", 
                chrono::Utc::now().format("%H:%M:%S"), command));
            self.metrics.total_automation_commands += 1;
        } else {
            self.status_message = "No session selected".to_string();
        }
        Ok(())
    }

    fn show_popup(&mut self, content: String) {
        self.popup_content = content;
        self.popup_visible = true;
    }

    fn hide_popup(&mut self) {
        self.popup_visible = false;
        self.popup_content.clear();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    
    info!("Starting KVirtualStage TUI");

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new().await?;
    let start_time = Instant::now();

    // Main event loop
    loop {
        // Update metrics
        app.metrics.uptime = start_time.elapsed();
        
        // Refresh sessions periodically
        if app.last_update.elapsed() > Duration::from_secs(2) {
            if let Err(e) = app.update_sessions().await {
                app.status_message = format!("Failed to update sessions: {}", e);
            }
            app.last_update = Instant::now();
        }

        // Render
        terminal.draw(|f| ui(f, &mut app))?;

        // Handle events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            if app.popup_visible {
                                app.hide_popup();
                            } else {
                                app.should_quit = true;
                            }
                        }
                        KeyCode::Char('1') => app.mode = AppMode::Sessions,
                        KeyCode::Char('2') => app.mode = AppMode::Automation,
                        KeyCode::Char('3') => app.mode = AppMode::Recording,
                        KeyCode::Char('4') => app.mode = AppMode::Metrics,
                        KeyCode::Char('h') | KeyCode::Char('?') => app.mode = AppMode::Help,
                        KeyCode::Down | KeyCode::Char('j') => app.next_session(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous_session(),
                        KeyCode::Enter => {
                            if app.mode == AppMode::Sessions {
                                if let Some(session) = app.get_selected_session() {
                                    app.show_popup(format!(
                                        "Session Details\n\nID: {}\nUser: {}\nDesktop: {}\nStatus: {}\nRecording: {}",
                                        session.session_id,
                                        session.user_id,
                                        session.desktop_type,
                                        session.status,
                                        session.recording_active
                                    ));
                                }
                            }
                        }
                        KeyCode::Char('c') => {
                            if app.mode == AppMode::Automation {
                                if let Err(e) = app.execute_automation_command("click").await {
                                    app.status_message = format!("Command failed: {}", e);
                                }
                            }
                        }
                        KeyCode::Char('m') => {
                            if app.mode == AppMode::Automation {
                                if let Err(e) = app.execute_automation_command("move_center").await {
                                    app.status_message = format!("Command failed: {}", e);
                                }
                            }
                        }
                        KeyCode::Char('t') => {
                            if app.mode == AppMode::Automation {
                                if let Err(e) = app.execute_automation_command("type_hello").await {
                                    app.status_message = format!("Command failed: {}", e);
                                }
                            }
                        }
                        KeyCode::Char('r') => {
                            if app.mode == AppMode::Recording {
                                if let Some(session) = app.get_selected_session() {
                                    match app.api.start_recording(
                                        &session.session_id,
                                        &format!("tui_recording_{}.mp4", 
                                            chrono::Utc::now().format("%Y%m%d_%H%M%S")),
                                        Some("medium".to_string()),
                                    ).await {
                                        Ok(_) => app.status_message = "Recording started".to_string(),
                                        Err(e) => app.status_message = format!("Recording failed: {}", e),
                                    }
                                }
                            }
                        }
                        KeyCode::Char('s') => {
                            if app.mode == AppMode::Recording {
                                if let Some(session) = app.get_selected_session() {
                                    match app.api.stop_recording(&session.session_id).await {
                                        Ok(path) => app.status_message = format!("Recording saved: {}", path),
                                        Err(e) => app.status_message = format!("Stop failed: {}", e),
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            break;
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

    info!("KVirtualStage TUI stopped");
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(size);

    // Header
    let header = Paragraph::new("KVirtualStage TUI - Playwright-equivalent Desktop Automation")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        );
    f.render_widget(header, chunks[0]);

    // Content area
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(1)])
        .split(chunks[1]);

    // Tabs
    let tab_titles = ["1:Sessions", "2:Automation", "3:Recording", "4:Metrics", "H:Help"];
    let selected_tab = match app.mode {
        AppMode::Sessions => 0,
        AppMode::Automation => 1,
        AppMode::Recording => 2,
        AppMode::Metrics => 3,
        AppMode::Help => 4,
    };
    
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Navigation"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .select(selected_tab);
    f.render_widget(tabs, content_chunks[0]);

    // Main content
    match app.mode {
        AppMode::Sessions => render_sessions_tab(f, app, content_chunks[1]),
        AppMode::Automation => render_automation_tab(f, app, content_chunks[1]),
        AppMode::Recording => render_recording_tab(f, app, content_chunks[1]),
        AppMode::Metrics => render_metrics_tab(f, app, content_chunks[1]),
        AppMode::Help => render_help_tab(f, content_chunks[1]),
    }

    // Footer with status
    let footer_text = format!("Status: {} | Sessions: {} | Commands: {} | Press 'q' to quit",
        app.status_message,
        app.metrics.active_sessions,
        app.metrics.total_automation_commands
    );
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        );
    f.render_widget(footer, chunks[2]);

    // Popup
    if app.popup_visible {
        render_popup(f, &app.popup_content, size);
    }
}

fn render_sessions_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title("Active Sessions")
        .borders(Borders::ALL);

    if app.sessions.is_empty() {
        let empty_msg = Paragraph::new("No active sessions\n\nPress Enter to refresh")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(block);
        f.render_widget(empty_msg, area);
        return;
    }

    let items: Vec<ListItem> = app
        .sessions
        .iter()
        .map(|session| {
            let status_color = if session.recording_active {
                Color::Red
            } else {
                Color::Green
            };
            
            ListItem::new(format!(
                "{} | {} | {} | {}",
                session.session_id,
                session.user_id,
                session.desktop_type,
                if session.recording_active { "🔴 REC" } else { "⚪ IDLE" }
            )).style(Style::default().fg(status_color))
        })
        .collect();

    let sessions_list = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ");

    f.render_stateful_widget(sessions_list, area, &mut app.session_list_state);
}

fn render_automation_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(1)])
        .split(area);

    // Controls
    let controls = Paragraph::new(
        "Automation Controls:\n\n\
        [C] Click at current position\n\
        [M] Move cursor to center\n\
        [T] Type hello message"
    )
    .style(Style::default().fg(Color::White))
    .block(
        Block::default()
            .title("Controls")
            .borders(Borders::ALL),
    );
    f.render_widget(controls, chunks[0]);

    // Command history
    let commands: Vec<ListItem> = app
        .automation_commands
        .iter()
        .rev()
        .take(20)
        .map(|cmd| ListItem::new(cmd.as_str()))
        .collect();

    let command_list = List::new(commands)
        .block(
            Block::default()
                .title("Command History")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(command_list, chunks[1]);
}

fn render_recording_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(1)])
        .split(area);

    // Recording controls
    let controls = Paragraph::new(
        "Recording Controls:\n\n\
        [R] Start recording session\n\
        [S] Stop recording\n\n\
        Recording Quality: Medium\n\
        Output Format: MP4"
    )
    .style(Style::default().fg(Color::White))
    .block(
        Block::default()
            .title("Recording")
            .borders(Borders::ALL),
    );
    f.render_widget(controls, chunks[0]);

    // Recording status
    let recording_status = if let Some(session) = app.get_selected_session() {
        if session.recording_active {
            "🔴 RECORDING ACTIVE".to_string()
        } else {
            "⚪ RECORDING IDLE".to_string()
        }
    } else {
        "No session selected".to_string()
    };

    let status = Paragraph::new(format!("Status: {}", recording_status))
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .title("Current Status")
                .borders(Borders::ALL),
        );
    f.render_widget(status, chunks[1]);
}

fn render_metrics_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(area);

    // Active sessions gauge
    let sessions_gauge = Gauge::default()
        .block(Block::default().title("Active Sessions").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent((app.metrics.active_sessions as f64 / 10.0 * 100.0).min(100.0) as u16)
        .label(format!("{} sessions", app.metrics.active_sessions));
    f.render_widget(sessions_gauge, chunks[0]);

    // CPU usage gauge (simulated)
    let cpu_gauge = Gauge::default()
        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(app.metrics.cpu_usage as u16)
        .label(format!("{:.1}%", app.metrics.cpu_usage));
    f.render_widget(cpu_gauge, chunks[1]);

    // Memory usage gauge (simulated)
    let memory_gauge = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(app.metrics.memory_usage as u16)
        .label(format!("{:.1}%", app.metrics.memory_usage));
    f.render_widget(memory_gauge, chunks[2]);

    // System info
    let uptime_str = format!("{}h {}m {}s",
        app.metrics.uptime.as_secs() / 3600,
        (app.metrics.uptime.as_secs() % 3600) / 60,
        app.metrics.uptime.as_secs() % 60
    );
    
    let info = Paragraph::new(format!(
        "System Information:\n\n\
        Uptime: {}\n\
        Total Commands: {}\n\
        Average Response: {:.1}ms\n\
        Version: {}",
        uptime_str,
        app.metrics.total_automation_commands,
        app.metrics.average_response_time,
        env!("CARGO_PKG_VERSION")
    ))
    .style(Style::default().fg(Color::White))
    .block(
        Block::default()
            .title("System Info")
            .borders(Borders::ALL),
    );
    f.render_widget(info, chunks[3]);
}

fn render_help_tab(f: &mut Frame, area: Rect) {
    let help_text = "\
KVirtualStage TUI Help\n\n\
NAVIGATION:\n\
  1-4      Switch between tabs\n\
  h, F1    Show this help\n\
  ↑/↓, j/k Navigate lists\n\
  Enter    Select/View details\n\
  q, Esc   Quit/Close popup\n\n\
SESSIONS TAB:\n\
  View active desktop sessions\n\
  Enter to see session details\n\n\
AUTOMATION TAB:\n\
  c        Click at current position\n\
  m        Move cursor to center\n\
  t        Type hello message\n\n\
RECORDING TAB:\n\
  r        Start recording\n\
  s        Stop recording\n\n\
METRICS TAB:\n\
  View system performance metrics\n\
  Real-time resource monitoring";

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("Help & Commands")
                .borders(Borders::ALL),
        );
    f.render_widget(help, area);
}

fn render_popup(f: &mut Frame, content: &str, area: Rect) {
    let popup_area = centered_rect(60, 50, area);
    
    f.render_widget(Clear, popup_area);
    
    let popup = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .block(
            Block::default()
                .title("Details")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );
    f.render_widget(popup, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}