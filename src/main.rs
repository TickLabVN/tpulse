use tpulse::afk::{AFKWatcher, Settings};
use dotenv::dotenv;
// qwqeqweqweqweqweqweqweeqeqweqweqeqweqweqwwwwwwwwww
fn main() {
    dotenv().ok();
    env_logger::init();

    let afk_settings = Settings::new(10, 4);
    let watcher = AFKWatcher::new(&afk_settings);
    watcher.run();
}
