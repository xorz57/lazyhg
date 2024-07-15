use std::io::{self, stdout};
use std::process::Command;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};

mod events;
mod ui;

use events::handle_events;
use ui::{draw_ui, ActiveFrame};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;

    let mut active_frame = ActiveFrame::Status;

    let hg_status_output = Command::new("hg")
        .args(&["status"])
        .output()
        .expect("Failed to execute command");

    let hg_branches_output = Command::new("hg")
        .args(&["branches"])
        .output()
        .expect("Failed to execute command");

    let hg_bookmarks_output = Command::new("hg")
        .args(&["bookmarks"])
        .output()
        .expect("Failed to execute command");

    let hg_log_output = Command::new("hg")
        .args(&["log"])
        .output()
        .expect("Failed to execute command");

    let hg_status_output = String::from_utf8_lossy(&hg_status_output.stdout).to_string();
    let hg_branches_output = String::from_utf8_lossy(&hg_branches_output.stdout).to_string();
    let hg_bookmarks_output = String::from_utf8_lossy(&hg_bookmarks_output.stdout).to_string();
    let hg_log_output = String::from_utf8_lossy(&hg_log_output.stdout).to_string();

    while !should_quit {
        terminal.draw(|f| {
            draw_ui(
                f,
                &active_frame,
                &hg_status_output,
                &hg_branches_output,
                &hg_bookmarks_output,
                &hg_log_output,
            )
        })?;
        should_quit = handle_events(&mut active_frame)?;
    }

    disable_raw_mode()?;

    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
