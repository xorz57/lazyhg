use std::io::{self, stdout};
use std::process::Command;

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::*,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    let hg_status_output = run_command("hg", &["status"]);
    let hg_log_output = run_command("hg", &["log"]);
    let hg_branches_output = run_command("hg", &["branches"]);

    while !should_quit {
        terminal.draw(|f| ui(f, &hg_status_output, &hg_log_output, &hg_branches_output))?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn run_command(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                if let KeyCode::Char('q') = key.code {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, hg_status_output: &str, hg_log_output: &str, hg_branches_output: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Percentage(20),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let status_block = Block::bordered().title("Status");
    let log_block = Block::bordered().title("Log");
    let branches_block = Block::bordered().title("Branches");

    let status_paragraph = Paragraph::new(hg_status_output).block(status_block);
    let log_paragraph = Paragraph::new(hg_log_output).block(log_block);
    let branches_paragraph = Paragraph::new(hg_branches_output).block(branches_block);

    frame.render_widget(status_paragraph, chunks[0]);
    frame.render_widget(log_paragraph, chunks[1]);
    frame.render_widget(branches_paragraph, chunks[2]);
}
