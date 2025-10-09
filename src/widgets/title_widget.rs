use crate::app::App;
use crate::THEME;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend, layout::Rect, style::Style, text::Spans, widgets::BorderType,
    widgets::Borders, widgets::Paragraph, Frame,
};

use super::block;

pub struct TitleWidget {}

impl TitleWidget {
    pub fn render(app: &App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let title = format!(" {} - {} ", app.app_name(), app.app_version());
        let block = block::new(title.as_str())
            .title_alignment(tui::layout::Alignment::Left)
            .border_type(BorderType::Thick)
            .borders(Borders::TOP);

        let empty_span = Spans::from("");

        let paragraph = Paragraph::new(empty_span)
            .block(block)
            .style(Style::default().fg(THEME.text_primary()))
            .alignment(tui::layout::Alignment::Left);

        frame.render_widget(paragraph, area);
    }
}
