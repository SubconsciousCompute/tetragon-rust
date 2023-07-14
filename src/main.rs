use logwatcher::{LogWatcher, LogWatcherAction, LogWatcherEvent};
use tetragon::network::TetraNetwork;
use tetragon::TetraEvent;

fn main() {
    let mut log_watcher = LogWatcher::register("/var/log/tetragon/tetragon.log").unwrap();

    log_watcher.watch(&mut move |result| {
        match result {
            Ok(event) => match event {
                LogWatcherEvent::Line(line) => {
                    //println!("\n\n{}\n\n", line);
                    let response: TetraEvent = serde_json::from_str(&line).unwrap();
                    match response {
                        TetraEvent::Process(_) => {}
                        TetraEvent::Network(k) => {
                            println!(
                                "network {}",
                                match k {
                                    TetraNetwork::Tcp(k) => {
                                        k.process_kprobe.unwrap().function_name.unwrap()
                                    }
                                }
                            )
                        }
                    }
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
