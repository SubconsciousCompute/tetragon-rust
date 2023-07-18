use crate::file::TetraFile;
use crate::network::TetraNetwork;
use crate::process::TetraProcess;
use serde::{Deserialize, Serialize};

pub mod file;
pub mod network;
pub mod process;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TetraEvent {
    Process(TetraProcess),
    Network(TetraNetwork),
    File(TetraFile),
}
