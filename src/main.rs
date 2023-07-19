use logwatcher::{LogWatcher, LogWatcherAction, LogWatcherEvent};
use tetragon::file::TetraFile;
use tetragon::network::TetraNetwork;
use tetragon::process::TetraProcess;
use tetragon::TetraEvent;

fn main() {
    let mut log_watcher = LogWatcher::register("/var/log/tetragon/tetragon.log").unwrap();

    println!("Tetragon logs...\n");

    log_watcher.watch(&mut move |result| {
        match result {
            Ok(event) => match event {
                LogWatcherEvent::Line(line) => {
                    //println!("{}", &line);
                    let response: TetraEvent = serde_json::from_str(&line).unwrap();
                    match response {
                        TetraEvent::Process(p) => match p {
                            TetraProcess::Start(s) => {
                                println!(
                                    "Process Start : {}",
                                    s.process_exec.process.unwrap().binary.unwrap()
                                );
                            }
                            TetraProcess::End(e) => {
                                println!(
                                    "Process End : {}",
                                    e.process_exit.process.unwrap().binary.unwrap()
                                );
                            }
                        },
                        TetraEvent::Network(n) => match n {
                            TetraNetwork::Tcp(t) => {
                                println!(
                                    "Tcp : {}",
                                    t.process_kprobe.unwrap().function_name.unwrap()
                                );
                            }
                        },
                        TetraEvent::File(f) => match f {
                            TetraFile::File(f) => {
                                println!("File : {:?}", f.process_kprobe.unwrap().args.unwrap());
                            }
                        },
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
