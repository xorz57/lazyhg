use std::io::{self, stdout};
use std::process::Command;

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    let mut output = String::new();

    while !should_quit {
        terminal.draw(|f| ui(f, &output))?;
        should_quit = handle_events(&mut output)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(output: &mut String) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('s') => {
                        let cmd_output = Command::new("hg")
                            .arg("status")
                            .output()
                            .expect("Failed to execute hg status");

                        *output = String::from_utf8_lossy(&cmd_output.stdout).to_string();
                    }
                    KeyCode::Char('l') => {
                        let cmd_output = Command::new("hg")
                            .arg("log")
                            .output()
                            .expect("Failed to execute hg log");

                        *output = String::from_utf8_lossy(&cmd_output.stdout).to_string();
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, output: &str) {
    let block = Block::bordered().title("Output");
    let paragraph = Paragraph::new(output).block(block);

    frame.render_widget(paragraph, frame.size());
}
