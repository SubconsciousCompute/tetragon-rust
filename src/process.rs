use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Start {
    pub process_exec: Pe,
    pub node_name: Option<String>,
    pub time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct End {
    pub process_exit: Pe,
    pub node_name: Option<String>,
    pub time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pe {
    pub process: Option<Process>,
    pub parent: Option<Process>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    pub exec_id: Option<String>,
    pub pid: Option<i64>,
    pub uid: Option<i64>,
    pub cwd: Option<String>,
    pub binary: Option<String>,
    pub flags: Option<String>,
    pub start_time: Option<String>,
    pub auid: Option<i64>,
    pub pod: Option<Pod>,
    pub docker: Option<String>,
    pub parent_exec_id: Option<String>,
    pub refcnt: Option<i64>,
    pub arguments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pod {
    pub namespace: Option<String>,
    pub name: Option<String>,
    pub container: Option<Container>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub id: Option<String>,
    pub name: Option<String>,
    pub image: Option<Image>,
    pub start_time: Option<String>,
    pub pid: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TetraProcess {
    Start(Start),
    End(End),
}
