use std::sync::mpsc::Receiver;

use crate::{
    events::UserMetric,
    sqlite::{insert_afk_log, insert_window_log},
};

pub fn handle_events(rx: Receiver<UserMetric>) {
    loop {
        let metric = rx.recv().unwrap();
        match metric {
            UserMetric::AFK(afk_event) => {
                insert_afk_log(&afk_event);
            }
            UserMetric::Window(window_event) => {
                insert_window_log(&window_event);
            }
        }
    }
}
