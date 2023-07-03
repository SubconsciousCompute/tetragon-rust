use logwatcher::{LogWatcher, LogWatcherAction};

fn main() {
    let mut log_watcher =
        LogWatcher::register("/var/log/tetragon/tetragon.log".to_string()).unwrap();

    log_watcher.watch(&mut move |line: String| {
        println!("Line {}", line);
        LogWatcherAction::None
    });
}
