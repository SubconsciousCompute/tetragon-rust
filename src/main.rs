use logwatcher::{LogWatcher, LogWatcherAction, LogWatcherEvent};
use tetragon::process::TetraProcess;

fn main() {
    let mut log_watcher = LogWatcher::register("/var/log/tetragon/tetragon.log").unwrap();

    log_watcher.watch(&mut move |result| {
        match result {
            Ok(event) => match event {
                LogWatcherEvent::Line(line) => {
                    println!("{}", line);
                    let response: TetraProcess = serde_json::from_str(&line).unwrap();
                    println!("{:#?}\n", response);
                }
                LogWatcherEvent::LogRotation => {
                    println!("Logfile rotation");
                }
            },
            Err(err) => {
                println!("Error {}", err);
            }
        }
        LogWatcherAction::None
    });
}
