//
// Copyright (C) 2025, 2026 by Martin Daser
//

use crate::THEME;
use tui::{
    style::Style,
    text::Span,
    widgets::{Block, Borders},
};

pub fn new(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(THEME.border()))
        .title_alignment(tui::layout::Alignment::Center)
        .border_type(tui::widgets::BorderType::Rounded)
        .title(Span::styled(title, Style::default().fg(THEME.title_fg())))
}
