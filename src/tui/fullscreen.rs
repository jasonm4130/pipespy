use ratatui::Frame;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::stats::StatsSnapshot;
use crate::tui::App;

pub fn render(frame: &mut Frame, snap: &StatsSnapshot, samples: &[String], app: &App) {
    let block = Block::default().title(" pipeview fullscreen (WIP) ").borders(Borders::ALL);
    let text = format!(
        "Lines: {} | Bytes: {} | Elapsed: {:.1}s\n\nPress 'f' to return to compact mode",
        snap.total_lines, snap.total_bytes, snap.elapsed_secs
    );
    frame.render_widget(Paragraph::new(text).block(block), frame.area());
}
