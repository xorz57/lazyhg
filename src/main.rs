use std::io::{self, stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};

mod commands;
mod events;
mod ui;

use commands::run_command;
use events::handle_events;
use ui::{draw_ui, ActiveFrame};

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
