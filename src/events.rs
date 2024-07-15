use ratatui::crossterm::event::{self, Event, KeyCode};
use std::io;

use crate::ui::ActiveFrame;

pub fn handle_events(active_frame: &mut ActiveFrame) -> io::Result<bool> {
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
