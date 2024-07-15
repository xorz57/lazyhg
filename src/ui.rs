use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::*,
    Frame,
};

pub enum ActiveFrame {
    Status,
    Branches,
    Bookmarks,
    Log,
}

pub fn draw_ui(
    frame: &mut Frame,
    active_frame: &ActiveFrame,
    hg_status_output: &str,
    hg_branches_output: &str,
    hg_bookmarks_output: &str,
    hg_log_output: &str,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
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
