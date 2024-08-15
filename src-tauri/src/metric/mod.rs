mod categorizer;
mod collector;
mod schema;

use categorizer::create_processor;
use collector::{watch_afk, watch_browser, watch_window};
use schema::Activity;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

pub fn start_collector() -> Vec<JoinHandle<()>> {
    let (tx, rx) = mpsc::channel::<Activity>();
    let afk_tx = tx.clone();
    let window_tx = tx.clone();
    let browser_tx = tx.clone();

    let mut processor = create_processor();

    let workers = vec![
        thread::spawn(move || watch_browser(browser_tx)),
        thread::spawn(move || watch_afk(afk_tx)),
        thread::spawn(move || watch_window(window_tx)),
        thread::spawn(move || {
            while let Ok(mut user_metric) = rx.recv() {
                processor.categorize(&mut user_metric);
            }
        }),
    ];
    workers
}
