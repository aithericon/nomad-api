#![allow(clippy::tabs_in_doc_comments)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
#[allow(clippy::tabs_in_doc_comments)]
/// Create mostly with https://transform.tools/json-to-rust-serde

#[derive(Debug, Eq, PartialEq, Hash, Deserialize, Serialize, Clone)]
pub enum RunningStatus {
    #[serde(alias = "queued")]
    Queued,
    #[serde(alias = "complete")]
    Complete,
    #[serde(alias = "failed")]
    Failed,
    #[serde(alias = "running")]
    Running,
    #[serde(alias = "starting")]
    Starting,
    #[serde(alias = "lost")]
    Lost,
    #[serde(alias = "pending")]
    Pending,
    #[serde(alias = "dead")]
    Dead,
    #[serde(alias = "")]
    Missing,
}

impl Default for RunningStatus {
    fn default() -> Self {
        RunningStatus::Pending
    }
}

/// https://www.nomadproject.io/api-docs/jobs#dispatch-job
/// Method	Path                    Produces
/// POST	/v1/job/:job_id/dispatch	application/json
///
/// Parameters
/// :job_id (string: <required>) - Specifies the id of the job (as specified in the job file during submission). This is specified as part of the path.
///
/// Payload (string: "") - Specifies a base64 encoded string containing the payload. This is limited to 16384 bytes (16KiB).
///
/// Meta (meta<string|string>: nil) - Specifies arbitrary metadata to pass to the job.
#[derive(Debug, Deserialize, Serialize)]
pub struct DispatchJobRequest {
    #[serde(rename = "Payload")]
    pub payload: String,
    #[serde(rename = "Meta")]
    pub meta: HashMap<String, String>,
}

/// Sample Response:
/// ```
/// {
///   "Index": 13,
///   "JobCreateIndex": 12,
///   "EvalCreateIndex": 13,
///   "eval_id": "e5f55fac-bc69-119d-528a-1fc7ade5e02c",
///   "DispatchedJobID": "example/dispatch-1485408778-81644024"
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct DispatchJobResponse {
    #[serde(rename = "Index")]
    pub index: u32,
    #[serde(rename = "JobCreateIndex")]
    pub job_create_index: Option<u32>,
    #[serde(rename = "EvalCreateIndex")]
    pub eval_create_index: Option<u32>,
    #[serde(rename = "eval_id")]
    pub eval_id: Option<String>,
    #[serde(rename = "DispatchedJobID")]
    pub dispatched_job_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobStopResponse {
    #[serde(rename = "EvalID")]
    pub eval_id: String,
    pub eval_create_index: u32,
    pub job_modify_index: u32,
}

/// https://www.nomadproject.io/api-docs/jobs#list-job-allocations
/// Method	Path                        Produces
/// GET    /v1/job/:job_id/allocations    application/json
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListJobAllocationsResponse {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "EvalID")]
    pub eval_id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "NodeID")]
    pub node_id: String,
    #[serde(rename = "NodeName")]
    pub node_name: String,
    #[serde(rename = "JobID")]
    pub job_id: String,
    #[serde(rename = "JobType")]
    pub job_type: String,
    #[serde(rename = "JobVersion")]
    pub job_version: i64,
    #[serde(rename = "TaskGroup")]
    pub task_group: String,
    #[serde(rename = "AllocatedResources")]
    pub allocated_resources: ::serde_json::Value,
    #[serde(rename = "DesiredStatus")]
    pub desired_status: String,
    #[serde(rename = "DesiredDescription")]
    pub desired_description: String,
    #[serde(rename = "ClientStatus")]
    pub client_status: String,
    #[serde(rename = "ClientDescription")]
    pub client_description: Option<String>,
    #[serde(rename = "DesiredTransition")]
    pub desired_transition: DesiredTransition,
    #[serde(rename = "TaskStates")]
    pub task_states: Option<HashMap<String, TaskState>>,
    #[serde(rename = "DeploymentStatus")]
    pub deployment_status: Option<::serde_json::Value>,
    #[serde(rename = "FollowupEvalID")]
    pub followup_eval_id: Option<String>,
    #[serde(rename = "RescheduleTracker")]
    pub reschedule_tracker: ::serde_json::Value,
    #[serde(rename = "PreemptedAllocations")]
    pub preempted_allocations: ::serde_json::Value,
    #[serde(rename = "PreemptedByAllocation")]
    pub preempted_by_allocation: String,
    #[serde(rename = "CreateIndex")]
    pub create_index: i64,
    #[serde(rename = "ModifyIndex")]
    pub modify_index: i64,
    #[serde(rename = "CreateTime")]
    pub create_time: i64,
    #[serde(rename = "ModifyTime")]
    pub modify_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesiredTransition {
    #[serde(rename = "Migrate")]
    pub migrate: Option<::serde_json::Value>,
    #[serde(rename = "Reschedule")]
    pub reschedule: Option<::serde_json::Value>,
    #[serde(rename = "ForceReschedule")]
    pub force_reschedule: Option<::serde_json::Value>,
}
/*
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct TaskState {
    State: RunningStatus,
    Failed: bool,
    StartedAt: String,
    FinishedAt: String,
    Events: Vec<Event>,
}
*/
/*
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    Type: String,
    Time: u32,
    FailsTask: bool,
    RestartReason: String,
    SetupError : String,
    DriverError: String,
    ExitCode: i32,
    Signal: i32,
    Message: String,
    KillTimeout: u32,
    KillError: String,
    KillReason: String,
    StartDelay: u32,
    DownloadError: String,
    ValidationError: String,
    DiskLimit: u64,
    FailedSibling: String,
    VaultError: String,
    TaskSignalReason: String,
    TaskSignal: String,
    DriverMessage: String,
}
*/

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Allocation {
    #[serde(rename = "AllocModifyIndex")]
    pub alloc_modify_index: i64,
    #[serde(rename = "AllocatedResources")]
    pub allocated_resources: AllocatedResources,
    #[serde(rename = "ClientDescription")]
    pub client_description: Option<String>,
    #[serde(rename = "ClientStatus")]
    pub client_status: RunningStatus,
    #[serde(rename = "CreateIndex")]
    pub create_index: i64,
    #[serde(rename = "CreateTime")]
    pub create_time: i64,
    #[serde(rename = "DeploymentID")]
    pub deployment_id: Option<String>,
    #[serde(rename = "DeploymentStatus")]
    pub deployment_status: Option<DeploymentStatus>,
    #[serde(rename = "DesiredStatus")]
    pub desired_status: String,
    #[serde(rename = "EvalID")]
    pub eval_id: Option<String>,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Job")]
    pub job: Job,
    #[serde(rename = "JobID")]
    pub job_id: String,
    #[serde(rename = "Metrics")]
    pub metrics: Metrics,
    #[serde(rename = "ModifyIndex")]
    pub modify_index: i64,
    #[serde(rename = "ModifyTime")]
    pub modify_time: i64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "NetworkStatus")]
    pub network_status: Option<NetworkStatus>,
    #[serde(rename = "NodeID")]
    pub node_id: String,
    #[serde(rename = "NodeName")]
    pub node_name: String,
    #[serde(rename = "Resources")]
    pub resources: Resources,
    #[serde(rename = "SharedResources")]
    pub shared_resources: Resources,
    #[serde(rename = "TaskGroup")]
    pub task_group: String,
    #[serde(rename = "TaskResources")]
    #[serde(default)]
    pub task_resources: HashMap<String, Resources>,
    #[serde(rename = "TaskStates")]
    #[serde(default)]
    pub task_states: HashMap<String, TaskState>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocatedResources {
    #[serde(rename = "Shared")]
    pub shared: Shared,
    #[serde(rename = "TaskLifecycles")]
    #[serde(default)]
    pub task_lifecycles: HashMap<String, serde_json::Value>,
    #[serde(rename = "Tasks")]
    #[serde(default)]
    pub tasks: HashMap<String, SharedResources>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shared {
    #[serde(rename = "DiskMB")]
    pub disk_mb: Option<i64>,
    #[serde(rename = "Networks")]
    #[serde(default)]
    pub networks: Vec<Network>,
    #[serde(rename = "Ports")]
    #[serde(default)]
    pub ports: Vec<Port>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    #[serde(rename = "CIDR")]
    pub cidr: String,
    #[serde(rename = "DNS")]
    pub dns: Option<String>,
    #[serde(rename = "Device")]
    pub device: String,
    #[serde(rename = "DynamicPorts")]
    #[serde(default)]
    pub dynamic_ports: Option<Vec<DynamicPort>>,
    #[serde(rename = "IP")]
    pub ip: String,
    #[serde(rename = "MBits")]
    pub mbits: Option<i64>,
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "ReservedPorts")]
    #[serde(default)]
    pub reserved_ports: Option<Vec<DynamicPort>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicPort {
    #[serde(rename = "HostNetwork")]
    pub host_network: String,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "To")]
    pub to: i64,
    #[serde(rename = "Value")]
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    #[serde(rename = "HostIP")]
    pub host_ip: String,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "To")]
    pub to: i64,
    #[serde(rename = "Value")]
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cpu {
    #[serde(rename = "CpuShares")]
    pub cpu_shares: i64,
    #[serde(rename = "ReservedCores")]
    pub reserved_cores: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicePlugin {
    #[serde(rename = "DeviceIDs")]
    #[serde(default)]
    pub device_ids: Option<Vec<String>>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Vendor")]
    pub vendor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    #[serde(rename = "MemoryMB")]
    pub memory_mb: i64,
    #[serde(rename = "MemoryMaxMB")]
    pub memory_max_mb: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStatus {
    #[serde(rename = "Canary")]
    pub canary: bool,
    #[serde(rename = "Healthy")]
    pub healthy: bool,
    #[serde(rename = "ModifyIndex")]
    pub modify_index: i64,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    #[serde(rename = "Affinities")]
    pub affinities: Option<::serde_json::Value>,
    #[serde(rename = "AllAtOnce")]
    pub all_at_once: Option<bool>,
    #[serde(rename = "Constraints")]
    pub constraints: Option<::serde_json::Value>,
    #[serde(rename = "ConsulNamespace")]
    pub consul_namespace: Option<String>,
    #[serde(rename = "ConsulToken")]
    pub consul_token: Option<String>,
    #[serde(rename = "CreateIndex")]
    pub create_index: i64,
    #[serde(rename = "Datacenters")]
    #[serde(default)]
    pub datacenters: Option<Vec<String>>,
    #[serde(rename = "Dispatched")]
    pub dispatched: Option<bool>,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "JobModifyIndex")]
    pub job_modify_index: i64,
    #[serde(rename = "Meta")]
    pub meta: Option<::serde_json::Value>,
    #[serde(rename = "ModifyIndex")]
    pub modify_index: i64,
    #[serde(rename = "Multiregion")]
    pub multiregion: Option<::serde_json::Value>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "NomadTokenID")]
    pub nomad_token_id: Option<String>,
    #[serde(rename = "ParameterizedJob")]
    pub parameterized_job: Option<::serde_json::Value>,
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
    #[serde(rename = "Payload")]
    pub payload: Option<::serde_json::Value>,
    #[serde(rename = "Periodic")]
    pub periodic: Option<::serde_json::Value>,
    #[serde(rename = "Priority")]
    pub priority: i64,
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "Spreads")]
    pub spreads: Option<::serde_json::Value>,
    #[serde(rename = "Stable")]
    pub stable: Option<bool>,
    #[serde(rename = "Status")]
    pub status: RunningStatus,
    #[serde(rename = "StatusDescription")]
    pub status_description: Option<String>,
    #[serde(rename = "Stop")]
    pub stop: bool,
    #[serde(rename = "SubmitTime")]
    pub submit_time: Option<i64>,
    #[serde(rename = "TaskGroups")]
    pub task_groups: Option<Vec<TaskGroup>>,
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Update")]
    pub update: Option<Update>,
    #[serde(rename = "VaultNamespace")]
    pub vault_namespace: Option<String>,
    #[serde(rename = "VaultToken")]
    pub vault_token: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateJobRequest {
    pub job: Job,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobResponse {
    #[serde(rename = "EvalID")]
    pub eval_id: String,
    #[serde(rename = "EvalCreateIndex")]
    pub eval_create_index: i64,
    #[serde(rename = "JobModifyIndex")]
    pub job_modify_index: i64,
    #[serde(rename = "Warnings")]
    pub warnings: String,
    #[serde(rename = "Index")]
    pub index: i64,
    #[serde(rename = "LastContact")]
    pub last_contact: i64,
    #[serde(rename = "KnownLeader")]
    pub known_leader: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskGroup {
    #[serde(rename = "Affinities")]
    pub affinities: Option<::serde_json::Value>,
    #[serde(rename = "Constraints")]
    pub constraints: Option<::serde_json::Value>,
    #[serde(rename = "Consul")]
    pub consul: Option<Consul>,
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "EphemeralDisk")]
    pub ephemeral_disk: Option<EphemeralDisk>,
    #[serde(rename = "Meta")]
    pub meta: Option<::serde_json::Value>,
    #[serde(rename = "Migrate")]
    pub migrate: Option<Migrate>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Networks")]
    #[serde(default)]
    pub networks: Option<Vec<Network>>,
    #[serde(rename = "ReschedulePolicy")]
    pub reschedule_policy: Option<ReschedulePolicy>,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: Option<RestartPolicy>,
    #[serde(rename = "Scaling")]
    pub scaling: Option<::serde_json::Value>,
    #[serde(rename = "Services")]
    pub services: Option<::serde_json::Value>,
    #[serde(rename = "ShutdownDelay")]
    pub shutdown_delay: Option<::serde_json::Value>,
    #[serde(rename = "Spreads")]
    pub spreads: Option<::serde_json::Value>,
    #[serde(rename = "StopAfterClientDisconnect")]
    pub stop_after_client_disconnect: Option<::serde_json::Value>,
    #[serde(rename = "Tasks")]
    #[serde(default)]
    pub tasks: Option<Vec<Task>>,
    #[serde(rename = "Update")]
    pub update: Option<Update>,
    #[serde(rename = "Volumes")]
    pub volumes: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consul {
    #[serde(rename = "Namespace")]
    pub namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EphemeralDisk {
    #[serde(rename = "Migrate")]
    pub migrate: bool,
    #[serde(rename = "SizeMB")]
    pub size_mb: i64,
    #[serde(rename = "Sticky")]
    pub sticky: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Migrate {
    #[serde(rename = "HealthCheck")]
    pub health_check: String,
    #[serde(rename = "HealthyDeadline")]
    pub healthy_deadline: i64,
    #[serde(rename = "MaxParallel")]
    pub max_parallel: i64,
    #[serde(rename = "MinHealthyTime")]
    pub min_healthy_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReschedulePolicy {
    #[serde(rename = "Attempts")]
    pub attempts: i64,
    #[serde(rename = "Delay")]
    pub delay: i64,
    #[serde(rename = "DelayFunction")]
    pub delay_function: Option<String>,
    #[serde(rename = "Interval")]
    pub interval: i64,
    #[serde(rename = "MaxDelay")]
    pub max_delay: i64,
    #[serde(rename = "Unlimited")]
    pub unlimited: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestartPolicy {
    #[serde(rename = "Attempts")]
    pub attempts: i64,
    #[serde(rename = "Delay")]
    pub delay: i64,
    #[serde(rename = "Interval")]
    pub interval: i64,
    #[serde(rename = "Mode")]
    pub mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    #[serde(rename = "Affinities")]
    pub affinities: Option<::serde_json::Value>,
    #[serde(rename = "Artifacts")]
    pub artifacts: Option<::serde_json::Value>,
    #[serde(rename = "CSIPluginConfig")]
    pub csiplugin_config: Option<::serde_json::Value>,
    #[serde(rename = "Config")]
    pub config: Config,
    #[serde(rename = "Constraints")]
    pub constraints: Option<::serde_json::Value>,
    #[serde(rename = "DispatchPayload")]
    pub dispatch_payload: Option<::serde_json::Value>,
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "Env")]
    pub env: Option<HashMap<String, String>>,
    #[serde(rename = "KillSignal")]
    pub kill_signal: String,
    #[serde(rename = "KillTimeout")]
    pub kill_timeout: i64,
    #[serde(rename = "Kind")]
    pub kind: String,
    #[serde(rename = "Leader")]
    pub leader: bool,
    #[serde(rename = "Lifecycle")]
    pub lifecycle: Option<::serde_json::Value>,
    #[serde(rename = "LogConfig")]
    pub log_config: LogConfig,
    #[serde(rename = "Meta")]
    pub meta: Option<::serde_json::Value>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Resources")]
    pub resources: Resources,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: RestartPolicy,
    #[serde(rename = "ScalingPolicies")]
    pub scaling_policies: Option<::serde_json::Value>,
    #[serde(rename = "Services")]
    pub services: Option<::serde_json::Value>,
    #[serde(rename = "ShutdownDelay")]
    pub shutdown_delay: i64,
    #[serde(rename = "Templates")]
    pub templates: Option<::serde_json::Value>,
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "Vault")]
    pub vault: Option<::serde_json::Value>,
    #[serde(rename = "VolumeMounts")]
    pub volume_mounts: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub image: String,
    #[serde(rename = "network_mode")]
    pub network_mode: Option<String>,
    #[serde(default)]
    pub ports: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogConfig {
    #[serde(rename = "MaxFileSizeMB")]
    pub max_file_size_mb: i64,
    #[serde(rename = "MaxFiles")]
    pub max_files: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[serde(rename = "Affinities")]
    pub affinities: Option<::serde_json::Value>,
    #[serde(rename = "Constraints")]
    pub constraints: Option<::serde_json::Value>,
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedDevice {
    #[serde(rename = "DeviceIDs")]
    pub device_ids: Vec<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub device_type: String,
    #[serde(rename = "Vendor")]
    pub vendor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    #[serde(rename = "AutoPromote")]
    pub auto_promote: bool,
    #[serde(rename = "AutoRevert")]
    pub auto_revert: bool,
    #[serde(rename = "Canary")]
    pub canary: i64,
    #[serde(rename = "HealthCheck")]
    pub health_check: String,
    #[serde(rename = "HealthyDeadline")]
    pub healthy_deadline: i64,
    #[serde(rename = "MaxParallel")]
    pub max_parallel: i64,
    #[serde(rename = "MinHealthyTime")]
    pub min_healthy_time: i64,
    #[serde(rename = "ProgressDeadline")]
    pub progress_deadline: i64,
    #[serde(rename = "Stagger")]
    pub stagger: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metrics {
    #[serde(rename = "AllocationTime")]
    pub allocation_time: i64,
    #[serde(rename = "ClassExhausted")]
    pub class_exhausted: Option<::serde_json::Value>,
    #[serde(rename = "ClassFiltered")]
    pub class_filtered: Option<::serde_json::Value>,
    #[serde(rename = "CoalescedFailures")]
    pub coalesced_failures: i64,
    #[serde(rename = "ConstraintFiltered")]
    pub constraint_filtered: Option<::serde_json::Value>,
    #[serde(rename = "DimensionExhausted")]
    pub dimension_exhausted: Option<DimensionExhausted>,
    #[serde(rename = "NodesAvailable")]
    pub nodes_available: Option<NodesAvailable>,
    #[serde(rename = "NodesEvaluated")]
    pub nodes_evaluated: i64,
    #[serde(rename = "NodesExhausted")]
    pub nodes_exhausted: i64,
    #[serde(rename = "NodesFiltered")]
    pub nodes_filtered: i64,
    #[serde(rename = "QuotaExhausted")]
    pub quota_exhausted: Option<::serde_json::Value>,
    #[serde(rename = "ResourcesExhausted")]
    pub resources_exhausted: Option<::serde_json::Value>,
    #[serde(rename = "ScoreMetaData")]
    #[serde(default)]
    pub score_meta_data: Option<Vec<ScoreMetaDaum>>,
    #[serde(rename = "Scores")]
    pub scores: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionExhausted {
    pub disk: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodesAvailable {
    pub dc1: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreMetaDaum {
    #[serde(rename = "NodeID")]
    pub node_id: String,
    #[serde(rename = "NormScore")]
    pub norm_score: f64,
    #[serde(rename = "Scores")]
    pub scores: Scores,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scores {
    pub binpack: f64,
    #[serde(rename = "job-anti-affinity")]
    pub job_anti_affinity: f32,
    #[serde(rename = "node-affinity")]
    pub node_affinity: f32,
    #[serde(rename = "node-reschedule-penalty")]
    pub node_reschedule_penalty: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkStatus {
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "DNS")]
    pub dns: Option<::serde_json::Value>,
    #[serde(rename = "InterfaceName")]
    pub interface_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    #[serde(rename = "CPU")]
    pub cpu: i64,
    #[serde(rename = "Cores")]
    pub cores: i64,
    #[serde(rename = "Devices")]
    #[serde(default)]
    pub devices: Option<Vec<Device>>,
    #[serde(rename = "DiskMB")]
    pub disk_mb: Option<i64>,
    #[serde(rename = "IOPS")]
    pub iops: Option<i64>,
    #[serde(rename = "MemoryMB")]
    pub memory_mb: i64,
    #[serde(rename = "MemoryMaxMB")]
    pub memory_max_mb: Option<i64>,
    #[serde(rename = "Networks")]
    #[serde(default)]
    pub networks: Option<Vec<Network>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedResources {
    #[serde(rename = "Cpu")]
    pub cpu: Cpu,
    #[serde(rename = "Devices")]
    #[serde(default)]
    pub devices: Option<Vec<SharedDevice>>,
    #[serde(rename = "DiskMB")]
    pub disk_mb: Option<i64>,
    #[serde(rename = "IOPS")]
    pub iops: Option<i64>,
    #[serde(rename = "Memory")]
    pub memory: Option<Memory>,
    #[serde(rename = "Networks")]
    #[serde(default)]
    pub networks: Option<Vec<Network>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskState {
    #[serde(rename = "Events")]
    #[serde(default)]
    pub events: Option<Vec<Event>>,
    #[serde(rename = "Failed")]
    pub failed: Option<bool>,
    #[serde(rename = "FinishedAt")]
    pub finished_at: Option<::serde_json::Value>,
    #[serde(rename = "LastRestart")]
    pub last_restart: Option<String>,
    #[serde(rename = "Restarts")]
    pub restarts: i64,
    #[serde(rename = "StartedAt")]
    pub started_at: Option<String>,
    #[serde(rename = "State")]
    pub state: RunningStatus,
    #[serde(rename = "TaskHandle")]
    pub task_handle: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "Details")]
    pub details: Details,
    #[serde(rename = "DiskLimit")]
    pub disk_limit: i64,
    #[serde(rename = "DisplayMessage")]
    pub display_message: String,
    #[serde(rename = "DownloadError")]
    pub download_error: String,
    #[serde(rename = "DriverError")]
    pub driver_error: String,
    #[serde(rename = "DriverMessage")]
    pub driver_message: String,
    #[serde(rename = "ExitCode")]
    pub exit_code: i64,
    #[serde(rename = "FailedSibling")]
    pub failed_sibling: String,
    #[serde(rename = "FailsTask")]
    pub fails_task: bool,
    #[serde(rename = "GenericSource")]
    pub generic_source: String,
    #[serde(rename = "KillError")]
    pub kill_error: String,
    #[serde(rename = "KillReason")]
    pub kill_reason: String,
    #[serde(rename = "KillTimeout")]
    pub kill_timeout: i64,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "RestartReason")]
    pub restart_reason: String,
    #[serde(rename = "SetupError")]
    pub setup_error: String,
    #[serde(rename = "Signal")]
    pub signal: i64,
    #[serde(rename = "StartDelay")]
    pub start_delay: i64,
    #[serde(rename = "TaskSignal")]
    pub task_signal: String,
    #[serde(rename = "TaskSignalReason")]
    pub task_signal_reason: String,
    #[serde(rename = "Time")]
    pub time: i64,
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "ValidationError")]
    pub validation_error: String,
    #[serde(rename = "VaultError")]
    pub vault_error: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    pub message: Option<String>,
    pub image: Option<String>,
    #[serde(rename = "restart_reason")]
    pub restart_reason: Option<String>,
    #[serde(rename = "exit_code")]
    pub exit_code: Option<String>,
    #[serde(rename = "exit_message")]
    pub exit_message: Option<String>,
    #[serde(rename = "oom_killed")]
    pub oom_killed: Option<String>,
    pub signal: Option<String>,
    #[serde(rename = "start_delay")]
    pub start_delay: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseJobPayload {
    #[serde(rename = "JobHCL")]
    pub job_hcl: String,
    pub canonicalize: bool,
}
