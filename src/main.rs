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

enum ActiveFrame {
    Status,
    Branches,
    Bookmarks,
    Log,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    let mut active_frame = ActiveFrame::Status;
    let hg_status_output = run_command("hg", &["status"]);
    let hg_branches_output = run_command("hg", &["branches"]);
    let hg_bookmarks_output = run_command("hg", &["bookmarks"]);
    let hg_log_output = run_command("hg", &["log"]);

    while !should_quit {
        terminal.draw(|f| {
            ui(
                f,
                &hg_status_output,
                &hg_branches_output,
                &hg_bookmarks_output,
                &hg_log_output,
                &active_frame,
            )
        })?;
        should_quit = handle_events(&mut active_frame)?;
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

fn handle_events(active_frame: &mut ActiveFrame) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('s') => *active_frame = ActiveFrame::Status,
                    KeyCode::Char('b') => {
                        *active_frame = match *active_frame {
                            ActiveFrame::Branches => ActiveFrame::Bookmarks,
                            _ => ActiveFrame::Branches,
                        }
                    }
                    KeyCode::Char('l') => *active_frame = ActiveFrame::Log,
                    _ => {}
                }
            }
        }
    }
    Ok(false)
}

fn ui(
    frame: &mut Frame,
    hg_status_output: &str,
    hg_branches_output: &str,
    hg_bookmarks_output: &str,
    hg_log_output: &str,
    active_frame: &ActiveFrame,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(frame.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let status_block = match active_frame {
        ActiveFrame::Status => Block::bordered()
            .title("Status")
            .style(Style::default().fg(Color::Yellow)),
        _ => Block::bordered().title("Status"),
    };
    let branches_block = match active_frame {
        ActiveFrame::Branches => Block::bordered()
            .title("Branches")
            .style(Style::default().fg(Color::Yellow)),
        _ => Block::bordered().title("Branches"),
    };
    let bookmarks_block = match active_frame {
        ActiveFrame::Bookmarks => Block::bordered()
            .title("Bookmarks")
            .style(Style::default().fg(Color::Yellow)),
        _ => Block::bordered().title("Bookmarks"),
    };
    let log_block = match active_frame {
        ActiveFrame::Log => Block::bordered()
            .title("Log")
            .style(Style::default().fg(Color::Yellow)),
        _ => Block::bordered().title("Log"),
    };

    let status_paragraph = Paragraph::new(hg_status_output).block(status_block);
    let branches_paragraph = Paragraph::new(hg_branches_output).block(branches_block);
    let bookmarks_paragraph = Paragraph::new(hg_bookmarks_output).block(bookmarks_block);
    let log_paragraph = Paragraph::new(hg_log_output).block(log_block);

    frame.render_widget(status_paragraph, left_chunks[0]);
    frame.render_widget(branches_paragraph, left_chunks[1]);
    frame.render_widget(bookmarks_paragraph, left_chunks[2]);
    frame.render_widget(log_paragraph, chunks[1]);
}
