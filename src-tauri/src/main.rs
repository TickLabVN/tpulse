// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate unix_named_pipe;
extern crate ctrlc;

use dotenv::dotenv;
use std::{thread, fs, env};
use std::path::Path;
use tpulse::watcher::{AFKSettings, AFKWatcher};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use unix_named_pipe::FileFIFOExt;
use std::io::{self, Read, BufRead};




fn main() {
    initialize_db();

    let afk_settings = AFKSettings::new(5000, 500);
    let afk_watcher = AFKWatcher::new(&afk_settings);
    // let afk_watch = thread::spawn(move || afk_watcher.run());
    let open_server_pipe = thread::spawn(move || {
        let pipe_path = "/home/tan17112003/Desktop/myunixnamedpipe";
        let running = make_loop_flag();
        let file = try_open(&pipe_path).expect("could not open pipe for reading");
        let mut reader = io::BufReader::new(file);
        while running.load(Ordering::SeqCst) {
            let mut payload= String::new();
            let res = reader.read_line(&mut payload);
            if let Err(err) = res {
                match err.kind() {
                io::ErrorKind::WouldBlock => continue,
                _ => panic!("{}", format!("error while reading from pipe: {:?}", err)),
            }
            } else if let Ok(count) = res {
                if count != payload.len() {
                // If there is no data yet, just `continue` and try again.
                    continue;
                } else {
                    let result: String = payload;
                    println!("got data from client: {}", result);
                }
        }
        }
        fs::remove_file(&pipe_path).expect("could not remove pipe during shutdown");
    });
    let afk_watch = thread::spawn(move || watch_afk(5000, 50000));
    let window_watch = thread::spawn(move || tpulse::watcher::watch_window(1000));

    tauri::Builder::default()
        // We cannot see log when running in bundled app.
        // This is a workaround to print log to stdout in production.
        // Can use other log targets
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout])
                .build(),
        )
        // This plugin support us access sqlite database directly from Frontend-side
        .plugin(tauri_plugin_sql::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
    open_server_pipe.join().unwrap();
    afk_watch.join().unwrap();
    window_watch.join().unwrap();
}
fn make_loop_flag() -> Arc<AtomicBool> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("keyboard interrupted: stopping read loop");
        r.store(false, Ordering::SeqCst);
    })
    .expect("could not set up keyboard interrupt handler");

    return running;
}
fn try_open<P: AsRef<Path> + Clone>(pipe_path: P) -> io::Result<fs::File> {
    let pipe = unix_named_pipe::open_read(&pipe_path);
    if let Err(err) = pipe {
        match err.kind() {
            io::ErrorKind::NotFound => {
                println!("creating pipe at: {:?}", pipe_path.clone().as_ref());
                unix_named_pipe::create(&pipe_path, Some(0o660))?;
                return try_open(pipe_path);
            }
            _ => {
                return Err(err);
            }
        }
    }

    let pipe_file = pipe.unwrap();
    let is_fifo = pipe_file
        .is_fifo()
        .expect("could not read type of file at pipe path");
    if !is_fifo {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "expected file at {:?} to be fifo, is actually {:?}",
                &pipe_path.clone().as_ref(),
                pipe_file.metadata()?.file_type(),
            ),
        ));
    }
    Ok(pipe_file)
}
