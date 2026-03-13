//
// Copyright (C) 2025, 2026 by Martin Daser
//

use crate::{App, THEME};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Style,
    text::Spans,
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::block;

pub struct ShortcutsWidget {}

impl ShortcutsWidget {
    pub fn render(_app: &App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let block = block::new(" Help ").title_alignment(tui::layout::Alignment::Left);

        let text = vec![
            Spans::from("'Enter': Validate        'Left/Right/Tab': Change Group   'p'| 'P' : Ping / raw (exp.)"),
            Spans::from("'c': Toggle Config View  'PageUp/Down': Scroll Config"),
            Spans::from("'s'|'/': Search Mode     'Esc' Exit Search Mode           'q': Exit"),
        ];

        let paragraph = Paragraph::new(text)
            .alignment(tui::layout::Alignment::Left)
            .block(block)
            .style(Style::default().fg(THEME.text_secondary()))
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}
