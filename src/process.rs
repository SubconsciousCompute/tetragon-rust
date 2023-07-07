use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Container {
    pub id: Option<String>,
    pub name: Option<String>,
    pub image: Option<Image>,
    pub start_time: Option<String>,
    pub pid: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pod {
    pub namespace: Option<String>,
    pub name: Option<String>,
    pub container: Option<Container>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Process {
    pub exec_id: Option<String>,
    pub pid: Option<u32>,
    pub uid: Option<u32>,
    pub cwd: Option<String>,
    pub binary: Option<String>,
    pub arguments: Option<String>,
    pub flags: Option<String>,
    pub start_time: Option<String>,
    pub auid: Option<u32>,
    pub pod: Option<Pod>,
    pub docker: Option<String>,
    pub parent_exec_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessExec {
    pub process: Option<Process>,
    pub parent: Option<Process>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessExit {
    pub process: Option<Process>,
    pub parent: Option<Process>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonResponse {
    pub process_exec: Option<ProcessExec>,
    pub process_exit: Option<ProcessExit>,
    pub node_name: Option<String>,
    pub time: Option<String>,
}
