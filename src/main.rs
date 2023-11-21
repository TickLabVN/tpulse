use tpulse::afk::{AFKWatcher, Settings};
use dotenv::dotenv;


fn main() {
    dotenv().ok();
    env_logger::init();

    let afk_settings = Settings::new(10, 1);
    let watcher = AFKWatcher::new(&afk_settings);
    watcher.run();
}
