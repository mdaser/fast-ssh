//
// Copyright (C) 2025, 2026 by Martin Daser
//

use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::App;

pub struct AppLayout {
    pub title: Vec<Rect>,
    pub chunks_top: Vec<Rect>,
    pub chunks_main: Vec<Rect>,
    pub chunks_bot: Vec<Rect>,
}

pub fn create_layout(app: &App, frame: &mut Frame<CrosstermBackend<Stdout>>) -> AppLayout {
    let base_chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .horizontal_margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(if app.show_help() { 5 } else { 3 }),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let title = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(base_chunk[0]);

    let chunks_top = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Length(1),
                Constraint::Length(10),
            ]
            .as_ref(),
        )
        .split(base_chunk[1]);

    let chunks_main = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .horizontal_margin(0)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Length(1),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(base_chunk[2]);

    let chunks_bot = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .horizontal_margin(0)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(base_chunk[3]);

    AppLayout {
        title,
        chunks_top,
        chunks_main,
        chunks_bot,
    }
}
