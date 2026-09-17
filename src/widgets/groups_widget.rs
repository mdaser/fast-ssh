//
// Copyright (C) 2025, 2026 by Martin Daser
//

use super::block;
use crate::App;
use std::io::Stdout;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::Tabs;
use tui::{backend::CrosstermBackend, Frame};

pub struct GroupsWidget {}

impl GroupsWidget {
    pub fn render(app: &App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let theme = &app.config.theme;

        let block = block::new(" Groups ")
            .title_alignment(tui::layout::Alignment::Left)
            .style(Style::default().fg(theme.title_fg()));
        let titles = app
            .scs
            .groups
            .iter()
            .map(|t| {
                Spans::from(Span::styled(
                    t.name.to_string(),
                    Style::default().fg(theme.select_fg()),
                ))
            })
            .collect();

        let tabs = Tabs::new(titles)
            .block(block)
            .select(app.selected_group)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(theme.select_active())
                    .bg(theme.select_bg()),
            );

        frame.render_widget(tabs, area);
    }
}
