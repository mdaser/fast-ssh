//
// Copyright (C) 2025 by Martin Daser
//

use layout::create_layout;
use lazy_static::lazy_static;
use std::process::Command;

mod app;
mod config;
mod database;
mod input_handler;
mod layout;
mod searcher;
mod ssh_config_store;
mod term;
mod theme;
mod widgets;

use app::*;
use config::*;
use input_handler::*;
use term::*;
use theme::*;
use widgets::{
    config_widget::ConfigWidget, groups_widget::GroupsWidget, help_widget::HelpWidget,
    hosts_widget::HostsWidget, shortcuts_widget::ShortcutsWidget, state_widget::StateWidget,
    title_widget::TitleWidget,
};

lazy_static! {
    pub static ref CONFIG: Config = resolve_config();
    pub static ref THEME: &'static Theme = &CONFIG.theme;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = match App::new().await {
        Ok(app) => app,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut terminal = init_terminal()?;

    app.host_state.select(Some(0));

    'fastssh: loop {
        terminal.draw(|frame| {
            let layout = create_layout(&app, frame);

            TitleWidget::render(&mut app, layout.title[0], frame);

            match app.state() {
                AppState::Searching => app.searcher.render(&app, layout.chunks_top[0], frame),
                _ => GroupsWidget::render(&app, layout.chunks_top[0], frame),
            };

            HelpWidget::render(&mut app, layout.chunks_top[2], frame);
            HostsWidget::render(&mut app, layout.chunks_main[0], frame);
            ConfigWidget::render(&mut app, layout.chunks_main[2], frame);

            StateWidget::render(&mut app, layout.chunks_bot[0], frame);

            if app.show_help() {
                ShortcutsWidget::render(&app, layout.chunks_main[4], frame);
            }
        })?;

        handle_inputs(&mut app)?;

        match app.state() {
            AppState::Normal => app.set_state_info(String::from(">> ")),
            AppState::Searching => {
                app.set_state_info(String::from("Search Mode ... Press ESC to cancel."))
            }
            AppState::Ping(host_name) => {
                // ping host, show result, and set state to normal
                app.set_state_info(format!(
                    "64 bytes from {host_name} (10.157.181.172): icmp_seq=1 ttl=53 time=70.0 ms"
                ));
                app.set_state(AppState::Normal);
            }

            AppState::SpawnSsh => {
                app.set_state_info(format!("Connect to ...").clone());
                break 'fastssh;
            }
            AppState::Quit => {
                app.set_state_info(String::from("Terminating."));
                break 'fastssh;
            }
        }
    }

    restore_terminal(&mut terminal)?;

    if *app.state() == AppState::SpawnSsh {
        let selected_config = app.get_selected_item().unwrap();
        let host_name = &selected_config.full_name;

        app.db.save_host_values(
            host_name,
            selected_config.connection_count + 1,
            chrono::offset::Local::now().timestamp(),
        )?;

        Command::new("ssh")
            .arg(host_name.split(' ').take(1).collect::<Vec<&str>>().join(""))
            .spawn()?
            .wait()?;
    }

    Ok(())
}
