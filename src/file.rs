use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAccess {
    pub process_kprobe: Option<ProcessKprobe>,
    pub node_name: Option<String>,
    pub time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessKprobe {
    pub process: Option<Parent>,
    pub parent: Option<Parent>,
    pub function_name: Option<String>,
    pub args: Option<Vec<Arg>>,
    #[serde(rename = "return")]
    pub process_kprobe_return: Option<Return>,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arg {
    pub file_arg: Option<FileArg>,
    pub int_arg: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileArg {
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parent {
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
    pub tid: Option<i64>,
    pub arguments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pod {
    pub namespace: Option<String>,
    pub name: Option<String>,
    pub container: Option<Container>,
    pub pod_labels: Option<PodLabels>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodLabels {
    #[serde(rename = "app.kubernetes.io/name")]
    pub app_kubernetes_io_name: Option<String>,
    pub class: Option<String>,
    pub org: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Return {
    pub int_arg: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TetraFile {
    File(FileAccess),
}
