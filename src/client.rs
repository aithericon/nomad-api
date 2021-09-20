use crate::jobs::{Job, ListJobAllocationsResponse, Allocation, DispatchJobRequest, DispatchJobResponse, JobStopResponse, ParseJobPayload, CreateJobResponse, CreateJobRequest};
use log::{debug, error, info, warn, trace};
use reqwest::Client;
use std::collections::HashMap;

#[derive(Clone)]
pub struct NomadClient {
    http_client: Client,
    base_url: String,
    authorization_token: String,
}

impl NomadClient {
    pub fn new(base_url: String, authorization_token: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            authorization_token,
        }
    }

    /// https://www.nomadproject.io/api-docs/jobs#list-jobs
    /// This endpoint lists all known jobs in the system registered with Nomad.
    /// Method      Path
    /// GET         /v1/jobs
    pub async fn list_jobs(&self) -> Result<Vec<Job>, reqwest::Error> {
        let url = format!("{}/v1/jobs", &self.base_url);
        trace!("ListJobs call to {}", &url);
        let response = self
            .http_client
            .get(&url)
            .header("X-Nomad-Token", &self.authorization_token)
            .send()
            .await?
            .json::<Vec<Job>>()
            .await?;
        Ok(response)
    }

    /// https://www.nomadproject.io/api-docs/jobs#stop-a-job
    /// This endpoint deregisters a job, and stops all allocations part of it.
    /// Method	    Path	            Produces
    /// DELETE	    /v1/job/:job_id	    application/json
    pub async fn stop_job(&self, job_id: &str) -> Result<JobStopResponse, reqwest::Error> {
        let url = format!("{}/v1/job/{}", &self.base_url, job_id);
        trace!("Stop job {} call to {}", &url, job_id);
        let response = self
            .http_client
            .delete(&url)
            .header("X-Nomad-Token", &self.authorization_token)
            .send()
            .await?
            .json::<JobStopResponse>()
            .await?;
        Ok(response)
    }


    /// https://www.nomadproject.io/api-docs/jobs#list-job-allocations
    /// Method	Path	                        Produces
    /// GET	    /v1/job/:job_id/allocations	    application/json
    pub async fn list_job_allocations(&self, id: &str) -> Result<Vec<ListJobAllocationsResponse>, reqwest::Error> {
        let url = format!("{}/v1/job/{}/allocations", &self.base_url, id);
        trace!("ListJobAllocations call to {}", &url);
        let response = self
            .http_client
            .get(&url)
            .header("X-Nomad-Token", &self.authorization_token)
            .send()
            .await?
            .json::<Vec<ListJobAllocationsResponse>>()
            .await?;
        Ok(response)
    }

    /// https://www.nomadproject.io/api-docs/jobs#read-allocation
    /// Method	Path	                        Produces
    /// GET	    /v1/allocations/:alloc_id	    application/json
    ///
    /// Parameters:
    /// :alloc_id (string: <required>)- Specifies the UUID of the allocation. This must be the full
    /// UUID, not the short 8-character one. This is specified as part of the path.
    pub async fn read_allocation(&self, id: &str) -> Result<Allocation, reqwest::Error> {
        let url = format!("{}/v1/allocation/{}", &self.base_url, id);
        trace!("ReadAllocation call to {}", &url);
        let response = self
            .http_client
            .get(&url)
            .header("X-Nomad-Token", &self.authorization_token)
            .send()
            .await?;
        // info!("ReadAllocation: {:?}", &response);
        let response = response.json::<Allocation>()
            .await?;
        Ok(response)
    }


    /// https://www.nomadproject.io/api-docs/jobs#dispatch-job
    /// Method	Path	                    Produces
    /// POST	/v1/job/:job_id/dispatch	application/json
    ///
    /// Parameters
    /// :job_id (string: <required>) - Specifies the id of the job (as specified in the job file during submission). This is specified as part of the path.
    ///
    /// Payload (string: "") - Specifies a base64 encoded string containing the payload. This is limited to 16384 bytes (16KiB).
    ///
    /// Meta (meta<string|string>: nil) - Specifies arbitrary metadata to pass to the job.
    pub async fn dispatch_job(&self, job: &str, payload: Option<String>, meta: HashMap<String, String>) -> Result<DispatchJobResponse, reqwest::Error> {
        let url = format!("{}/v1/job/{}/dispatch", &self.base_url, job);
        trace!("Dispatch job call to {}", &url);
        let request = DispatchJobRequest {
            payload: payload.unwrap_or_default(),
            meta,
        };
        let test = serde_json::to_string(&request).unwrap();
        info!("{}", test);
        // println!("{}", test);
        let response = self
            .http_client
            .post(&url)
            .header("X-Nomad-Token", &self.authorization_token)
            .json(&request)
            .send()
            .await?;
        // println!("{:?}", &response);
        let response = response.json::<DispatchJobResponse>()
           .await?;

        Ok(response)
    }

    /// Parse Job
    ///
    /// This endpoint will parse a HCL jobspec and produce the equivalent JSON encoded job.
    /// Method	Path	Produces
    /// POST	/v1/jobs/parse	application/json
    ///
    /// The table below shows this endpoint's support for blocking queries and required ACLs.
    /// Blocking Queries	ACL Required
    /// NO	none
    /// »Parameters
    ///
    ///     JobHCL (string: <required>) - Specifies the HCL definition of the job encoded in a JSON string.
    ///     Canonicalize (bool: false) - Flag to enable setting any unset fields to their default values.
    pub async fn parse_job(&self, hcl: &str, canonicalize: bool) -> Result<Job, reqwest::Error> {
        let url = format!("{}/v1/jobs/parse", &self.base_url);
        trace!("Parse job call to {}", &url);
        let request = ParseJobPayload {
            job_hcl: hcl.to_string(),
            canonicalize,
        };
        // let test = serde_json::to_string(&request).unwrap();
        // println!("{}", test);
        let response = self
            .http_client
            .post(&url)
            .header("X-Nomad-Token", &self.authorization_token)
            .json(&request)
            .send()
            .await?;
        debug!("Response: {:?}", &response);
        let response = response.json::<Job>()
            .await?;

        Ok(response)
    }

    /// Create Job
    ///
    /// This endpoint creates (aka "registers") a new job in the system.
    /// Method	Path	Produces
    /// POST	/v1/jobs	application/json
    ///
    /// The table below shows this endpoint's support for blocking queries and required ACLs.
    /// Blocking Queries	ACL Required
    /// NO	namespace:submit-job
    /// namespace:sentinel-override if PolicyOverride set
    /// »Parameters
    ///
    ///     Job (Job: <required>) - Specifies the JSON definition of the job.
    ///
    ///     EnforceIndex (bool: false) - If set, the job will only be registered if the passed JobModifyIndex matches the current job's index. If the index is zero, the register only occurs if the job is new. This paradigm allows check-and-set style job updating.
    ///
    ///     JobModifyIndex (int: 0) - Specifies the JobModifyIndex to enforce the current job is at.
    ///
    ///     PolicyOverride (bool: false) - If set, any soft mandatory Sentinel policies will be overridden. This allows a job to be registered when it would be denied by policy.
    ///
    ///     PreserveCounts (bool: false) - If set, existing task group counts are preserved, over those specified in the new job spec.
    pub async fn create_job(&self, job: &Job) -> Result<CreateJobResponse, reqwest::Error> {
        let url = format!("{}/v1/jobs", &self.base_url);
        trace!("Parse job call to {}", &url);
        let request = CreateJobRequest {
            job: job.clone(),
        };

        let test = serde_json::to_string(&request).unwrap();
        println!("{}", test);
        let response = self
            .http_client
            .post(&url)
            .header("X-Nomad-Token", &self.authorization_token)
            .json(&request)
            .send()
            .await?;
        info!("Response: {:?}", &response);
        println!("Response: {:?}", &response);
        let response = response.json::<CreateJobResponse>()
            .await?;

        Ok(response)

    }
}
