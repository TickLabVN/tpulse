// import logging
// import os
// import platform
// from datetime import datetime, timedelta, timezone
// from time import sleep

// from aw_client import ActivityWatchClient
// from aw_core.models import Event

// from .config import load_config

// system = platform.system()

// if system == "Windows":
//     # noreorder
//     from .windows import seconds_since_last_input  # fmt: skip
// elif system == "Darwin":
//     # noreorder
//     from .macos import seconds_since_last_input  # fmt: skip
// elif system == "Linux":
//     # noreorder
//     from .unix import seconds_since_last_input  # fmt: skip
// else:
//     raise Exception(f"Unsupported platform: {system}")

// logger = logging.getLogger(__name__)
// td1ms = timedelta(milliseconds=1)
use log::info;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone)]
pub struct Settings {
    timeout: u64,
    poll_time: u64,
}
impl Settings {
    pub fn new(timeout: u64, poll_time: u64) -> Self {
        assert!(
            timeout >= poll_time,
            "Timeout should be greater than or equal to poll time"
        );

        Settings { timeout, poll_time }
    }
}

pub struct AFKWatcher {
    settings: Settings,
    bucketname: String,
}
impl AFKWatcher {
    pub fn new(settings: &Settings) -> Self {
        AFKWatcher {
            settings: settings.clone(),
            bucketname: "AFKWatcher".to_string(), // TODO: Make this dynamic
        }
    }
    pub fn run(&self) {
        info!("aw-watcher-afk started");
        info!("bucket name: {}", self.bucketname);
        info!("timeout {}", self.settings.timeout);
        info!("poll time {}", self.settings.poll_time);

        self.watch();
    }
    fn watch(&self) {
        loop {
            info!("watching");
            sleep(Duration::from_secs(self.settings.poll_time));
        }
    }
}

// class AFKWatcher:
//     def __init__(self, args, testing=False):
//         # Read settings from config
//         self.settings = Settings(
//             load_config(testing), timeout=args.timeout, poll_time=args.poll_time
//         )

//         self.client = ActivityWatchClient(
//             "aw-watcher-afk", host=args.host, port=args.port, testing=testing
//         )
//         self.bucketname = "{}_{}".format(
//             self.client.client_name, self.client.client_hostname
//         )

//     def ping(self, afk: bool, timestamp: datetime, duration: float = 0):
//         data = {"status": "afk" if afk else "not-afk"}
//         e = Event(timestamp=timestamp, duration=duration, data=data)
//         pulsetime = self.settings.timeout + self.settings.poll_time
//         self.client.heartbeat(self.bucketname, e, pulsetime=pulsetime, queued=True)

//     def run(self):
//         logger.info("aw-watcher-afk started")

//         # Initialization
//         sleep(1)

//         eventtype = "afkstatus"
//         self.client.create_bucket(self.bucketname, eventtype, queued=True)

//         # Start afk checking loop
//         with self.client:
//             self.heartbeat_loop()

//     def heartbeat_loop(self):
//         afk = False
//         while True:
//             try:
//                 if system in ["Darwin", "Linux"] and os.getppid() == 1:
//                     # TODO: This won't work with PyInstaller which starts a bootloader process which will become the parent.
//                     #       There is a solution however.
//                     #       See: https://github.com/ActivityWatch/aw-qt/issues/19#issuecomment-316741125
//                     logger.info("afkwatcher stopped because parent process died")
//                     break

//                 now = datetime.now(timezone.utc)
//                 seconds_since_input = seconds_since_last_input()
//                 last_input = now - timedelta(seconds=seconds_since_input)
//                 logger.debug(f"Seconds since last input: {seconds_since_input}")

//                 # If no longer AFK
//                 if afk and seconds_since_input < self.settings.timeout:
//                     logger.info("No longer AFK")
//                     self.ping(afk, timestamp=last_input)
//                     afk = False
//                     # ping with timestamp+1ms with the next event (to ensure the latest event gets retrieved by get_event)
//                     self.ping(afk, timestamp=last_input + td1ms)
//                 # If becomes AFK
//                 elif not afk and seconds_since_input >= self.settings.timeout:
//                     logger.info("Became AFK")
//                     self.ping(afk, timestamp=last_input)
//                     afk = True
//                     # ping with timestamp+1ms with the next event (to ensure the latest event gets retrieved by get_event)
//                     self.ping(
//                         afk, timestamp=last_input + td1ms, duration=seconds_since_input
//                     )
//                 # Send a heartbeat if no state change was made
//                 else:
//                     if afk:
//                         # we need the +1ms here too, to make sure we don't "miss" the last heartbeat
//                         # (if last_input hasn't changed)
//                         self.ping(
//                             afk,
//                             timestamp=last_input + td1ms,
//                             duration=seconds_since_input,
//                         )
//                     else:
//                         self.ping(afk, timestamp=last_input)

//                 sleep(self.settings.poll_time)

//             except KeyboardInterrupt:
//                 logger.info("aw-watcher-afk stopped by keyboard interrupt")
//                 break

//     # Start watcher
//     watcher = AFKWatcher(args, testing=args.testing)
//     watcher.run()

// if __name__ == "__main__":
//     main()
