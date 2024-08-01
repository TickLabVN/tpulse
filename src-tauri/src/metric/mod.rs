mod collector;
mod processor;
mod categorizer;

use std::thread::{self, JoinHandle};
use crate::metrics::UserMetric;
use collector::{watch_afk, watch_browser, watch_window};
use std::sync::mpsc;
use processor::create_processor;

pub fn start_collector() -> Vec<JoinHandle<()>> {
    let (tx, rx) = mpsc::channel::<UserMetric>();
    let afk_tx = tx.clone();
    let window_tx = tx.clone();
    let browser_tx = tx.clone();

    let processor = create_processor();

    let workers = vec![
        thread::spawn(move || watch_browser(browser_tx)),
        thread::spawn(move || watch_afk(afk_tx)),
        thread::spawn(move || watch_window(window_tx)),
        thread::spawn(move || {
            while let Ok(user_metric) = rx.recv() {
                processor.process(&user_metric);
            }
        }),
    ];

    workers
}
