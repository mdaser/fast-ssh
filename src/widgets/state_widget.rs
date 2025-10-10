use crate::app::App;
use crate::THEME;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend, layout::Rect, style::Style, text::Spans, widgets::Paragraph, Frame,
};

use super::block;

pub struct StateWidget {}

impl StateWidget {
    pub fn render(app: &App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let block = block::new(" State ").title_alignment(tui::layout::Alignment::Left);

        let empty_span = Spans::from(format!(" {}", app.state_info()));

        let paragraph = Paragraph::new(empty_span)
            .block(block)
            .style(Style::default().fg(THEME.text_primary()))
            .alignment(tui::layout::Alignment::Left);

        frame.render_widget(paragraph, area);
    }
}