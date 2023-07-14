use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tcp {
    pub process_kprobe: Option<ProcessKprobe>,
    pub time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessKprobe {
    pub process: Option<Parent>,
    pub parent: Option<Parent>,
    pub function_name: Option<String>,
    pub args: Option<Vec<Arg>>,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arg {
    pub sock_arg: Option<SockArg>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SockArg {
    pub family: Option<String>,
    #[serde(rename = "type")]
    pub sock_arg_type: Option<String>,
    pub protocol: Option<String>,
    pub saddr: Option<String>,
    pub daddr: Option<String>,
    pub sport: Option<i64>,
    pub dport: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parent {
    pub exec_id: Option<String>,
    pub pid: Option<i64>,
    pub uid: Option<i64>,
    pub cwd: Option<String>,
    pub binary: Option<String>,
    pub arguments: Option<String>,
    pub flags: Option<String>,
    pub start_time: Option<String>,
    pub auid: Option<i64>,
    pub parent_exec_id: Option<String>,
    pub refcnt: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TetraNetwork {
    Tcp(Tcp),
}
