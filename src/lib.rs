use crate::process::TetraProcess;

pub mod process;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TetraEvent {
    Process(TetraProcess),
}
